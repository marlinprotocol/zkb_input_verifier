# ZKBob Input Verifier

The kalypso input verification service is an optional input data verification service specific to a given market. To avoid fund losses from incorrect signatures sent by users, users have the option to verify their requests by checking if the data is properly encrypted. The input verifier checks if the input format for a given request is valid and if the secret inputs correspond to the respective public inputs.

## Environment variables
Add the following details to the `ivs_config/ivs_config.json` file

```
{
    "secp256k1_private_key": "****************************************************************"
}
```

## Instructions
To start the Input Verification Service, use `cargo run --release`