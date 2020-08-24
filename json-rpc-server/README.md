
// execute say_hello method
curl -H 'Content-Type: application/json' -XPOST -d '{"jsonrpc": "2.0", "method": "say_hello", "params": {}, "id" : 3}'  http://localhost:26657

// execute /abci_query method
curl -H 'Content-Type: application/json' -XPOST -d '{"jsonrpc": "2.0", "method": "/abci_query", "params": {}, "id" : 3}'  http://localhost:26657