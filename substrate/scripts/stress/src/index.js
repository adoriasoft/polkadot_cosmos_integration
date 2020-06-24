require('module-alias/register')
const Api = require('@/api');
const { initAccounts, initSudoAccount } = require('@/utils');
const { analyzer } = require('@/scenarios');

const API_URL = process.env.NODE_ENV !== 'production' ? 'ws://127.0.0.1:9944' : '';

async function start() {
    const api = await Api.connect(API_URL);
    const scenario = process.argv.slice(2)[0];
    if (!scenario) {
        throw Error('Invalid scenario');
    }
    switch (scenario) {
        case 'analyzer': {
            const accounts = initAccounts();
            const sudoAccount = initSudoAccount();
            await analyzer.start(api, sudoAccount, accounts);
            return;
        }
    }
}

start().finally(() => process.exit(0));
