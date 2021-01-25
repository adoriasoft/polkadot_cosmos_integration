const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

const appConstants = require('./app.constants');

function getBlockchainAccount(keypair) {
    const keyring = new Keyring({ type: "sr25519" });
    return keyring.addFromUri(process.env.SUDO || keypair);
}

async function init() {
    const [_, __, substrate_address, substrate_address_key_type, cosmos_address] = process.argv;
    const WS_URL = process.env.NODE_ENV === 'production' ? appConstants.ENDPOINT_PROD : appConstants.ENDPOINT_LOCAL;
    const provider = new WsProvider(WS_URL);
    const api = await ApiPromise.create({ provider });

    const response = await new Promise(resovle => {
        setTimeout(async () => {
            if (substrate_address && cosmos_address) {
                const signer = getBlockchainAccount(substrate_address);
                try {
                    const txHash = await api.tx.cosmosAbci
                        .insertCosmosAccount(cosmos_address, substrate_address_key_type, 0)
                        .signAndSend(signer);
                    resovle(txHash.toString());
                } catch (err) {
                    resovle(err);
                }
            }
            // Set 5s delay between txs.
        }, 5000);
    });

    console.log(response);
}

init()
   .finally(() => process.exit(0));
