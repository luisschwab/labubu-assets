//! Labubu Assets
//!
//! Types module

use bitcoin::{Amount, BlockHash, Txid};
use serde::{Deserialize, Serialize};

/// A type that models the status of a UTXO.
// #[derive(Debug, Serialize, Deserialize)]
// pub(crate) struct UtxoStatus {
//     pub(crate) confirmed: bool,
//     // pub(crate) block_height: u32,
//     pub(crate) block_hash: BlockHash,
//     pub(crate) block_time: u32,
// }

/// A type that models an UTXO.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Utxo {
    pub(crate) txid: Txid,
    pub(crate) vout: u32,
    // pub(crate) status: UtxoStatus,
    pub(crate) value: Amount,
}
