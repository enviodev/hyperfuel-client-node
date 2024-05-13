use anyhow::{anyhow, Context, Result};

mod config;
mod query;
mod response;
mod types;

use config::Config;
use query::Query;
use response::{LogResponse, QueryResponseTyped};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct HyperfuelClient {
    inner: hyperfuel_client::Client,
}

#[napi]
impl HyperfuelClient {
    /// Create a new client with given config
    #[napi]
    pub fn new(cfg: Config) -> napi::Result<HyperfuelClient> {
        env_logger::try_init().ok();

        Self::new_impl(cfg).map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    fn new_impl(cfg: Config) -> Result<HyperfuelClient> {
        let cfg = cfg.try_convert().context("parse config")?;

        let inner = hyperfuel_client::Client::new(cfg).context("build client")?;

        Ok(HyperfuelClient { inner })
    }

    /// Get the height of the source hyperfuel instance
    #[napi]
    pub async fn get_height(&self) -> napi::Result<i64> {
        let height = self
            .inner
            .get_height()
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

        Ok(height.try_into().unwrap())
    }

    /// Get the height of the source hyperfuel instance
    /// Internally calls get_height.
    /// On an error from the source hyperfuel instance, sleeps for
    /// 1 second (increasing by 1 each failure up to max of 5 seconds)
    /// and retries query until success.
    #[napi]
    pub async fn get_height_with_retry(&self) -> napi::Result<i64> {
        let height = self
            .inner
            .get_height_with_retry()
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;

        Ok(height.try_into().unwrap())
    }

    /// Create a parquet file by executing a query.
    ///
    /// Path should point to a folder that will contain the parquet files in the end.
    #[napi]
    pub async fn create_parquet_folder(&self, query: Query, path: String) -> napi::Result<()> {
        self.create_parquet_folder_impl(query, path)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn create_parquet_folder_impl(&self, query: Query, path: String) -> Result<()> {
        let query = query.try_convert().context("parse query")?;

        self.inner
            .create_parquet_folder(query, path)
            .await
            .context("create parquet folder")?;

        Ok(())
    }

    /// Send a query request to the source hyperfuel instance.
    ///
    /// Returns a query response which contains typed data.
    ///
    /// NOTE: this query returns loads all transactions that your match your receipt, input, or output selections
    /// and applies the field selection to all these loaded transactions.  So your query will return the data you
    /// want plus additional data from the loaded transactions.  This functionality is in case you want to associate
    /// receipts, inputs, or outputs with eachother.
    #[napi]
    pub async fn get_data(&self, query: Query) -> napi::Result<QueryResponseTyped> {
        self.get_data_impl(query)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn get_data_impl(&self, query: Query) -> Result<QueryResponseTyped> {
        let query = query.try_convert().context("parse query")?;
        let resp = self.inner.get_data(&query).await.context("get data")?;
        Ok(resp.into())
    }

    /// Send a query request to the source hyperfuel instance.
    ///
    /// Returns a query response that which contains structured data that doesn't include any inputs, outputs,
    /// and receipts that don't exactly match the query's input, outout, or receipt selection.
    #[napi]
    pub async fn get_selected_data(&self, query: Query) -> napi::Result<QueryResponseTyped> {
        self.get_selected_data_impl(query)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn get_selected_data_impl(&self, query: Query) -> Result<QueryResponseTyped> {
        let query = query.try_convert().context("parse query")?;
        let resp = self
            .inner
            .get_selected_data(&query)
            .await
            .context("get data")?;
        Ok(resp.into())
    }

    /// Send a query request to the source hyperfuel instance.
    ///
    /// Returns all log and logdata receipts of logs emitted by any of the specified contracts
    /// within the block range.
    /// If no 'to_block' is specified, query will run to the head of the chain.
    /// Returned data contains all the data needed to decode Fuel Log or LogData
    /// receipts as well as some extra data for context.  This query doesn't return any logs that
    /// were a part of a failed transaction.
    ///
    /// NOTE: this function is experimental and might be removed in future versions.
    #[napi]
    pub async fn preset_query_get_logs(
        &self,
        emitting_contracts: Vec<String>,
        from_block: i64,
        to_block: Option<i64>,
    ) -> napi::Result<LogResponse> {
        self.preset_query_get_logs_impl(emitting_contracts, from_block, to_block)
            .await
            .map_err(|e| napi::Error::from_reason(format!("{:?}", e)))
    }

    async fn preset_query_get_logs_impl(
        &self,
        emitting_contracts: Vec<String>,
        from_block: i64,
        to_block: Option<i64>,
    ) -> Result<LogResponse> {
        // cut the "0x" off the address
        let mut emitting_contracts_args = vec![];
        for contract_address in emitting_contracts {
            let address: &str = if &contract_address[..2] == "0x" {
                &contract_address[2..]
            } else {
                &contract_address
            };
            let address = hex_str_address_to_byte_array(address)
                .context(format!("convert address {}", address))?;
            emitting_contracts_args.push(address)
        }

        let from_block = from_block as u64;
        let to_block = to_block.map(|i| i as u64);

        let resp = self
            .inner
            .preset_query_get_logs(emitting_contracts_args, from_block, to_block)
            .await
            .context("get logs")?;
        Ok(resp.into())
    }
}

// helper function to decode hex string as address
fn hex_str_address_to_byte_array(hex_str: &str) -> Result<[u8; 32]> {
    if hex_str.len() != 64 {
        return Err(anyhow!("address must be 64 hex characters".to_owned()));
    }

    let mut dst = [0u8; 32];
    match faster_hex::hex_decode(hex_str.as_bytes(), &mut dst) {
        Ok(()) => Ok(dst),
        Err(e) => Err(anyhow!("Failed to decode hex string: {}", e)),
    }
}
