# XCVM Hackathon

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
./start-relayer.sh
```
