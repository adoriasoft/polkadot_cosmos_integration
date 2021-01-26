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

    // Query node data.
    server.on('request', async (req, res) => {
        const { type } = req.query || {};
        if (type) {
            res.write(type);
            res.end();
        } else {
            const validators = await api.query.session.validators();
            res.write(JSON.stringify(validators));
            res.end();
        }
    });

    server.listen(8000);
}

startServer();
