# installation
`npm install`

# Configuration File
Fill up the `.env` file
`evm_contracts/.env`

# Deployment

Deploys to locally running network

`npm run deploy`

You may run end all that you need using [this](../docker-compose.yml)

## TODO
Fix geth connection string setting networks.geth.url @ hardhat.config.js

# Check linter
`npm run lint-check`

# Apply linting
`npm run lint-fix`

# Run test
`npm run test`

# Check contracts size limit
`npm run size`

# Watch for changes

`npm run watch-test`


```shell
npx hardhat accounts
npx hardhat compile
npx hardhat clean
npx hardhat test
npx hardhat node
node scripts/sample-script.js
npx hardhat help
```
