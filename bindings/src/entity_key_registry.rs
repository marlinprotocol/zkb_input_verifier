pub use entity_key_registry::*;
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
pub mod entity_key_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_av"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IAttestationVerifier",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("KEY_REGISTER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("KEY_REGISTER_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addGeneratorRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addGeneratorRegistry",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_generatorRegistry",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowOnlyVerified"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowOnlyVerified"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleMember"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMember"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "initWhitelistImages",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct AttestationAutherUpgradeable.EnclaveImage[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pub_key"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pub_key"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removePubkey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removePubkey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyIndex"),
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
                (
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updatePubkey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updatePubkey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("keyIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestation_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                                    name: ::std::borrow::ToOwned::to_owned("attestation_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
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
                (
                    ::std::borrow::ToOwned::to_owned("whitelistImageUsingPcrs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "whitelistImageUsingPcrs",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pcrs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("AdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BeaconUpgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("beacon"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                (
                    ::std::borrow::ToOwned::to_owned("RemoveKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemoveKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("keyIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
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
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UpdateKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UpdateKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("keyIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
    pub static ENTITYKEYREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R0`\x80R4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\0/|8\x03\x80b\0/|\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\x01cV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0Ra\xEA``\xC0R`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\0lWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\0\x88WP0;\x15\x80\x15b\0\0\x88WP`\0T`\xFF\x16`\x01\x14[b\0\0\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x01\x14W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x80\x15b\0\x01[W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPb\0\x01\x95V[`\0` \x82\x84\x03\x12\x15b\0\x01vW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x8EW`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Qa-\x8Db\0\x01\xEF`\09`\0\x81\x81a\x037\x01Ra\x11\xB5\x01R`\0\x81\x81a\x03\xE2\x01Ra\x14\x05\x01R`\0\x81\x81a\x06\xB9\x01R\x81\x81a\x06\xF9\x01R\x81\x81a\x07\x98\x01R\x81\x81a\x07\xD8\x01Ra\x08g\x01Ra-\x8D`\0\xF3\xFE`\x80`@R`\x046\x10a\x01KW`\x005`\xE0\x1C\x80c\x91\xD1HT\x11a\0\xB6W\x80c\xCDy\xF9\x06\x11a\0oW\x80c\xCDy\xF9\x06\x14a\x03\xD0W\x80c\xD5Gt\x1F\x14a\x04\x04W\x80c\xE1\xA610\x14a\x04$W\x80c\xEC\xEA\x1D}\x14a\x04QW\x80c\xED8\r\x03\x14a\x04qW\x80c\xFB\x94\xDB+\x14a\x04\x91W`\0\x80\xFD[\x80c\x91\xD1HT\x14a\x03\x05W\x80c\x9A\xEC\x99\x0E\x14a\x03%W\x80c\xA2\x17\xFD\xDF\x14a\x03YW\x80c\xAA\x0E\xD0\x9F\x14a\x03nW\x80c\xB8\n\xAA\x89\x14a\x03\x8EW\x80c\xCA\x15\xC8s\x14a\x03\xB0W`\0\x80\xFD[\x80c6Y\xCF\xE6\x11a\x01\x08W\x80c6Y\xCF\xE6\x14a\x02EW\x80cO\x1E\xF2\x86\x14a\x02eW\x80cR\xD1\x90-\x14a\x02xW\x80ci\xFD\xBC\xCA\x14a\x02\x8DW\x80c~ \x1Be\x14a\x02\xADW\x80c\x90\x10\xD0|\x14a\x02\xCDW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01PW\x80c\x07\x07Y\x1F\x14a\x01\x85W\x80c$\x8A\x9C\xA3\x14a\x01\xA7W\x80c.\xB3\x9E\xE9\x14a\x01\xE5W\x80c//\xF1]\x14a\x02\x05W\x80c6V\x8A\xBE\x14a\x02%W[`\0\x80\xFD[4\x80\x15a\x01\\W`\0\x80\xFD[Pa\x01pa\x01k6`\x04a \xFDV[a\x04\xB1V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x91W`\0\x80\xFD[Pa\x01\xA5a\x01\xA06`\x04a!oV[a\x04\xC2V[\0[4\x80\x15a\x01\xB3W`\0\x80\xFD[Pa\x01\xD7a\x01\xC26`\x04a!\xB0V[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01|V[4\x80\x15a\x01\xF1W`\0\x80\xFD[Pa\x01\xA5a\x02\x006`\x04a\"\xC2V[a\x04\xE9V[4\x80\x15a\x02\x11W`\0\x80\xFD[Pa\x01\xA5a\x02 6`\x04a$\x19V[a\x06\x0CV[4\x80\x15a\x021W`\0\x80\xFD[Pa\x01\xA5a\x02@6`\x04a$\x19V[a\x061V[4\x80\x15a\x02QW`\0\x80\xFD[Pa\x01\xA5a\x02`6`\x04a$EV[a\x06\xAFV[a\x01\xA5a\x02s6`\x04a$`V[a\x07\x8EV[4\x80\x15a\x02\x84W`\0\x80\xFD[Pa\x01\xD7a\x08ZV[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x01\xA5a\x02\xA86`\x04a$\xADV[a\t\rV[4\x80\x15a\x02\xB9W`\0\x80\xFD[Pa\x01pa\x02\xC86`\x04a%6V[a\n}V[4\x80\x15a\x02\xD9W`\0\x80\xFD[Pa\x02\xEDa\x02\xE86`\x04a%`V[a\n\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01|V[4\x80\x15a\x03\x11W`\0\x80\xFD[Pa\x01pa\x03 6`\x04a$\x19V[a\n\xA8V[4\x80\x15a\x031W`\0\x80\xFD[Pa\x01\xD7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03eW`\0\x80\xFD[Pa\x01\xD7`\0\x81V[4\x80\x15a\x03zW`\0\x80\xFD[Pa\x01\xA5a\x03\x896`\x04a$EV[a\n\xD3V[4\x80\x15a\x03\x9AW`\0\x80\xFD[Pa\x01\xD7`\0\x80Q` a-8\x839\x81Q\x91R\x81V[4\x80\x15a\x03\xBCW`\0\x80\xFD[Pa\x01\xD7a\x03\xCB6`\x04a!\xB0V[a\n\xF6V[4\x80\x15a\x03\xDCW`\0\x80\xFD[Pa\x02\xED\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\x10W`\0\x80\xFD[Pa\x01\xA5a\x04\x1F6`\x04a$\x19V[a\x0B\rV[4\x80\x15a\x040W`\0\x80\xFD[Pa\x04Da\x04?6`\x04a%6V[a\x0B2V[`@Qa\x01|\x91\x90a%\xD2V[4\x80\x15a\x04]W`\0\x80\xFD[Pa\x01\xA5a\x04l6`\x04a!oV[a\x0B\xD8V[4\x80\x15a\x04}W`\0\x80\xFD[Pa\x01\xA5a\x04\x8C6`\x04a%6V[a\x0C\x19V[4\x80\x15a\x04\x9DW`\0\x80\xFD[Pa\x01\xA5a\x04\xAC6`\x04a%\xE5V[a\x0C\x99V[`\0a\x04\xBC\x82a\x0C\xA7V[\x92\x91PPV[`\0\x80Q` a-8\x839\x81Q\x91Ra\x04\xDA\x81a\x0C\xCCV[a\x04\xE4\x83\x83a\x0C\xD6V[PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x05\tWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x05#WP0;\x15\x80\x15a\x05#WP`\0T`\xFF\x16`\x01\x14[a\x05\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x05\xAEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x05\xB9`\0\x84a\r)V[a\x05\xC2\x82a\r3V[\x80\x15a\x04\xE4W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x06'\x81a\x0C\xCCV[a\x04\xE4\x83\x83a\r\xDFV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x06\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05\x82V[a\x06\xAB\x82\x82a\r\xE9V[PPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x06\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x90a&kV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x07@`\0\x80Q` a,\xF1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07fW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x90a&\xB7V[a\x07o\x81a\x0E4V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\x07\x8B\x91\x83\x91\x90a\x0E?V[PV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x07\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x90a&kV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x08\x1F`\0\x80Q` a,\xF1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x90a&\xB7V[a\x08N\x82a\x0E4V[a\x06\xAB\x82\x82`\x01a\x0E?V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x82V[P`\0\x80Q` a,\xF1\x839\x81Q\x91R\x90V[`\0\x80Q` a-8\x839\x81Q\x91Ra\t%\x81a\x0C\xCCV[\x84\x84`\0a\th\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0F\xAA\x92PPPV[\x90P`\0a\t\x8C\x82`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x01`` R`@\x90 T\x90V[\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG13`\xE8\x1B\x81RP\x90a\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[P`@\x80Q\x80\x82\x01\x82R`\x02\x81RaG7`\xF0\x1B` \x82\x01R\x90\x88\x14a\n\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[P`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81Ra\x03\x85` \x90\x81R`@\x80\x83 \x8C\x84R\x90\x91R\x90 a\n0\x88\x8A\x83a'\x83V[Pa\n;\x86\x86a\x0C\xD6V[`@Q\x89\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7FR\x1E=\xAC\x83\xE2\x8EU\xEF\xB9A\xFDXz\xBA\xB2\ty|\xEAqA\xEFP\xAA2\xF0\\\n;\xDB\x17\x90`\0\x90\xA3PPPPPPPPPPV[`\0a\n\x89\x83\x83a\x0F\xF7V[\x93\x92PPPV[`\0\x82\x81R`\x97` R`@\x81 a\n\x89\x90\x83a\x10MV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\n\xDE\x81a\x0C\xCCV[a\x06\xAB`\0\x80Q` a-8\x839\x81Q\x91R\x83a\r\xDFV[`\0\x81\x81R`\x97` R`@\x81 a\x04\xBC\x90a\x10YV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x0B(\x81a\x0C\xCCV[a\x04\xE4\x83\x83a\r\xE9V[a\x03\x85` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x0BW\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x83\x90a'\x03V[\x80\x15a\x0B\xD0W\x80`\x1F\x10a\x0B\xA5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xD0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x80Q` a-8\x839\x81Q\x91Ra\x0B\xF0\x81a\x0C\xCCV[`\0\x80\x80a\x0C\0\x85\x87\x01\x87a(BV[\x92P\x92P\x92Pa\x0C\x11\x83\x83\x83a\x10cV[PPPPPPV[`\0\x80Q` a-8\x839\x81Q\x91Ra\x0C1\x81a\x0C\xCCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\x85` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 a\x0C^\x91a \xAFV[`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xD4\x08O{z\x07\x9E|\xBF\xEE\x05\x96\xA4r1\xBAK\x1F\x97\x83a,\xB2r\x0E\xDE{\xF3\x8Eg\x10\xA2\x90`\0\x90\xA3PPPV[a\x0C\x11\x86\x86\x86\x86\x86\x86a\x10\xAFV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x04\xBCWPa\x04\xBC\x82a\x14\xFFV[a\x07\x8B\x813a\x154V[`\0\x80\x80\x80\x80\x80\x80\x80a\x0C\xEB\x89\x8B\x01\x8Ba(\xC9V[\x97P\x97P\x97P\x97P\x97P\x97P\x97P\x97Pa\r\x1D\x88\x88a\r\x15\x88\x88\x8Ba\x15\x8D\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86\x86\x86a\x10\xAFV[PPPPPPPPPPV[a\x06\xAB\x82\x82a\r\xDFV[`\0Ta\x01\0\x90\x04`\xFF\x16a\r\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0[\x81Q\x81\x10\x15a\x06\xABWa\r\xCC\x82\x82\x81Q\x81\x10a\r\xBFWa\r\xBFa)\xB5V[` \x02` \x01\x01Qa\x15\xC6V[P\x80a\r\xD7\x81a)\xE1V[\x91PPa\r\xA1V[a\x06\xAB\x82\x82a\x17\xA6V[a\r\xF3\x82\x82a\x17\xC8V[a\r\xFD`\0a\n\xF6V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra \x99`\xF1\x1B` \x82\x01R\x90a\x04\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[`\0a\x06\xAB\x81a\x0C\xCCV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x0ErWa\x04\xE4\x83a\x17\xEAV[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0E\xCCWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0E\xC9\x91\x81\x01\x90a)\xFAV[`\x01[a\x0F/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0\x80Q` a,\xF1\x839\x81Q\x91R\x81\x14a\x0F\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01a\x05\x82V[Pa\x04\xE4\x83\x83\x83a\x18\x86V[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90a\x0F\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[PP\x80Q` \x90\x91\x01 \x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` R`@\x81 T\x80\x15\x80\x15\x90a\x10\x1FWP\x82\x81\x14[\x80\x15a\x10EWP`\0\x81\x81Ra\x01_` R`@\x90 \x80Ta\x10@\x90a'\x03V[\x15\x15\x90P[\x94\x93PPPPV[`\0a\n\x89\x83\x83a\x18\xABV[`\0a\x04\xBC\x82T\x90V[`\0a\x10p\x84\x84\x84a\x15\x8DV[\x90Pa\x10{\x81a\x18\xD5V[QQ`\0\x03a\x10\xA9Wa\x10\xA7`@Q\x80``\x01`@R\x80\x86\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x15\xC6V[P[PPPPV[`\0\x84\x81Ra\x01_` R`@\x90 \x80Ta\x10\xC9\x90a'\x03V[\x90P`\0\x03a\x110W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAA:VK-Enclave image to verify no`D\x82\x01Rl\x1D\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\x9A\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0a\x11;\x86a\x1A\xCFV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01`` R`@\x90 T\x90\x91P\x15a\x11\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAA:VK-Enclave key already verifi`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01a\x05\x82V[a\x11\xDA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba*\x13V[a\x11\xE6a\x03\xE8\x84a*&V[\x11a\x123W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAA:VK-Attestation too old\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[`\0\x85\x81Ra\x01_` R`@\x80\x82 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x12[\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\x87\x90a'\x03V[\x80\x15a\x12\xD4W\x80`\x1F\x10a\x12\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\xD4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xB7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x12\xED\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x19\x90a'\x03V[\x80\x15a\x13fW\x80`\x1F\x10a\x13;Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13fV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13IW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x13\x7F\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\xAB\x90a'\x03V[\x80\x15a\x13\xF8W\x80`\x1F\x10a\x13\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xF8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xAC\x0F\x0B\xD5\x89\x89\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x8B\x8B\x8B`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14i\x98\x97\x96\x95\x94\x93\x92\x91\x90a*HV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\x81W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\x95W=`\0\x80>=`\0\xFD[PPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x01`` R`@\x90\x81\x90 \x88\x90UQ\x87\x91Pa\x14\xC5\x90\x89\x90a*\xCDV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3PPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\xBCWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04\xBCV[a\x15>\x82\x82a\n\xA8V[a\x06\xABWa\x15K\x81a\x1B.V[a\x15V\x83` a\x1B@V[`@Q` \x01a\x15g\x92\x91\x90a*\xE9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05\x82\x91`\x04\x01a%\xD2V[`\0\x80\x84\x84\x84`@Q` \x01a\x15\xA5\x93\x92\x91\x90a+^V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[\x80QQ`\0\x90`0\x14\x80\x15a\x15\xE0WP\x81` \x01QQ`0\x14[\x80\x15a\x15\xF1WP\x81`@\x01QQ`0\x14[a\x16GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FAA:WI-PCR values must be 48 byte`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q` \x01a\x16j\x93\x92\x91\x90a+^V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81Ra\x01_\x90\x93R\x91 \x80T\x91\x92P\x90a\x16\x9E\x90a'\x03V[\x15\x90Pa\x16\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FAA:WI-image already whitelisted\0`D\x82\x01R`d\x01a\x05\x82V[`@\x80Q``\x81\x01\x82R\x84Q\x81R` \x80\x86\x01Q\x81\x83\x01R\x85\x83\x01Q\x82\x84\x01R`\0\x84\x81Ra\x01_\x90\x91R\x91\x90\x91 \x81Q\x81\x90a\x17*\x90\x82a+\xA1V[P` \x82\x01Q`\x01\x82\x01\x90a\x17?\x90\x82a+\xA1V[P`@\x82\x01Q`\x02\x82\x01\x90a\x17T\x90\x82a+\xA1V[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q`@Qa\x17\x98\x93\x92\x91\x90a,`V[`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[a\x17\xB0\x82\x82a\x1C\xDBV[`\0\x82\x81R`\x97` R`@\x90 a\x04\xE4\x90\x82a\x1DaV[a\x17\xD2\x82\x82a\x1DvV[`\0\x82\x81R`\x97` R`@\x90 a\x04\xE4\x90\x82a\x1D\xDDV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x18WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0\x80Q` a,\xF1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x18\x8F\x83a\x1D\xF2V[`\0\x82Q\x11\x80a\x18\x9CWP\x80[\x15a\x04\xE4Wa\x10\xA9\x83\x83a\x1E2V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x18\xC2Wa\x18\xC2a)\xB5V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[a\x18\xF9`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81Ra\x01_` R`@\x90\x81\x90 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x19\"\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19N\x90a'\x03V[\x80\x15a\x19\x9BW\x80`\x1F\x10a\x19pWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\x9BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19~W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x19\xB4\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xE0\x90a'\x03V[\x80\x15a\x1A-W\x80`\x1F\x10a\x1A\x02Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A-V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\x10W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x1AF\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Ar\x90a'\x03V[\x80\x15a\x1A\xBFW\x80`\x1F\x10a\x1A\x94Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xBFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xA2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0\x81Q`@\x14a\x1B\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid public key length\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[P\x80Q` \x90\x91\x01 \x90V[``a\x04\xBC`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x1BO\x83`\x02a,\x99V[a\x1BZ\x90`\x02a,\xB0V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BqWa\x1Bqa!\xE5V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1B\x9BW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x1B\xB6Wa\x1B\xB6a)\xB5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x1B\xE5Wa\x1B\xE5a)\xB5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x1C\t\x84`\x02a,\x99V[a\x1C\x14\x90`\x01a,\xB0V[\x90P[`\x01\x81\x11\x15a\x1C\x8CWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x1CHWa\x1CHa)\xB5V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1C^Wa\x1C^a)\xB5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x1C\x85\x81a,\xC3V[\x90Pa\x1C\x17V[P\x83\x15a\n\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05\x82V[a\x1C\xE5\x82\x82a\n\xA8V[a\x06\xABW`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1D\x1D3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\n\x89\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1EWV[a\x1D\x80\x82\x82a\n\xA8V[\x15a\x06\xABW`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\n\x89\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1E\xA6V[a\x1D\xFB\x81a\x17\xEAV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\n\x89\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a-\x11`'\x919a\x1F\x99V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x1E\x9EWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x04\xBCV[P`\0a\x04\xBCV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x1F\x8FW`\0a\x1E\xCA`\x01\x83a*\x13V[\x85T\x90\x91P`\0\x90a\x1E\xDE\x90`\x01\x90a*\x13V[\x90P\x81\x81\x14a\x1FCW`\0\x86`\0\x01\x82\x81T\x81\x10a\x1E\xFEWa\x1E\xFEa)\xB5V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x1F!Wa\x1F!a)\xB5V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x1FTWa\x1FTa,\xDAV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x04\xBCV[`\0\x91PPa\x04\xBCV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x1F\xB6\x91\x90a*\xCDV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1F\xF1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1F\xF6V[``\x91P[P\x91P\x91Pa \x07\x86\x83\x83\x87a \x11V[\x96\x95PPPPPPV[``\x83\x15a \x80W\x82Q`\0\x03a yW`\x01`\x01`\xA0\x1B\x03\x85\x16;a yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\x82V[P\x81a\x10EV[a\x10E\x83\x83\x81Q\x15a \x95W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[P\x80Ta \xBB\x90a'\x03V[`\0\x82U\x80`\x1F\x10a \xCBWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x07\x8B\x91\x90[\x80\x82\x11\x15a \xF9W`\0\x81U`\x01\x01a \xE5V[P\x90V[`\0` \x82\x84\x03\x12\x15a!\x0FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\n\x89W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a!9W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!PW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a!hW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a!\x82W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x98W`\0\x80\xFD[a!\xA4\x85\x82\x86\x01a!'V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a!\xC2W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\xE0W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\"\x1DWa\"\x1Da!\xE5V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\"KWa\"Ka!\xE5V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\"dW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"}Wa\"}a!\xE5V[a\"\x90`\x1F\x82\x01`\x1F\x19\x16` \x01a\"#V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\"\xA5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\"\xD5W`\0\x80\xFD[a\"\xDE\x83a!\xC9V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"\xFBW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a#\x0FW`\0\x80\xFD[\x815\x81\x81\x11\x15a#!Wa#!a!\xE5V[\x80`\x05\x1Ba#0\x85\x82\x01a\"#V[\x91\x82R\x83\x81\x01\x85\x01\x91\x85\x81\x01\x90\x8A\x84\x11\x15a#JW`\0\x80\xFD[\x86\x86\x01\x92P[\x83\x83\x10\x15a$\x08W\x825\x85\x81\x11\x15a#hW`\0\x80\x81\xFD[\x86\x01``\x81\x8D\x03`\x1F\x19\x01\x81\x13\x15a#\x80W`\0\x80\x81\xFD[a#\x88a!\xFBV[\x89\x83\x015\x88\x81\x11\x15a#\x9AW`\0\x80\x81\xFD[a#\xA8\x8F\x8C\x83\x87\x01\x01a\"SV[\x82RP`@\x83\x015\x88\x81\x11\x15a#\xBEW`\0\x80\x81\xFD[a#\xCC\x8F\x8C\x83\x87\x01\x01a\"SV[\x82\x8C\x01RP\x90\x82\x015\x90\x87\x82\x11\x15a#\xE4W`\0\x80\x81\xFD[a#\xF2\x8E\x8B\x84\x86\x01\x01a\"SV[`@\x82\x01R\x84RPP\x91\x86\x01\x91\x90\x86\x01\x90a#PV[\x80\x97PPPPPPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$,W`\0\x80\xFD[\x825\x91Pa$<` \x84\x01a!\xC9V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a$WW`\0\x80\xFD[a\n\x89\x82a!\xC9V[`\0\x80`@\x83\x85\x03\x12\x15a$sW`\0\x80\xFD[a$|\x83a!\xC9V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x97W`\0\x80\xFD[a$\xA3\x85\x82\x86\x01a\"SV[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a$\xC6W`\0\x80\xFD[a$\xCF\x87a!\xC9V[\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$\xF2W`\0\x80\xFD[a$\xFE\x8A\x83\x8B\x01a!'V[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a%\x17W`\0\x80\xFD[Pa%$\x89\x82\x8A\x01a!'V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a%IW`\0\x80\xFD[a%R\x83a!\xC9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a%sW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a%\x9DW\x81\x81\x01Q\x83\x82\x01R` \x01a%\x85V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra%\xBE\x81` \x86\x01` \x86\x01a%\x82V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\n\x89` \x83\x01\x84a%\xA6V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a%\xFEW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a&\x15W`\0\x80\xFD[a&!\x8A\x83\x8B\x01a\"SV[\x97P` \x89\x015\x91P\x80\x82\x11\x15a&7W`\0\x80\xFD[Pa&D\x89\x82\x8A\x01a\"SV[\x96\x99\x96\x98PPPP`@\x85\x015\x94``\x81\x015\x94`\x80\x82\x015\x94P`\xA0\x90\x91\x015\x92P\x90PV[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a'\x17W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x04\xE4W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a'dWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\x11W\x82\x81U`\x01\x01a'pV[`\x01`\x01`@\x1B\x03\x83\x11\x15a'\x9AWa'\x9Aa!\xE5V[a'\xAE\x83a'\xA8\x83Ta'\x03V[\x83a'=V[`\0`\x1F\x84\x11`\x01\x81\x14a'\xE2W`\0\x85\x15a'\xCAWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x10\xA7V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a(\x13W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a'\xF3V[P\x86\x82\x10\x15a(0W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a(WW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a(nW`\0\x80\xFD[a(z\x87\x83\x88\x01a\"SV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a(\x90W`\0\x80\xFD[a(\x9C\x87\x83\x88\x01a\"SV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a(\xB2W`\0\x80\xFD[Pa(\xBF\x86\x82\x87\x01a\"SV[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a(\xE6W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a(\xFDW`\0\x80\xFD[a)\t\x8C\x83\x8D\x01a\"SV[\x99P` \x8B\x015\x91P\x80\x82\x11\x15a)\x1FW`\0\x80\xFD[a)+\x8C\x83\x8D\x01a\"SV[\x98P`@\x8B\x015\x91P\x80\x82\x11\x15a)AW`\0\x80\xFD[a)M\x8C\x83\x8D\x01a\"SV[\x97P``\x8B\x015\x91P\x80\x82\x11\x15a)cW`\0\x80\xFD[a)o\x8C\x83\x8D\x01a\"SV[\x96P`\x80\x8B\x015\x91P\x80\x82\x11\x15a)\x85W`\0\x80\xFD[Pa)\x92\x8B\x82\x8C\x01a\"SV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x96`\xA0\x86\x015\x96P`\xC0\x86\x015\x95`\xE0\x015\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a)\xF3Wa)\xF3a)\xCBV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a*\x0CW`\0\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x04\xBCWa\x04\xBCa)\xCBV[`\0\x82a*CWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0a\x01\0\x80\x83Ra*\\\x81\x84\x01\x8Ca%\xA6V[\x90P\x82\x81\x03` \x84\x01Ra*p\x81\x8Ba%\xA6V[\x90P\x82\x81\x03`@\x84\x01Ra*\x84\x81\x8Aa%\xA6V[\x90P\x82\x81\x03``\x84\x01Ra*\x98\x81\x89a%\xA6V[\x90P\x82\x81\x03`\x80\x84\x01Ra*\xAC\x81\x88a%\xA6V[`\xA0\x84\x01\x96\x90\x96RPP`\xC0\x81\x01\x92\x90\x92R`\xE0\x90\x91\x01R\x95\x94PPPPPV[`\0\x82Qa*\xDF\x81\x84` \x87\x01a%\x82V[\x91\x90\x91\x01\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa+!\x81`\x17\x85\x01` \x88\x01a%\x82V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa+R\x81`(\x84\x01` \x88\x01a%\x82V[\x01`(\x01\x94\x93PPPPV[`\0\x84Qa+p\x81\x84` \x89\x01a%\x82V[\x84Q\x90\x83\x01\x90a+\x84\x81\x83` \x89\x01a%\x82V[\x84Q\x91\x01\x90a+\x97\x81\x83` \x88\x01a%\x82V[\x01\x95\x94PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xBAWa+\xBAa!\xE5V[a+\xCE\x81a+\xC8\x84Ta'\x03V[\x84a'=V[` \x80`\x1F\x83\x11`\x01\x81\x14a,\x03W`\0\x84\x15a+\xEBWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\x11V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a,2W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a,\x13V[P\x85\x82\x10\x15a,PW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R`\0a,s``\x83\x01\x86a%\xA6V[\x82\x81\x03` \x84\x01Ra,\x85\x81\x86a%\xA6V[\x90P\x82\x81\x03`@\x84\x01Ra \x07\x81\x85a%\xA6V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xBCWa\x04\xBCa)\xCBV[\x80\x82\x01\x80\x82\x11\x15a\x04\xBCWa\x04\xBCa)\xCBV[`\0\x81a,\xD2Wa,\xD2a)\xCBV[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCAddress: low-level delegate call failed\xDCi|\xE6\xC0\xC3\xF8\x86\xFB\xBC\x8E\r#:\xBF\xACZ\xC56S\xAC\x08\xB3c[\xB0\xAE\xC7\x0C9\\B\xA2dipfsX\"\x12 O\xE5\x1B\xDD\x0B\x12En\xA9\xCD\x8C\xCE.\xF9F\xAF&\r3\xE5\xD4\xF1w\xC1O\xE0u+[\xD0\xF9\xD4dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ENTITYKEYREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01KW`\x005`\xE0\x1C\x80c\x91\xD1HT\x11a\0\xB6W\x80c\xCDy\xF9\x06\x11a\0oW\x80c\xCDy\xF9\x06\x14a\x03\xD0W\x80c\xD5Gt\x1F\x14a\x04\x04W\x80c\xE1\xA610\x14a\x04$W\x80c\xEC\xEA\x1D}\x14a\x04QW\x80c\xED8\r\x03\x14a\x04qW\x80c\xFB\x94\xDB+\x14a\x04\x91W`\0\x80\xFD[\x80c\x91\xD1HT\x14a\x03\x05W\x80c\x9A\xEC\x99\x0E\x14a\x03%W\x80c\xA2\x17\xFD\xDF\x14a\x03YW\x80c\xAA\x0E\xD0\x9F\x14a\x03nW\x80c\xB8\n\xAA\x89\x14a\x03\x8EW\x80c\xCA\x15\xC8s\x14a\x03\xB0W`\0\x80\xFD[\x80c6Y\xCF\xE6\x11a\x01\x08W\x80c6Y\xCF\xE6\x14a\x02EW\x80cO\x1E\xF2\x86\x14a\x02eW\x80cR\xD1\x90-\x14a\x02xW\x80ci\xFD\xBC\xCA\x14a\x02\x8DW\x80c~ \x1Be\x14a\x02\xADW\x80c\x90\x10\xD0|\x14a\x02\xCDW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01PW\x80c\x07\x07Y\x1F\x14a\x01\x85W\x80c$\x8A\x9C\xA3\x14a\x01\xA7W\x80c.\xB3\x9E\xE9\x14a\x01\xE5W\x80c//\xF1]\x14a\x02\x05W\x80c6V\x8A\xBE\x14a\x02%W[`\0\x80\xFD[4\x80\x15a\x01\\W`\0\x80\xFD[Pa\x01pa\x01k6`\x04a \xFDV[a\x04\xB1V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x91W`\0\x80\xFD[Pa\x01\xA5a\x01\xA06`\x04a!oV[a\x04\xC2V[\0[4\x80\x15a\x01\xB3W`\0\x80\xFD[Pa\x01\xD7a\x01\xC26`\x04a!\xB0V[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01|V[4\x80\x15a\x01\xF1W`\0\x80\xFD[Pa\x01\xA5a\x02\x006`\x04a\"\xC2V[a\x04\xE9V[4\x80\x15a\x02\x11W`\0\x80\xFD[Pa\x01\xA5a\x02 6`\x04a$\x19V[a\x06\x0CV[4\x80\x15a\x021W`\0\x80\xFD[Pa\x01\xA5a\x02@6`\x04a$\x19V[a\x061V[4\x80\x15a\x02QW`\0\x80\xFD[Pa\x01\xA5a\x02`6`\x04a$EV[a\x06\xAFV[a\x01\xA5a\x02s6`\x04a$`V[a\x07\x8EV[4\x80\x15a\x02\x84W`\0\x80\xFD[Pa\x01\xD7a\x08ZV[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x01\xA5a\x02\xA86`\x04a$\xADV[a\t\rV[4\x80\x15a\x02\xB9W`\0\x80\xFD[Pa\x01pa\x02\xC86`\x04a%6V[a\n}V[4\x80\x15a\x02\xD9W`\0\x80\xFD[Pa\x02\xEDa\x02\xE86`\x04a%`V[a\n\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01|V[4\x80\x15a\x03\x11W`\0\x80\xFD[Pa\x01pa\x03 6`\x04a$\x19V[a\n\xA8V[4\x80\x15a\x031W`\0\x80\xFD[Pa\x01\xD7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03eW`\0\x80\xFD[Pa\x01\xD7`\0\x81V[4\x80\x15a\x03zW`\0\x80\xFD[Pa\x01\xA5a\x03\x896`\x04a$EV[a\n\xD3V[4\x80\x15a\x03\x9AW`\0\x80\xFD[Pa\x01\xD7`\0\x80Q` a-8\x839\x81Q\x91R\x81V[4\x80\x15a\x03\xBCW`\0\x80\xFD[Pa\x01\xD7a\x03\xCB6`\x04a!\xB0V[a\n\xF6V[4\x80\x15a\x03\xDCW`\0\x80\xFD[Pa\x02\xED\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\x10W`\0\x80\xFD[Pa\x01\xA5a\x04\x1F6`\x04a$\x19V[a\x0B\rV[4\x80\x15a\x040W`\0\x80\xFD[Pa\x04Da\x04?6`\x04a%6V[a\x0B2V[`@Qa\x01|\x91\x90a%\xD2V[4\x80\x15a\x04]W`\0\x80\xFD[Pa\x01\xA5a\x04l6`\x04a!oV[a\x0B\xD8V[4\x80\x15a\x04}W`\0\x80\xFD[Pa\x01\xA5a\x04\x8C6`\x04a%6V[a\x0C\x19V[4\x80\x15a\x04\x9DW`\0\x80\xFD[Pa\x01\xA5a\x04\xAC6`\x04a%\xE5V[a\x0C\x99V[`\0a\x04\xBC\x82a\x0C\xA7V[\x92\x91PPV[`\0\x80Q` a-8\x839\x81Q\x91Ra\x04\xDA\x81a\x0C\xCCV[a\x04\xE4\x83\x83a\x0C\xD6V[PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x05\tWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x05#WP0;\x15\x80\x15a\x05#WP`\0T`\xFF\x16`\x01\x14[a\x05\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x05\xAEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x05\xB9`\0\x84a\r)V[a\x05\xC2\x82a\r3V[\x80\x15a\x04\xE4W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x06'\x81a\x0C\xCCV[a\x04\xE4\x83\x83a\r\xDFV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x06\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x05\x82V[a\x06\xAB\x82\x82a\r\xE9V[PPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x06\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x90a&kV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x07@`\0\x80Q` a,\xF1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07fW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x90a&\xB7V[a\x07o\x81a\x0E4V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\x07\x8B\x91\x83\x91\x90a\x0E?V[PV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x07\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x90a&kV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x08\x1F`\0\x80Q` a,\xF1\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x90a&\xB7V[a\x08N\x82a\x0E4V[a\x06\xAB\x82\x82`\x01a\x0E?V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x82V[P`\0\x80Q` a,\xF1\x839\x81Q\x91R\x90V[`\0\x80Q` a-8\x839\x81Q\x91Ra\t%\x81a\x0C\xCCV[\x84\x84`\0a\th\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0F\xAA\x92PPPV[\x90P`\0a\t\x8C\x82`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81Ra\x01`` R`@\x90 T\x90V[\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG13`\xE8\x1B\x81RP\x90a\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[P`@\x80Q\x80\x82\x01\x82R`\x02\x81RaG7`\xF0\x1B` \x82\x01R\x90\x88\x14a\n\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[P`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81Ra\x03\x85` \x90\x81R`@\x80\x83 \x8C\x84R\x90\x91R\x90 a\n0\x88\x8A\x83a'\x83V[Pa\n;\x86\x86a\x0C\xD6V[`@Q\x89\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7FR\x1E=\xAC\x83\xE2\x8EU\xEF\xB9A\xFDXz\xBA\xB2\ty|\xEAqA\xEFP\xAA2\xF0\\\n;\xDB\x17\x90`\0\x90\xA3PPPPPPPPPPV[`\0a\n\x89\x83\x83a\x0F\xF7V[\x93\x92PPPV[`\0\x82\x81R`\x97` R`@\x81 a\n\x89\x90\x83a\x10MV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0a\n\xDE\x81a\x0C\xCCV[a\x06\xAB`\0\x80Q` a-8\x839\x81Q\x91R\x83a\r\xDFV[`\0\x81\x81R`\x97` R`@\x81 a\x04\xBC\x90a\x10YV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x0B(\x81a\x0C\xCCV[a\x04\xE4\x83\x83a\r\xE9V[a\x03\x85` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80Ta\x0BW\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x83\x90a'\x03V[\x80\x15a\x0B\xD0W\x80`\x1F\x10a\x0B\xA5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xD0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x80Q` a-8\x839\x81Q\x91Ra\x0B\xF0\x81a\x0C\xCCV[`\0\x80\x80a\x0C\0\x85\x87\x01\x87a(BV[\x92P\x92P\x92Pa\x0C\x11\x83\x83\x83a\x10cV[PPPPPPV[`\0\x80Q` a-8\x839\x81Q\x91Ra\x0C1\x81a\x0C\xCCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03\x85` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 a\x0C^\x91a \xAFV[`@Q\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xD4\x08O{z\x07\x9E|\xBF\xEE\x05\x96\xA4r1\xBAK\x1F\x97\x83a,\xB2r\x0E\xDE{\xF3\x8Eg\x10\xA2\x90`\0\x90\xA3PPPV[a\x0C\x11\x86\x86\x86\x86\x86\x86a\x10\xAFV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x04\xBCWPa\x04\xBC\x82a\x14\xFFV[a\x07\x8B\x813a\x154V[`\0\x80\x80\x80\x80\x80\x80\x80a\x0C\xEB\x89\x8B\x01\x8Ba(\xC9V[\x97P\x97P\x97P\x97P\x97P\x97P\x97P\x97Pa\r\x1D\x88\x88a\r\x15\x88\x88\x8Ba\x15\x8D\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86\x86\x86a\x10\xAFV[PPPPPPPPPPV[a\x06\xAB\x82\x82a\r\xDFV[`\0Ta\x01\0\x90\x04`\xFF\x16a\r\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0[\x81Q\x81\x10\x15a\x06\xABWa\r\xCC\x82\x82\x81Q\x81\x10a\r\xBFWa\r\xBFa)\xB5V[` \x02` \x01\x01Qa\x15\xC6V[P\x80a\r\xD7\x81a)\xE1V[\x91PPa\r\xA1V[a\x06\xAB\x82\x82a\x17\xA6V[a\r\xF3\x82\x82a\x17\xC8V[a\r\xFD`\0a\n\xF6V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra \x99`\xF1\x1B` \x82\x01R\x90a\x04\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[`\0a\x06\xAB\x81a\x0C\xCCV[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a\x0ErWa\x04\xE4\x83a\x17\xEAV[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0E\xCCWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0E\xC9\x91\x81\x01\x90a)\xFAV[`\x01[a\x0F/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0\x80Q` a,\xF1\x839\x81Q\x91R\x81\x14a\x0F\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01a\x05\x82V[Pa\x04\xE4\x83\x83\x83a\x18\x86V[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90a\x0F\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[PP\x80Q` \x90\x91\x01 \x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x01`` R`@\x81 T\x80\x15\x80\x15\x90a\x10\x1FWP\x82\x81\x14[\x80\x15a\x10EWP`\0\x81\x81Ra\x01_` R`@\x90 \x80Ta\x10@\x90a'\x03V[\x15\x15\x90P[\x94\x93PPPPV[`\0a\n\x89\x83\x83a\x18\xABV[`\0a\x04\xBC\x82T\x90V[`\0a\x10p\x84\x84\x84a\x15\x8DV[\x90Pa\x10{\x81a\x18\xD5V[QQ`\0\x03a\x10\xA9Wa\x10\xA7`@Q\x80``\x01`@R\x80\x86\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x15\xC6V[P[PPPPV[`\0\x84\x81Ra\x01_` R`@\x90 \x80Ta\x10\xC9\x90a'\x03V[\x90P`\0\x03a\x110W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAA:VK-Enclave image to verify no`D\x82\x01Rl\x1D\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\x9A\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0a\x11;\x86a\x1A\xCFV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81Ra\x01`` R`@\x90 T\x90\x91P\x15a\x11\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FAA:VK-Enclave key already verifi`D\x82\x01Ra\x19Y`\xF2\x1B`d\x82\x01R`\x84\x01a\x05\x82V[a\x11\xDA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba*\x13V[a\x11\xE6a\x03\xE8\x84a*&V[\x11a\x123W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FAA:VK-Attestation too old\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[`\0\x85\x81Ra\x01_` R`@\x80\x82 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x12[\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\x87\x90a'\x03V[\x80\x15a\x12\xD4W\x80`\x1F\x10a\x12\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\xD4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xB7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x12\xED\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x19\x90a'\x03V[\x80\x15a\x13fW\x80`\x1F\x10a\x13;Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13fV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13IW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x13\x7F\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\xAB\x90a'\x03V[\x80\x15a\x13\xF8W\x80`\x1F\x10a\x13\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xF8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xAC\x0F\x0B\xD5\x89\x89\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x8B\x8B\x8B`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14i\x98\x97\x96\x95\x94\x93\x92\x91\x90a*HV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14\x81W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\x95W=`\0\x80>=`\0\xFD[PPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x01`` R`@\x90\x81\x90 \x88\x90UQ\x87\x91Pa\x14\xC5\x90\x89\x90a*\xCDV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xF4\x83\x7F\x1Af\xA1\xE5\xFB\xDE,@\xD0\xF6\xEF\xF7Q\x83\x13\x9Ey\xB0\xFC\x1E\xAA\xB2\x80\xA3\xA0\xBE8\xE7\xB1\x90`\0\x90\xA3PPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\xBCWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04\xBCV[a\x15>\x82\x82a\n\xA8V[a\x06\xABWa\x15K\x81a\x1B.V[a\x15V\x83` a\x1B@V[`@Q` \x01a\x15g\x92\x91\x90a*\xE9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x05\x82\x91`\x04\x01a%\xD2V[`\0\x80\x84\x84\x84`@Q` \x01a\x15\xA5\x93\x92\x91\x90a+^V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[\x80QQ`\0\x90`0\x14\x80\x15a\x15\xE0WP\x81` \x01QQ`0\x14[\x80\x15a\x15\xF1WP\x81`@\x01QQ`0\x14[a\x16GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FAA:WI-PCR values must be 48 byte`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q` \x01a\x16j\x93\x92\x91\x90a+^V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81Ra\x01_\x90\x93R\x91 \x80T\x91\x92P\x90a\x16\x9E\x90a'\x03V[\x15\x90Pa\x16\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FAA:WI-image already whitelisted\0`D\x82\x01R`d\x01a\x05\x82V[`@\x80Q``\x81\x01\x82R\x84Q\x81R` \x80\x86\x01Q\x81\x83\x01R\x85\x83\x01Q\x82\x84\x01R`\0\x84\x81Ra\x01_\x90\x91R\x91\x90\x91 \x81Q\x81\x90a\x17*\x90\x82a+\xA1V[P` \x82\x01Q`\x01\x82\x01\x90a\x17?\x90\x82a+\xA1V[P`@\x82\x01Q`\x02\x82\x01\x90a\x17T\x90\x82a+\xA1V[P\x90PP\x80\x7FR\xB2\x9B\xBD\xD9z\xB9\x83A\x9FP\xFA\x15\x90\xE5\xABu\xE9\x94\"y\xE9^\x10\xA0\x86\x07\xB0l##\x8B\x84`\0\x01Q\x85` \x01Q\x86`@\x01Q`@Qa\x17\x98\x93\x92\x91\x90a,`V[`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[a\x17\xB0\x82\x82a\x1C\xDBV[`\0\x82\x81R`\x97` R`@\x90 a\x04\xE4\x90\x82a\x1DaV[a\x17\xD2\x82\x82a\x1DvV[`\0\x82\x81R`\x97` R`@\x90 a\x04\xE4\x90\x82a\x1D\xDDV[`\x01`\x01`\xA0\x1B\x03\x81\x16;a\x18WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\x05\x82V[`\0\x80Q` a,\xF1\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x18\x8F\x83a\x1D\xF2V[`\0\x82Q\x11\x80a\x18\x9CWP\x80[\x15a\x04\xE4Wa\x10\xA9\x83\x83a\x1E2V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x18\xC2Wa\x18\xC2a)\xB5V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[a\x18\xF9`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x82\x81Ra\x01_` R`@\x90\x81\x90 \x81Q``\x81\x01\x90\x92R\x80T\x82\x90\x82\x90a\x19\"\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19N\x90a'\x03V[\x80\x15a\x19\x9BW\x80`\x1F\x10a\x19pWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x19\x9BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19~W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x19\xB4\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xE0\x90a'\x03V[\x80\x15a\x1A-W\x80`\x1F\x10a\x1A\x02Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A-V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\x10W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x1AF\x90a'\x03V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Ar\x90a'\x03V[\x80\x15a\x1A\xBFW\x80`\x1F\x10a\x1A\x94Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xBFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xA2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0\x81Q`@\x14a\x1B\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FInvalid public key length\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x82V[P\x80Q` \x90\x91\x01 \x90V[``a\x04\xBC`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x1BO\x83`\x02a,\x99V[a\x1BZ\x90`\x02a,\xB0V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BqWa\x1Bqa!\xE5V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1B\x9BW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x1B\xB6Wa\x1B\xB6a)\xB5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x1B\xE5Wa\x1B\xE5a)\xB5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x1C\t\x84`\x02a,\x99V[a\x1C\x14\x90`\x01a,\xB0V[\x90P[`\x01\x81\x11\x15a\x1C\x8CWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x1CHWa\x1CHa)\xB5V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1C^Wa\x1C^a)\xB5V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x1C\x85\x81a,\xC3V[\x90Pa\x1C\x17V[P\x83\x15a\n\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x05\x82V[a\x1C\xE5\x82\x82a\n\xA8V[a\x06\xABW`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1D\x1D3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\n\x89\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1EWV[a\x1D\x80\x82\x82a\n\xA8V[\x15a\x06\xABW`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\n\x89\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1E\xA6V[a\x1D\xFB\x81a\x17\xEAV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a\n\x89\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a-\x11`'\x919a\x1F\x99V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x1E\x9EWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x04\xBCV[P`\0a\x04\xBCV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x1F\x8FW`\0a\x1E\xCA`\x01\x83a*\x13V[\x85T\x90\x91P`\0\x90a\x1E\xDE\x90`\x01\x90a*\x13V[\x90P\x81\x81\x14a\x1FCW`\0\x86`\0\x01\x82\x81T\x81\x10a\x1E\xFEWa\x1E\xFEa)\xB5V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x1F!Wa\x1F!a)\xB5V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x1FTWa\x1FTa,\xDAV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x04\xBCV[`\0\x91PPa\x04\xBCV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qa\x1F\xB6\x91\x90a*\xCDV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1F\xF1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1F\xF6V[``\x91P[P\x91P\x91Pa \x07\x86\x83\x83\x87a \x11V[\x96\x95PPPPPPV[``\x83\x15a \x80W\x82Q`\0\x03a yW`\x01`\x01`\xA0\x1B\x03\x85\x16;a yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\x82V[P\x81a\x10EV[a\x10E\x83\x83\x81Q\x15a \x95W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x82\x91\x90a%\xD2V[P\x80Ta \xBB\x90a'\x03V[`\0\x82U\x80`\x1F\x10a \xCBWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x07\x8B\x91\x90[\x80\x82\x11\x15a \xF9W`\0\x81U`\x01\x01a \xE5V[P\x90V[`\0` \x82\x84\x03\x12\x15a!\x0FW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\n\x89W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a!9W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!PW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a!hW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a!\x82W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x98W`\0\x80\xFD[a!\xA4\x85\x82\x86\x01a!'V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15a!\xC2W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\xE0W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\"\x1DWa\"\x1Da!\xE5V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\"KWa\"Ka!\xE5V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\"dW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"}Wa\"}a!\xE5V[a\"\x90`\x1F\x82\x01`\x1F\x19\x16` \x01a\"#V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\"\xA5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\"\xD5W`\0\x80\xFD[a\"\xDE\x83a!\xC9V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"\xFBW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a#\x0FW`\0\x80\xFD[\x815\x81\x81\x11\x15a#!Wa#!a!\xE5V[\x80`\x05\x1Ba#0\x85\x82\x01a\"#V[\x91\x82R\x83\x81\x01\x85\x01\x91\x85\x81\x01\x90\x8A\x84\x11\x15a#JW`\0\x80\xFD[\x86\x86\x01\x92P[\x83\x83\x10\x15a$\x08W\x825\x85\x81\x11\x15a#hW`\0\x80\x81\xFD[\x86\x01``\x81\x8D\x03`\x1F\x19\x01\x81\x13\x15a#\x80W`\0\x80\x81\xFD[a#\x88a!\xFBV[\x89\x83\x015\x88\x81\x11\x15a#\x9AW`\0\x80\x81\xFD[a#\xA8\x8F\x8C\x83\x87\x01\x01a\"SV[\x82RP`@\x83\x015\x88\x81\x11\x15a#\xBEW`\0\x80\x81\xFD[a#\xCC\x8F\x8C\x83\x87\x01\x01a\"SV[\x82\x8C\x01RP\x90\x82\x015\x90\x87\x82\x11\x15a#\xE4W`\0\x80\x81\xFD[a#\xF2\x8E\x8B\x84\x86\x01\x01a\"SV[`@\x82\x01R\x84RPP\x91\x86\x01\x91\x90\x86\x01\x90a#PV[\x80\x97PPPPPPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$,W`\0\x80\xFD[\x825\x91Pa$<` \x84\x01a!\xC9V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a$WW`\0\x80\xFD[a\n\x89\x82a!\xC9V[`\0\x80`@\x83\x85\x03\x12\x15a$sW`\0\x80\xFD[a$|\x83a!\xC9V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x97W`\0\x80\xFD[a$\xA3\x85\x82\x86\x01a\"SV[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a$\xC6W`\0\x80\xFD[a$\xCF\x87a!\xC9V[\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$\xF2W`\0\x80\xFD[a$\xFE\x8A\x83\x8B\x01a!'V[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15a%\x17W`\0\x80\xFD[Pa%$\x89\x82\x8A\x01a!'V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a%IW`\0\x80\xFD[a%R\x83a!\xC9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a%sW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a%\x9DW\x81\x81\x01Q\x83\x82\x01R` \x01a%\x85V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra%\xBE\x81` \x86\x01` \x86\x01a%\x82V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\n\x89` \x83\x01\x84a%\xA6V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a%\xFEW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a&\x15W`\0\x80\xFD[a&!\x8A\x83\x8B\x01a\"SV[\x97P` \x89\x015\x91P\x80\x82\x11\x15a&7W`\0\x80\xFD[Pa&D\x89\x82\x8A\x01a\"SV[\x96\x99\x96\x98PPPP`@\x85\x015\x94``\x81\x015\x94`\x80\x82\x015\x94P`\xA0\x90\x91\x015\x92P\x90PV[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a'\x17W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x04\xE4W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a'dWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\x11W\x82\x81U`\x01\x01a'pV[`\x01`\x01`@\x1B\x03\x83\x11\x15a'\x9AWa'\x9Aa!\xE5V[a'\xAE\x83a'\xA8\x83Ta'\x03V[\x83a'=V[`\0`\x1F\x84\x11`\x01\x81\x14a'\xE2W`\0\x85\x15a'\xCAWP\x83\x82\x015[`\0\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua\x10\xA7V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a(\x13W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a'\xF3V[P\x86\x82\x10\x15a(0W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a(WW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a(nW`\0\x80\xFD[a(z\x87\x83\x88\x01a\"SV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a(\x90W`\0\x80\xFD[a(\x9C\x87\x83\x88\x01a\"SV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a(\xB2W`\0\x80\xFD[Pa(\xBF\x86\x82\x87\x01a\"SV[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a(\xE6W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a(\xFDW`\0\x80\xFD[a)\t\x8C\x83\x8D\x01a\"SV[\x99P` \x8B\x015\x91P\x80\x82\x11\x15a)\x1FW`\0\x80\xFD[a)+\x8C\x83\x8D\x01a\"SV[\x98P`@\x8B\x015\x91P\x80\x82\x11\x15a)AW`\0\x80\xFD[a)M\x8C\x83\x8D\x01a\"SV[\x97P``\x8B\x015\x91P\x80\x82\x11\x15a)cW`\0\x80\xFD[a)o\x8C\x83\x8D\x01a\"SV[\x96P`\x80\x8B\x015\x91P\x80\x82\x11\x15a)\x85W`\0\x80\xFD[Pa)\x92\x8B\x82\x8C\x01a\"SV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x96`\xA0\x86\x015\x96P`\xC0\x86\x015\x95`\xE0\x015\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a)\xF3Wa)\xF3a)\xCBV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a*\x0CW`\0\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x04\xBCWa\x04\xBCa)\xCBV[`\0\x82a*CWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0a\x01\0\x80\x83Ra*\\\x81\x84\x01\x8Ca%\xA6V[\x90P\x82\x81\x03` \x84\x01Ra*p\x81\x8Ba%\xA6V[\x90P\x82\x81\x03`@\x84\x01Ra*\x84\x81\x8Aa%\xA6V[\x90P\x82\x81\x03``\x84\x01Ra*\x98\x81\x89a%\xA6V[\x90P\x82\x81\x03`\x80\x84\x01Ra*\xAC\x81\x88a%\xA6V[`\xA0\x84\x01\x96\x90\x96RPP`\xC0\x81\x01\x92\x90\x92R`\xE0\x90\x91\x01R\x95\x94PPPPPV[`\0\x82Qa*\xDF\x81\x84` \x87\x01a%\x82V[\x91\x90\x91\x01\x92\x91PPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa+!\x81`\x17\x85\x01` \x88\x01a%\x82V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa+R\x81`(\x84\x01` \x88\x01a%\x82V[\x01`(\x01\x94\x93PPPPV[`\0\x84Qa+p\x81\x84` \x89\x01a%\x82V[\x84Q\x90\x83\x01\x90a+\x84\x81\x83` \x89\x01a%\x82V[\x84Q\x91\x01\x90a+\x97\x81\x83` \x88\x01a%\x82V[\x01\x95\x94PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xBAWa+\xBAa!\xE5V[a+\xCE\x81a+\xC8\x84Ta'\x03V[\x84a'=V[` \x80`\x1F\x83\x11`\x01\x81\x14a,\x03W`\0\x84\x15a+\xEBWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\x11V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a,2W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a,\x13V[P\x85\x82\x10\x15a,PW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[``\x81R`\0a,s``\x83\x01\x86a%\xA6V[\x82\x81\x03` \x84\x01Ra,\x85\x81\x86a%\xA6V[\x90P\x82\x81\x03`@\x84\x01Ra \x07\x81\x85a%\xA6V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xBCWa\x04\xBCa)\xCBV[\x80\x82\x01\x80\x82\x11\x15a\x04\xBCWa\x04\xBCa)\xCBV[`\0\x81a,\xD2Wa,\xD2a)\xCBV[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCAddress: low-level delegate call failed\xDCi|\xE6\xC0\xC3\xF8\x86\xFB\xBC\x8E\r#:\xBF\xACZ\xC56S\xAC\x08\xB3c[\xB0\xAE\xC7\x0C9\\B\xA2dipfsX\"\x12 O\xE5\x1B\xDD\x0B\x12En\xA9\xCD\x8C\xCE.\xF9F\xAF&\r3\xE5\xD4\xF1w\xC1O\xE0u+[\xD0\xF9\xD4dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ENTITYKEYREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct EntityKeyRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EntityKeyRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EntityKeyRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EntityKeyRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EntityKeyRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EntityKeyRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EntityKeyRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ENTITYKEYREGISTRY_ABI.clone(),
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
                ENTITYKEYREGISTRY_ABI.clone(),
                ENTITYKEYREGISTRY_BYTECODE.clone().into(),
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
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `KEY_REGISTER_ROLE` (0xb80aaa89) function
        pub fn key_register_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([184, 10, 170, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addGeneratorRegistry` (0xaa0ed09f) function
        pub fn add_generator_registry(
            &self,
            generator_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 14, 208, 159], generator_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowOnlyVerified` (0x7e201b65) function
        pub fn allow_only_verified(
            &self,
            key: ::ethers::core::types::Address,
            image_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([126, 32, 27, 101], (key, image_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMember` (0x9010d07c) function
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMemberCount` (0xca15c873) function
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 21, 200, 115], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x2eb39ee9) function
        pub fn initialize(
            &self,
            admin: ::ethers::core::types::Address,
            init_whitelist_images: ::std::vec::Vec<EnclaveImage>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 179, 158, 233], (admin, init_whitelist_images))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pub_key` (0xe1a63130) function
        pub fn pub_key(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([225, 166, 49, 48], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePubkey` (0xed380d03) function
        pub fn remove_pubkey(
            &self,
            key_owner: ::ethers::core::types::Address,
            key_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 56, 13, 3], (key_owner, key_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePubkey` (0x69fdbcca) function
        pub fn update_pubkey(
            &self,
            key_owner: ::ethers::core::types::Address,
            key_index: ::ethers::core::types::U256,
            pubkey: ::ethers::core::types::Bytes,
            attestation_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [105, 253, 188, 202],
                    (key_owner, key_index, pubkey, attestation_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeTo` (0x3659cfe6) function
        pub fn upgrade_to(
            &self,
            new_implementation: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyKey` (0x0707591f) function
        pub fn verify_key(
            &self,
            attestation_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 7, 89, 31], attestation_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyKey` (0xfb94db2b) function
        pub fn verify_key_with_signature(
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
        ///Calls the contract's `whitelistImageUsingPcrs` (0xecea1d7d) function
        pub fn whitelist_image_using_pcrs(
            &self,
            pcrs: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 234, 29, 125], pcrs)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BeaconUpgraded` event
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BeaconUpgradedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `RemoveKey` event
        pub fn remove_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoveKeyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UpdateKey` event
        pub fn update_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdateKeyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EntityKeyRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for EntityKeyRegistry<M> {
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ::ethers::core::types::Address,
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
    #[ethevent(name = "RemoveKey", abi = "RemoveKey(address,uint256)")]
    pub struct RemoveKeyFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub key_index: ::ethers::core::types::U256,
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "UpdateKey", abi = "UpdateKey(address,uint256)")]
    pub struct UpdateKeyFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub key_index: ::ethers::core::types::U256,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
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
    pub enum EntityKeyRegistryEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        EnclaveImageRevokedFilter(EnclaveImageRevokedFilter),
        EnclaveImageWhitelistedFilter(EnclaveImageWhitelistedFilter),
        EnclaveKeyRevokedFilter(EnclaveKeyRevokedFilter),
        EnclaveKeyVerifiedFilter(EnclaveKeyVerifiedFilter),
        EnclaveKeyWhitelistedFilter(EnclaveKeyWhitelistedFilter),
        InitializedFilter(InitializedFilter),
        RemoveKeyFilter(RemoveKeyFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UpdateKeyFilter(UpdateKeyFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for EntityKeyRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageRevokedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveImageRevokedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveImageWhitelistedFilter::decode_log(log) {
                return Ok(
                    EntityKeyRegistryEvents::EnclaveImageWhitelistedFilter(decoded),
                );
            }
            if let Ok(decoded) = EnclaveKeyRevokedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveKeyRevokedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyVerifiedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveKeyVerifiedFilter(decoded));
            }
            if let Ok(decoded) = EnclaveKeyWhitelistedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::EnclaveKeyWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = RemoveKeyFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RemoveKeyFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UpdateKeyFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::UpdateKeyFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(EntityKeyRegistryEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EntityKeyRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::RemoveKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter> for EntityKeyRegistryEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for EntityKeyRegistryEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageRevokedFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveImageRevokedFilter) -> Self {
            Self::EnclaveImageRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveImageWhitelistedFilter>
    for EntityKeyRegistryEvents {
        fn from(value: EnclaveImageWhitelistedFilter) -> Self {
            Self::EnclaveImageWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyRevokedFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveKeyRevokedFilter) -> Self {
            Self::EnclaveKeyRevokedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyVerifiedFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveKeyVerifiedFilter) -> Self {
            Self::EnclaveKeyVerifiedFilter(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyWhitelistedFilter> for EntityKeyRegistryEvents {
        fn from(value: EnclaveKeyWhitelistedFilter) -> Self {
            Self::EnclaveKeyWhitelistedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for EntityKeyRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<RemoveKeyFilter> for EntityKeyRegistryEvents {
        fn from(value: RemoveKeyFilter) -> Self {
            Self::RemoveKeyFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for EntityKeyRegistryEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for EntityKeyRegistryEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for EntityKeyRegistryEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UpdateKeyFilter> for EntityKeyRegistryEvents {
        fn from(value: UpdateKeyFilter) -> Self {
            Self::UpdateKeyFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for EntityKeyRegistryEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
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
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `KEY_REGISTER_ROLE` function with signature `KEY_REGISTER_ROLE()` and selector `0xb80aaa89`
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
    #[ethcall(name = "KEY_REGISTER_ROLE", abi = "KEY_REGISTER_ROLE()")]
    pub struct KeyRegisterRoleCall;
    ///Container type for all input parameters for the `addGeneratorRegistry` function with signature `addGeneratorRegistry(address)` and selector `0xaa0ed09f`
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
    #[ethcall(name = "addGeneratorRegistry", abi = "addGeneratorRegistry(address)")]
    pub struct AddGeneratorRegistryCall {
        pub generator_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allowOnlyVerified` function with signature `allowOnlyVerified(address,bytes32)` and selector `0x7e201b65`
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
    #[ethcall(name = "allowOnlyVerified", abi = "allowOnlyVerified(address,bytes32)")]
    pub struct AllowOnlyVerifiedCall {
        pub key: ::ethers::core::types::Address,
        pub image_id: [u8; 32],
    }
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,(bytes,bytes,bytes)[])` and selector `0x2eb39ee9`
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
    #[ethcall(name = "initialize", abi = "initialize(address,(bytes,bytes,bytes)[])")]
    pub struct InitializeCall {
        pub admin: ::ethers::core::types::Address,
        pub init_whitelist_images: ::std::vec::Vec<EnclaveImage>,
    }
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `pub_key` function with signature `pub_key(address,uint256)` and selector `0xe1a63130`
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
    #[ethcall(name = "pub_key", abi = "pub_key(address,uint256)")]
    pub struct PubKeyCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `removePubkey` function with signature `removePubkey(address,uint256)` and selector `0xed380d03`
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
    #[ethcall(name = "removePubkey", abi = "removePubkey(address,uint256)")]
    pub struct RemovePubkeyCall {
        pub key_owner: ::ethers::core::types::Address,
        pub key_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `updatePubkey` function with signature `updatePubkey(address,uint256,bytes,bytes)` and selector `0x69fdbcca`
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
    #[ethcall(name = "updatePubkey", abi = "updatePubkey(address,uint256,bytes,bytes)")]
    pub struct UpdatePubkeyCall {
        pub key_owner: ::ethers::core::types::Address,
        pub key_index: ::ethers::core::types::U256,
        pub pubkey: ::ethers::core::types::Bytes,
        pub attestation_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `0x3659cfe6`
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
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
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
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verifyKey` function with signature `verifyKey(bytes)` and selector `0x0707591f`
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
    #[ethcall(name = "verifyKey", abi = "verifyKey(bytes)")]
    pub struct VerifyKeyCall {
        pub attestation_data: ::ethers::core::types::Bytes,
    }
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
    pub struct VerifyKeyWithSignatureCall {
        pub signature: ::ethers::core::types::Bytes,
        pub enclave_pub_key: ::ethers::core::types::Bytes,
        pub image_id: [u8; 32],
        pub enclave_cp_us: ::ethers::core::types::U256,
        pub enclave_memory: ::ethers::core::types::U256,
        pub timestamp_in_milliseconds: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `whitelistImageUsingPcrs` function with signature `whitelistImageUsingPcrs(bytes)` and selector `0xecea1d7d`
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
    #[ethcall(name = "whitelistImageUsingPcrs", abi = "whitelistImageUsingPcrs(bytes)")]
    pub struct WhitelistImageUsingPcrsCall {
        pub pcrs: ::ethers::core::types::Bytes,
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
    pub enum EntityKeyRegistryCalls {
        AttestationMaxAge(AttestationMaxAgeCall),
        AttestationVerifier(AttestationVerifierCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        KeyRegisterRole(KeyRegisterRoleCall),
        AddGeneratorRegistry(AddGeneratorRegistryCall),
        AllowOnlyVerified(AllowOnlyVerifiedCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Initialize(InitializeCall),
        ProxiableUUID(ProxiableUUIDCall),
        PubKey(PubKeyCall),
        RemovePubkey(RemovePubkeyCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SupportsInterface(SupportsInterfaceCall),
        UpdatePubkey(UpdatePubkeyCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        VerifyKey(VerifyKeyCall),
        VerifyKeyWithSignature(VerifyKeyWithSignatureCall),
        WhitelistImageUsingPcrs(WhitelistImageUsingPcrsCall),
    }
    impl ::ethers::core::abi::AbiDecode for EntityKeyRegistryCalls {
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
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <KeyRegisterRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KeyRegisterRole(decoded));
            }
            if let Ok(decoded) = <AddGeneratorRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddGeneratorRegistry(decoded));
            }
            if let Ok(decoded) = <AllowOnlyVerifiedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowOnlyVerified(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMember(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMemberCount(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <PubKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PubKey(decoded));
            }
            if let Ok(decoded) = <RemovePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemovePubkey(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UpdatePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdatePubkey(decoded));
            }
            if let Ok(decoded) = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeTo(decoded));
            }
            if let Ok(decoded) = <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <VerifyKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyKey(decoded));
            }
            if let Ok(decoded) = <VerifyKeyWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyKeyWithSignature(decoded));
            }
            if let Ok(decoded) = <WhitelistImageUsingPcrsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WhitelistImageUsingPcrs(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EntityKeyRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AttestationMaxAge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationVerifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KeyRegisterRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddGeneratorRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllowOnlyVerified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMember(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemovePubkey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePubkey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyKeyWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WhitelistImageUsingPcrs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EntityKeyRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttestationMaxAge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AttestationVerifier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::KeyRegisterRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddGeneratorRegistry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowOnlyVerified(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyKeyWithSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WhitelistImageUsingPcrs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AttestationMaxAgeCall> for EntityKeyRegistryCalls {
        fn from(value: AttestationMaxAgeCall) -> Self {
            Self::AttestationMaxAge(value)
        }
    }
    impl ::core::convert::From<AttestationVerifierCall> for EntityKeyRegistryCalls {
        fn from(value: AttestationVerifierCall) -> Self {
            Self::AttestationVerifier(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for EntityKeyRegistryCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<KeyRegisterRoleCall> for EntityKeyRegistryCalls {
        fn from(value: KeyRegisterRoleCall) -> Self {
            Self::KeyRegisterRole(value)
        }
    }
    impl ::core::convert::From<AddGeneratorRegistryCall> for EntityKeyRegistryCalls {
        fn from(value: AddGeneratorRegistryCall) -> Self {
            Self::AddGeneratorRegistry(value)
        }
    }
    impl ::core::convert::From<AllowOnlyVerifiedCall> for EntityKeyRegistryCalls {
        fn from(value: AllowOnlyVerifiedCall) -> Self {
            Self::AllowOnlyVerified(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for EntityKeyRegistryCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for EntityKeyRegistryCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for EntityKeyRegistryCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for EntityKeyRegistryCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for EntityKeyRegistryCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for EntityKeyRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for EntityKeyRegistryCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<PubKeyCall> for EntityKeyRegistryCalls {
        fn from(value: PubKeyCall) -> Self {
            Self::PubKey(value)
        }
    }
    impl ::core::convert::From<RemovePubkeyCall> for EntityKeyRegistryCalls {
        fn from(value: RemovePubkeyCall) -> Self {
            Self::RemovePubkey(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for EntityKeyRegistryCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for EntityKeyRegistryCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for EntityKeyRegistryCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpdatePubkeyCall> for EntityKeyRegistryCalls {
        fn from(value: UpdatePubkeyCall) -> Self {
            Self::UpdatePubkey(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for EntityKeyRegistryCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for EntityKeyRegistryCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VerifyKeyCall> for EntityKeyRegistryCalls {
        fn from(value: VerifyKeyCall) -> Self {
            Self::VerifyKey(value)
        }
    }
    impl ::core::convert::From<VerifyKeyWithSignatureCall> for EntityKeyRegistryCalls {
        fn from(value: VerifyKeyWithSignatureCall) -> Self {
            Self::VerifyKeyWithSignature(value)
        }
    }
    impl ::core::convert::From<WhitelistImageUsingPcrsCall> for EntityKeyRegistryCalls {
        fn from(value: WhitelistImageUsingPcrsCall) -> Self {
            Self::WhitelistImageUsingPcrs(value)
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
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `KEY_REGISTER_ROLE` function with signature `KEY_REGISTER_ROLE()` and selector `0xb80aaa89`
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
    pub struct KeyRegisterRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowOnlyVerified` function with signature `allowOnlyVerified(address,bytes32)` and selector `0x7e201b65`
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
    pub struct AllowOnlyVerifiedReturn(pub bool);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    pub struct GetRoleMemberReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
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
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `pub_key` function with signature `pub_key(address,uint256)` and selector `0xe1a63130`
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
    pub struct PubKeyReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///`EnclaveImage(bytes,bytes,bytes)`
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
    pub struct EnclaveImage {
        pub pcr0: ::ethers::core::types::Bytes,
        pub pcr1: ::ethers::core::types::Bytes,
        pub pcr2: ::ethers::core::types::Bytes,
    }
}
