import { ethers } from "hardhat";

async function main() {

  const [onwer] = await  ethers.getSigners();

  console.log("deploying contracts with accont:", onwer.address);

  
  const WSol = await ethers.getContractFactory("WSol");


  const wsol = await WSol.deploy(onwer.address);
  //await wsol.waitForDeployment();

  console.log("✅ WSol deployed to:", await wsol.getAddress());
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
