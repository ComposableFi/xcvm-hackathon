// SPDX-License-Identifier: MIT
pragma solidity ^0.5.16;

import "@uniswap/v2-core/contracts/interfaces/IUniswapV2Pair.sol";
import "@uniswap/v2-core/contracts/interfaces/IERC20.sol";
import "@uniswap/v2-core/contracts/UniswapV2Pair.sol";

import "@uniswap/v2-core/contracts/UniswapV2Factory.sol";

import "hardhat/console.sol";

contract XCVMSwap {
    struct Pair {
        IUniswapV2Pair pair;
        IERC20 a1;
        IERC20 a2;
    }
    // asset -> asset -> pool

    event PairCreated(address addr);
    mapping(uint32 => mapping(uint32 => Pair)) public pairs;
    UniswapV2Factory factory;

    constructor () public {
        factory = new UniswapV2Factory(address(this));
    }

    function createPair(uint32 a1i,  uint32 a2i, address a1, address a2) public payable {
        IUniswapV2Pair pair = IUniswapV2Pair(factory.createPair(a1, a2));
        Pair memory data = Pair(pair, IERC20(a1), IERC20(a2));
        pairs[a1i][a2i] = data;
        emit PairCreated(address(pair));
    }

    function mint(uint32 a1, uint32 a2) external {
        Pair memory pair = pairs[a1][a2];
        pair.pair.mint(msg.sender);
    }

    function swap(uint32 a1, uint32 a2, uint128 amount1In, uint128 amount2Out) public payable {
        Pair memory pair  = pairs[a1][a2];
        IERC20(pair.a1).transferFrom(msg.sender, address(this), amount1In);
        IERC20(pair.a2).transfer(msg.sender, amount2Out);
    }
}
