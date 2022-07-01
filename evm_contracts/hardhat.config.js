require("@nomiclabs/hardhat-ethers");
// require("@nomiclabs/hardhat-truffle5");
require("@nomiclabs/hardhat-waffle");
require("hardhat-abi-exporter");
require("hardhat-contract-sizer");
require("hardhat-watcher");
require("solidity-docgen");

const dotenv = require("dotenv");
dotenv.config();

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

/**
 * @type import('hardhat/config').HardhatUserConfig
 */

module.exports = {
  defaultNetwork: "hardhat",
  networks: {
    hardhat: {},
    geth: {
      url: process.env.GETH_URL,
      accounts: ["0x" + process.env.ETH_PRIVATE_KEY],
      gas: 210000000,
      gasPrice: 8000000000
    },
  },
  dependencyCompiler: {
    paths: [
      './node_modules/@uniswap/v2-core/contracts/UniswapV2ERC20.sol'
    ],
  },
  solidity: {
    compilers: [
      {
        version: "0.8.15",
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
      {
        version: "0.5.16",
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      }
    ],
  },
  contractSizer: {
    disambiguatePaths: false,
    runOnCompile: false,
    strict: true,
    // only: [],
    except: ["mock"],
  },
  // mocha: {
  //   timeout: 4000000, //default 40000
  // },
  watcher: {
    compilation: {
      tasks: ["compile"],
    },
    test: {
      tasks: [{ command: "test", params: { testFiles: ["{path}"], bail: true } }],
      files: ["./test/**/*"],
      verbose: true,
    },
    retest: {
      tasks: ["compile", "test"],
      files: ["./test/**/*", "./contracts/**/*"],
      verbose: true,
    },
    deploy: {
      tasks:
      [
        "compile",
        { command: "run",  params : { network: "localhost", script : "./scripts/deploy.js" }}
      ],
      files: ["./test/**/*", "./contracts/**/*", "./scripts/**/*", "./hardhat.config.js"],
      verbose: true,
    },
    retest2: {
      tasks: [
        "compile",
        { command: "test", params: { parallel: true } },
      ],
      files: ["./test/**/*", "./contracts/**/*"],
      verbose: true,
    },
    scripts: {
      tasks: [{ command: "run", params: { script: ["{path}"] } }],
      files: ["./scripts/**/*"],
      verbose: true,
    },
    docs: {
      tasks: ["docgen"],
      files: ["./contracts/**/*"],
      verbose: true,
    },
  },
  abiExporter: {
    path: "./build/hardhat/abis",
    runOnCompile: true,
    clear: true,
    flat: false,
    // only: [':ERC20$'],
    spacing: 2,
    pretty: true,
  },
};
