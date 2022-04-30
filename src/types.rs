#[derive(Debug, serde::Deserialize)]
pub struct ValidatorRegistrationV1 {
    pub a: i64,
    // feeRecipient: Bytes20,
    // timestamp: u64,
    // pubkey: BLSPubkey,
}

#[derive(Debug, serde::Deserialize)]
pub struct ProposalRequest {
    pub a: i64,
    // slot: Slot,
    // pubkey: BLSPubkey,
    // parentHash: Hash,
}

#[derive(Debug, serde::Serialize)]
pub struct BuilderBidV1 {
    pub a: i64,
    // header: ExecutionPayloadHeader,
    // value: U256,
    // pubkey: BLSPubkey,
}

#[derive(Debug, serde::Deserialize)]
pub struct SignedBlindedBeaconBlock {
    pub a: i64,
    // message: BlindedBeaconBlock,
    // signature: BLSSignature,
}

#[derive(Debug, serde::Serialize)]
pub struct ExecutionPayload {
    pub a: i64,
    // ...
}
