# Crosschain Deployer

## Components 

- Deployer contracts on EVM Chains 
- Deployer Contracts on Router Chain 

## Installation 

Download git repo

    https://github.com/router-protocol/crosschain-deployer

Compile crosschain Deployer Rust contracts using ( Mac )

    cd rust && docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer-arm64:0.12.6

Above command will generate wasm file in artifacts. 
        
Compile Crosschain Solidity Contracts using 

    cd solidity && yarn install 
    npx hardhat clean && npx hardhat clean 


## Deployer Contracts on Router Chain 

### Upload and instantiate contracts 

Upload and instantiate contract on router station using following instantiate message 

    { "owner" : "<ownerAddress>" }

Where ownerAddress is owner address on router chain and will be owner of the initated crosschain deployer contract on routerchain

### Deploy CrossChain Deployer on EVM chains 

after setting .env parameters on solidity folder deploy Deployer Contracts on EVM chains using following commands 

    npx hardhat DEPLOY_CROSSCHAIN_DEPLOYER --network <NETWORK NAME>

The Deployed address will be stored on deployments/deployment.json file. Additional networks can be added on to hardhat config.

### Register Deployer on router Chain 

Using router station use following commands to register deployer on router chain contracts 

    {
        "register_deployer":{
            "address":"0x65264210b86Fe3Fd8017D74B7125f57036d20514",
            "chainid":80001
        }
    }

Where address and chainid can be changed to exact specification.
Note - This function can be triggeed only by owner. 

### Deploying crosschain Contracts 

Using following command from router station a evm contracts can be created on desired chains 

    {
        "deploy_contract": {
            "code": "0x6080604052348015600f57600080fd5b50601680601d6000396000f3fe6080604052600080fdfea164736f6c6343000804000a",
            "chainids": [ 80001, 97 ],
            "gas_price": [ 15000000000 ,15000000000 ],
            "gas_limit": [ 1000000 , 1000000 ] 
        }
    }

    {
        "deploy_contract": {
            "code": "<CONTRACT BYTE CODE>",
            "salt": "0x9d51687d04a49f2f9df398db0dedd78c9e543c8919f0c0024d04cd0ee8a87062",
            "constructor_args": [
                "< Contract Contractor arguments>",
            ],
            "chainids": [
                80001
            ],
            "chain_types" :[
                "CHAIN_TYPE_EVM"
            ],
            "gas_limit": [
                30000000
            ],
            "gas_prices":[
                300000000000
            ],
            "forwarder_contract" : "router1d4sad30uj59lxg56ylwn7457v8z4k5m3323r9u360q85w8ga3kfsfxrgc6"
        }
    }


Where bytecode can change as per user need. Gas_price and gas_limit are specified in array corresponding to the respective chainid. 

### Generation of bytecode for contract which has constructor parameters

THere can be instances where contract would need to be broadcasted which has constructor parameters attached to it. 
This is edge case which can be factored into using following method of ethers js .

```
let ConstuctorParams = ethers.utils.defaultAbiCoder.encode([< Data Types of consturctor elements>],[< Array of Constuctor Elements ]).slice(2)
let ExampleParams = ethers.utils.defaultAbiCoder.encode(['address'],['0x33B4A007EcC80Bc99578c18Da07da704c5403236']).slice(2) // Assuming constructor element has address 
let DeployedBytecode = `${contractByteCode}${ConstuctorParams}`
console.log(DeployedBytecode);

```

Use deployed bytecode generated by this method in deploying contracts with constructor paramters.

### Fetching Deployer Address 

Use following commands to fetch deployer address from router chain Contract 

```json
{
  "fetch_deployer": {
    "chainid": 97
  }
}
```

### Fetch deployed Contact on EVM Chain 

In Deployer Contract on respective EVM chain We can find deployEvent which has deployed Bytecode parameter which would correspond to our bytecode and newly generated contract address will also be given over there. 