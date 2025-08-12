import { ethers } from "hardhat";

async function main() {

  const [onwer] = await  ethers.getSigners();

  console.log("deploying contracts with accont:", onwer.address);

  
  const CWSol = await ethers.getContractFactory("CWSol");


  const cwsol = await CWSol.deploy(onwer.address);
  //await wsol.waitForDeployment();

  console.log("✅ CWSol deployed to:", await cwsol.getAddress());
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
