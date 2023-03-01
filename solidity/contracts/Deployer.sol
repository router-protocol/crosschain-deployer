// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import "hardhat/console.sol";

import "evm-gateway-contract/contracts/IGateway.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract CrossChainDeployer {

    address public gateway;

    uint64 public chainID;
    event deployEvent( bytes _code , bytes _decodedPayload , bytes32 _salt , address _contractAddress , bytes32 _digest );

    modifier isGateway (){
        require ( msg.sender == address(gateway) ,"ERC20 : Sender must be gateway Contract ");
        _;
    }

    constructor( address _gateway ) {
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
    ) internal returns ( uint64 , bytes32 , bytes32 , address  ) {
        address addr;
        ( bytes memory decodedPayload , bytes32 salt , bytes32 digest )= abi.decode ( payload , (  bytes, bytes32 , bytes32 ) );
        assembly {
            addr := create2(0, add(decodedPayload, 0x20), mload(decodedPayload), salt)
        }
        emit deployEvent( payload  , decodedPayload , salt , addr , digest );
        if (addr == address(0)){
            revert();
        }
        return ( chainID , digest , salt , addr );
    }

    function handleRequestFromRouter(string memory sender, bytes memory payload) external returns ( uint64 , bytes32 , bytes32 , address ) {
        require(msg.sender == address(gateway), "Only gateway can call this function");
        return deployContract( sender , payload);
    }

}
