use hyperfuel_format::{Hex, UInt};
use napi::bindgen_prelude::BigInt;

/// The block header contains metadata about a certain block.

#[napi(object)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Block {
    /// String of the header
    pub id: String,
    /// The block height for the data availability layer up to which (inclusive) input messages are processed.
    pub da_height: i64,
    pub consensus_parameters_version: i64,
    pub state_transition_bytecode_version: i64,
    /// The number of transactions in the block.
    pub transactions_count: String,
    /// The number of receipt messages in the block.
    pub message_receipt_count: String,
    /// The merkle root of the transactions in the block.
    pub transactions_root: String,
    pub message_outbox_root: String,
    pub event_inbox_root: String,
    /// The block height.
    pub height: i64,
    /// The merkle root of all previous consensus header Stringes (not including this block).
    pub prev_root: String,
    /// The timestamp for the block.
    pub time: i64,
    /// The String of the serialized application header for this block.
    pub application_hash: String,
}

/// An object containing information about a transaction.
#[napi(object)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// block the transaction is in.
    pub block_height: i64,
    /// A unique transaction id.
    pub id: String,
    /// An array of asset ids used for the transaction inputs.
    pub input_asset_ids: Option<Vec<String>>,
    // Contract object -> bincode into schema
    /// An array of contracts used for the transaction inputs.
    pub input_contracts: Option<Vec<String>>,
    /// A contract used for the transaction input.
    /// A unique 32 byte identifier for the UTXO for a contract used for the transaction input.
    pub input_contract_utxo_id: Option<String>,
    /// The root of amount of coins owned by contract before transaction execution for a contract used for the transaction input.
    pub input_contract_balance_root: Option<String>,
    /// The state root of contract before transaction execution for a contract used for the transaction input.
    pub input_contract_state_root: Option<String>,
    /// A pointer to the TX whose output is being spent for a contract used for the transaction input.
    pub input_contract_tx_pointer_block_height: Option<i64>,
    /// A pointer to the TX whose output is being spent for a contract used for the transaction input.
    pub input_contract_tx_pointer_tx_index: Option<i64>,
    /// The contract id for a contract used for the transaction input.
    pub input_contract: Option<String>,
    pub policies_tip: Option<i64>,
    pub policies_witness_limit: Option<i64>,
    pub policies_maturity: Option<i64>,
    pub policies_max_fee: Option<i64>,
    pub script_gas_limit: Option<i64>,
    /// The minimum block height that the transaction can be included at.
    pub maturity: Option<i64>,
    /// The amount minted in the transaction.
    pub mint_amount: Option<i64>,
    /// The asset ID for coins minted in the transaction.
    pub mint_asset_id: Option<String>,
    pub mint_gas_price: Option<i64>,
    /// The location of the transaction in the block.
    pub tx_pointer_block_height: Option<i64>,
    pub tx_pointer_tx_index: Option<i64>,
    /// Script, creating a new contract, or minting new coins
    pub tx_type: u8,
    /// The index of the input from a transaction that changed the state of a contract.
    pub output_contract_input_index: Option<i64>,
    /// The root of amount of coins owned by contract after transaction execution from a transaction that changed the state of a contract.
    pub output_contract_balance_root: Option<String>,
    /// The state root of contract after transaction execution from a transaction that changed the state of a contract.
    pub output_contract_state_root: Option<String>,
    /// An array of witnesses.
    pub witnesses: Option<String>,
    /// The root of the receipts.
    pub receipts_root: Option<String>,
    /// The status type of the transaction.
    pub status: u8,
    /// for SubmittedStatus, SuccessStatus, and FailureStatus, the time a transaction was submitted, successful, or failed
    pub time: i64,
    /// for SuccessStatus, the state of the program execution
    // pub program_state: Option<ProgramState>
    /// for SqueezedOutStatus & FailureStatus, the reason the transaction was squeezed out or failed
    pub reason: Option<String>,
    /// The script to execute.
    pub script: Option<String>,
    /// The script input parameters.
    pub script_data: Option<String>,
    /// The witness index of contract bytecode.
    pub bytecode_witness_index: Option<i64>,
    pub bytecode_root: Option<String>,
    pub subsection_index: Option<i64>,
    pub subsections_number: Option<i64>,
    pub proof_set: Option<String>,
    pub consensus_parameters_upgrade_purpose_witness_index: Option<i64>,
    pub consensus_parameters_upgrade_purpose_checksum: Option<String>,
    pub state_transition_upgrade_purpose_root: Option<String>,
    /// The salt value for the transaction.
    pub salt: Option<String>,
}

