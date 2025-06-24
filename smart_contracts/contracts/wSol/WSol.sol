// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract WSol is ERC20, Ownable {
    constructor(address initialOwner) ERC20("Wrapped SOL", "wSOL") Ownable(initialOwner) {}
    
    // Override decimals to match SOL (9 decimals instead of default 18)
    function decimals() public pure override returns (uint8) {
        return 9;
    }
    
    function deposit() public payable {
        // Convert from 18 decimals (wei) to 9 decimals (wSOL)
        // This maintains 1:1 ratio between ETH and wSOL
        uint256 wsolAmount = msg.value / 10**9;
        _mint(msg.sender, wsolAmount);
    }
    
    function withdraw(uint256 amount) public {
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        _burn(msg.sender, amount);
        
        // Convert from 9 decimals (wSOL) back to 18 decimals (wei)
        uint256 ethAmount = amount * 10**9;
        payable(msg.sender).transfer(ethAmount);
    }
    
    mapping(bytes32 => bool) public processedTxs;
    
    function mint(address to, uint256 amount, bytes32 solanaTxHash) public onlyOwner {
        require(!processedTxs[solanaTxHash], "Transaction already processed");
        processedTxs[solanaTxHash] = true;
        _mint(to, amount);
    }
    event Burned(address indexed user, uint256 amount);

    function burn( uint256 amount) public  {
        _burn(msg.sender, amount);
        emit Burned(msg.sender, amount);
    }
    
    receive() external payable {
        deposit();
    }
}