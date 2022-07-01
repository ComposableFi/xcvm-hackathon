// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
pragma experimental ABIEncoderV2;

import "./interfaces/IRouter.sol";
import "./XCVM.sol";
import "./tokens/BridgedToken.sol";

import "hardhat/console.sol";

contract Bridge is IRouter {
    mapping(uint32 => BridgedToken) public bridgedAssets;
    mapping(uint256 => mapping(bytes => address)) public XCVMs;
    address swapper;

    event InstanceCreated(IRouter.Origin origin, address instance);
    event AssetCreated(uint32 assetId, string symbol, address tokenAddress);
    event Spawn(
        IRouter.Origin origin,
        bytes salt,
        uint256 network,
        IRouter.AssetsAmount[] assets,
        bytes childSalt,
        string program
    );
    event Transfer(IRouter.Origin origin, bytes salt, address to, address from, IRouter.AssetsAmount asset);
    event Call(IRouter.Origin origin, bytes salt, address to, bytes payload);
    event Succeeded(IRouter.Origin origin, bytes salt);
    event Failed(IRouter.Origin origin, bytes salt, string reason);

    constructor() {
        createAsset(1, "PICA");
        createAsset(2, "WETH");
        createAsset(3, "USDT");
        createAsset(4, "USDC");
        createAsset(0xDEADC0DE, "UST");
    }

    function setSwapper(address _swapper) public {
        swapper = _swapper;
    }

    function bridge(
        IRouter.Origin calldata origin,
        bytes calldata salt,
        IRouter.AssetsAmount[] calldata assets,
        string calldata program
    ) public {
        XCVM xcvm = XCVM(_instantiateXCVM(origin));
        _provisionXCVMWithTokens(assets, xcvm);
        xcvm.call(this, origin, salt, program);
    }

    function createAsset(uint32 assetId, string memory symbol) internal {
        require(address(bridgedAssets[assetId]) == address(0), "Bridge: assetId already in use");
        BridgedToken token = new BridgedToken(symbol);
        bridgedAssets[assetId] = token;
        emit AssetCreated(assetId, symbol, address(token));
    }

    function _provisionXCVMWithTokens(IRouter.AssetsAmount[] calldata assets, XCVM xcvm) internal {
        for (uint256 i = 0; i < assets.length; i++) {
            IRouter.AssetsAmount calldata assetAmount = assets[i];
            mint(address(xcvm), assetAmount.id, assetAmount.amount);
        }
    }

    function _instantiateXCVM(Origin memory origin) internal returns (address xcvmAddress) {
        xcvmAddress = XCVMs[origin.networkId][origin.account];
        if(xcvmAddress == address(0)) {
            xcvmAddress = address(new XCVM(origin, this, swapper));
            XCVMs[origin.networkId][origin.account] = xcvmAddress;
            emit InstanceCreated(origin, xcvmAddress);
        }
        return xcvmAddress;
    }

    function spawn(
        Origin memory origin,
        bytes memory salt,
        uint32 network,
        IRouter.AssetsAmount[] memory assets,
        bytes memory childSalt,
        string memory program
    ) public {
        emit Spawn(origin, salt, network, assets, childSalt, program);
    }

    function transferred(
        Origin memory origin,
        bytes memory salt,
        address to,
        address from,
        IRouter.AssetsAmount memory asset
    ) public {
        emit Transfer(origin, salt, to, from, asset);
    }

    function called(
        Origin memory origin,
        bytes memory salt,
        address to,
        bytes memory payload
    ) public {
        emit Call(origin, salt, to, payload);
    }

    function succeeded(Origin memory origin, bytes memory salt) public {
        emit Succeeded(origin, salt);
    }

    function failed(
        Origin memory origin,
        bytes memory salt,
        string memory reason
    ) public {
        emit Failed(origin, salt, reason);
    }

    function burn(
        address account,
        uint32 assetId,
        uint128 amount
    ) public {
        BridgedToken token = bridgedAssets[assetId];
        require(address(token) != address(0), "Bridge: assetId is invalid");
        token.burn(account, amount);
    }

    function mint(
        address account,
        uint32 assetId,
        uint128 amount
    ) public {
        BridgedToken token = bridgedAssets[assetId];
        require(address(token) != address(0), "Bridge: assetId is invalid");
        token.mint(account, amount);
    }

    function token(uint32 assetId) public returns (BridgedToken token) {
        token = bridgedAssets[assetId];
        require(address(token) != address(0), "Bridge: assetId is invalid");
    }

    function balanceOf(address account, uint32 assetId) public returns (uint256 balance) {
        BridgedToken token = bridgedAssets[assetId];
        require(address(token) != address(0), "Bridge: assetId is invalid");
        balance = token.balanceOf(account);
    }
}
