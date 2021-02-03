const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const { ENDPOINTS, METADATA_TYPES } = require('./app.constants');

function getBlockchainAccount(keypair) {
    const keyring = new Keyring({ type: "sr25519" });
    return keyring.addFromUri(process.env.SUDO || keypair);
}

async function init() {
    const [_, __, substrate_address] = process.argv;

    const WS_URL = process.env.NODE_ENV === 'production' ? ENDPOINTS.ENDPOINT_PROD : ENDPOINTS.ENDPOINT_LOCAL;
    const provider = new WsProvider(WS_URL);

    try {
        const api = await ApiPromise.create({
            provider,
            ...METADATA_TYPES,
        });
        const response = await new Promise(resovle => {
            setTimeout(async () => {
                if (substrate_address) {
                    const signer = getBlockchainAccount(substrate_address);
                    try {
                        const accountWeight = await api.query.cosmosAbci.substrateAccountWeights(signer.publicKey);
                        resovle(accountWeight.toHuman());
                    } catch (err) {
                        resovle(err);
                    }
                }
            }, 0);
        });
    
        console.log(response);
    } catch(err) {
        console.log(err)
    }
}

init()
    .finally(() => process.exit(0));
