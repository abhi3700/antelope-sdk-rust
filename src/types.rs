//! All the data types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResourceLimits {
    pub used: u32,
    pub available: u32,
    pub max: u32,
    pub last_usage_update_time: String,
    pub current_used: u32,
}

#[derive(Debug, Deserialize)]
pub struct Key {
    pub key: String,
    pub weight: u64,
}

#[derive(Debug, Deserialize)]
pub struct Wait {
    pub wait_sec: u64,
    pub weight: u64,
}

#[derive(Debug, Deserialize)]
pub struct AccountPermission {
    pub actor: String,
    pub permission: String,
}

#[derive(Debug, Deserialize)]
pub struct Account2 {
    pub weight: u64,
    pub permission: AccountPermission,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub threshold: u32,
    pub keys: Vec<Key>,
    pub accounts: Vec<Account2>,
    pub waits: Vec<Wait>,
}

#[derive(Debug, Deserialize)]
pub struct Permission {
    // TODO: add enum with Active, Owner type instead of string
    pub perm_name: String,
    pub parent: String,
    pub required_auth: Auth,
    pub linked_actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResourceOverview {
    pub owner: String,
    pub net_weight: String,
    pub cpu_weight: String,
    pub ram_bytes: u32,
}

#[derive(Debug, Deserialize)]
pub struct ResourceDelegation {
    pub from: String,
    pub to: String,
    pub net_weight: String,
    pub cpu_weight: String,
}

#[derive(Debug, Deserialize)]
pub struct RefundRequest;

#[derive(Debug, Deserialize)]
pub struct RexInfo;

#[derive(Debug, Deserialize)]
pub struct VoterInfo {
    pub owner: String,
    pub proxy: String,
    pub producers: Vec<String>,
    pub staked: u64,
    pub last_vote_weight: String,
    pub proxied_vote_weight: String,
    pub is_proxy: u32,
    pub flags1: u32,
    pub reserved2: u32,
    pub reserved3: String,
}

#[derive(Debug, Deserialize)]
pub struct ProducerAuthorityList {
    pub threshold: u32,
    pub keys: Vec<Key>,
}

#[derive(Debug, Deserialize)]
pub struct Producer {
    producer_name: String,
    authority: Vec<ProducerAuthorityList>,
}

#[derive(Debug, Deserialize)]
pub struct ProducerSchedule {
    version: u32,
    producers: Vec<Producer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Executed,
    SoftFail,
    HardFail,
    Delayed,
    Expired,
}

#[derive(Debug, Deserialize)]
pub struct Authorization {
    actor: String,
    permission: String,
}

// TODO: try out with different responses and check for its
// consistent field.
#[derive(Debug, Deserialize)]
pub struct ActionData {
    // miner: String,
    // nonce: u32,
    // entry: u32,
}

#[derive(Debug, Deserialize)]
pub struct Action {
    account: String,
    name: String,
    authorization: Vec<Authorization>,
    data: ActionData,
    hex_data: String,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    expiration: String,
    ref_block_num: u32,
    ref_block_prefix: u32,
    max_net_usage_words: u32,
    max_cpu_usage_ms: u32,
    delay_sec: u32,
    context_free_actions: Vec<String>,
    actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
pub struct Trx {
    id: String,
    signatures: Vec<String>,
    compression: String,
    packed_context_free_data: String,
    context_free_data: Vec<String>,
    packed_trx: String,
    transaction: Transaction,
}

// TODO: need to check for its consistent struct fields with different blocks
#[derive(Debug, Deserialize)]
pub struct TransactionReceipt {
    status: TransactionStatus,
    cpu_usage_us: u64,
    // net_usage_us: u64,
    trx: Trx,
}

#[derive(Debug, Deserialize)]
pub struct ProtocolFeature;

#[derive(Debug, Deserialize)]
pub struct Block {
    timestamp: String,
    producer: String,
    confirmed: u32,
    previous: String,
    transaction_mroot: String,
    action_mroot: String,
    schedule_version: u32,
    new_producers: Option<ProducerSchedule>,
    // header_extensions: Vec<u32>,
    // new_protocol_features: Vec<ProtocolFeature>,
    producer_signature: String,
    transactions: Vec<TransactionReceipt>,
    // block_extensions: Vec<u32>,
    id: String,
    block_num: u32,
    ref_block_prefix: u32,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub account_name: String,
    pub head_block_num: u32,
    pub head_block_time: String,
    pub privileged: bool,
    pub last_code_update: String,
    pub created: String,
    pub core_liquid_balance: String,
    pub ram_quota: u32,
    pub net_weight: u32,
    pub cpu_weight: u32,
    pub net_limit: ResourceLimits,
    pub cpu_limit: ResourceLimits,
    pub ram_usage: u32,
    pub permissions: Vec<Permission>,
    pub total_resources: ResourceOverview,
    pub self_delegated_bandwidth: Option<ResourceDelegation>,
    pub refund_request: Option<RefundRequest>,
    pub voter_info: Option<VoterInfo>,
    pub rex_info: RexInfo,
    pub subjective_cpu_bill_limit: ResourceLimits,
    pub eosio_any_linked_actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChainInfo {
    pub server_version: String,
    pub chain_id: String,
    pub head_block_num: u32,
    pub last_irreversible_block_num: u32,
    pub last_irreversible_block_id: String,
    pub head_block_id: String,
    pub head_block_time: String,
    pub head_block_producer: String,
    pub virtual_block_cpu_limit: u32,
    pub virtual_block_net_limit: u32,
    pub block_cpu_limit: u32,
    pub block_net_limit: u32,
    pub server_version_string: String,
    pub fork_db_head_block_num: u32,
    pub fork_db_head_block_id: String,
    pub server_full_version_string: String,
    pub total_cpu_weight: String,
    pub total_net_weight: String,
    pub earliest_available_block_num: u32,
    pub last_irreversible_block_time: String,
}
