require('module-alias/register')
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

const API_URL = process.env.NODE_ENV !== 'production' ? 'ws://127.0.0.1:9944' : 'wss://polka.adoriasoft.com/first';

const defaultTypes = {
    "Address": "AccountId",
    "LookupSource": "AccountId",
    "String": "Vec<u8>",
    "TxMessage": {
        "tx": "String"
    }
};

function initSudoAccount() {
    const keyring = new Keyring({ type: "sr25519" });
    const keypair = keyring.addFromUri(process.env.SUDO || "//Alice");
    return keypair;
}

async function start() {
    const wsProvider = new WsProvider(API_URL);
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: defaultTypes,
    });
    try {
        const sudoAccount = initSudoAccount();

        var text = '{ "From": "Alice", "To": "Alex", "Amount": 4000, "Signature": "IAAAAOM6uUoQYmboLlGlSf2DVJhoQ1uGuTgZcLPiE9onV5zIIAAAAPox8pBXRBO2r3mH5S+PNJftIAIJJoGQNSCVREKaBCpo"}';
        var obj = JSON.parse(text);

        const reuslt = await api.tx.abciDirect.deliverTx({ tx: text}).signAndSend(sudoAccount);
        console.log(reuslt);
    } catch (err) {
        console.log(err);
    }
}

start().finally(() => process.exit(0));
