# Example of usage gRPC with hyperium/tonic

To run examples with installed `cargo-make`:

```sh
# Hello world example
cargo make helloworld

# ABCI example
cargo make abci
```

To run examples with `cargo`:

```sh
# Hello world example
cargo run --bin helloworld-server
# In new terminal
cargo run --bin helloworld-client

# ABCI example
cargo run --bin abci-server
# In new terminal
cargo run --bin abci-client
```
