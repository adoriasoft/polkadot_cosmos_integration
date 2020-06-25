const { sleep, getRandomInt } = require('@/utils');

const BLOCK_TIME = 6000;

module.exports.start = async (api, sudoAccount, accounts = []) => {
    if (!accounts.length) {
        throw Error('Invalid accounts');
    }
    console.log('Start flow logic')
    try {
        const reuslt = await api.api_tx.abci.deliverTx({ tx: "123" }).signAndSend(sudoAccount);
        console.log(reuslt);
    } catch (err) {
        console.log(err);
    } finally {
        await sleep(BLOCK_TIME);
    }
}