/// An object representing all possible types of receipts.
#[napi(object)]
#[derive(Debug, Default, Clone)]
pub struct Receipt {
    /// Index of the receipt in the block
    pub receipt_index: i64,
    /// Contract that produced the receipt
    pub root_contract_id: Option<String>,
    /// transaction that this receipt originated from
    pub tx_id: String,
    /// The status type of the transaction this receipt originated from
    pub tx_status: u8,
    /// block that the receipt originated in
    pub block_height: i64,
    /// The value of the program counter register $pc, which is the memory address of the current instruction.
    pub pc: Option<String>,
    /// The value of register $is, which is the pointer to the start of the currently-executing code.
    pub is: Option<String>,
    /// The recipient contract
    pub to: Option<String>,
    /// The recipient address
    pub to_address: Option<String>,
    /// The amount of coins transferred.
    pub amount: Option<BigInt>,
    /// The asset id of the coins transferred.
    pub asset_id: Option<String>,
    /// The gas used for the transaction.
    pub gas: Option<i64>,
    /// The first parameter for a CALL receipt type, holds the function selector.
    pub param1: Option<BigInt>,
    /// The second parameter for a CALL receipt type, typically used for the user-specified input to the ABI function being selected.
    pub param2: Option<BigInt>,
    /// The value of registers at the end of execution, used for debugging.
    pub val: Option<BigInt>,
    /// The value of the pointer register, used for debugging.
    pub ptr: Option<BigInt>,
    /// A 32-byte String of MEM[$rC, $rD]. The syntax MEM[x, y] means the memory range starting at byte x, of length y bytes.
    pub digest: Option<String>,
    /// The decimal string representation of an 8-bit unsigned integer for the panic reason. Only returned if the receipt type is PANIC.
    pub reason: Option<i64>,
    /// The value of register $rA.
    pub ra: Option<BigInt>,
    /// The value of register $rB.
    pub rb: Option<BigInt>,
    /// The value of register $rC.
    pub rc: Option<BigInt>,
    /// The value of register $rD.
    pub rd: Option<BigInt>,
    /// The length of the receipt.
    pub len: Option<BigInt>,
    /// The type of receipt.
    pub receipt_type: u8,
    /// 0 if script exited successfully, any otherwise.
    pub result: Option<i64>,
    /// The amount of gas consumed by the script.
    pub gas_used: Option<i64>,
    /// The receipt data.
    pub data: Option<String>,
    /// The address of the message sender.
    pub sender: Option<String>,
    /// The address of the message recipient.
    pub recipient: Option<String>,
    /// The nonce value for a message.
    pub nonce: Option<String>,
    /// Current context if in an internal context. null otherwise
    pub contract_id: Option<String>,
    /// The sub id.
    pub sub_id: Option<String>,
}

/// An object representing all possible types of inputs.  InputCoin, InputContract, InputMessage
#[napi(object)]
#[derive(Debug, Default, Clone)]
pub struct Input {
    /// transaction that this input originated from
    pub tx_id: String,
    /// The status type of the transaction this input originated from
    pub tx_status: u8,
    /// block that the input originated in
    pub block_height: i64,
    /// InputCoin, InputContract, or InputMessage
    pub input_type: u8,
    /// A unique 32 byte identifier for the UTXO.
    pub utxo_id: Option<String>,
    /// The owning address or predicate root.
    pub owner: Option<String>,
    /// for InputCoin type: The amount of coins.
    /// for InputMessage type: The amount sent in the message.
    pub amount: Option<BigInt>,
    /// The asset ID of the coins.
    pub asset_id: Option<String>,
    /// A pointer to the transaction whose output is being spent.
    pub tx_pointer_block_height: Option<i64>,
    pub tx_pointer_tx_index: Option<i64>,
    /// The index of the witness that authorizes spending the coin.
    pub witness_index: Option<i64>,
    /// The amount of gas used in the predicate transaction.
    pub predicate_gas_used: Option<i64>,
    /// The predicate bytecode.
    pub predicate: Option<String>,
    /// The predicate input parameters.
    pub predicate_data: Option<String>,
    /// The root of amount of coins owned by contract before transaction execution.
    pub balance_root: Option<String>,
    /// The state root of contract before transaction execution.
    pub state_root: Option<String>,
    /// The input contract.
    pub contract: Option<String>,
    /// The sender address of the message.
    pub sender: Option<String>,
    /// The recipient address of the message.
    pub recipient: Option<String>,
    /// A nonce value for the message input, which is determined by the sending system and is published at the time the message is sent.
    pub nonce: Option<String>,
    /// The message data.
    pub data: Option<String>,
}

