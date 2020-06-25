const { ApiPromise, WsProvider } = require('@polkadot/api');
const defaultTypes = require('$/types.json');

module.exports = class Api {
    constructor(api) {
        this.api = api;
        this.txCount = 0;
        this.errorCount = 0;
    }

    static async connect(url) {
        const wsProvider = new WsProvider(url);
        const api = await ApiPromise.create({
            provider: wsProvider,
            types: defaultTypes,
        });
        return new Api(api);
    }

    async exec(module = '', extrinsic = '', sender, args = []) {
        return new Promise((resolve, reject) => {
            this.api.query.system.accountNonce(sender.address).then(async (nonce) => {
                const signed = this.api.tx[module][extrinsic](...args).sign(sender, { nonce });
                const unsub = await signed.send(({ status, events = [] }) => {
                    if (status.isInvalid || status.isDropped || status.isUsurped) {
                        this.errorCount++;
                        unsub();
                        reject(`Transaction failed`);
                    }
                    if (status.isFinalized) {
                        this.txCount++;
                        let error, result;
                        events.forEach(({ phase, event: { data, method, section } }) => {
                            const res = `${phase.toString()}: ${section}.${method} ${data.toString()}`;
                            if (res.indexOf('Failed') !== -1) {
                                error = res;
                            } else if (res.indexOf('treasury.ResourceSupply') !== -1) {
                                let tmp = data.toString().match(/\[(.*?)\]/);
                                if (tmp.length != 2) {
                                    throw Error('Invalid resource supply data');
                                }
                                result = tmp[1].split(',')[1];
                            }
                        });
                        if (error) {
                            this.errorCount++;
                            unsub();
                            reject(`Transaction failed with error: ${error}`);
                        } else {
                            console.log(`Transaction included at blockHash: ${status.asFinalized}`);
                            unsub();
                            resolve({ nonce: nonce.toString(), result });
                        }
                    }
                });
            });
        });
    }

    async execSudo(module = '', extrinsic = '', sender, args = []) {
        const nonce = await this.api.query.system.accountNonce(sender.address);
        const unsub = await this.api.tx.sudo.sudo(
            this.api.tx[module][extrinsic](...args),
        ).sign(sender, { nonce }).send(({ status, events = [] }) => {
            if (status.isFinalized) {
                this.txCount++;
                let error;
                events.forEach(({ phase, event: { data, method, section } }) => {
                    const res = `${phase.toString()}: ${section}.${method} ${data.toString()}`;
                    if (res.indexOf('Failed') !== -1) {
                        error = res;
                    }
                });
                if (error) {
                    console.error(`Transaction failed with error: ${error}`);
                    this.errorCount++;
                } else {
                    console.log(`Transaction included at blockHash: ${status.asFinalized}`);
                }
                unsub();
            }
        });
        return nonce.toString();
    }

    get types() {
        return defaultTypes;
    }

    get query() {
        return this.api.query;
    }

    get api_tx() {
        return this.api.tx;
    }
}
