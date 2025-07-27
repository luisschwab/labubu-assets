//! Labubu Assets
//!
//! Labubu Assets home page.

use std::str::FromStr;

use dioxus::prelude::*;

use bitcoin::{Address, Amount, Network, OutPoint, Sequence, Transaction, TxIn, TxOut, Txid};
use esplora_client::{deserialize, AsyncClient};
use hex::FromHex;
use secp256k1::{rand::thread_rng, Keypair, SecretKey, SECP256K1};
use web_sys::window;

use crate::esplora::{broadcast_tx, create_esplora_client, fetch_address_utxos};
use crate::labubu::{create_control_block_address, mint};
use crate::labubu_maker::labubu_maker;
use crate::ESPLORA_ENDPOINT;

#[component]
pub fn Home() -> Element {
    // Save some stuff to local storage:
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let storage_c1 = storage.clone();
    let storage_c2 = storage.clone();
    let storage_c3 = storage.clone();

    let mut sk_string = use_signal(|| "nil".to_string());
    let mut deposit_address_string = use_signal(|| "nil".to_string());
    let mut destination_address_input = use_signal(|| String::new());
    let mut destination_address_string = use_signal(|| "nil".to_string());
    let mut image_uri = use_signal(|| "data:image/png;base64,".to_string());
    let mut labubu_txid = use_signal(|| "".to_string());

    // Load the private key from local storage.
    use_effect(move || {
        if let Ok(Some(sk)) = storage.get("SecretKey") {
            sk_string.set(sk);
        }
        if let Ok(Some(deposit)) = storage.get("DepositAddress") {
            deposit_address_string.set(deposit);
        }
        if let Ok(Some(destination)) = storage.get("DestinationAddress") {
            destination_address_string.set(destination);
        }
    });

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-indigo-500 to-purple-600 p-8",

            div { class: "max-w-4xl mx-auto bg-white/95 backdrop-blur-sm rounded-3xl p-8 shadow-2xl",

                div { class: "text-center mb-12",
                    h1 { class: "text-5xl font-bold mb-3 bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent",
                        "🎯 Labubu Assets"
                    }
                    p { class: "text-xl text-gray-600", "Labubu Assets™ Mint" }
                }

                // Secret Key Section
                div { class: "bg-gradient-to-r from-blue-50 to-indigo-50 rounded-2xl p-6 mb-8 border border-blue-200",

                    h3 { class: "text-2xl font-bold text-gray-800 mb-4 flex items-center gap-3",
                        "🔐 Secret Key"
                    }

                    div { class: "bg-white/90 border-2 border-gray-200 rounded-xl p-4 mb-4 min-h-[4rem] flex items-center shadow-inner",
                        if sk_string() == "nil" {
                            span { class: "text-gray-400 italic", "No secret key generated yet" }
                        } else {
                            span { class: "break-all text-sm leading-relaxed text-gray-800",
                                "{sk_string}"
                            }
                        }
                    }

                    button {
                        class: "w-full bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl",
                        onclick: move |_| {
                            let mut rng = thread_rng();
                            let sk = SecretKey::new(&mut rng);
                            let _ = storage_c1
                                .set("SecretKey", &serde_json::to_string(&sk).unwrap())
                                .unwrap();
                            sk_string.set(storage_c1.get("SecretKey").unwrap().unwrap());
                        },
                        "🎲 Generate New Secret Key"
                    }
                }


                // Deposit Address Section
                div { class: "bg-gradient-to-r from-green-50 to-emerald-50 rounded-2xl p-6 mb-8 border border-green-200",

                    h3 { class: "text-2xl font-bold text-gray-800 mb-4 flex items-center gap-3",
                        "📍 Deposit Address"
                    }

                    div { class: "bg-white/90 border-2 border-green-200 rounded-xl p-4 mb-4 min-h-[4rem] flex items-center shadow-inner",
                        if deposit_address_string() == "nil" {
                            span { class: "text-gray-400 italic",
                                "Generate a secret key first, then create your deposit address"
                            }
                        } else {
                            span { class: "break-all text-sm leading-relaxed text-gray-800",
                                "{deposit_address_string}"
                            }
                        }
                    }

                    button {
                        class: "w-full bg-gradient-to-r from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl",
                        onclick: move |_| {
                            if let Ok(Some(sk_string)) = storage_c2.get("SecretKey") {
                                let sk: SecretKey = serde_json::from_str(&sk_string).unwrap();
                                let keypair = Keypair::from_secret_key(&SECP256K1, &sk);
                                let (pk, _) = keypair.x_only_public_key();
                                let pk_ser = pk.serialize();
                                let seed = u64::from_be_bytes([
                                    pk_ser[0],
                                    pk_ser[1],
                                    pk_ser[2],
                                    pk_ser[3],
                                    pk_ser[4],
                                    pk_ser[5],
                                    pk_ser[6],
                                    pk_ser[7],
                                ]);
                                let payload = labubu_maker(seed);
                                let spend_info = create_control_block_address(pk, payload.clone()).unwrap();
                                let address = Address::p2tr_tweaked(
                                    spend_info.output_key(),
                                    Network::Bitcoin,
                                );
                                let _ = storage_c2.set("DepositAddress", &address.to_string()).unwrap();
                                deposit_address_string
                                    .set(storage_c2.get("DepositAddress").unwrap().unwrap());
                                let image_data = format!(
                                    "data:image/png;base64,{}",
                                    base64::encode(&payload),
                                );
                                image_uri.set(image_data);
                            }
                        },
                        "🏗️ Generate Deposit Address"
                    }

                    div { class: "flex flex-col items-enter gap-4 p-4",
                        img {
                            class: "w-fit items-center gap-4 p-4",
                            src: "{image_uri}",
                        }
                    }
                }

                // Destination Address Section
                div { class: "bg-gradient-to-r from-orange-50 to-amber-50 rounded-2xl p-6 mb-8 border border-orange-200",

                    h3 { class: "text-2xl font-bold text-gray-800 mb-4 flex items-center gap-3",
                        "🎯 Destination Address"
                    }

                    div { class: "bg-white/90 border-2 border-orange-200 rounded-xl p-4 mb-4 min-h-[4rem] flex items-center shadow-inner",
                        if destination_address_string() == "nil" {
                            span { class: "text-gray-400 italic", "Enter a destination address below" }
                        } else {
                            span { class: "break-all text-sm leading-relaxed text-gray-800",
                                "{destination_address_string}"
                            }
                        }
                    }

                    input {
                        class: "w-full px-5 py-4 border-2 border-gray-200 rounded-xl text-base transition-all duration-300 bg-white/90 shadow-inner focus:outline-none focus:border-orange-500 focus:ring-4 focus:ring-orange-100 focus:-translate-y-0.5 mb-4",
                        r#type: "text",
                        value: "{destination_address_input}",
                        oninput: move |e| destination_address_input.set(e.value()),
                        placeholder: "bc1p... (Enter Bitcoin address)",
                    }

                    button {
                        class: "w-full bg-gradient-to-r from-orange-500 to-orange-600 hover:from-orange-600 hover:to-orange-700 text-white font-semibold py-4 px-8 rounded-xl transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl",
                        onclick: move |_| {
                            let address_string = destination_address_input();
                            let _ = storage_c3.set("DestinationAddress", &address_string).unwrap();
                            destination_address_string
                                .set(storage_c3.get("DestinationAddress").unwrap().unwrap());
                        },
                        "💾 Save Address"
                    }
                }
            }

            // Big Mint Button Section
            div { class: "max-w-4xl mx-auto mt-12",
                div { class: "bg-gradient-to-r from-purple-50 to-pink-50 rounded-2xl p-8 border border-purple-200",
                    h3 { class: "text-2xl font-bold text-gray-800 mb-6 flex items-center justify-center gap-3",
                        "🚀 Ready to Mint?"
                    }

                    button {
                        class: "w-full text-6xl font-black py-8 px-12 bg-gradient-to-r from-purple-600 via-pink-600 to-red-600 hover:from-purple-700 hover:via-pink-700 hover:to-red-700 text-white rounded-3xl transition-all duration-500 transform hover:scale-105 hover:shadow-2xl shadow-xl border-4 border-white/20",
                        onclick: move |_| {
                            // TODO(@stutxo): make tx

                            spawn(async move {
                                let utxos = fetch_address_utxos(
                                    &ESPLORA_ENDPOINT.read(),
                                    &Address::from_str(&deposit_address_string())
                                        .unwrap()
                                        .require_network(Network::Bitcoin)
                                        .unwrap(),
                                )
                                .await
                                .unwrap();
                              let inputs: Vec<TxIn> = utxos
                                  .iter()
                                  .map(|utxo| TxIn {
                                      previous_output: OutPoint::new(
                                          utxo.txid.clone(),
                                          utxo.vout,
                                      ),
                                      sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                                      ..Default::default()
                                  })
                                  .collect();

                                  let mut prev_txouts = Vec::new();

                                let sk: SecretKey = serde_json::from_str(&sk_string()).unwrap();
                                let keypair = Keypair::from_secret_key(&SECP256K1, &sk);
                                let (pk, _) = keypair.x_only_public_key();
                                let pk_ser = pk.serialize();
                                let seed = u64::from_be_bytes([
                                    pk_ser[0],
                                    pk_ser[1],
                                    pk_ser[2],
                                    pk_ser[3],
                                    pk_ser[4],
                                    pk_ser[5],
                                    pk_ser[6],
                                    pk_ser[7],
                                ]);
                                let payload = labubu_maker(seed);
                                let spend_info = create_control_block_address(pk, payload.clone()).unwrap();
                                let address = Address::p2tr_tweaked(
                                    spend_info.output_key(),
                                    Network::Bitcoin,
                                );

                                let prevouts: Vec<TxOut> = Vec::new();
                                for input in inputs.clone() {
                                    let url = format!(
                                        "https://mempool.space/api/tx/{}/hex",
                                        input.previous_output.txid
                                    );
                                    let response = reqwest::get(&url).await.unwrap().text().await.unwrap();
                                    let tx: Transaction = deserialize(&Vec::<u8>::from_hex(&response).unwrap()).unwrap();

                                    let mut outpoint: Option<OutPoint> = None;
                                    for (i, out) in tx.output.iter().enumerate() {
                                        if address.script_pubkey() == out.script_pubkey {
                                            outpoint = Some(OutPoint::new(tx.compute_txid(), i as u32));
                                            break;
                                        }
                                    }
                                    let prevout = outpoint.expect("Outpoint must exist in tx");
                                    prev_txouts.push(tx.output[prevout.vout as usize].clone());
                                }

                                let total_amount = utxos.iter().map(|utxo| utxo.value.to_sat()).sum::<u64>();


                                let destination_address = Address::from_str(&destination_address_string())
                                    .unwrap()
                                    .require_network(Network::Bitcoin)
                                    .unwrap();
                                // utxo is a Vec<Utxo>, so pass it as inputs or handle accordingly
                                let tx = mint(pk, total_amount, destination_address, 1337, inputs, prevouts, spend_info, keypair)
                                    .expect("Minting transaction failed");

                                let client = create_esplora_client(&ESPLORA_ENDPOINT.read())
                                    .expect("Failed to create Esplora client");
                                broadcast_tx(&client, &tx).await.expect("Broadcast failed");
                            });
                        },
                        "🚀 MINT LABUBU 🚀"
                    }
                }
            }
        }
    }
}