/// An object representing all possible types of Outputs. CoinOutput, ContractOutput, ChangeOutput, VariableOutput, ContractCreated
#[napi(object)]
#[derive(Debug, Default, Clone)]
pub struct Output {
    /// transaction that this out originated from
    pub tx_id: String,
    /// The status type of the transaction this output originated from
    pub tx_status: u8,
    /// block that the output originated in
    pub block_height: i64,
    /// CoinOutput, ContractOutput, ChangeOutput, VariableOutput, or ContractCreated
    pub output_type: u8,
    /// The address the coins were sent to.
    pub to: Option<String>,
    /// The amount of coins in the output.
    pub amount: Option<BigInt>,
    /// The asset id for the coins sent.
    pub asset_id: Option<String>,
    /// The index of the input.
    pub input_index: Option<i64>,
    /// The root of amount of coins owned by contract after transaction execution.
    pub balance_root: Option<String>,
    /// for ContractedCreated type: The initial state root of contract.
    /// for ContractOutput type: The state root of contract after transaction execution.
    pub state_root: Option<String>,
    /// for ContractCreated type: The contract that was created.
    pub contract: Option<String>,
}

impl From<hyperfuel_format::BlockHeader> for Block {
    fn from(b: hyperfuel_format::BlockHeader) -> Self {
        Self {
            id: b.id.encode_hex(),
            da_height: as_i64(b.da_height),
            transactions_count: b.transactions_count.encode_hex(),
            message_receipt_count: b.message_receipt_count.encode_hex(),
            transactions_root: b.transactions_root.encode_hex(),
            height: as_i64(b.height),
            prev_root: b.prev_root.encode_hex(),
            time: as_i64(b.time),
            application_hash: b.application_hash.encode_hex(),
            consensus_parameters_version: as_i64(b.consensus_parameters_version),
            state_transition_bytecode_version: as_i64(b.state_transition_bytecode_version),
            message_outbox_root: b.message_outbox_root.encode_hex(),
            event_inbox_root: b.event_inbox_root.encode_hex(),
        }
    }
}

impl From<hyperfuel_format::Transaction> for Transaction {
    fn from(t: hyperfuel_format::Transaction) -> Self {
        Self {
            block_height: as_i64(t.block_height),
            id: t.id.encode_hex(),
            input_asset_ids: t
                .input_asset_ids
                .map(|d| d.into_iter().map(|i| i.encode_hex()).collect()),
            input_contracts: t
                .input_contracts
                .map(|d| d.into_iter().map(|i| i.encode_hex()).collect()),
            input_contract_utxo_id: t.input_contract_utxo_id.map(|d| d.encode_hex()),
            input_contract_balance_root: t.input_contract_balance_root.map(|d| d.encode_hex()),
            input_contract_state_root: t.input_contract_state_root.map(|d| d.encode_hex()),
            input_contract_tx_pointer_block_height: t
                .input_contract_tx_pointer_block_height
                .map(as_i64),
            input_contract_tx_pointer_tx_index: t.input_contract_tx_pointer_tx_index.map(as_i64),
            input_contract: t.input_contract.map(|d| d.encode_hex()),
            maturity: t.maturity.map(as_i64),
            mint_amount: t.mint_amount.map(as_i64),
            mint_asset_id: t.mint_asset_id.map(|d| d.encode_hex()),
            tx_pointer_block_height: t.tx_pointer_block_height.map(as_i64),
            tx_pointer_tx_index: t.tx_pointer_tx_index.map(as_i64),
            tx_type: t.tx_type.to_u8(),
            output_contract_input_index: t.output_contract_input_index.map(as_i64),
            output_contract_balance_root: t.output_contract_balance_root.map(|d| d.encode_hex()),
            output_contract_state_root: t.output_contract_state_root.map(|d| d.encode_hex()),
            witnesses: t.witnesses.map(|d| d.encode_hex()),
            receipts_root: t.receipts_root.map(|d| d.encode_hex()),
            status: t.status.as_u8(),
            time: as_i64(t.time),
            reason: t.reason,
            script: t.script.map(|d| d.encode_hex()),
            script_data: t.script_data.map(|d| d.encode_hex()),
            bytecode_witness_index: t.bytecode_witness_index.map(as_i64),
            salt: t.salt.map(|d| d.encode_hex()),
            policies_tip: t.policies_tip.map(as_i64),
            policies_witness_limit: t.policies_witness_limit.map(as_i64),
            policies_maturity: t.policies_maturity.map(as_i64),
            policies_max_fee: t.policies_max_fee.map(as_i64),
            script_gas_limit: t.script_gas_limit.map(as_i64),
            mint_gas_price: t.mint_amount.map(as_i64),
            bytecode_root: t.bytecode_root.map(|r| r.encode_hex()),
            subsection_index: t.subsection_index.map(as_i64),
            subsections_number: t.subsections_number.map(as_i64),
            proof_set: t.proof_set.map(|p| p.encode_hex()),
            consensus_parameters_upgrade_purpose_witness_index: t
                .consensus_parameters_upgrade_purpose_witness_index
                .map(as_i64),
            consensus_parameters_upgrade_purpose_checksum: t
                .consensus_parameters_upgrade_purpose_checksum
                .map(|a| a.encode_hex()),
            state_transition_upgrade_purpose_root: t
                .state_transition_upgrade_purpose_root
                .map(|a| a.encode_hex()),
        }
    }
}

