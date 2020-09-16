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

### Run docker compose

To run docker-compose file in this folder you need to init nginx with ssl certs, and execute next command:

```sh
docker-compose up
```

After launching docker containers you can see logs with next command:

```sh
docker-compose logs --tail 100 -f
```

### Run docker compose testnet with nginx

To run docker-compose file in this folder you need to init nginx with ssl certs, and execute next command:

```sh
docker-compose -f docker-testnet.yml up -d
```
