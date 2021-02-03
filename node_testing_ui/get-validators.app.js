const { ApiPromise, WsProvider } = require('@polkadot/api');
const { ENDPOINTS, METADATA_TYPES } = require('./app.constants');

async function init() {
    const WS_URL = process.env.NODE_ENV === 'production' ? ENDPOINTS.ENDPOINT_PROD : ENDPOINTS.ENDPOINT_LOCAL;
    const provider = new WsProvider(WS_URL);
    const api = await ApiPromise.create({
        provider,
        ...METADATA_TYPES,
    });
    const validators = await api.query.session.validators();

    console.log(validators.map(v => v.toString()).join('@'));
}

init()
    .finally(() => process.exit(0));
