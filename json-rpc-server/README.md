
// execute say_hello method
curl -H 'Content-Type: application/json' -XPOST -d '{"jsonrpc": "2.0", "method": "say_hello", "params": {}, "id" : 3}'  http://localhost:26657

// execute /abci_query method
curl -H 'Content-Type: application/json' -XPOST -d '{"jsonrpc": "2.0", "method": "abci_query", "params": {"path" : "/cosmos.auth.Query/Account", "data" : "0A1402C3EC2A3CC111EB1825267FCCCC546CF2D25A2E", "height" : "0", "prove" : false}, "id" : 0}'  http://localhost:26657