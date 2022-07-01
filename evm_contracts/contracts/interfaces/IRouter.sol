// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
pragma experimental ABIEncoderV2;

interface IRouter {
    struct AssetsAmount {
        uint32 id;
        uint128 amount;
    }

    struct Origin {
        uint32 networkId;
        bytes account;
    }
}
