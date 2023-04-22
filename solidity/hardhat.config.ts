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

const mnemonic = process.env.MEMNONIC

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
      accounts: {
        initialIndex: 1,
        mnemonic,
        path: "m/44'/60'/0"
      }
    },
    avalancheFujiTestnet: {
      url: "https://api.avax-test.network/ext/bc/C/rpc",
      chainId: 43113,
      accounts: {
        initialIndex: 1,
        mnemonic,
        path: "m/44'/60'/0"
      },
    },
    polygonMumbai: {
      url: process.env.POLYGONTESTNET_RPC || '',
      accounts: {
        initialIndex: 1,
        mnemonic,
        path: "m/44'/60'/0"
      }
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
      bscTestnet : process.env.BSCSCAN_API,
      bsc : process.env.BSCSCAN_API,
      polygonMumbai: process.env.POLYGONSCAN_API,
      avalancheFujiTestnet: process.env.FUJI_ETHERSCAN_API,
    },
  },
};

export default config;
