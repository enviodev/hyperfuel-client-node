import { HyperfuelClient } from "@envio-dev/hyperfuel-client";

async function main() {
  // Create hyperfuel client using the hyperfuel endpoint
  const client = HyperfuelClient.new({
    url: "https://fuel-15.hypersync.xyz"
  });

  const contracts = ["0xff63ad3cdb5fde197dfa2d248330d458bffe631bda65938aa7ab7e37efa561d0"]
  const from_block = 8076516
  const to_block = 8076517
  const logs = await client.presetQueryGetLogs(contracts, from_block, to_block);

  console.log(`number of logs: ${logs.data.length}`);
  console.log(`logs: ${JSON.stringify(logs.data)}`);

}

main();

