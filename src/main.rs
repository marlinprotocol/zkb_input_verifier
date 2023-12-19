use std::fs;

use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod helpers;
mod zkb_inputs;

#[derive(Debug, Serialize, Deserialize)]
struct IvsSigner {
    secp256k1_private_key: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(check_input).service(welcome))
        .bind(("127.0.0.1", 3030))?
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
    if zkb_inputs::verify_zkbob_secret(payload.clone())
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
    payload: web::Json<helpers::input::InputPayload>,
) -> Result<HttpResponse, helpers::error::InputError> {
    if zkb_inputs::verify_zkbob_secret(payload.clone())
        .await
        .unwrap()
    {
        let ivs_signer_path = "./ivs_config.json";
        let file_content = fs::read_to_string(ivs_signer_path).unwrap();
        let ivs_signer: IvsSigner = serde_json::from_str(&file_content).unwrap();
        dbg!(ivs_signer);
        todo!("sign abi.encode(input_bytes || encrypted_secrets and return signature");
    } else {
        return Err(helpers::error::InputError::PayloadNotValid);
    }
}
