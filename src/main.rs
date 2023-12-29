use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use ethers::prelude::*;
use std::fs;

mod helpers;
mod zkb_inputs;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct IvsConfig {
    chain_id: String,
    secp256k1_private_key: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    log::info!("Starting Input Verifier...");
    HttpServer::new(|| App::new().service(check_input).service(check_input_with_signature).service(welcome))
        .bind(("0.0.0.0", 3030))?
        .run()
        .await
}

#[get("welcome")]
async fn welcome() -> Result<HttpResponse, helpers::error::InputError> {
    return Ok(HttpResponse::Ok().body("Welcome!..."));
}

#[post("/checkInput")]
async fn check_input(
    payload: web::Json<helpers::input::InputPayload>,
) -> Result<HttpResponse, helpers::error::InputError> {
    let ivs_signer_path = "./ivs_config.json";
    let file_content = fs::read_to_string(ivs_signer_path).unwrap();
    let value: Value = serde_json::from_str(&file_content).unwrap();
    let config: IvsConfig = serde_json::from_value(value).unwrap();

    if zkb_inputs::verify_zkbob_secret(payload.clone(), config.secp256k1_private_key)
        .await
        .unwrap()
    {
        return Ok(HttpResponse::Ok().body("Payload is valid"));
    } else {
        return Err(helpers::error::InputError::PayloadNotValid);
    }
}

#[post("/checkInputWithSignature")]
async fn check_input_with_signature(
    payload: web::Json<helpers::input::AskPayload>,
) -> Result<HttpResponse, helpers::error::InputError> {
    let ivs_signer_path = "./ivs_config.json";
    let file_content = fs::read_to_string(ivs_signer_path).unwrap();
    let value: Value = serde_json::from_str(&file_content).unwrap();
    let config: IvsConfig = serde_json::from_value(value).unwrap();
    // dbg!(config.clone());

    let chain_id = config.chain_id;
    let ivs_key = config.secp256k1_private_key;
    let ivs_signer = ivs_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(U64::from_dec_str(&chain_id).unwrap().as_u64());

    // Verify the inputs
    let data = json!({
        "public_inputs": payload.public_inputs,
        "encrypted_secret": payload.encrypted_secret,
        "acl": payload.acl,
    });
    let input_payload: helpers::input::InputPayload = serde_json::from_value(data).unwrap();


    // Sign the result
    let verification = zkb_inputs::verify_zkbob_secret(input_payload, ivs_key)
        .await
        .unwrap();
    if !verification {
        let verification_result =json!({
            "ask_id": payload.ask_id,
            "verification": verification,
        });
        let verification_string = serde_json::to_vec(&verification_result).unwrap();
        let encoded = hex::encode(&verification_string);
        let digest = ethers::utils::keccak256(encoded);
    
        let signature = ivs_signer
            .sign_message(ethers::types::H256(digest))
            .await
            .unwrap();
        println!("Signature: {:?}", signature);
        return Ok(HttpResponse::BadRequest().body(signature.to_string()));
    } else {
        return Ok(HttpResponse::Ok().body("Payload is valid"));
    }
 
    
}
