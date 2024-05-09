import { HyperfuelClient, Query } from "hyperfuel-client";

async function main() {
  // Create hyperfuel client using the hypersync-fuel endpoint
  const client = HyperfuelClient.new({
    url: "https://fuel-15.hypersync.xyz"
  });

  const query: Query = {
    "fromBlock": 7980000,
    "toBlock": 7980100,
    "inputs": [
      {
        "assetId": ["0x0000000000000000000000000000000000000000000000000000000000000000"]
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
        "asset_id"
      ]
    }
  }

  const res = await client.getSelectedData(query);

  console.log(`inputs: ${JSON.stringify(res.data.inputs)}`);

}

main();
