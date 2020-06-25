require('module-alias/register')
const Api = require('@/api');
const { initAccounts, initSudoAccount } = require('@/utils');
const { flow } = require('@/scenarios');

const API_URL = process.env.NODE_ENV !== 'production' ? 'ws://127.0.0.1:9944' : '';

async function start() {
    const api = await Api.connect(API_URL);
    const scenario = process.argv.slice(2)[0];
    if (!scenario) {
        throw Error('Invalid scenario');
    }
    switch (scenario) {
        case 'flow': {
            const accounts = initAccounts();
            const sudoAccount = initSudoAccount();
            await flow.start(api, sudoAccount, accounts);
            return;
        }
    }
}

start().finally(() => process.exit(0));
