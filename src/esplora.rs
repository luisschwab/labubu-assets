//! Labubu Assests
//!
//! Esplora
//!
//! Chain-source module for interactions with the Bitcoin blockchain.

use dioxus::prelude::*;

use std::collections::HashMap;

use bitcoin::{Address, Amount, Transaction, Txid};
use esplora_client::{r#async::DefaultSleeper, AsyncClient, Builder};

use crate::error::LabubuError;
use crate::types::{Utxo, UtxoStatus};
use crate::ESPLORA_ENDPOINT;

/// Fee estimates as a map of tuples of target blocks and feerate in `sat/vB`.
pub(crate) type FeeEstimate = HashMap<u16, f64>;

/// Create a new `async` Esplora client.
pub(crate) fn create_esplora_client(url: &str) -> Result<AsyncClient<DefaultSleeper>, LabubuError> {
    Ok(Builder::new(url).build_async()?)
}

/// Fetch fee estimates from the Esplora client.
pub(crate) async fn fetch_fee_estimates(
    client: &AsyncClient<DefaultSleeper>,
) -> Result<FeeEstimate, LabubuError> {
    Ok(client.get_fee_estimates().await?)
}

/// Fetch the balance from an [`Address`] as an [`Amount`] from the Esplora client.
pub(crate) async fn fetch_address_balance(
    client: &AsyncClient,
    address: &Address,
) -> Result<Amount, LabubuError> {
    let response = client.get_address_stats(address).await?;
    let balance = response.chain_stats.funded_txo_sum - response.chain_stats.spent_txo_sum;

    Ok(Amount::from_sat(balance))
}

/// Fetch an [`Address`]es [`Utxo`]s.
///
/// Esplora's spec has no endpoint for this, so we just make a GET
/// request directly to the mempool.space API. Note: if you use another
/// non-mempool endpoint, this will break Labubu Assets.
pub(crate) async fn fetch_address_utxos(
    esplora_endpoint: &String,
    address: &Address,
) -> Result<Vec<Utxo>, LabubuError> {
    let url = format!("{}/address/{}/utxo", esplora_endpoint, address.to_string());

    #[cfg(target_arch = "wasm32")]
    let response = reqwest_wasm::get(url).await?.text().await?;
    #[cfg(not(target_arch = "wasm32"))]
    let response = reqwest::get(url).await?.text().await?;

    let utxos: Vec<Utxo> = serde_json::from_str(&response)?;

    Ok(utxos)
}

/// Broadcast a [`Transaction`] using the Esplora client.
pub(crate) async fn broadcast_tx(
    client: &AsyncClient,
    tx: &Transaction,
) -> Result<(), LabubuError> {
    client.broadcast(tx).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::Network;
    use std::sync::LazyLock;

    const SIGNET_URL: &str = "https://mempool.space/signet/api";
    static SIGNET_ADDRESS: LazyLock<Address> = LazyLock::new(|| {
        "tb1q6nvcwpndfy9wdhf6ts8epwwj7fk0kcap2ka2lw"
            .parse::<Address<_>>()
            .unwrap()
            .require_network(Network::Signet)
            .unwrap()
    });

    #[tokio::test]
    async fn fee_estimates() {
        let client = create_esplora_client(SIGNET_URL).unwrap();
        let fee_estimates = fetch_fee_estimates(&client).await.unwrap();
        println!("{:?}", fee_estimates);
        assert!(!fee_estimates.is_empty());
    }

    #[tokio::test]
    async fn address_balance() {
        let client = create_esplora_client(SIGNET_URL).unwrap();
        let balance = fetch_address_balance(&client, &SIGNET_ADDRESS)
            .await
            .unwrap();
        assert!(balance > Amount::from_sat(0));
    }

    #[tokio::test]
    async fn address_utxos() {
        let utxos: Vec<Utxo> = fetch_address_utxos(&SIGNET_URL.to_string(), &SIGNET_ADDRESS)
            .await
            .unwrap();
        println!("{:?}", utxos);
        assert!(utxos.len() > 0);
    }
}
