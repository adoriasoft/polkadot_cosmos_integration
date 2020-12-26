const { ApiPromise, WsProvider } = require('@polkadot/api');
const appConstants = require('./app.constants');

async function init() {
    const WS_URL = process.env.NODE_ENV === 'production' ? appConstants.ENDPOINT_PROD : appConstants.ENDPOINT_LOCAL;
    const provider = new WsProvider(WS_URL);
    const api = await ApiPromise.create({ provider });
    const lastHeader = await api.rpc.chain.getHeader();

    console.log(lastHeader.number.toNumber());
}

init()
    .finally(() => process.exit(0));
