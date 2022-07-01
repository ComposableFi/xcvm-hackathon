// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol";

contract BridgedToken is ERC20PresetMinterPauser {
    constructor(string memory symbol) ERC20PresetMinterPauser(symbol, symbol) {
        _setupRole(MINTER_ROLE, _msgSender());
        _setupRole(PAUSER_ROLE, _msgSender());
    }

    function burn(address account, uint256 amount) external {
        require(hasRole(MINTER_ROLE, _msgSender()), "BridgedToken: must have minter role to mint");
        _burn(account, amount);
    }
}
