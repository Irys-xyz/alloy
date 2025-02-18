use crate::{BlobsBundleV1, ExecutionPayloadV3};
use alloy_primitives::{B256, U256};
use irys_primitives::{ShadowReceipt, Shadows};
use serde::{Deserialize, Serialize};

/// Irys V1 Execution Envelope Payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayloadEnvelopeV1Irys {
    /// Execution payload V3
    pub execution_payload: ExecutionPayloadV1Irys,
    /// The expected value to be received by the feeRecipient in wei
    pub block_value: U256,
    /// The blobs, commitments, and proofs associated with the executed payload.
    pub blobs_bundle: BlobsBundleV1,
    /// Introduced in V3, this represents a suggestion from the execution layer if the payload
    /// should be used instead of an externally provided one.
    pub should_override_builder: bool,
    // pub is_empty: bool, // pub shadows: Shadows,
    /// Vector of Shadow Receipts.
    pub shadow_receipts: Vec<ShadowReceipt>,
}

/// Irys V1 Execution Payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayloadV1Irys {
    /// Inner V4 payload
    #[serde(flatten)]
    pub payload_inner: ExecutionPayloadV3,

    // /// Array of hex [`u64`] representing blob gas used, enabled with V3
    // /// See <https://github.com/ethereum/execution-apis/blob/fe8e13c288c592ec154ce25c534e26cb7ce0530d/src/engine/cancun.md#ExecutionPayloadV3>
    // #[serde(with = "alloy_serde::u64_via_ruint")]
    // pub blob_gas_used: u64,
    // /// Array of hex[`u64`] representing excess blob gas, enabled with V3
    // /// See <https://github.com/ethereum/execution-apis/blob/fe8e13c288c592ec154ce25c534e26cb7ce0530d/src/engine/cancun.md#ExecutionPayloadV3>
    // #[serde(with = "alloy_serde::u64_via_ruint")]
    // pub excess_blob_gas: u64,
    /// shadows
    pub shadows: Option<Shadows>,
    /// RLP proof root for shadows for quick payload validation
    pub shadows_root: B256,
}
