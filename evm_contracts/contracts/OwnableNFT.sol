// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

// Contract to mint an NFT for an Ownable.
contract OwnableNFT is ERC721, Ownable {
    using Counters for Counters.Counter;
    Counters.Counter private tokenIds;

    mapping(bytes32 => uint256) private ownables;
    string private baseURI;

    constructor(string memory _name, string memory _symbol) ERC721(_name, _symbol) {}

    function mint(address to, bytes32 ownableId) public returns (uint256) {
        tokenIds.increment();
        uint256 id = tokenIds.current();

        _safeMint(to, id);

        return id;
    }

    function burn(bytes32 ownableId) public {
        uint256 id = ownables[ownableId];
        require(id > 0, "No NFT exists for ownable");

        _burn(id);
        delete ownables[ownableId];
    }

    function tokenId(bytes32 ownableId) external view returns (uint256) {
        return ownables[ownableId];
    }

    function ownerOfOwnable(bytes32 ownableId) external view returns (address) {
        uint256 id = ownables[ownableId];
        require(id > 0, "No NFT exists for ownable");

        return ownerOf(id);
    }

    // Set the location for the NFT images
    function setBaseURI(string calldata _uri) public onlyOwner {
        baseURI = _uri;
    }
}
