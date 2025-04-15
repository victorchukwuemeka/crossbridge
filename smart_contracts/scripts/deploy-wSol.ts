import { ethers } from "hardhat";  
import { expect } from "chai";  

describe("WSol", function () {  // Start of the test suite for the WSol contract.
  it("should mint wSOL when ETH is deposited", async function () {  // A single test case to check if the wSOL is minted when ETH is deposited.
    const [owner] = await ethers.getSigners();  // Get the signers (accounts) available in the Hardhat environment. Here we take the first account as 'owner'.
    
    const WSol = await ethers.getContractFactory("WSol");  // Get the contract factory for the WSol contract (this allows us to deploy the contract).
    const wsol = await WSol.deploy();  // Deploy the WSol contract. This sends a transaction to deploy the contract to the blockchain.
    await wsol.waitForDeployment();  // Wait for the deployment transaction to be mined and the contract to be fully deployed.

    // Send 1 ETH to the contract from the 'owner' address.
    // We are calling the contract's deposit function implicitly by sending ETH to it.
    await owner.sendTransaction({
      to: await wsol.getAddress(),  // Get the address of the deployed WSol contract to send ETH to it.
      value: ethers.parseEther("1"),  // Send 1 ETH to the contract. We use parseEther to convert "1" to the proper wei amount.
    });
    
    //owners address balance 
    const balance = await wsol.balanceOf(owner.address); 

    
    // Assert that the balance of the 'owner' address is 1 wSOL (since 1 ETH was deposited).
    expect(balance).to.equal(ethers.parseEther("1"));
  });
});
