const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const http = require('http');
const url = require('url');
const appConstants = require('./app.constants');

function getBlockchainAccount(keypair) {
    const keyring = new Keyring({ type: "sr25519" });
    return keyring.addFromUri(process.env.SUDO || keypair);
}

async function startServer() {
    const WS_URL = process.env.NODE_ENV === 'production' ? appConstants.ENDPOINT_PROD : appConstants.ENDPOINT_LOCAL;
    const provider = new WsProvider(WS_URL);
    const api = await ApiPromise.create({ provider });
    const server = http.createServer();

    server.on('request', async (req, res) => {
        const params = url.parse(req.url, true).query;
        const substrateAccountAddress = params.substrate_address;
        const cosmosAccountAddress = params.cosmos_address;
        if (substrateAccountAddress, cosmosAccountAddress) {
            const signAccount = getBlockchainAccount(substrateAccountAddress);
            if (signAccount.address && signAccount.address.length > 0) {
                try {
                    const txHash = await api.tx.cosmosAbci
                        .insertCosmosAccount(cosmosAccountAddress)
                        .signAndSend(signAccount);
                    res.setHeader('Content-Type', 'application/json')
                    res.end(JSON.stringify({
                        txHash,
                    }));
                } catch (err) {
                    res.end('Error send tx', JSON.stringify(err));
                }
            }
        }
    });
    server.listen(8000);
}

startServer();
