# **XION World of Rogues Game Smart Contract**

## Streaks


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

https://explorer.burnt.com/xion-testnet-1/account/xion1gn6m2ff4yzqgxlk7sxrx03g2zqfxgdr22d9yp6
```
WALLET="xion1gn6m2ff4yzqgxlk7sxrx03g2zqfxgdr22d9yp6"
```
```
RES=$(xiond tx wasm store ./artifacts/xion_wor.wasm \
      --chain-id xion-testnet-1 \
      --gas-adjustment 1.3 \
      --gas-prices 0.1uxion \
      --gas auto \
      -y --output json \
      --node https://rpc.xion-testnet-1.burnt.com:443 \
      --from $WALLET)
```
```
echo $RES
```
```
TXHASH=D5A2EAEF5A94F1EB86E96F4084F46F17CEFAF4D8103EF29DDC297F75C1F705FE
```
```
CODE_ID=$(xiond query tx $TXHASH \
  --node https://rpc.xion-testnet-1.burnt.com:443 \
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
  --chain-id xion-testnet-1 \
  --node https://rpc.xion-testnet-1.burnt.com:443
```
```
TXHASH=7093FDD3106173EAF40EB67DCE14B48B36E61276B9449695F3F9C438121937CE
```
```
CONTRACT=$(xiond query tx $TXHASH \
  --node https://rpc.xion-testnet-1.burnt.com:443 \
  --output json | jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
```
```
echo $CONTRACT
```
```
xiond query wasm contract-state smart $CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-1.burnt.com:443
```
```
QUERY='{"get_streak":{ "address": "xion1gn6m2ff4yzqgxlk7sxrx03g2zqfxgdr22d9yp6"}}'
```
```
xiond query wasm contract-state smart $CONTRACT "$QUERY" --output json --node https://rpc.xion-testnet-1.burnt.com:443
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
  --node https://rpc.xion-testnet-1.burnt.com:443 \
  --chain-id xion-testnet-1
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

