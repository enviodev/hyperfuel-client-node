use skar_format_fuel::Hex;

use crate::types::{as_i64, Block, Input, Output, Receipt, Transaction};

#[napi(object)]
#[derive(Debug, Clone)]
pub struct QueryResponseTyped {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so
    /// the caller should continue the query from this block if they
    /// didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data
    pub data: QueryResponseDataTyped,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct QueryResponseDataTyped {
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<Receipt>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl From<skar_client_fuel::QueryResponseTyped> for QueryResponseTyped {
    fn from(r: skar_client_fuel::QueryResponseTyped) -> Self {
        let archive_height = r.archive_height.map(|u| u as i64);
        let next_block = r.next_block as i64;
        let total_execution_time = r.total_execution_time as i64;
        let data = QueryResponseDataTyped {
            blocks: r.data.blocks.into_iter().map(|b| b.into()).collect(),
            transactions: r.data.transactions.into_iter().map(|b| b.into()).collect(),
            receipts: r.data.receipts.into_iter().map(|b| b.into()).collect(),
            inputs: r.data.inputs.into_iter().map(|b| b.into()).collect(),
            outputs: r.data.outputs.into_iter().map(|b| b.into()).collect(),
        };

        Self {
            archive_height,
            next_block,
            total_execution_time,
            data,
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct LogResponse {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so
    /// the caller should continue the query from this block if they
    /// didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data
    pub data: Vec<LogContext>,
}

/// Contains all the fields needed for decoding plus some additional fields
/// for context.

#[napi(object)]
#[derive(Debug, Clone)]
pub struct LogContext {
    pub block_height: i64,
    pub tx_id: String,
    pub receipt_index: i64,
    pub receipt_type: u8,
    pub contract_id: Option<String>,
    pub root_contract_id: Option<String>,
    pub ra: Option<i64>,
    pub rb: Option<i64>,
    pub rc: Option<i64>,
    pub rd: Option<i64>,
    pub pc: Option<i64>,
    pub is: Option<i64>,
    pub ptr: Option<i64>,
    pub len: Option<i64>,
    pub digest: Option<String>,
    pub data: Option<String>,
}

impl From<skar_client_fuel::LogResponse> for LogResponse {
    fn from(r: skar_client_fuel::LogResponse) -> Self {
        let archive_height = r.archive_height.map(|h| h as i64);
        let next_block = r.next_block as i64;
        let total_execution_time = r.total_execution_time as i64;
        let data = r
            .data
            .into_iter()
            .map(|c| LogContext {
                block_height: as_i64(c.block_height),
                tx_id: c.tx_id.encode_hex(),
                receipt_index: as_i64(c.receipt_index),
                receipt_type: c.receipt_type.to_u8(),
                contract_id: c.contract_id.map(|i| i.encode_hex()),
                root_contract_id: c.root_contract_id.map(|i| i.encode_hex()),
                ra: c.ra.map(as_i64),
                rb: c.rb.map(as_i64),
                rc: c.rc.map(as_i64),
                rd: c.rd.map(as_i64),
                pc: c.pc.map(as_i64),
                is: c.is.map(as_i64),
                ptr: c.ptr.map(as_i64),
                len: c.len.map(as_i64),
                digest: c.digest.map(|i| i.encode_hex()),
                data: c.data.map(|i| i.encode_hex()),
            })
            .collect();
        Self {
            archive_height,
            next_block,
            total_execution_time,
            data,
        }
    }
}
