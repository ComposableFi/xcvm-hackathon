cd cosmwasm
docker run --rm -v $(pwd):/code \
		--mount type=volume,source="devcontract_cache_xcvm",target=/code/contracts/xcvm/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.12.6 ./contracts/xcvm
cd ..
cp cosmwasm/artifacts/xcvm.wasm scripts/
cd scripts
npm run go
cd ..
