import * as dotenv from "dotenv";

import { HardhatUserConfig, task } from "hardhat/config";
import "@nomiclabs/hardhat-etherscan";
import "@nomiclabs/hardhat-waffle";
import "@typechain/hardhat";
import "hardhat-gas-reporter";
import "solidity-coverage";
import "hardhat-docgen";

import "./tasks/storeDeployment";

import "./tasks/deploy-crossChainDeployer";

import "./tasks/verify/config-verify-crosschaindeployer"

dotenv.config();

const privateKey = process.env.PRIVATE_KEY;
if (!privateKey) {
  throw new Error("Please set your PRIVATE_KEY in a .env file");
}

const bscTestnet = process.env.BSCSCAN_API;
const bsc = process.env.BSCSCAN_API;
const polygonMumbai = process.env.POLYGONSCAN_API;

if (!bscTestnet || !bsc || !polygonMumbai) {
  throw new Error("Please set your etherscan Keys in a .env file");
}

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.4",
    settings: {
      evmVersion: "berlin",
      metadata: {
        // Not including the metadata hash
        // https://github.com/paulrberg/solidity-template/issues/31
        bytecodeHash: "none",
      },
      // You should disable the optimizer when debugging
      // https://hardhat.org/hardhat-network/#solidity-optimizer-support
      optimizer: {
        enabled: true,
        runs: 500,
      },
    },
  },
  networks: {
    bscTestnet: {
      url: process.env.BSCTESTNET_RPC || '',
      accounts: [privateKey]
    },
    avalancheFujiTestnet: {
      url: "https://api.avax-test.network/ext/bc/C/rpc",
      chainId: 43113,
      accounts: [privateKey]
    },
    polygonMumbai: {
      url: process.env.POLYGONTESTNET_RPC || '',
      accounts: [privateKey]
    },
  },
  docgen: {
    path: './docs',
    clear: true,
    runOnCompile: true,
  },
  gasReporter: {
    enabled: process.env.REPORT_GAS !== undefined,
    currency: "USD",
  },
  etherscan: {
    apiKey: {
      bscTestnet : bscTestnet,
      bsc : bsc,
      polygonMumbai: polygonMumbai
    },
  },
};

export default config;
