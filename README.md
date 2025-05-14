# **XION World of Rogues Game Smart Contract**

## Streak System



## Deployment
Run docker then...

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0
```

xiond keys add rogues
xiond keys show rogues

Add testnet tokens here:

https://faucet.xion.burnt.com/

Check here:

https://explorer.burnt.com/xion-testnet-2/account/xion1gn6m2ff4yzqgxlk7sxrx03g2zqfxgdr22d9yp6
```
WALLET=xion1gn6m2ff4yzqgxlk7sxrx03g2zqfxgdr22d9yp6
```
```
RES=$(xiond tx wasm store ./artifacts/xion_wor.wasm \
      --chain-id xion-testnet-2 \
      --gas-adjustment 1.3 \
      --gas-prices 0.1uxion \
      --gas auto \
      -y --output json \
      --node https://rpc.xion-testnet-2.burnt.com:443 \
      --from $WALLET)
```
```
echo $RES
```
```
TXHASH=82755E187F29B35DA56EC6EBB1A26029663EB67772ECE792C420094080394F56
```
```
CODE_ID=$(xiond query tx $TXHASH \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --output json | jq -r '.events[-1].attributes[1].value')
```
```
echo $CODE_ID
```
```
xiond tx wasm instantiate $CODE_ID "{}" \
  --from $WALLET \
  --label "xion-wor" \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y --no-admin \
  --chain-id xion-testnet-2 \
  --node https://rpc.xion-testnet-2.burnt.com:443
```
```
TXHASH=FF9B8A0209090BC66A51D134A35BB101407B128C20BEB606FB6E5E19E009E783
```
```
CONTRACT=$(xiond query tx $TXHASH \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --output json | jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
```
```
echo $CONTRACT
```
```
QUERY='{"get_streak":{ "address": "xion1gn6m2ff4yzqgxlk7sxrx03g2zqfxgdr22d9yp6"}}'
```
```
xiond query wasm contract-state smart $CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-2.burnt.com:443
```

```
CLAIM_STREAK='{"claim_streak": {}}'
```
```
xiond tx wasm execute $CONTRACT "$CLAIM_STREAK" \
  --from $WALLET \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --chain-id xion-testnet-2
```

Setup treasury contract here:
https://dev.testnet.burnt.com/



## CAVEATS
You may need to install jq using:
```
brew install jq
```
You can instal cosmwasm-check by:
```
cargo install cosmwasm-check
```
Update rustup by:
```
rustup update
```

