echo "Starting Cross Chain Deployer Wasm Deployment"
echo "current directory => $PWD"

echo "Changing current directory to crosschain-deployer cosmwasm Contract"
cd ../rust/
sh scripts/build.sh

cd ../deployments/
echo "current directory => $PWD"

npx ts-node scripts/init.ts 
npx ts-node scripts/set_chain_types.ts
