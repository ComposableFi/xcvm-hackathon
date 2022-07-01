// SPDX-License-Identifier: MIT
pragma solidity ^0.5.16;
// this worked, plugin not 
// https://ethereum.stackexchange.com/questions/114376/how-to-compile-external-contracts-using-hardhat/114377#114377
import "@uniswap/v2-core/contracts/UniswapV2Factory.sol";
import "hardhat/console.sol";

contract Ping {
    function ping() external {
         console.log("Pong");
    }
}
