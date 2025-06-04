// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";



contract WSol is ERC20, Ownable {
    constructor(address initialOwner) ERC20("Wrapped SOL", "wSOL") Ownable(initialOwner) {}
    
    
    function deposit() public payable {
        _mint(msg.sender, msg.value);
    }

    function withdraw(uint256 amount) public {
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        _burn(msg.sender, amount);
        payable(msg.sender).transfer(amount);
    }

    mapping(bytes32 => bool) public processedTxs;

    function mint(address to, uint256 amount, bytes32 solanaTxHash) public onlyOwner {
        require(!processedTxs[solanaTxHash], "Transaction already processed");
        
         processedTxs[solanaTxHash] = true;
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) public onlyOwner {
        _burn(from, amount);
    }

    receive() external payable {
        deposit();
    }
}
