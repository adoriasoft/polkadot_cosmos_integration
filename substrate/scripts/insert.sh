#!/usr/bin/env bash

set -e

echo "*** Inserting key ***"

# ➜ subkey -es inspect 'again cinnamon mesh post loop strike this equip door metal exhibit collect'
# Secret phrase `again cinnamon mesh post loop strike this equip door metal exhibit collect` is account:
#   Secret seed:      0x70a6af8c96c50519912efaa7c759cb9b80dbe7607850af912120ae7eb8b900f9
#   Public key (hex): 0x0913a0136221cdd32a760591366a7de950f0ed658c00087a4f2e2b16c82df6a0
#   Account ID:       0x0913a0136221cdd32a760591366a7de950f0ed658c00087a4f2e2b16c82df6a0
#   SS58 Address:     5CGc7oPGtzFvdDXTr3QwE6DKGdr5gmSUqqWMyJhEjWHEWxmY
# enfipy in polkadot_cosmos_integration/substrate on  dev/grpc [!] via 🦀 v1.43.0 
# ➜ subkey -s inspect 'again cinnamon mesh post loop strike this equip door metal exhibit collect' 
# Secret phrase `again cinnamon mesh post loop strike this equip door metal exhibit collect` is account:
#   Secret seed:      0x70a6af8c96c50519912efaa7c759cb9b80dbe7607850af912120ae7eb8b900f9
#   Public key (hex): 0x9c12ab2f3a643369726366d8422d8bb6cdd3bb20c018cdd8c0f2e5b7e6930711
#   Account ID:       0x9c12ab2f3a643369726366d8422d8bb6cdd3bb20c018cdd8c0f2e5b7e6930711
#   SS58 Address:     5FbLsaoSof77wDu9s62J875DMfKJ55miBLKDx8jFiu5w8XpH

# curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["abci", "again cinnamon mesh post loop strike this equip door metal exhibit collect", "0x0913a0136221cdd32a760591366a7de950f0ed658c00087a4f2e2b16c82df6a0"] }'
curl "http://localhost:9933" -H "Content-Type:application/json;charset=utf-8" -d '{ "jsonrpc":"2.0", "id":1, "method":"author_insertKey", "params": ["abci", "again cinnamon mesh post loop strike this equip door metal exhibit collect", "0x9c12ab2f3a643369726366d8422d8bb6cdd3bb20c018cdd8c0f2e5b7e6930711"] }'
