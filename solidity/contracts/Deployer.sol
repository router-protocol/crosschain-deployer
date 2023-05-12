// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import "@routerprotocol/evm-gateway-contracts/contracts/IGateway.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract CrossChainDeployer {
    address public gateway;

    uint64 public chainID;
    event deployEvent(
        bytes _code,
        bytes _decodedPayload,
        bytes32 _salt,
        address _contractAddress,
        bytes32 _digest,
        string sender
    );

    modifier isGateway() {
        require(
            msg.sender == address(gateway),
            "ERC20 : Sender must be gateway Contract "
        );
        _;
    }

    constructor(address _gateway) {
        gateway = _gateway;
        uint256 id;
        assembly {
            id := chainid()
        }
        chainID = uint64(id);
    }

    //Factory Fx
    function deployContract(
        string memory sender,
        bytes memory payload
    ) internal returns (uint64, bytes32, bytes32, address) {
        address addr;
        (bytes memory decodedPayload, bytes32 salt, bytes32 digest) = abi
            .decode(payload, (bytes, bytes32, bytes32));
        assembly {
            addr := create2(
                0,
                add(decodedPayload, 0x20),
                mload(decodedPayload),
                salt
            )
        }
        emit deployEvent(payload, decodedPayload, salt, addr, digest, sender);
        if (addr == address(0)) {
            revert();
        }
        return (chainID, digest, salt, addr);
    }

    function iReceive(
        string memory requestSender,
        bytes memory packet,
        string memory srcChainId
    ) external returns (uint64, bytes32, bytes32, address) {
        require(
            msg.sender == address(gateway),
            "Only gateway can call this function"
        );
        require(
            keccak256(abi.encodePacked(srcChainId)) ==
                keccak256(abi.encodePacked("router_9000-1")),
            "String should not be empty"
        );
        (
            uint64 cId,
            bytes32 digest,
            bytes32 salt,
            address addr
        ) = deployContract(requestSender, packet);
        return (cId, digest, salt, addr);
    }
}
