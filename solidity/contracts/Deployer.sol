// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import "hardhat/console.sol";

import "evm-gateway-contract/contracts/IGateway.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract CrossChainDeployer {

    address public gateway;

    event deployEvent( bytes _code , bytes _decodedPayload , bytes32 _salt , address _contractAddress );

    modifier isGateway (){
        require ( msg.sender == address(gateway) ,"ERC20 : Sender must be gateway Contract ");
        _;
    }

    constructor( address _gateway ) {
        gateway = _gateway;
    }

    //Factory Fx
    function deployContract(
        string memory sender,
        bytes memory payload
    ) internal{
        address addr;
        bytes32 salt = keccak256(abi.encodePacked(block.number , payload , sender  ));
        bytes memory decodedPayload = abi.decode ( payload , (bytes) );
        assembly {
            addr := create2(0, add(decodedPayload, 0x20), mload(decodedPayload), salt)
        }
        emit deployEvent( payload  , decodedPayload , salt , addr );
    }

    function handleRequestFromRouter(string memory sender, bytes memory payload) external {
        require(msg.sender == address(gateway), "Only gateway can call this function");
        deployContract( sender , payload);
    }

}