impl From<hyperfuel_format::Receipt> for Receipt {
    fn from(r: hyperfuel_format::Receipt) -> Self {
        Self {
            receipt_index: as_i64(r.receipt_index),
            root_contract_id: r.root_contract_id.map(|d| d.encode_hex()),
            tx_id: r.tx_id.encode_hex(),
            tx_status: r.tx_status.as_u8(),
            block_height: as_i64(r.block_height),
            pc: r.pc.map(|x| x.to_string()),
            is: r.is.map(|x| x.to_string()),
            to: r.to.map(|d| d.encode_hex()),
            to_address: r.to_address.map(|d| d.encode_hex()),
            amount: r.amount.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            asset_id: r.asset_id.map(|d| d.encode_hex()),
            gas: r.gas.map(as_i64),
            param1: r.param1.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            param2: r.param2.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            val: r.val.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            ptr: r.ptr.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            digest: r.digest.map(|d| d.encode_hex()),
            reason: r.reason.map(as_i64),
            ra: r.ra.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            rb: r.rb.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            rc: r.rc.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            rd: r.rd.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            len: r.len.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            receipt_type: r.receipt_type.to_u8(),
            result: r.result.map(as_i64),
            gas_used: r.gas_used.map(as_i64),
            data: r.data.map(|d| d.encode_hex()),
            sender: r.sender.map(|d| d.encode_hex()),
            recipient: r.recipient.map(|d| d.encode_hex()),
            nonce: r.nonce.map(|d| d.encode_hex()),
            contract_id: r.contract_id.map(|d| d.encode_hex()),
            sub_id: r.sub_id.map(|d| d.encode_hex()),
        }
    }
}

impl From<hyperfuel_format::Input> for Input {
    fn from(i: hyperfuel_format::Input) -> Self {
        Self {
            tx_id: i.tx_id.encode_hex(),
            tx_status: i.tx_status.as_u8(),
            block_height: as_i64(i.block_height),
            input_type: i.input_type.as_u8(),
            utxo_id: i.utxo_id.map(|d| d.encode_hex()),
            owner: i.owner.map(|d| d.encode_hex()),
            amount: i.amount.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            asset_id: i.asset_id.map(|d| d.encode_hex()),
            tx_pointer_block_height: i.tx_pointer_block_height.map(as_i64),
            tx_pointer_tx_index: i.tx_pointer_tx_index.map(as_i64),
            witness_index: i.witness_index.map(as_i64),
            predicate_gas_used: i.predicate_gas_used.map(as_i64),
            predicate: i.predicate.map(|d| d.encode_hex()),
            predicate_data: i.predicate_data.map(|d| d.encode_hex()),
            balance_root: i.balance_root.map(|d| d.encode_hex()),
            state_root: i.state_root.map(|d| d.encode_hex()),
            contract: i.contract.map(|d| d.encode_hex()),
            sender: i.sender.map(|d| d.encode_hex()),
            recipient: i.recipient.map(|d| d.encode_hex()),
            nonce: i.nonce.map(|d| d.encode_hex()),
            data: i.data.map(|d| d.encode_hex()),
        }
    }
}

impl From<hyperfuel_format::Output> for Output {
    fn from(o: hyperfuel_format::Output) -> Self {
        Self {
            tx_id: o.tx_id.encode_hex(),
            tx_status: o.tx_status.as_u8(),
            block_height: as_i64(o.block_height),
            output_type: o.output_type.as_u8(),
            to: o.to.map(|d| d.encode_hex()),
            amount: o.amount.map(|x| {
                let as_u64: u64 = *x;
                let big: BigInt = as_u64.into();
                big
            }),
            asset_id: o.asset_id.map(|d| d.encode_hex()),
            input_index: o.input_index.map(as_i64),
            balance_root: o.balance_root.map(|d| d.encode_hex()),
            state_root: o.state_root.map(|d| d.encode_hex()),
            contract: o.contract.map(|d| d.encode_hex()),
        }
    }
}

pub fn as_i64(uint: UInt) -> i64 {
    let cast: u64 = uint.into();
    cast as i64
}
