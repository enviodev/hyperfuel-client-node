import { HyperfuelClient, Query } from "@envio-dev/hyperfuel-client";

async function main() {
  const client = HyperfuelClient.new({
    url: "https://fuel-testnet.hypersync.xyz"
  });

  const query: Query = {
    // start query from block 0
    "fromBlock": 0,
    // if to_block is not set, query runs to the end of the chain
    "toBlock": 1427625,
    // which inputs to load data from
    "inputs": [
      {
        "owner": ["0x94a8e322ff02baeb1d625e83dadf5ec88870ac801da370d4b15bbd5f0af01169"]
      }
    ],
    // what data we want returned from the inputs we loaded
    "fieldSelection": {
      "input": [
        "tx_id",
        "block_height",
        "input_type",
        "utxo_id",
        "owner",
        "amount",
        "asset_id",
        "predicate_gas_used",
        "predicate",
        "predicate_data"
      ]
    }
  }

  const res = await client.getSelectedData(query);

  console.log(`inputs: ${JSON.stringify(res.data.inputs)}`);

}

main();
