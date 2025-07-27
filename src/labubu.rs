//! Labubu Assets
//!
//! Labubu module
//!
//! This module builds a `Labubu Asset` from a funded address generated in the user's browser.

use bitcoin::{
    absolute,
    hashes::Hash,
    opcodes::all::OP_CHECKSIG,
    script::Builder,
    sighash::{Prevouts, SighashCache},
    taproot::{LeafVersion, NodeInfo, TaprootSpendInfo},
    Amount, ScriptBuf, TapLeafHash, TapNodeHash, TapSighashType, Transaction, TxOut,
    XOnlyPublicKey,
};
use secp256k1::{Keypair, Message, Secp256k1};

pub fn mint(
    pubkey: XOnlyPublicKey,
    amount: u64,
    destination_address: bitcoin::Address,
    fee: u64,
    inputs: Vec<bitcoin::TxIn>,
    prev_txouts: Vec<TxOut>,
    spend_info: TaprootSpendInfo,
    keypair: Keypair,
) -> Result<Transaction, bitcoin::taproot::TaprootBuilderError> {
    let secp = Secp256k1::new();

    let mut tx_outs = Vec::new();
    tx_outs.push(TxOut {
        value: Amount::from_sat(amount - fee),
        script_pubkey: destination_address.script_pubkey(),
    });

    let mut unsigned_tx: Transaction = Transaction {
        version: bitcoin::transaction::Version(2),
        lock_time: absolute::LockTime::ZERO,
        input: inputs,
        output: tx_outs,
    };

    let spend_script = spend_script(pubkey.into());

    let unsigned_tx_clone = unsigned_tx.clone();

    let tap_leaf_hash = TapLeafHash::from_script(&spend_script, LeafVersion::TapScript);

    for input in unsigned_tx.input.iter_mut() {
        let sighash = SighashCache::new(&unsigned_tx_clone)
            .taproot_script_spend_signature_hash(
                0,
                &Prevouts::All(&prev_txouts),
                tap_leaf_hash,
                TapSighashType::Default,
            )
            .expect("failed to construct sighash");

        let message = Message::from(sighash);
        let sig = secp.sign_schnorr_no_aux_rand(&message, &keypair);
        let script_ver = (spend_script.clone(), LeafVersion::TapScript);
        let ctrl_block = spend_info.control_block(&script_ver).unwrap();

        input.witness.push(sig.serialize());
        input.witness.push(script_ver.0.into_bytes());
        input.witness.push(ctrl_block.serialize());
    }
    Ok(unsigned_tx)
}

pub fn create_control_block_address(
    pubkey: XOnlyPublicKey,
    payload_bytes: Vec<u8>,
) -> Result<TaprootSpendInfo, bitcoin::taproot::TaprootBuilderError> {
    let secp = Secp256k1::new();
    let script = spend_script(pubkey);

    let mut root_node = NodeInfo::new_leaf_with_ver(script.clone(), LeafVersion::TapScript);

    let merkle_path = build_merkle_path_from_bytes(&payload_bytes);

    for sibling_hash in &merkle_path {
        let sibling_node = NodeInfo::new_hidden_node(*sibling_hash);
        root_node = NodeInfo::combine(root_node, sibling_node)?;
    }

    let taproot_spend_info = TaprootSpendInfo::from_node_info(&secp, pubkey, root_node);

    Ok(taproot_spend_info)
}

fn build_merkle_path_from_bytes(bytes: &[u8]) -> Vec<TapNodeHash> {
    let mut padded = bytes.to_vec();
    while padded.len() % 32 != 0 {
        padded.push(0);
    }

    padded
        .chunks(32)
        .map(|chunk| TapNodeHash::from_byte_array(chunk.try_into().unwrap()))
        .collect()
}

pub fn spend_script(pubkey: XOnlyPublicKey) -> ScriptBuf {
    Builder::new()
        .push_x_only_key(&pubkey)
        .push_opcode(OP_CHECKSIG)
        .into_script()
}

/// TODO(@luisschwab): remove later (sepc256k1 test)
#[cfg(test)]
mod tests {
    use secp256k1::{rand::thread_rng, Keypair, SecretKey, SECP256K1};

    #[test]
    fn secp256k1() {
        let mut rng = thread_rng();

        let sk = SecretKey::new(&mut rng);
        let keypair = Keypair::from_secret_key(SECP256K1, &sk);
        let pk = keypair.public_key();
        println!("{:?}", sk);
        println!("{:?}", pk);
    }
}
