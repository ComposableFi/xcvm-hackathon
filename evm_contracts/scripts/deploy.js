// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// When running the script with `npx hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
const hre = require("hardhat");

const { ethers } = require("hardhat");

// TODO: add network mappings from config
async function main() {
  // Hardhat always runs the compile task when running scripts with its command
  // line interface.
  //
  // If this script is run directly using `node` you may want to call compile
  // manually to make sure everything is compiled
  // await hre.run('compile');

  // We get the contract to deploy
  console.log("=== Deploying bridge");

  const Bridge = await hre.ethers.getContractFactory("Bridge");
  const bridge = await Bridge.deploy();
  await bridge.deployed();

  console.log("=== Bridge Contract deployed at :", bridge.address);

  const PICA = await bridge.bridgedAssets(1);
  const WETH = await bridge.bridgedAssets(2);
  const USDT = await bridge.bridgedAssets(3);
  const USDC = await bridge.bridgedAssets(4);
  const UST = await bridge.bridgedAssets(0xDEADC0DE);
  console.log("PICA: " + PICA);
  console.log("WETH: " + WETH);
  console.log("USDT: " + USDT)
  console.log("USDC: " + USDC);
  console.log("UST: " + UST);

  /// alice is whale
  const amount = hre.ethers.utils.parseEther("10000000000000000");

  //await pingProgram(bridge);
  //await randomProgram(bridge);
  // console.log(amount);
  const swapperContract = await deployDexes(bridge, 1, 4, PICA, USDC, amount);
  console.log("Swap address: ", swapperContract);
  // await swapProgram(bridge, swapperContract, 1, 4, 1337000, 1337000);
  // await swapProgram(bridge, swapperContract, 1, 4, 1337000, 1337000);
  // await swapProgram(bridge, swapperContract, 1, 4, 100000000000, 100);
}

async function randomProgram(bridge) {
  const program = JSON.stringify({
    "tag": [0xCA, 0xFE],
    "instructions": [{
      "spawn": {
        "network": 2,
        "salt": [0xBA, 0xBE],
        "assets": {
          "1": {
            "ratio": 4294967295
          }
        },
        "program": {
          "tag": [0xDE, 0xAD, 0xC0, 0xDE],
          "instructions": [{
            "spawn": {
              "network": 1,
              "salt": [0xDE, 0xAD],
              "assets": {
                "1": {
                  "ratio": 100
                }
              },
              "program": {
                "tag": [0xBB, 0xAA],
                "instructions": [],
              }
            }
          }],
        }
      }
    }],
  });
  console.log(program);
  const tx = await bridge.bridge([0x1337, [0, 1, 2, 3, 4, 5]], [0xC0, 0xDE], [[1, 10000000000000], [4, 0xBEEFBEEF]], program);
  console.log((await tx.wait()).events);
}

async function swapProgram(bridge, pairAddress, a1i, a2i, a1, a2) {
  console.log("Swap program");
  const abiCoder = new hre.ethers.utils.AbiCoder();
  let call = encodedSwap(a1i, a2i, a1, a2);

  let payload = abiCoder.encode(["address", "bytes"], [pairAddress, call]);

  console.log([pairAddress, call]);
  const callArray = hexStringToBytesArray(payload);
  console.log(callArray.length);

  const program = JSON.stringify({
    tag: [],
    instructions: [{
      call: {
        encoded: callArray,
      },
    }]
  });
  const tx = await bridge.bridge([0x1337, [0, 1, 2, 3, 4, 5]], [0xC0, 0xDE], [[a1i, a1]], program);
  await tx.wait();
  // console.log((await tx.wait()).events);
}

async function pingProgram(bridge) {
  console.log("calling ping");
  const Ping = await hre.ethers.getContractFactory("Ping");
  const ping = await Ping.deploy();
  await ping.deployed();
  await ping.ping();
  const abiCoder = new hre.ethers.utils.AbiCoder();

  let pingCall = encodeFunctionCall(
    "ping",
    ["function ping()"],
    []
  );

  console.log("Ping address ", ping.address);
  let payload = abiCoder.encode(["address", "bytes"], [ping.address, pingCall]);

  const program = JSON.stringify({
    tag: [],
    instructions: [{
      call: {
        encoded: hexStringToBytesArray(payload),
      },
    }]
  });
  console.log(program);

  const tx = await bridge.bridge([0x1337, [0, 1, 2, 3, 4, 5]], [0xC0, 0xDE], [[1, 10000], [4, 0xBEEFBEEF]], program);
  console.log((await tx.wait()).events);
}



function hexStringToBytesArray(payload) {
  return Array.from(hre.ethers.utils.arrayify(payload).values());
}

/// deploys dexes to showcase XCVM calls to these
async function deployDexes(bridge, a1i, a2i, a1a, a2a, amount) {
  console.log("=== Deploying DEXes");

  const XCVMSwap = await hre.ethers.getContractFactory("XCVMSwap");
  const swap = await XCVMSwap.deploy();
  await bridge.setSwapper(swap.address);
  await bridge.mint(swap.address, 1, amount);
  await bridge.mint(swap.address, 2, amount);
  await bridge.mint(swap.address, 3, amount);
  await bridge.mint(swap.address, 4, amount);
  await bridge.mint(swap.address, 0xDEADC0DE, amount);
  const pool = await (await swap.createPair(a1i, a2i, a1a, a2a)).wait();
  const addr = pool.events.filter(e => e.event === "PairCreated")[0].args[0];
  console.log("Underlying Univ2 address: ", addr);

  // await addLiquidity(addr, a1i, a2i, a1a, a2a, swap, amount);

  // const tx = swapTx(swap, a1i, a2i, 100, 100);
  // console.log(await (await hre.ethers.getSigner()).sendTransaction(tx));

  // const alice = ((await hre.ethers.getSigner())).address;
  // await pool.swap(0, 14000, alice, '0x');
  // await swap.swap(a1i, a2i, 100, 100);
  // await swap.swap(a1i, a2i, 100, 100);
  // await swap.swap(a1i, a2i, 100, 100);

  return swap.address;
}


async function addLiquidity(pool, a1i, a2i, a1a, a2a, swap, amount) {
  console.log("=== Add liquidity");
  const leftTokenContract = await hre.ethers.getContractAt("ERC20", a1a);
  const rigthTokenContract = await hre.ethers.getContractAt("ERC20", a2a);

  console.log("Value for liquidity: " + amount);
  await leftTokenContract.transfer(pool, amount);
  await rigthTokenContract.transfer(pool, amount);
  await swap.mint(a1i, a2i);
  // await leftTokenContract.transfer(pool, amount);
  // await rigthTokenContract.transfer(pool, amount);
}

function swapTx(swap, a1i, a2i, a1, a2) {
  let swapCall = encodedSwap(a1i, a2i, a1, a2);
  const tx = {
    to: swap.address,
    data: swapCall,
  };
  return tx;
}

function encodedSwap(a1i, a2i, a1, a2) {
  return encodeFunctionCall(
    "swap",
    ["function swap(uint32,uint32, uint128, uint128)"],
    [a1i, a2i, a1, a2]
  );
}

/// encoded function calls with no signature ABI of ethereum
function encodeFunctionCall(functionSignature, abiArray, argsValuesArray) {
  const iFace = new ethers.utils.Interface(abiArray);
  const call = iFace.encodeFunctionData(functionSignature, argsValuesArray);
  return call;
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
