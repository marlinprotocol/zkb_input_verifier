use crate::helpers::{input::InputPayload, secret_input_helpers};
use actix_web::error::Error;

use libzeropool_zkbob::{
    fawkes_crypto::{
        ff_uint::Num,
        engines::bn256::Fr, 
        native::poseidon::poseidon_merkle_proof_root, 
        core::sizedvec::SizedVec
    },
    native::{
        key,
        params::PoolParams,
        tx::{self, TransferPub, TransferSec},
    },
    POOL_PARAMS,
};
use serde_json::{json, Value};

fn into_zkbob_secret(decoded_secret: String) -> Result<TransferSec<Fr>, Error> {
    let decoded_secret_bytes = hex::decode(decoded_secret).unwrap();
    let secret_string = String::from_utf8(decoded_secret_bytes).unwrap();
    let secret_value: Value = serde_json::from_str(&secret_string).unwrap();
    let zkbob_secret: TransferSec<Fr> = serde_json::from_value(secret_value).unwrap();

    Ok(zkbob_secret)
}

fn into_zkbob_pub_input(decoded_pub_input: String) -> Result<TransferPub<Fr>, Error> {
    use ethers::abi::{decode, ParamType};
    use ethers::prelude::*;

    fn decode_input(
        encoded_input: Bytes,
    ) -> Result<[ethers::types::U256; 5], Box<dyn std::error::Error>> {
        let param_types = vec![ParamType::FixedArray(Box::new(ParamType::Uint(256)), 5)];
        let tokens = decode(&param_types, &encoded_input.0)?;

        if let Some(ethers::abi::Token::FixedArray(arr)) = tokens.get(0) {
            if arr.len() == 5 {
                let mut output = [U256::zero(); 5];
                for (i, token) in arr.iter().enumerate() {
                    if let ethers::abi::Token::Uint(u) = token {
                        output[i] = *u;
                    } else {
                        return Err("Expected a U256 inside the FixedArray".into());
                    }
                }
                Ok(output)
            } else {
                Err("Unexpected number of decoded tokens inside the FixedArray".into())
            }
        } else {
            Err("Unexpected decoded token type".into())
        }
    }

    let decoded_pub_input_bytes = hex::decode(decoded_pub_input).unwrap();
    let public = decode_input(decoded_pub_input_bytes.into()).unwrap();
    let public_value = json!({
        "root": public[0].to_string(),
        "nullifier": public[1].to_string(),
        "out_commit": public[2].to_string(),
        "delta": public[3].to_string(),
        "memo": public[4].to_string()
    });

    let zkbob_pub_input: TransferPub<Fr> = serde_json::from_value(public_value).unwrap();

    Ok(zkbob_pub_input)
}

pub async fn decrypted_secret(encrypted_secret: String, acl: String, ivs_private_key: String) -> String {
    // todo!("Fetch the decrypted secret from matching engine here");
    let decrypted_data = secret_input_helpers::decrypt_data_with_ecies_and_aes(encrypted_secret.as_bytes(), acl.as_bytes(), ivs_private_key.as_bytes()).unwrap();
    return String::from_utf8(decrypted_data).unwrap();
}

pub async fn verify_zkbob_secret(payload: InputPayload, ivs_private_key: String) -> Result<bool, Error> {
    let mut result = false;
    let zkbob_public = into_zkbob_pub_input(payload.public_inputs).unwrap();
    let zkbob_secret =
        into_zkbob_secret(decrypted_secret(payload.encrypted_secret, payload.acl, ivs_private_key).await)
            .unwrap();

    // calculating output hashes
    let out_account_hash = zkbob_secret.tx.output.0.hash(&POOL_PARAMS.clone());
    let out_note_hash = zkbob_secret
        .tx
        .output
        .1
        .iter()
        .map(|e| e.hash(&POOL_PARAMS.clone()))
        .collect::<Vec<_>>();
    let out_hash = [[out_account_hash].as_ref(), out_note_hash.as_slice()].concat();

    // calculating input hashes
    let in_account_hash = zkbob_secret.tx.input.0.hash(&POOL_PARAMS.clone());
    let in_note_hash = zkbob_secret
        .tx
        .input
        .1
        .iter()
        .map(|n| n.hash(&POOL_PARAMS.clone()))
        .collect::<Vec<_>>();
    let in_hash = [[in_account_hash].as_ref(), in_note_hash.as_slice()].concat();
    let inproof = zkbob_secret.in_proof.0;
    let eta = key::derive_key_eta(zkbob_secret.eddsa_a, &POOL_PARAMS.clone());

    let out_commit = tx::out_commitment_hash(&out_hash, &POOL_PARAMS.clone());
    let root = poseidon_merkle_proof_root(in_account_hash, &inproof, POOL_PARAMS.compress());
    let path_num = from_bool_to_num(inproof.path).unwrap();
    let nullifier = tx::nullifier(in_account_hash, eta, path_num, &POOL_PARAMS.clone());
    let _tx_hash = tx::tx_hash(&in_hash, zkbob_public.out_commit, &POOL_PARAMS.clone()); 
    
    if out_commit == zkbob_public.out_commit && root == zkbob_public.root && nullifier == zkbob_public.nullifier {
        result = true;
    }
    Ok(result)
}

pub fn from_bool_to_num(path: SizedVec<bool, 48>) -> Result<Num<Fr>, Error> {
    let mut acc: Num<Fr> = path[0].into();
    let mut k = Num::ONE;
    for n in 1..48 {
        k = k.double();
        let num: Num<Fr> = path[n].into();
        acc += k * num;
    }
    Ok(acc)
}
