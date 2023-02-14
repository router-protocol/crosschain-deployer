import { Contract } from 'ethers'
import { task, types } from 'hardhat/config'

import {DEPLOY_CROSSCHAIN_DEPLOYER, STORE_DEPLOYMENTS} from './task-names'

task(DEPLOY_CROSSCHAIN_DEPLOYER, 'Deploy CrossChain Deployer Contract ')
  .setAction(async (taskArgs, { ethers }): Promise<Contract> => {
    const hre = require('hardhat')
    const contractName = 'CrossChainDeployer'
    const config = require("../config/config.json");
    const networkID = await ethers.provider.getNetwork();

    const C1 = await ethers.getContractFactory(contractName)
    const C1i = await C1.deploy( config.gateway[networkID.chainId] );
    await C1i.deployed()
    console.log(`Contract ${contractName} has been deployed to: ${C1i.address}`)

    await hre.run(STORE_DEPLOYMENTS, { contractname: contractName, contractaddress: C1i.address })

    return C1i
  })

// npx hardhat DEPLOY_CROSSCHAIN_DEPLOYER --network bscTestnet  && npx hardhat DEPLOY_CROSSCHAIN_DEPLOYER --network polygonMumbai
// npx hardhat VERIFY_CROSSCHAIN_DEPLOYER --network bscTestnet  && npx hardhat VERIFY_CROSSCHAIN_DEPLOYER --network polygonMumbai