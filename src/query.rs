use core::fmt;

use anyhow::{Context, Result};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[napi(object)]
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ReceiptSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_contract_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_type: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "vec_u64_as_string")]
    pub ra: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "vec_u64_as_string")]
    pub rb: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "vec_u64_as_string")]
    pub rc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "vec_u64_as_string")]
    pub rd: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct InputSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<Vec<u8>>,
}

#[napi(object)]
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct OutputSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<Vec<u8>>,
}

#[napi(object)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FieldSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Query {
    /// The block to start the query from
    pub from_block: i64,
    /// The block to end the query at. If not specified, the query will go until the
    ///  end of data. Exclusive, the returned range will be [from_block..to_block).
    ///
    /// The query will return before it reaches this target block if it hits the time limit
    ///  configured on the server. The user should continue their query by putting the
    ///  next_block field in the response into from_block field of their next query. This implements
    ///  pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<i64>,
    /// List of receipt selections, the query will return receipts that match any of these selections and
    ///  it will return receipts that are related to the returned objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Vec<ReceiptSelection>>,
    /// List of input selections, the query will return inputs that match any of these selections and
    ///  it will return inputs that are related to the returned objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<InputSelection>>,
    /// List of output selections, the query will return outputs that match any of these selections and
    ///  it will return outputs that are related to the returned objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<OutputSelection>>,
    /// Whether to include all blocks regardless of if they are related to a returned transaction or log. Normally
    ///  the server will return only the blocks that are related to the transaction or logs in the response. But if this
    ///  is set to true, the server will return data for all blocks in the requested range [from_block, to_block).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_blocks: Option<bool>,
    /// Field selection. The user can select which fields they are interested in, requesting less fields will improve
    ///  query execution time and reduce the payload size so the user should always use a minimal number of fields.
    pub field_selection: FieldSelection,
    /// Maximum number of blocks that should be returned, the server might return more blocks than this number but
    ///  it won't overshoot by too much.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_blocks: Option<i64>,
    /// Maximum number of transactions that should be returned, the server might return more transactions than this number but
    ///  it won't overshoot by too much.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_transactions: Option<i64>,
}

impl Query {
    pub fn try_convert(&self) -> Result<hyperfuel_net_types::Query> {
        let json = serde_json::to_vec(self).context("serialize to json")?;
        serde_json::from_slice(&json).context("parse json")
    }
}

impl TryFrom<hyperfuel_net_types::Query> for Query {
    type Error = anyhow::Error;

    fn try_from(hyperfuel_query: hyperfuel_net_types::Query) -> Result<Self> {
        let json = serde_json::to_vec(&hyperfuel_query).context("serialize query to json")?;
        serde_json::from_slice(&json).context("parse json")
    }
}

mod vec_u64_as_string {
    use super::*;

    pub fn serialize<S>(opt: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(vec) = opt {
            let u64_vec: Vec<u64> = vec.iter().map(|s| s.parse().unwrap_or(0)).collect();
            serializer.serialize_some(&u64_vec)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VecU64Visitor;

        impl<'de> Visitor<'de> for VecU64Visitor {
            type Value = Option<Vec<u64>>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an optional vector of u64")
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                let vec: Vec<u64> = Deserialize::deserialize(deserializer)?;
                Ok(Some(vec))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        let opt = deserializer.deserialize_option(VecU64Visitor)?;
        Ok(opt.map(|vec| vec.iter().map(|&num| num.to_string()).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_of_receipt_selection() {
        let selection: ReceiptSelection = ReceiptSelection {
            root_contract_id: None,
            to_address: None,
            asset_id: None,
            receipt_type: None,
            sender: None,
            recipient: None,
            contract_id: None,
            ra: Some(vec!["1234".into()]),
            rb: Some(vec!["21".into(), "9999999999".into()]),
            rc: Some(vec![format!("{}", u64::MAX)]),
            rd: None,
        };

        let se = serde_json::to_string(&selection).unwrap();

        let de: ReceiptSelection = serde_json::from_str(&se).unwrap();

        assert_eq!(de.ra, Some(vec!["1234".into()]));
        assert_eq!(de.rb, Some(vec!["21".into(), "9999999999".into()]));
        assert_eq!(de.rc, Some(vec![format!("{}", u64::MAX)]));
        assert_eq!(de.rd, None);
    }

    #[test]
    fn test_serialization_into_net_types() {
        let selection: ReceiptSelection = ReceiptSelection {
            root_contract_id: None,
            to_address: None,
            asset_id: None,
            receipt_type: None,
            sender: None,
            recipient: None,
            contract_id: None,
            ra: Some(vec!["1234".into()]),
            rb: Some(vec!["21".into(), "9999999999".into()]),
            rc: Some(vec![format!("{}", u64::MAX)]),
            rd: None,
        };

        let json = serde_json::to_vec(&selection).unwrap();
        let net_type: hyperfuel_net_types::ReceiptSelection =
            serde_json::from_slice(&json).unwrap();

        assert_eq!(net_type.ra, vec![1234]);
        assert_eq!(net_type.rb, vec![21, 9999999999]);
        assert_eq!(net_type.rc, vec![u64::MAX]);
    }
}
