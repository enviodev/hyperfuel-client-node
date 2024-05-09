import { HyperfuelClient, Query } from "hyperfuel-client";

async function main() {
  // Create hyperfuel client using the hypersync-fuel endpoint
  const client = HyperfuelClient.new({
    url: "https://fuel-15.hypersync.xyz"
  });

  const query: Query = {
    "fromBlock": 4105960,
    "toBlock": 4106000,
    "inputs": [
      {
        "owner": ["0x48a0f31c78e1c837ff6a885785ceb7c2090f86ed93db3ed2d8821d13739fe981"]
      }
    ],
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
