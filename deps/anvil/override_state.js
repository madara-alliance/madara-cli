/// Taken from https://github.com/madara-alliance/madara/blob/main/orchestrator/scripts/init_state.js#L212
/// override the state on the core contract
const starknet = require("starknet");
const ethers = require("ethers");

// Using default anvil key which has funds
const MADARA_ORCHESTRATOR_ETHEREUM_PRIVATE_KEY =
  "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const eth_provider = new ethers.JsonRpcProvider("http://anvil:8545");
const wallet = new ethers.Wallet(
  MADARA_ORCHESTRATOR_ETHEREUM_PRIVATE_KEY,
  eth_provider
);

const starknet_provider = new starknet.RpcProvider({
  nodeUrl: "http://madara:9945",
});

// Due to restrictions in SNOS at the moment (as it's designed for Sepolia right now),
// we need to skip the starting few blocks from running on SNOS.
// This function overrides the state on the core contract to the block after which we
// can run SNOS
async function overrideStateOnCoreContract(
  block_number,
  core_contract_address
) {
  let state_update = await starknet_provider.getStateUpdate(block_number);
  let abi = [
    {
      type: "function",
      name: "updateStateOverride",
      inputs: [
        {
          name: "globalRoot",
          type: "uint256",
          internalType: "uint256",
        },
        {
          name: "blockNumber",
          type: "int256",
          internalType: "int256",
        },
        {
          name: "blockHash",
          type: "uint256",
          internalType: "uint256",
        },
      ],
      outputs: [],
      stateMutability: "nonpayable",
    },
  ];

  const contract = new ethers.Contract(core_contract_address, abi, wallet);
  const tx = await contract.updateStateOverride(
    state_update.new_root,
    block_number,
    state_update.block_hash
  );
  const receipt = await tx.wait();
  if (!receipt.status) {
    console.log("❌ Failed to override state on core contract");
    process.exit(1);
  }
  console.log("✅ Successfully overridden state on core contract");
}

async function main() {
  let block_number = await starknet_provider.getBlockNumber();
  let core_contract_address = "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0"
  await overrideStateOnCoreContract(block_number, core_contract_address)
}

// Call the main function and handle any potential errors
main().catch((error) => {
  console.error("Error running main function:", error);
});
