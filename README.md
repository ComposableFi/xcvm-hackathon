# XCVM Hackathon

![ownables](https://user-images.githubusercontent.com/100821/177121121-a1c3dc8c-8108-4c07-9e15-b83ebfdf8f98.png)

Ownables are CosmWasm smart contracts that define ownership. In addition to running on a cosmos blockchain, ownables
can run directly in a wallet using the [LTO Network](https://ltonetwork.com) private layer.

We're using XCVM to make ownable contracts tradable. An owner can deposit an ownable to the XCVM bridge. The bridge
will mint an NFT on Ethereum, which can be traded on a platform like OpenSea. The NFT holder can transfer the NFT
to their XCVM contract, which corresponds to their Cosmos account, and claim the ownable.

## How to run

### Starting your personal blockchain
To automatically setup everything you need to get started,
please open this repository using a GitHub Codespace.

When it has finished loading, please open a terminal and enter the following:
`docker-compose up`

This takes a few minutes to load. When it has finished you'll see the following in your terminal:

`composable-xcvm_1  | 🚀 POLKADOT LAUNCH COMPLETE 🚀` 

You can see your personal blockchain working by visiting the following link:
[PolkadotJS Blockchain Explorer](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9988#/explorer)

### Running the relayer (only when the node is running)
```bash
cd relayer
cargo build --release
./target/release/relayer
```

### Compiling and deploying the XCVM contract
Running `./go.sh` will compile the contract and run the deployment script.

#### Working on the XCVM contract
Under the `cosmwasm/contracts/xcvm` directory, edit the contract and make sure it compile with `cargo wasm`.

#### Working on the polkadotjs script
Under the `scripts` directory, edit the polkadotjs script `main.ts` and run `npm run go` to start deploying.
