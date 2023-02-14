import { Contract } from "ethers";
import { task, types } from "hardhat/config";

import {VERIFY_CROSSCHAIN_DEPLOYER} from "../task-names";

task(VERIFY_CROSSCHAIN_DEPLOYER, "Verify CrossChain Deployer Contract").setAction(
  async (taskArgs, { ethers }): Promise<null> => {
    const hre = require("hardhat");
    const config = require("../../config/config.json");
    const deployedContracts = require("../../deployments/deployment.json");
    const networkID = await ethers.provider.getNetwork();
    const contractName = "CrossChainDeployer";
    const contractAddress = deployedContracts[networkID.chainId][contractName];
    let constructorArg: any[];

    constructorArg = [config.gateway[networkID.chainId]];

    await hre.run("verify:verify", {
      address: contractAddress,
      constructorArguments: constructorArg,
    });
    return null;
  }
);
