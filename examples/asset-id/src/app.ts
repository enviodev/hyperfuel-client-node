import { HyperfuelClient, Query } from "@envio-dev/hyperfuel-client";

async function main() {
  const client = HyperfuelClient.new({
    url: "https://fuel-testnet.hypersync.xyz"
  });

  const query: Query = {
    // start query from block 0
    "fromBlock": 0,
    // if to_block is not set, query runs to the end of the chain
    "toBlock": 1300000,
    // which inputs to load data from
    "inputs": [
      {
        "assetId": ["0x2a0d0ed9d2217ec7f32dcd9a1902ce2a66d68437aeff84e3a3cc8bebee0d2eea"]
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
        "asset_id"
      ]
    }
  }

  const res = await client.getSelectedData(query);

  console.log(`inputs: ${JSON.stringify(res.data.inputs)}`);

}

main();
