# Oxidefier

> From Solidity to Rust! 🦀 🟣

## Usage

We translate the JSON corresponding to the Yul of a Solidity smart contract to a Rust file. This Yul might or might not be optimized. We recommend to use the unoptimized Yul to have a more predictable translation in Rust, and let the Rust compiler optimize the output.

```sh
python oxidefier.py contracts/erc20_single_file/contract.json erc20_single_file
```

To generate the Yul's JSON, you can use the following command:

```sh
solc --ir-optimized-ast-json --optimize --yul-optimizations hg contracts/erc20_single_file/contract.sol \
  | tail -1 \
  > contracts/erc20_single_file/contract.json
```

We provide more example in our [CI file](.github/workflows/check.yml).
