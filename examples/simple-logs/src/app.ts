import { HypersyncClient } from "@envio-dev/hypersync-fuel-client";

async function main() {
  // Create hypersync client using the mainnet hypersync endpoint
  const client = HypersyncClient.new({
    url: "https://fuel-15.hypersync.xyz",
  });

  // TODO

  await client.preset_query_get_logs();

  console.log("finished writing parquet");
}

main();
