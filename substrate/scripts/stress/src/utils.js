const { Keyring } = require('@polkadot/api');
const { randomAsU8a } = require('@polkadot/util-crypto');
const { u8aToHex } = require('@polkadot/util');

module.exports.sleep = (ms = 1000) => {
    return new Promise(resolve => setTimeout(resolve, ms));
}

module.exports.initAccounts = (count = 5) => {
    const keyring = new Keyring({ type: 'sr25519' });
    const accounts = [];
    for (let i = 0; i < count; i++) {
        accounts.push(keyring.addFromUri(u8aToHex(randomAsU8a())));
    }
    return accounts;
}

module.exports.initSudoAccount = () => {
    const keyring = new Keyring({ type: 'sr25519' });
    const keypair = keyring.addFromUri(process.env.SUDO || '//Alice');
    return keypair;
}

module.exports.getRandomInt = (min, max) => {
    return Math.floor(Math.random() * (max - min + 1) + min);
}
