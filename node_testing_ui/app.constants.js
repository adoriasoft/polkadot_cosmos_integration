const ENDPOINTS = {
    ENDPOINT_LOCAL: 'ws://127.0.0.1:9944',
    ENDPOINT_PROD: 'wss://polka.adoriasoft.com/first',
};

const METADATA_TYPES = {
    types: {
        CosmosAccount: {
            pub_key: 'Vec<u8>',
            power: 'u64',
        },
        ABCITxs: {
            data_array: 'Vec<Vec<u8>>'
        },
        OptionalLedger: 'Option<(AccountId, Balance)>',
    }
}

module.exports = {
    ENDPOINTS,
    METADATA_TYPES,
};
