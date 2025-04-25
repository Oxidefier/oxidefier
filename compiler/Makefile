all:
	dune exec src/compiler.exe ../sol_contracts/minimal.yul.json > ../evm_opcodes/out.rs

run:
	dune exec src/compiler.exe ../sol_contracts/minimal.yul.json > out/out.rs
	cat out/out.rs
