// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
pragma experimental ABIEncoderV2;

import "./interfaces/IRouter.sol";
import "./libs/JsmnSolLib.sol";
import "./Bridge.sol";

import "hardhat/console.sol";

contract XCVM {
    struct Lexer {
        JsmnSolLib.Token[] tokens;
        uint256 index;
        uint256 elements;
    }

    IRouter.Origin owner;

    constructor(IRouter.Origin memory _owner, Bridge bridge, address swapper) {
        owner = _owner;
        if(swapper != address(0)) {
            bridge.token(1).approve(swapper, type(uint256).max);
            bridge.token(2).approve(swapper, type(uint256).max);
            bridge.token(3).approve(swapper, type(uint256).max);
            bridge.token(4).approve(swapper, type(uint256).max);
            bridge.token(0xDEADC0DE).approve(swapper, type(uint256).max);
        }
    }

    function newLexer(string memory data, uint256 limit) public pure returns (Lexer memory lexer) {
        (uint256 returnValue, JsmnSolLib.Token[] memory tokens, uint256 elements) = JsmnSolLib.parse(data, limit);
        lexer = Lexer(tokens, 0, elements);
    }

    function nextToken(Lexer memory lexer) public pure returns (JsmnSolLib.Token memory token) {
        token = lexer.tokens[lexer.index];
        lexer.index++;
    }

    function call(
        Bridge bridge,
        IRouter.Origin calldata origin,
        bytes calldata salt,
        string calldata program
    ) public payable {
        Lexer memory lexer = newLexer(program, 4096);
        interpretProgram(bridge, origin, salt, program, lexer);
    }

    function parseAssets(
        Bridge bridge,
        Lexer memory lexer,
        string memory program
    ) public returns (IRouter.AssetsAmount[] memory assets) {
        JsmnSolLib.Token memory token;
        nextToken(lexer);
        token = nextToken(lexer);
        uint16 assetsSize = token.size;
        assets = new IRouter.AssetsAmount[](assetsSize);
        for (uint256 i = 0; i < assetsSize; i++) {
            IRouter.AssetsAmount memory asset = assets[i];
            // id: { type: value }
            token = nextToken(lexer);
            asset.id = uint32(uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end))));
            // Object
            nextToken(lexer);
            // Type
            token = nextToken(lexer);
            bytes32 amountType = keccak256(bytes(JsmnSolLib.getBytes(program, token.start, token.end)));
            // Value
            token = nextToken(lexer);
            if (amountType == keccak256("fixed") || amountType == keccak256("Fixed")) {
                asset.amount = uint128(
                    uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end)))
                );
            } else if (amountType == keccak256("ratio") || amountType == keccak256("Ratio")) {
                uint32 ratio = uint32(
                    uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end)))
                );
                uint256 balance = bridge.balanceOf(address(this), asset.id);
                asset.amount = uint128((balance / 100) * ratio);
                console.log(asset.amount);
            } else {
                revert("invalid amount type");
            }
        }
    }

    function interpretTransfer(
        Bridge bridge,
        IRouter.Origin calldata origin,
        bytes calldata salt,
        string calldata program,
        Lexer memory lexer,
        address to,
        address from
    ) public {
        JsmnSolLib.Token memory token;
        // Object
        token = nextToken(lexer);
        uint16 assets = token.size;
        IRouter.AssetsAmount memory asset;
        for (uint256 i = 0; i < assets; i++) {
            // id: { type: value }
            token = nextToken(lexer);
            asset.id = uint32(uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end))));
            // Object
            nextToken(lexer);
            // Type
            token = nextToken(lexer);
            bytes32 amountType = keccak256(bytes(JsmnSolLib.getBytes(program, token.start, token.end)));
            // Value
            token = nextToken(lexer);
            if (amountType == keccak256("fixed") || amountType == keccak256("Fixed")) {
                asset.amount = uint128(
                    uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end)))
                );
            } else if (amountType == keccak256("ratio") || amountType == keccak256("Ratio")) {
                uint32 ratio = uint32(
                    uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end)))
                );
                uint256 balance = bridge.balanceOf(address(this), asset.id);
                asset.amount = uint128((balance * ratio) / 100);
            } else {
                revert("invalid amount type");
            }
            bridge.token(asset.id).transfer(to, asset.amount);
            bridge.transferred(origin, salt, to, from, asset);
        }
    }

    function interpretCall(
        Bridge bridge,
        IRouter.Origin calldata origin,
        bytes calldata salt,
        string calldata program,
        Lexer memory lexer
    ) public payable {
        JsmnSolLib.Token memory token;
        token = nextToken(lexer);
        // encoded: payload
        nextToken(lexer);
        token = nextToken(lexer);
        uint16 payloadSize = token.size;
        bytes memory payload = new bytes(payloadSize);
        for (uint256 j; j < payloadSize; j++) {
            token = nextToken(lexer);
            payload[j] = bytes1(
                uint8(uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end))))
            );
        }
        (address target, bytes memory data) = abi.decode(payload, (address, bytes));
        (bool success, bytes memory result) = target.call(data);
        bridge.called(origin, salt, target, data);
    }

    function interpretSpawn(
        Bridge bridge,
        IRouter.Origin calldata origin,
        bytes calldata salt,
        string calldata program,
        Lexer memory lexer
    ) public {
        JsmnSolLib.Token memory token;
        // Object
        nextToken(lexer);
        // network: value
        nextToken(lexer);
        token = nextToken(lexer);
        uint32 network = uint8(uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end))));
        // salt: [values]
        nextToken(lexer);
        token = nextToken(lexer);
        uint16 saltSize = token.size;
        bytes memory childSalt = new bytes(saltSize);
        for (uint256 i; i < saltSize; i++) {
            token = nextToken(lexer);
            childSalt[i] = bytes1(
                uint8(uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end))))
            );
        }
        // assets: { id: amount... }
        IRouter.AssetsAmount[] memory assets = parseAssets(bridge, lexer, program);
        for (uint256 i = 0; i < assets.length; i++) {
            bridge.burn(address(this), assets[i].id, assets[i].amount);
        }
        nextToken(lexer);
        token = nextToken(lexer);
        string memory childProgram = JsmnSolLib.getBytes(program, token.start, token.end);
        bridge.spawn(origin, salt, network, assets, childSalt, childProgram);
    }

    function interpretProgram(
        Bridge bridge,
        IRouter.Origin calldata origin,
        bytes calldata salt,
        string calldata program,
        Lexer memory lexer
    ) public {
        JsmnSolLib.Token memory token;
        // Object
        nextToken(lexer);
        // Tag => tag: value
        nextToken(lexer);
        token = nextToken(lexer);
        uint16 tagSize = token.size;
        for(uint256 i; i < tagSize; i++) {
            token = nextToken(lexer);
        }
        // Instructions => [Instruction]
        nextToken(lexer);
        token = nextToken(lexer);
        uint16 instructions = token.size;
        for(uint256 i; i < instructions; i++) {
            // Object
            nextToken(lexer);
            // Type
            token = nextToken(lexer);
            bytes32 instructionType = keccak256(bytes(JsmnSolLib.getBytes(program, token.start, token.end)));
            if (instructionType == keccak256("transfer")) {
                // Object
                nextToken(lexer);
                // to: address
                nextToken(lexer);
                token = nextToken(lexer);
                uint16 addrSize = token.size;
                if(addrSize != 20) {
                    revert("address must be 20 bytes");
                }
                bytes memory addrBytes = new bytes(20);
                for (uint256 j; j < addrSize; j++) {
                    token = nextToken(lexer);
                    addrBytes[j] = bytes1(
                        uint8(uint256(JsmnSolLib.parseInt(JsmnSolLib.getBytes(program, token.start, token.end))))
                    );
                }
                address to = address(uint160(bytes20(addrBytes)));
                // assets: ...
                token = nextToken(lexer);
                interpretTransfer(bridge, origin, salt, program, lexer, to, address(this));
            } else if (instructionType == keccak256("call")) {
                interpretCall(bridge, origin, salt, program, lexer);
            } else if (instructionType == keccak256("spawn")) {
                interpretSpawn(bridge, origin, salt, program, lexer);
            } else {
                revert("invalid instruction type");
            }
        }
        bridge.succeeded(origin, salt);
    }
}
