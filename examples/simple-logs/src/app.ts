import { HyperfuelClient } from "@envio-dev/hyperfuel-client";

async function main() {
  const client = HyperfuelClient.new({
    url: "https://fuel-testnet.hypersync.xyz"
  });

  // contract(s) we want logs from
  const contracts = ["0x4a2ce054e3e94155f7092f7365b212f7f45105b74819c623744ebcc5d065c6ac"]

  // get logs from blocks 0(inclusive) to 1627509(exclusive)
  const logs = await client.presetQueryGetLogs(contracts, 0, 1627509);

  console.log(`number of logs: ${logs.data.length}`);
  console.log(`logs: ${JSON.stringify(logs.data)}`);

}

main();

