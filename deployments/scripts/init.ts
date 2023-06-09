import fs from "fs";
import dotenv from "dotenv";
import { init_wasm_code } from "./instantiate_msg";
import { upload_wasm_code } from "./upload_wasm";
import { Network, PrivateKey } from "@routerprotocol/router-chain-sdk-ts";
dotenv.config();

async function main() {
  let network = Network.AlphaDevnet;
  if (process.env.ENV == "devnet") {
    network = Network.Devnet;
  } else if (process.env.ENV == "testnet") {
    network = Network.Testnet;
  } else if (process.env.ENV == "mainnet") {
    network = Network.Mainnet;
  } else if (process.env.ENV && process.env.ENV != "alpha-devnet") {
    throw new Error("Please set your NETWORK in the .env file");
  }

  const privateKeyHash = process.env.PRIVATE_KEY;

  if (!privateKeyHash) {
    throw new Error("Please set your PRIVATE_KEY in the .env file");
  }
  const privateKey = PrivateKey.fromPrivateKey(privateKeyHash);
  const owner = privateKey.toBech32();

  let wasmSuffix = ".wasm";
  if (process.env.IS_APPLE_CHIPSET == "YES" ) {
    wasmSuffix = "-aarch64.wasm"
  }
  const crossChainDeployerFilePath = "config/cross-chain-deployer.json";
  const crossChainDeployerSetup = JSON.parse(
    fs.readFileSync(crossChainDeployerFilePath, "utf-8")
  );
  console.log("Present Deployment Details -> ", crossChainDeployerSetup[network]);

  const crossChainDeployerCodeId = await upload_wasm_code(
    network,
    privateKeyHash,
    "../rust/artifacts/router_crosschain_deployer" + wasmSuffix
  );

  const crossChainDeployerInitMsg = JSON.stringify({
    owner: owner,
  });
  
  const crossChainDeployerAddr = await init_wasm_code(
    crossChainDeployerCodeId,
    "Cross Chain Deployer",
    crossChainDeployerInitMsg
  );
  console.log("crossChainDeployerAddr", crossChainDeployerAddr);


  console.log("admin ->", owner);
  console.log(
    "CrossChain Deployer -> code_id-",
    crossChainDeployerCodeId,
    "addr-",
    crossChainDeployerAddr
  );
  
  if (!crossChainDeployerSetup[network]) {
    crossChainDeployerSetup[network] = {};
  }
  crossChainDeployerSetup[network]["crossChainDeployer"] = {
    addr: crossChainDeployerAddr,
    code_id: crossChainDeployerCodeId,
  };
  fs.writeFileSync(crossChainDeployerFilePath, JSON.stringify(crossChainDeployerSetup));
}

main();
