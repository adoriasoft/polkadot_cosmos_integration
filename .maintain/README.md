# Files for polkadot/apps setup

Command to get cert:

```sh
docker run -it --rm \
-v ~/letsencrypt/:/etc/letsencrypt \
-v ~/site:/data/letsencrypt \
certbot/certbot \
certonly --webroot \
--email ceo@adoriasoft.com --agree-tos --no-eff-email \
--webroot-path=/data/letsencrypt \
-d polka.adoriasoft.com
```

### Build spec

To build specs for local testnet:

```sh
docker run -it --rm node build-spec --disable-default-bootnode --chain local > res/customSpec.json
docker run -it --rm -v "$(pwd)/res:/res" node build-spec --chain=res/customSpec.json --raw --disable-default-bootnode > res/customSpecRaw.json
```

Demo server IP: `164.90.208.88`.
