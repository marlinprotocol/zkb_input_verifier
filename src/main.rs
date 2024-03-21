use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, str::FromStr, sync::Arc};

use ethers::prelude::*;

mod helpers;
mod zkb_inputs;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct IvsConfig {
    secp256k1_private_key: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Input Verifier...");
    HttpServer::new(|| {
        App::new()
            .service(check_input)
            .service(check_input_with_signature)
            .service(welcome)
    })
    .bind(("0.0.0.0", 3030))?
    .run()
    .await
}

#[get("/welcome")]
async fn welcome() -> Result<HttpResponse, helpers::error::InputError> {
    println!("In welcome");
    return Ok(HttpResponse::Ok().body("Welcome!..."));
}

#[post("/checkInput")]
async fn check_input(
    payload: web::Json<helpers::input::InputPayload>,
) -> Result<HttpResponse, helpers::error::InputError> {
    let ivs_config_path = "../ivs_config/ivs_config.json".to_string();
    let alt_ivs_config_path = "./ivs_config/ivs_config.json".to_string();
    let file_content =
        fs::read_to_string(&ivs_config_path).or_else(|_| fs::read_to_string(&alt_ivs_config_path));

    match file_content {
        Ok(content) => {
            let value: Value = serde_json::from_str(&content).unwrap();
            let config: std::result::Result<IvsConfig, serde_json::Error> =
                serde_json::from_value(value);

            match config {
                Ok(config_check) => {
                    let verification = zkb_inputs::verify_zkbob_secret(
                        payload.clone(),
                        config_check.secp256k1_private_key,
                    )
                    .await;

                    match verification {
                        Ok(verify) => {
                            if verify {
                                return Ok(HttpResponse::Ok().body("Payload is valid"));
                            } else {
                                return Err(helpers::error::InputError::PayloadNotValid);
                            }
                        }
                        Err(_) => {
                            return Err(helpers::error::InputError::DecryptionFailed);
                        }
                    }
                }
                Err(_) => {
                    return Err(helpers::error::InputError::BadConfigData);
                }
            }
        }
        Err(_) => {
            return Err(helpers::error::InputError::FileNotFound);
        }
    }
}

#[post("/checkInputWithSignature")]
async fn check_input_with_signature(
    payload: web::Json<helpers::input::AskPayload>,
) -> Result<HttpResponse, helpers::error::InputError> {
    let ivs_config_path = "../ivs_config/ivs_config.json".to_string();
    let alt_ivs_config_path = "./ivs_config/ivs_config.json".to_string();
    let file_content =
        fs::read_to_string(&ivs_config_path).or_else(|_| fs::read_to_string(&alt_ivs_config_path));

    if file_content.is_err() {
        return Err(helpers::error::InputError::FileNotFound);
    }

    let content = file_content.unwrap();

    let value: Value = serde_json::from_str(&content).unwrap();
    let config: std::result::Result<IvsConfig, serde_json::Error> = serde_json::from_value(value);

    if config.is_err() {
        return Err(helpers::error::InputError::BadConfigData);
    }

    let config_check = config.unwrap();

    let ivs_key = config_check.secp256k1_private_key;
    let ivs_signer = ivs_key.parse::<LocalWallet>().unwrap();

    let rpc_url = "https://arb-sepolia.g.alchemy.com/v2/WPcL0MatIn2ai-4O6BcJgfeuXqD7WxRi";
    let proof_market_place_address = "0x81C80965f4E1b073858cc9D55d7D9A517C9fF258";

    let proof_marketplace_address = Address::from_str(&proof_market_place_address).unwrap();
    let provider_http = Provider::<Http>::try_from(rpc_url)
        .unwrap()
        .with_signer(ivs_signer.clone());
    let client = Arc::new(provider_http.clone());

    let proof_marketplace = bindings::proof_marketplace::ProofMarketplace::new(
        proof_marketplace_address,
        client.clone(),
    );

    let ask_data: (bindings::proof_marketplace::Ask, _, _, _) = proof_marketplace
        .list_of_ask(payload.ask_id.into())
        .call()
        .await
        .unwrap();

    // Verify the inputs
    let data = json!({
        "public_inputs": hex::encode(ask_data.clone().0.prover_data),
        "encrypted_secret": payload.encrypted_secret,
        "acl": payload.acl,
    });
    let input_payload: helpers::input::InputPayload = serde_json::from_value(data).unwrap();

    let verification = zkb_inputs::verify_zkbob_secret(input_payload, ivs_key).await;

    if verification.is_err() {
        return Err(helpers::error::InputError::DecryptionFailed);
    }

    let verify = verification.unwrap();
    if !verify {
        let value = vec![
            ethers::abi::Token::Uint(payload.ask_id.into()),
            ethers::abi::Token::Bytes(ask_data.0.prover_data.to_vec()),
        ];
        let encoded = ethers::abi::encode(&value);
        let digest = ethers::utils::keccak256(encoded);

        let signature = ivs_signer
            .sign_message(ethers::types::H256(digest))
            .await
            .unwrap();
        println!("Signature: {:?}", signature);
        let verification_result = json!({
            "ask_id": payload.ask_id,
            "signature": "0x".to_owned() + &signature.to_string(),
        });
        return Ok(HttpResponse::Ok().body(serde_json::to_string(&verification_result).unwrap()));
    } else {
        return Ok(HttpResponse::Ok().body("Payload is valid"));
    }
}
