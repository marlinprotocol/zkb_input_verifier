pub use attestation_auther_upgradeable::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod attestation_auther_upgradeable {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("attestationVerifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IAttestationVerifier",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("maxAge"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ATTESTATION_MAX_AGE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ATTESTATION_MAX_AGE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ATTESTATION_VERIFIER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ATTESTATION_VERIFIER",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IAttestationVerifier",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifyKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifyKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveCPUs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveMemory"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "timestampInMilliseconds",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveImageRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EnclaveImageRevoked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveImageWhitelisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EnclaveImageWhitelisted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("PCR0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("PCR1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("PCR2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveKeyRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EnclaveKeyRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveKeyVerified"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EnclaveKeyVerified"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveKeyWhitelisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EnclaveKeyWhitelisted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enclavePubKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATTESTATIONAUTHERUPGRADEABLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\t\x1E8\x03\x80a\t\x1E\x839\x81\x01`@\x81\x90Ra\0/\x91a\0EV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x80R`\xA0Ra\0\x7FV[`\0\x80`@\x83\x85\x03\x12\x15a\0XW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0oW`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\x80Q`\xA0Qa\x08na\0\xB0`\09`\0\x81\x81`K\x01Ra\x01\xF3\x01R`\0\x81\x81`\x85\x01Ra\x04B\x01Ra\x08n`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x9A\xEC\x99\x0E\x14a\0FW\x80c\xCDy\xF9\x06\x14a\0\x80W\x80c\xFB\x94\xDB+\x14a\0\xBFW[`\0\x80\xFD[a\0m\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0wV[a\0\xD2a\0\xCD6`\x04a\x06=V[a\0\xD4V[\0[a\0\xE2\x86\x86\x86\x86\x86\x86a\0\xEAV[PPPPPPV[`\0\x84\x81R`\x01` R`@\x90 \x80Ta\x01\x03\x90a\x06\xC4V[\x90P`\0\x03a\x01oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAA:VK-Enclave image to verify no`D\x82\x01Rl\x1D\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\x9A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x01z\x86a\x05;V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x90\x91P\x15a\x01\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAA:VK-Enclave key already verifi`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01a\x01fV[a\x02\x18\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba\x06\xFEV[a\x02$a\x03\xE8\x84a\x07%V[\x11a\x02qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAA:VK-Attestation too old\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01fV[`\0\x85\x81R`\x01` R`@\x80\x82 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x02\x98\x90a\x06\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xC4\x90a\x06\xC4V[\x80\x15a\x03\x11W\x80`\x1F\x10a\x02\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x11V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x03*\x90a\x06\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03V\x90a\x06\xC4V[\x80\x15a\x03\xA3W\x80`\x1F\x10a\x03xWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xA3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x86W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x03\xBC\x90a\x06\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xE8\x90a\x06\xC4V[\x80\x15a\x045W\x80`\x1F\x10a\x04\nWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x045V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x18W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xAC\x0F\x0B\xD5\x89\x89\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x8B\x8B\x8B`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xA6\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x07\x97V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xBEW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x04\xD2W=`\0\x80>=`\0\xFD[PPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90\x81\x90 \x88\x90UQ\x87\x91Pa\x05\x01\x90\x89\x90a\x08\x1CV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3PPPPPPPPV[`\0\x81Q`@\x14a\x05\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid public key length\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01fV[P\x80Q` \x90\x91\x01 \x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x05\xC1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xDCWa\x05\xDCa\x05\x9AV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x06\x04Wa\x06\x04a\x05\x9AV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x06\x1DW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x06VW`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06nW`\0\x80\xFD[a\x06z\x8A\x83\x8B\x01a\x05\xB0V[\x97P` \x89\x015\x91P\x80\x82\x11\x15a\x06\x90W`\0\x80\xFD[Pa\x06\x9D\x89\x82\x8A\x01a\x05\xB0V[\x96\x99\x96\x98PPPP`@\x85\x015\x94``\x81\x015\x94`\x80\x82\x015\x94P`\xA0\x90\x91\x015\x92P\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\xD8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06\xF8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x07\x1FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x82a\x07BWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0[\x83\x81\x10\x15a\x07bW\x81\x81\x01Q\x83\x82\x01R` \x01a\x07JV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x07\x83\x81` \x86\x01` \x86\x01a\x07GV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0a\x01\0\x80\x83Ra\x07\xAB\x81\x84\x01\x8Ca\x07kV[\x90P\x82\x81\x03` \x84\x01Ra\x07\xBF\x81\x8Ba\x07kV[\x90P\x82\x81\x03`@\x84\x01Ra\x07\xD3\x81\x8Aa\x07kV[\x90P\x82\x81\x03``\x84\x01Ra\x07\xE7\x81\x89a\x07kV[\x90P\x82\x81\x03`\x80\x84\x01Ra\x07\xFB\x81\x88a\x07kV[`\xA0\x84\x01\x96\x90\x96RPP`\xC0\x81\x01\x92\x90\x92R`\xE0\x90\x91\x01R\x95\x94PPPPPV[`\0\x82Qa\x08.\x81\x84` \x87\x01a\x07GV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x84|\x99\x86\xBFp7v\x0B\xDF\\hKK\xF8hW\x1D\xBA\xFD\x9D\x8Ev\xE6>\xC7\xA4\xB8\xA1\xBE\x03CdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ATTESTATIONAUTHERUPGRADEABLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x9A\xEC\x99\x0E\x14a\0FW\x80c\xCDy\xF9\x06\x14a\0\x80W\x80c\xFB\x94\xDB+\x14a\0\xBFW[`\0\x80\xFD[a\0m\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0wV[a\0\xD2a\0\xCD6`\x04a\x06=V[a\0\xD4V[\0[a\0\xE2\x86\x86\x86\x86\x86\x86a\0\xEAV[PPPPPPV[`\0\x84\x81R`\x01` R`@\x90 \x80Ta\x01\x03\x90a\x06\xC4V[\x90P`\0\x03a\x01oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAA:VK-Enclave image to verify no`D\x82\x01Rl\x1D\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\x9A\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x01z\x86a\x05;V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x90\x91P\x15a\x01\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAA:VK-Enclave key already verifi`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01a\x01fV[a\x02\x18\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba\x06\xFEV[a\x02$a\x03\xE8\x84a\x07%V[\x11a\x02qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAA:VK-Attestation too old\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01fV[`\0\x85\x81R`\x01` R`@\x80\x82 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x02\x98\x90a\x06\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xC4\x90a\x06\xC4V[\x80\x15a\x03\x11W\x80`\x1F\x10a\x02\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x11V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x03*\x90a\x06\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03V\x90a\x06\xC4V[\x80\x15a\x03\xA3W\x80`\x1F\x10a\x03xWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xA3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x86W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x03\xBC\x90a\x06\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xE8\x90a\x06\xC4V[\x80\x15a\x045W\x80`\x1F\x10a\x04\nWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x045V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x18W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xAC\x0F\x0B\xD5\x89\x89\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x8B\x8B\x8B`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xA6\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x07\x97V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xBEW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x04\xD2W=`\0\x80>=`\0\xFD[PPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90\x81\x90 \x88\x90UQ\x87\x91Pa\x05\x01\x90\x89\x90a\x08\x1CV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3PPPPPPPPV[`\0\x81Q`@\x14a\x05\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid public key length\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01fV[P\x80Q` \x90\x91\x01 \x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x05\xC1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xDCWa\x05\xDCa\x05\x9AV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x06\x04Wa\x06\x04a\x05\x9AV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x06\x1DW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x06VW`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06nW`\0\x80\xFD[a\x06z\x8A\x83\x8B\x01a\x05\xB0V[\x97P` \x89\x015\x91P\x80\x82\x11\x15a\x06\x90W`\0\x80\xFD[Pa\x06\x9D\x89\x82\x8A\x01a\x05\xB0V[\x96\x99\x96\x98PPPP`@\x85\x015\x94``\x81\x015\x94`\x80\x82\x015\x94P`\xA0\x90\x91\x015\x92P\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\xD8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06\xF8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x07\x1FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x82a\x07BWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0[\x83\x81\x10\x15a\x07bW\x81\x81\x01Q\x83\x82\x01R` \x01a\x07JV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x07\x83\x81` \x86\x01` \x86\x01a\x07GV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0a\x01\0\x80\x83Ra\x07\xAB\x81\x84\x01\x8Ca\x07kV[\x90P\x82\x81\x03` \x84\x01Ra\x07\xBF\x81\x8Ba\x07kV[\x90P\x82\x81\x03`@\x84\x01Ra\x07\xD3\x81\x8Aa\x07kV[\x90P\x82\x81\x03``\x84\x01Ra\x07\xE7\x81\x89a\x07kV[\x90P\x82\x81\x03`\x80\x84\x01Ra\x07\xFB\x81\x88a\x07kV[`\xA0\x84\x01\x96\x90\x96RPP`\xC0\x81\x01\x92\x90\x92R`\xE0\x90\x91\x01R\x95\x94PPPPPV[`\0\x82Qa\x08.\x81\x84` \x87\x01a\x07GV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x84|\x99\x86\xBFp7v\x0B\xDF\\hKK\xF8hW\x1D\xBA\xFD\x9D\x8Ev\xE6>\xC7\xA4\xB8\xA1\xBE\x03CdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ATTESTATIONAUTHERUPGRADEABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AttestationAutherUpgradeable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AttestationAutherUpgradeable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AttestationAutherUpgradeable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AttestationAutherUpgradeable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AttestationAutherUpgradeable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AttestationAutherUpgradeable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AttestationAutherUpgradeable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ATTESTATIONAUTHERUPGRADEABLE_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ATTESTATIONAUTHERUPGRADEABLE_ABI.clone(),
                ATTESTATIONAUTHERUPGRADEABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ATTESTATION_MAX_AGE` (0x9aec990e) function
        pub fn attestation_max_age(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 236, 153, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ATTESTATION_VERIFIER` (0xcd79f906) function
        pub fn attestation_verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([205, 121, 249, 6], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyKey` (0xfb94db2b) function
        pub fn verify_key(
            &self,
            signature: ::ethers::core::types::Bytes,
            enclave_pub_key: ::ethers::core::types::Bytes,
            image_id: [u8; 32],
            enclave_cp_us: ::ethers::core::types::U256,
            enclave_memory: ::ethers::core::types::U256,
            timestamp_in_milliseconds: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [251, 148, 219, 43],
                    (
                        signature,
                        enclave_pub_key,
                        image_id,
                        enclave_cp_us,
                        enclave_memory,
                        timestamp_in_milliseconds,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EnclaveImageRevoked` event
        pub fn enclave_image_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveImageRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveImageWhitelisted` event
        pub fn enclave_image_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveImageWhitelistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveKeyRevoked` event
        pub fn enclave_key_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveKeyRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveKeyVerified` event
        pub fn enclave_key_verified_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveKeyVerifiedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnclaveKeyWhitelisted` event
        pub fn enclave_key_whitelisted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveKeyWhitelistedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AttestationAutherUpgradeableEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AttestationAutherUpgradeable<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "EnclaveImageRevoked", abi = "EnclaveImageRevoked(bytes32)")]
    pub struct EnclaveImageRevokedFilter {
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "EnclaveImageWhitelisted",
        abi = "EnclaveImageWhitelisted(bytes32,bytes,bytes,bytes)"
    )]
    pub struct EnclaveImageWhitelistedFilter {
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "EnclaveKeyRevoked", abi = "EnclaveKeyRevoked(bytes)")]
    pub struct EnclaveKeyRevokedFilter {
        #[ethevent(indexed)]
        pub enclave_pub_key: ::ethers::core::types::H256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "EnclaveKeyVerified", abi = "EnclaveKeyVerified(bytes,bytes32)")]
    pub struct EnclaveKeyVerifiedFilter {
        #[ethevent(indexed)]
        pub enclave_pub_key: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "EnclaveKeyWhitelisted",
        abi = "EnclaveKeyWhitelisted(bytes,bytes32)"
    )]
    pub struct EnclaveKeyWhitelistedFilter {
        #[ethevent(indexed)]
        pub enclave_pub_key: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub image_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum AttestationAutherUpgradeableEvents {
        EnclaveImageRevokedFilter(EnclaveImageRevokedFilter),
        EnclaveImageWhitelistedFilter(EnclaveImageWhitelistedFilter),
        EnclaveKeyRevokedFilter(EnclaveKeyRevokedFilter),
        EnclaveKeyVerifiedFilter(EnclaveKeyVerifiedFilter),
        EnclaveKeyWhitelistedFilter(EnclaveKeyWhitelistedFilter),
        InitializedFilter(InitializedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AttestationAutherUpgradeableEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EnclaveImageRevokedFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::EnclaveImageRevokedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = EnclaveImageWhitelistedFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::EnclaveImageWhitelistedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = EnclaveKeyRevokedFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::EnclaveKeyRevokedFilter(decoded),
                );
            }
            if let Ok(decoded) = EnclaveKeyVerifiedFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::EnclaveKeyVerifiedFilter(decoded),
                );
            }
            if let Ok(decoded) = EnclaveKeyWhitelistedFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::EnclaveKeyWhitelistedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(
                    AttestationAutherUpgradeableEvents::InitializedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AttestationAutherUpgradeableEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EnclaveImageRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveImageWhitelistedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveKeyRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveKeyVerifiedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveKeyWhitelistedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EnclaveImageRevokedFilter>
    for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveImageRevokedFilter) -> Self {
            Self::EnclaveImageRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageWhitelistedFilter>
    for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveImageWhitelistedFilter) -> Self {
            Self::EnclaveImageWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyRevokedFilter>
    for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveKeyRevokedFilter) -> Self {
            Self::EnclaveKeyRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyVerifiedFilter>
    for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveKeyVerifiedFilter) -> Self {
            Self::EnclaveKeyVerifiedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyWhitelistedFilter>
    for AttestationAutherUpgradeableEvents {
        fn from(value: EnclaveKeyWhitelistedFilter) -> Self {
            Self::EnclaveKeyWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter>
    for AttestationAutherUpgradeableEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    ///Container type for all input parameters for the `ATTESTATION_MAX_AGE` function with signature `ATTESTATION_MAX_AGE()` and selector `0x9aec990e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ATTESTATION_MAX_AGE", abi = "ATTESTATION_MAX_AGE()")]
    pub struct AttestationMaxAgeCall;
    ///Container type for all input parameters for the `ATTESTATION_VERIFIER` function with signature `ATTESTATION_VERIFIER()` and selector `0xcd79f906`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ATTESTATION_VERIFIER", abi = "ATTESTATION_VERIFIER()")]
    pub struct AttestationVerifierCall;
    ///Container type for all input parameters for the `verifyKey` function with signature `verifyKey(bytes,bytes,bytes32,uint256,uint256,uint256)` and selector `0xfb94db2b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "verifyKey",
        abi = "verifyKey(bytes,bytes,bytes32,uint256,uint256,uint256)"
    )]
    pub struct VerifyKeyCall {
        pub signature: ::ethers::core::types::Bytes,
        pub enclave_pub_key: ::ethers::core::types::Bytes,
        pub image_id: [u8; 32],
        pub enclave_cp_us: ::ethers::core::types::U256,
        pub enclave_memory: ::ethers::core::types::U256,
        pub timestamp_in_milliseconds: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum AttestationAutherUpgradeableCalls {
        AttestationMaxAge(AttestationMaxAgeCall),
        AttestationVerifier(AttestationVerifierCall),
        VerifyKey(VerifyKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for AttestationAutherUpgradeableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AttestationMaxAgeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttestationMaxAge(decoded));
            }
            if let Ok(decoded) = <AttestationVerifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttestationVerifier(decoded));
            }
            if let Ok(decoded) = <VerifyKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyKey(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AttestationAutherUpgradeableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AttestationMaxAge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AttestationAutherUpgradeableCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestationMaxAge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationVerifier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyKey(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AttestationMaxAgeCall>
    for AttestationAutherUpgradeableCalls {
        fn from(value: AttestationMaxAgeCall) -> Self {
            Self::AttestationMaxAge(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierCall>
    for AttestationAutherUpgradeableCalls {
        fn from(value: AttestationVerifierCall) -> Self {
            Self::AttestationVerifier(value)
        }
    }
    impl ::core::convert::From<VerifyKeyCall> for AttestationAutherUpgradeableCalls {
        fn from(value: VerifyKeyCall) -> Self {
            Self::VerifyKey(value)
        }
    }
    ///Container type for all return fields from the `ATTESTATION_MAX_AGE` function with signature `ATTESTATION_MAX_AGE()` and selector `0x9aec990e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AttestationMaxAgeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ATTESTATION_VERIFIER` function with signature `ATTESTATION_VERIFIER()` and selector `0xcd79f906`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AttestationVerifierReturn(pub ::ethers::core::types::Address);
}
