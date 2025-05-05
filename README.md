# Oxidefier

## USage

We translate the JSON corresponding to the Yul of a Solidity smart contract to a Rust file. This Yul might or might not be optimized. It is probably better not to take it optimized to keep a code which is easier to relate.

```sh
python scripts/shallow_embed.py sol_contracts/erc20.json >evm_opcodes/main.rs
```
