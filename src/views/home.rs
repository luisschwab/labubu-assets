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





use crate::esplora::{
    broadcast_tx, create_esplora_client, fetch_address_utxos, fetch_fee_estimates,
};
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
    let _labubu_txid = use_signal(|| "".to_string());

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
        div { 
            class: "min-h-screen bg-gradient-to-br from-purple-400 via-pink-400 to-yellow-400 p-4",
            style: "background-image: radial-gradient(circle at 20% 80%, rgba(120, 119, 198, 0.3) 0%, transparent 50%), radial-gradient(circle at 80% 20%, rgba(255, 119, 198, 0.3) 0%, transparent 50%), radial-gradient(circle at 40% 40%, rgba(120, 219, 255, 0.3) 0%, transparent 50%);",
            
            // Animated background elements
            div { 
                class: "fixed inset-0 pointer-events-none",
                style: "background: repeating-linear-gradient(45deg, transparent, transparent 10px, rgba(255,255,255,0.1) 10px, rgba(255,255,255,0.1) 20px); animation: slide 20s linear infinite;"
            }
            
            div { class: "max-w-6xl mx-auto bg-white/90 backdrop-blur-sm rounded-3xl p-8 shadow-2xl border-4 border-purple-300",
                style: "box-shadow: 0 0 30px rgba(147, 51, 234, 0.3), inset 0 0 30px rgba(255, 255, 255, 0.1);",
                
                // Header with vintage styling
                div { class: "text-center mb-12",
                    div { class: "inline-block bg-gradient-to-r from-purple-600 via-pink-500 to-yellow-400 p-2 rounded-full mb-4",
                        style: "box-shadow: 0 0 20px rgba(147, 51, 234, 0.5);"
                    }
                    h1 { 
                        class: "text-6xl font-bold mb-3 bg-gradient-to-r from-purple-600 via-pink-500 to-yellow-400 bg-clip-text text-transparent",
                        style: "text-shadow: 3px 3px 0px rgba(0,0,0,0.2), -1px -1px 0px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "🎯 Labubu Assets 🎯"
                    }
                    p { 
                        class: "text-2xl text-gray-700 font-bold mb-8", 
                        style: "text-shadow: 1px 1px 2px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "Labubu Assets™ Mint - Best viewed in Netscape Navigator 4.0! ✨"
                    }
                }

                // Navigation with vintage styling
                div { 
                    class: "bg-gradient-to-r from-blue-100 via-purple-100 to-pink-100 rounded-3xl p-6 mb-8 border-4 border-blue-300 text-center",
                    style: "box-shadow: inset 0 0 20px rgba(59, 130, 246, 0.2), 0 8px 32px rgba(0,0,0,0.1);",
                    
                    h3 { 
                        class: "text-2xl font-bold text-gray-800 mb-4 flex items-center gap-3 justify-center",
                        style: "text-shadow: 2px 2px 4px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "🧭 Navigation 🧭"
                    }
                    
                    div { class: "flex justify-center gap-6 flex-wrap",
                        a {
                            href: "/",
                            class: "inline-block bg-gradient-to-r from-purple-500 via-pink-500 to-yellow-400 hover:from-purple-600 hover:via-pink-600 hover:to-yellow-500 text-white font-bold py-3 px-6 rounded-2xl transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl text-lg",
                            style: "box-shadow: 0 6px 20px rgba(147, 51, 234, 0.4), inset 0 1px 0 rgba(255,255,255,0.3); text-shadow: 2px 2px 4px rgba(0,0,0,0.3); font-family: 'Comic Sans MS', cursive;",
                            "🏠 Home"
                        }
                        a {
                            href: "/converter",
                            class: "inline-block bg-gradient-to-r from-purple-500 via-pink-500 to-yellow-400 hover:from-purple-600 hover:via-pink-600 hover:to-yellow-500 text-white font-bold py-3 px-6 rounded-2xl transition-all duration-300 transform hover:-translate-y-1 hover:shadow-xl text-lg",
                            style: "box-shadow: 0 6px 20px rgba(147, 51, 234, 0.4), inset 0 1px 0 rgba(255,255,255,0.3); text-shadow: 2px 2px 4px rgba(0,0,0,0.3); font-family: 'Comic Sans MS', cursive;",
                            "🔧 Hex Converter"
                        }
                    }
                }

                // Keys Section with vintage styling
                div { 
                    class: "bg-gradient-to-r from-blue-100 via-purple-100 to-pink-100 rounded-3xl p-8 mb-8 border-4 border-blue-300",
                    style: "box-shadow: inset 0 0 20px rgba(59, 130, 246, 0.2), 0 8px 32px rgba(0,0,0,0.1);",
                    
                    h3 { 
                        class: "text-3xl font-bold text-gray-800 mb-6 flex items-center gap-3 justify-center",
                        style: "text-shadow: 2px 2px 4px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "🔐 Keys 🔐"
                    }
                    
                    div { class: "bg-white/90 rounded-2xl p-6 mb-6 border-4 border-purple-300",
                        style: "box-shadow: inset 0 0 10px rgba(147, 51, 234, 0.2);",
                        
                        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6",
                            div { class: "text-center",
                                h4 { 
                                    class: "text-xl font-bold text-gray-800 mb-3",
                                    style: "font-family: 'Comic Sans MS', cursive;",
                                    "🔑 Private Key"
                                }
                                div { class: "bg-gray-100 rounded-xl p-4 border-2 border-gray-300 text-center",
                                    if sk_string() == "nil" {
                                        span { class: "text-gray-500 italic", "No secret key generated yet" }
                                    } else {
                                        span { class: "word-break break-all text-sm font-mono", "{sk_string}" }
                                    }
                                }
                            }
                            div { class: "text-center",
                                h4 { 
                                    class: "text-xl font-bold text-gray-800 mb-3",
                                    style: "font-family: 'Comic Sans MS', cursive;",
                                    "💰 Public Address"
                                }
                                div { class: "bg-gray-100 rounded-xl p-4 border-2 border-gray-300 text-center",
                                    if deposit_address_string() == "nil" {
                                        span { class: "text-gray-500 italic", "Generate keys to create your deposit address" }
                                    } else {
                                        span { class: "word-break break-all text-sm font-mono", "{deposit_address_string}" }
                                    }
                                }
                            }
                        }
                        
                        div { class: "text-center",
                            button {
                                class: "bg-gradient-to-r from-purple-500 via-pink-500 to-yellow-400 hover:from-purple-600 hover:via-pink-600 hover:to-yellow-500 text-white font-bold py-4 px-8 rounded-2xl transition-all duration-300 transform hover:-translate-y-2 hover:shadow-2xl text-xl",
                                style: "box-shadow: 0 8px 25px rgba(147, 51, 234, 0.4), inset 0 1px 0 rgba(255,255,255,0.3); text-shadow: 2px 2px 4px rgba(0,0,0,0.3); font-family: 'Comic Sans MS', cursive;",
                                onclick: move |_| {
                                    let storage_clone1 = storage_c1.clone();
                                    let storage_clone2 = storage_c2.clone();
                                    spawn(async move {
                                        let mut rng = thread_rng();
                                        let sk = SecretKey::new(&mut rng);
                                        let _ = storage_clone1.set("SecretKey", &serde_json::to_string(&sk).unwrap()).unwrap();
                                        if let Ok(Some(sk_str)) = storage_clone1.get("SecretKey") {
                                            sk_string.set(sk_str);
                                        }
                                        // Generate deposit address
                                        let keypair = Keypair::from_secret_key(&SECP256K1, &sk);
                                        let (pk, _) = keypair.x_only_public_key();
                                        let pk_ser = pk.serialize();
                                        let seed = u64::from_be_bytes([
                                            pk_ser[0], pk_ser[1], pk_ser[2], pk_ser[3],
                                            pk_ser[4], pk_ser[5], pk_ser[6], pk_ser[7],
                                        ]);
                                        let payload = labubu_maker(seed);
                                        let spend_info = create_control_block_address(pk, payload.clone()).unwrap();
                                        let address = Address::p2tr_tweaked(
                                            spend_info.output_key(),
                                            Network::Bitcoin,
                                        );
                                        let _ = storage_clone2.set("DepositAddress", &address.to_string()).unwrap();
                                        if let Ok(Some(deposit_addr)) = storage_clone2.get("DepositAddress") {
                                            deposit_address_string.set(deposit_addr);
                                        }
                                        let image_data = format!(
                                            "data:image/png;base64,{}",
                                            base64::encode(&payload),
                                        );
                                        image_uri.set(image_data);
                                    });
                                },
                                "🔑 Generate Keys! 🔑"
                            }
                        }
                    }
                    
                    // Labubu Image Display
                    div { class: "text-center mt-8",
                        div { 
                            class: "bg-white rounded-3xl p-6 shadow-2xl border-4 border-purple-300 inline-block",
                            style: "box-shadow: 0 0 30px rgba(147, 51, 234, 0.4), inset 0 0 20px rgba(255,255,255,0.5);",
                            img {
                                class: "max-w-full h-auto rounded-2xl",
                                style: "max-height: 400px; min-height: 200px; box-shadow: 0 10px 30px rgba(0,0,0,0.3); image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
                                src: "{image_uri}",
                                alt: "Generated Labubu"
                            }
                        }
                    }
                }

                // Destination Address Section with vintage styling
                div { 
                    class: "bg-gradient-to-r from-orange-100 via-amber-100 to-yellow-100 rounded-3xl p-8 mb-8 border-4 border-orange-300",
                    style: "box-shadow: inset 0 0 20px rgba(251, 146, 60, 0.2), 0 8px 32px rgba(0,0,0,0.1);",
                    
                    h3 { 
                        class: "text-3xl font-bold text-gray-800 mb-6 flex items-center gap-3 justify-center",
                        style: "text-shadow: 2px 2px 4px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "🎯 Destination Address 🎯"
                    }

                    div { class: "bg-white/90 rounded-2xl p-6 mb-6 border-4 border-orange-300",
                        style: "box-shadow: inset 0 0 10px rgba(251, 146, 60, 0.2);",
                        
                        div { class: "mb-6",
                            h4 { 
                                class: "text-xl font-bold text-gray-800 mb-3",
                                style: "font-family: 'Comic Sans MS', cursive;",
                                "📝 Current Address"
                            }
                            div { class: "bg-gray-100 rounded-xl p-4 border-2 border-gray-300 min-h-[4rem] flex items-center",
                                if destination_address_string() == "nil" {
                                    span { class: "text-gray-500 italic", "Enter a destination address below" }
                                } else {
                                    span { class: "break-all text-sm leading-relaxed text-gray-800 font-mono", "{destination_address_string}" }
                                }
                            }
                        }

                        input {
                            class: "w-full px-6 py-5 border-4 border-orange-300 rounded-2xl text-base transition-all duration-300 bg-white/95 shadow-inner focus:outline-none focus:border-pink-500 focus:ring-4 focus:ring-pink-200 focus:-translate-y-1 mb-6 font-mono text-sm",
                            style: "box-shadow: inset 0 0 10px rgba(251, 146, 60, 0.2), 0 4px 15px rgba(0,0,0,0.1);",
                            r#type: "text",
                            value: "{destination_address_input}",
                            oninput: move |e| destination_address_input.set(e.value()),
                            placeholder: "bc1p... (Enter Bitcoin address)",
                        }

                        div { class: "text-center",
                            button {
                                class: "bg-gradient-to-r from-orange-500 via-amber-500 to-yellow-500 hover:from-orange-600 hover:via-amber-600 hover:to-yellow-600 text-white font-bold py-4 px-8 rounded-2xl transition-all duration-300 transform hover:-translate-y-2 hover:shadow-2xl text-xl",
                                style: "box-shadow: 0 8px 25px rgba(251, 146, 60, 0.4), inset 0 1px 0 rgba(255,255,255,0.3); text-shadow: 2px 2px 4px rgba(0,0,0,0.3); font-family: 'Comic Sans MS', cursive;",
                                onclick: move |_| {
                                    let storage_clone = storage_c3.clone();
                                    spawn(async move {
                                        let address_string = destination_address_input();
                                        let _ = storage_clone.set("DestinationAddress", &address_string).unwrap();
                                        if let Ok(Some(dest_addr)) = storage_clone.get("DestinationAddress") {
                                            destination_address_string.set(dest_addr);
                                        }
                                    });
                                },
                                "💾 Save Address! 💾"
                            }
                        }
                    }
                }

                // Big Mint Button Section with vintage styling
                div { 
                    class: "bg-gradient-to-r from-purple-100 via-pink-100 to-red-100 rounded-3xl p-8 border-4 border-purple-300",
                    style: "box-shadow: inset 0 0 20px rgba(147, 51, 234, 0.2), 0 8px 32px rgba(0,0,0,0.1);",
                    
                    h3 { 
                        class: "text-4xl font-bold text-gray-800 mb-8 flex items-center justify-center gap-3",
                        style: "text-shadow: 2px 2px 4px rgba(255,255,255,0.8); font-family: 'Comic Sans MS', cursive;",
                        "🚀 Ready to Mint? 🚀"
                    }

                    div { class: "text-center",
                        button {
                            class: "text-5xl font-black py-8 px-12 bg-gradient-to-r from-purple-600 via-pink-600 to-red-600 hover:from-purple-700 hover:via-pink-700 hover:to-red-700 text-white rounded-3xl transition-all duration-500 transform hover:scale-105 hover:shadow-2xl shadow-xl border-4 border-white/20",
                            style: "box-shadow: 0 0 40px rgba(147, 51, 234, 0.6), inset 0 1px 0 rgba(255,255,255,0.3); text-shadow: 3px 3px 6px rgba(0,0,0,0.5); font-family: 'Comic Sans MS', cursive;",
                            onclick: move |_| {
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

                                    if utxos.is_empty() {
                                        return;
                                    }

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

                                    let mut prev_txouts = Vec::new();
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
                                    let tx = mint(pk, total_amount, destination_address, 1337, inputs, prev_txouts, spend_info, keypair)
                                        .expect("Minting transaction failed");

                                    let client = create_esplora_client(&ESPLORA_ENDPOINT.read())
                                        .expect("Failed to create Esplora client");
                                    broadcast_tx(&client, &tx).await.expect("Broadcast failed");
                                });
                            },
                            "🚀 MINT LABUBU! 🚀"
                        }
                    }
                }
            }
        }
    }
}
