pragma solidity ^0.8.4;

contract Account {
    address public owner;

    constructor(address _owner) {
        owner = _owner;
    }

    function setOwner(address _owner) public {
        require(msg.sender == owner);
        owner = _owner;
    }

}