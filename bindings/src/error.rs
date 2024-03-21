pub use error::*;
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
pub mod error {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ALREADY_JOINED_MARKET"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ALREADY_JOINED_MARKET",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ARITY_MISMATCH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ARITY_MISMATCH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ASSIGN_ONLY_TO_IDLE_GENERATORS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ASSIGN_ONLY_TO_IDLE_GENERATORS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ATTESTATION_TIMEOUT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ATTESTATION_TIMEOUT",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CANNOT_BE_ADMIN_LESS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CANNOT_BE_ADMIN_LESS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CANNOT_BE_ZERO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CANNOT_BE_ZERO"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CANNOT_USE_MATCHING_ENGINE_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CANNOT_USE_MATCHING_ENGINE_ROLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CAN_N0T_BE_SLASHED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CAN_N0T_BE_SLASHED"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CAN_NOT_ASSIGN_EXPIRED_TASKS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CAN_NOT_ASSIGN_EXPIRED_TASKS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CAN_NOT_BE_MORE_THAN_DECLARED_COMPUTE",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CAN_NOT_BE_MORE_THAN_DECLARED_COMPUTE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CAN_NOT_LEAVE_MARKET_WITH_ACTIVE_REQUEST",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CAN_NOT_LEAVE_MARKET_WITH_ACTIVE_REQUEST",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CAN_NOT_LEAVE_WITH_ACTIVE_MARKET"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CAN_NOT_LEAVE_WITH_ACTIVE_MARKET",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CAN_NOT_SLASH_USING_VALID_INPUTS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CAN_NOT_SLASH_USING_VALID_INPUTS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ENCLAVE_KEY_NOT_VERIFIED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ENCLAVE_KEY_NOT_VERIFIED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EXCEEDS_ACCEPTABLE_RANGE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EXCEEDS_ACCEPTABLE_RANGE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GENERATOR_ALREADY_EXISTS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "GENERATOR_ALREADY_EXISTS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INACTIVE_MARKET"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INACTIVE_MARKET"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INCORRECT_IMAGE_ID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INCORRECT_IMAGE_ID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "INSUFFICIENT_GENERATOR_COMPUTE_AVAILABLE",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INSUFFICIENT_GENERATOR_COMPUTE_AVAILABLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INSUFFICIENT_STAKE_TO_LOCK"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INSUFFICIENT_STAKE_TO_LOCK",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_CONTRACT_ADDRESS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_CONTRACT_ADDRESS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_ECIES_ACL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_ECIES_ACL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_ENCLAVE_KEY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_ENCLAVE_KEY",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_ENCLAVE_SIGNATURE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_ENCLAVE_SIGNATURE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_GENERATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_GENERATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "INVALID_GENERATOR_STATE_PER_MARKET",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "INVALID_GENERATOR_STATE_PER_MARKET",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_INPUTS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_INPUTS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_MARKET"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_MARKET"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("INVALID_PROOF"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("INVALID_PROOF"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("KEY_ALREADY_EXISTS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("KEY_ALREADY_EXISTS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MARKET_ALREADY_EXISTS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MARKET_ALREADY_EXISTS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "MAX_PARALLEL_REQUESTS_PER_MARKET_EXCEEDED",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_PARALLEL_REQUESTS_PER_MARKET_EXCEEDED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ONLY_ADMIN_CAN_CALL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ONLY_ADMIN_CAN_CALL",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ONLY_ASSIGNED_ASKS_CAN_BE_PROVED"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ONLY_ASSIGNED_ASKS_CAN_BE_PROVED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ONLY_EXPIRED_ASKS_CAN_BE_CANCELLED",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ONLY_EXPIRED_ASKS_CAN_BE_CANCELLED",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ONLY_GENERATOR_CAN_DISCARD_REQUEST",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ONLY_GENERATOR_CAN_DISCARD_REQUEST",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ONLY_MATCHING_ENGINE_CAN_ASSIGN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ONLY_MATCHING_ENGINE_CAN_ASSIGN",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ONLY_VALID_GENERATORS_CAN_REQUEST_EXIT",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ONLY_VALID_GENERATORS_CAN_REQUEST_EXIT",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ONLY_WORKING_GENERATORS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ONLY_WORKING_GENERATORS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PROOF_PRICE_MISMATCH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PROOF_PRICE_MISMATCH",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PROOF_TIME_MISMATCH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PROOF_TIME_MISMATCH",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PUBLIC_MARKETS_DONT_NEED_KEY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PUBLIC_MARKETS_DONT_NEED_KEY",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "REDUCE_COMPUTE_REQUEST_NOT_IN_PLACE",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "REDUCE_COMPUTE_REQUEST_NOT_IN_PLACE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("REDUCTION_REQUEST_NOT_VALID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "REDUCTION_REQUEST_NOT_VALID",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("REQUEST_ALREADY_IN_PLACE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "REQUEST_ALREADY_IN_PLACE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SHOULD_BE_IN_ASSIGNED_STATE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SHOULD_BE_IN_ASSIGNED_STATE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SHOULD_BE_IN_CREATE_STATE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SHOULD_BE_IN_CREATE_STATE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "SHOULD_BE_IN_CROSSED_DEADLINE_STATE",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SHOULD_BE_IN_CROSSED_DEADLINE_STATE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UNSTAKE_REQUEST_NOT_IN_PLACE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UNSTAKE_REQUEST_NOT_IN_PLACE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERROR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\t\xCEa\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x02\xC5W`\x005`\xE0\x1C\x80c\x95\xB7#\xC2\x11a\x01\x8CW\x80c\xBD0\x85\xB2\x11a\0\xF3W\x80c\xDA.I\xB3\x11a\0\xACW\x80c\xDE\x1FA\xA1\x11a\0\x86W\x80c\xDE\x1FA\xA1\x14a\x08\xC4W\x80c\xED\x1A\xEE\x10\x14a\x08\xE5W\x80c\xEET\x92\xB5\x14a\t\x07W\x80c\xFA\xC3\xC0\x99\x14a\t)W`\0\x80\xFD[\x80c\xDA.I\xB3\x14a\x08^W\x80c\xDC]\xA8$\x14a\x08\x80W\x80c\xDD\xAC\xD5S\x14a\x08\xA2W`\0\x80\xFD[\x80c\xBD0\x85\xB2\x14a\x07\x94W\x80c\xBD\x89\xB3\xBC\x14a\x07\xB6W\x80c\xBFF\xF4\xD0\x14a\x07\xD7W\x80c\xC2hC\xE6\x14a\x07\xF9W\x80c\xC5\xEC?\xB9\x14a\x08\x1AW\x80c\xD1,H\x88\x14a\x08<W`\0\x80\xFD[\x80c\xA3\x92\x9B\xB3\x11a\x01EW\x80c\xA3\x92\x9B\xB3\x14a\x06\xCBW\x80c\xA8y\x01\xF8\x14a\x06\xEDW\x80c\xAB\r\x9B\xB6\x14a\x07\x0FW\x80c\xABiV^\x14a\x070W\x80c\xB1\x1B\xAE\xAF\x14a\x07QW\x80c\xB1](\x11\x14a\x07sW`\0\x80\xFD[\x80c\x95\xB7#\xC2\x14a\x06\x01W\x80c\x9Bm\xED\x16\x14a\x06#W\x80c\x9F\xC7\xA1\x84\x14a\x06EW\x80c\xA0\x15\xA2$\x14a\x06gW\x80c\xA1\xBB\\b\x14a\x06\x88W\x80c\xA1\xF3\xE0R\x14a\x06\xA9W`\0\x80\xFD[\x80cO\xC8\xD1z\x11a\x020W\x80cm\xB0\xFF\x1D\x11a\x01\xE9W\x80cm\xB0\xFF\x1D\x14a\x05:W\x80cn\xFA\x9F\xCD\x14a\x05[W\x80cq.\xB0\x87\x14a\x05|W\x80ct\xB1gH\x14a\x05\x9EW\x80c\x92\x95\xC7[\x14a\x05\xBFW\x80c\x93x\x16,\x14a\x05\xE0W`\0\x80\xFD[\x80cO\xC8\xD1z\x14a\x04pW\x80c^|\xD3'\x14a\x04\x91W\x80c`\xF6\xE4@\x14a\x04\xB3W\x80ch[\xB0\xFF\x14a\x04\xD5W\x80ci!\x13\xEF\x14a\x04\xF7W\x80clY/%\x14a\x05\x19W`\0\x80\xFD[\x80c\x16\x8A\x1E\xCF\x11a\x02\x82W\x80c\x16\x8A\x1E\xCF\x14a\x03\xA9W\x80c\x19\x0C\xA5\x8B\x14a\x03\xCAW\x80c\x1F\x118\xE0\x14a\x03\xEBW\x80c-N\xF0M\x14a\x04\x0CW\x80c<weg\x14a\x04-W\x80c?\xC6\xB7~\x14a\x04OW`\0\x80\xFD[\x80c\x06\xA2\xC0,\x14a\x02\xCAW\x80c\n/\xD5\x07\x14a\x03\x01W\x80c\nj\xE5]\x14a\x03\"W\x80c\x0C\x8D\x1B\n\x14a\x03DW\x80c\r\xF3V\x0F\x14a\x03fW\x80c\x14\t\xC3\x97\x14a\x03\x87W[`\0\x80\xFD[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x08\xE7`\xF3\x1B\x81RP\x81V[`@Qa\x02\xF8\x91\x90a\tJV[`@Q\x80\x91\x03\x90\xF3[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG9`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b*)\x19`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a#\x99`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR3`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x08'`\xF3\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA3`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x10M`\xF2\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a&\x99`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS7`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM3`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b*)\x1B`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b \x98\x99`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b#\x98\x99`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA9`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x11\xCD`\xF2\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a \x9B`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b()\x9B`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA1`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a#\x9B`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b()\x99`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR1`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA7`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM1`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG15`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS9`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG11`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG3`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a&\x9B`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA13`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x13M`\xF2\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\n\ng`\xEB\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG1`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x13\x13`\xEC\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM5`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS5`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS3`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04s\x13`\xEC\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG13`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS1`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a \x99`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x14\x14\xCD`\xEA\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x15\x14\x8D`\xEA\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x81V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\twW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\t[V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFE\xA2dipfsX\"\x12 {\xEFZC\xBF$:\xA3\x8B\x19Q]\xC6c\xA7w6\xC2\x93\xF2\x1C-\x0F+\x0Ez\xAC$V\x99\xDE\xA2dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static ERROR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x02\xC5W`\x005`\xE0\x1C\x80c\x95\xB7#\xC2\x11a\x01\x8CW\x80c\xBD0\x85\xB2\x11a\0\xF3W\x80c\xDA.I\xB3\x11a\0\xACW\x80c\xDE\x1FA\xA1\x11a\0\x86W\x80c\xDE\x1FA\xA1\x14a\x08\xC4W\x80c\xED\x1A\xEE\x10\x14a\x08\xE5W\x80c\xEET\x92\xB5\x14a\t\x07W\x80c\xFA\xC3\xC0\x99\x14a\t)W`\0\x80\xFD[\x80c\xDA.I\xB3\x14a\x08^W\x80c\xDC]\xA8$\x14a\x08\x80W\x80c\xDD\xAC\xD5S\x14a\x08\xA2W`\0\x80\xFD[\x80c\xBD0\x85\xB2\x14a\x07\x94W\x80c\xBD\x89\xB3\xBC\x14a\x07\xB6W\x80c\xBFF\xF4\xD0\x14a\x07\xD7W\x80c\xC2hC\xE6\x14a\x07\xF9W\x80c\xC5\xEC?\xB9\x14a\x08\x1AW\x80c\xD1,H\x88\x14a\x08<W`\0\x80\xFD[\x80c\xA3\x92\x9B\xB3\x11a\x01EW\x80c\xA3\x92\x9B\xB3\x14a\x06\xCBW\x80c\xA8y\x01\xF8\x14a\x06\xEDW\x80c\xAB\r\x9B\xB6\x14a\x07\x0FW\x80c\xABiV^\x14a\x070W\x80c\xB1\x1B\xAE\xAF\x14a\x07QW\x80c\xB1](\x11\x14a\x07sW`\0\x80\xFD[\x80c\x95\xB7#\xC2\x14a\x06\x01W\x80c\x9Bm\xED\x16\x14a\x06#W\x80c\x9F\xC7\xA1\x84\x14a\x06EW\x80c\xA0\x15\xA2$\x14a\x06gW\x80c\xA1\xBB\\b\x14a\x06\x88W\x80c\xA1\xF3\xE0R\x14a\x06\xA9W`\0\x80\xFD[\x80cO\xC8\xD1z\x11a\x020W\x80cm\xB0\xFF\x1D\x11a\x01\xE9W\x80cm\xB0\xFF\x1D\x14a\x05:W\x80cn\xFA\x9F\xCD\x14a\x05[W\x80cq.\xB0\x87\x14a\x05|W\x80ct\xB1gH\x14a\x05\x9EW\x80c\x92\x95\xC7[\x14a\x05\xBFW\x80c\x93x\x16,\x14a\x05\xE0W`\0\x80\xFD[\x80cO\xC8\xD1z\x14a\x04pW\x80c^|\xD3'\x14a\x04\x91W\x80c`\xF6\xE4@\x14a\x04\xB3W\x80ch[\xB0\xFF\x14a\x04\xD5W\x80ci!\x13\xEF\x14a\x04\xF7W\x80clY/%\x14a\x05\x19W`\0\x80\xFD[\x80c\x16\x8A\x1E\xCF\x11a\x02\x82W\x80c\x16\x8A\x1E\xCF\x14a\x03\xA9W\x80c\x19\x0C\xA5\x8B\x14a\x03\xCAW\x80c\x1F\x118\xE0\x14a\x03\xEBW\x80c-N\xF0M\x14a\x04\x0CW\x80c<weg\x14a\x04-W\x80c?\xC6\xB7~\x14a\x04OW`\0\x80\xFD[\x80c\x06\xA2\xC0,\x14a\x02\xCAW\x80c\n/\xD5\x07\x14a\x03\x01W\x80c\nj\xE5]\x14a\x03\"W\x80c\x0C\x8D\x1B\n\x14a\x03DW\x80c\r\xF3V\x0F\x14a\x03fW\x80c\x14\t\xC3\x97\x14a\x03\x87W[`\0\x80\xFD[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x08\xE7`\xF3\x1B\x81RP\x81V[`@Qa\x02\xF8\x91\x90a\tJV[`@Q\x80\x91\x03\x90\xF3[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG9`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b*)\x19`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a#\x99`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR3`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x08'`\xF3\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA3`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x10M`\xF2\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a&\x99`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS7`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM3`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b*)\x1B`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b \x98\x99`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b#\x98\x99`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA9`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x11\xCD`\xF2\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a \x9B`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b()\x9B`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA1`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a#\x9B`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b()\x99`\xE9\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR1`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA7`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM1`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG15`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS9`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG11`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG3`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a&\x9B`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA13`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x13M`\xF2\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\n\ng`\xEB\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG1`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x13\x13`\xEC\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM5`\xF0\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS5`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS3`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04s\x13`\xEC\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG13`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bPS1`\xE8\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a \x99`\xF1\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x14\x14\xCD`\xEA\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x15\x14\x8D`\xEA\x1B\x81RP\x81V[a\x02\xEB`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x81V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\twW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\t[V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFE\xA2dipfsX\"\x12 {\xEFZC\xBF$:\xA3\x8B\x19Q]\xC6c\xA7w6\xC2\x93\xF2\x1C-\x0F+\x0Ez\xAC$V\x99\xDE\xA2dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static ERROR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Error<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Error<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Error<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Error<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Error<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Error)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Error<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERROR_ABI.clone(),
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
                ERROR_ABI.clone(),
                ERROR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ALREADY_JOINED_MARKET` (0x2d4ef04d) function
        pub fn already_joined_market(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([45, 78, 240, 77], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ARITY_MISMATCH` (0x1409c397) function
        pub fn arity_mismatch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([20, 9, 195, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ASSIGN_ONLY_TO_IDLE_GENERATORS` (0x6db0ff1d) function
        pub fn assign_only_to_idle_generators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([109, 176, 255, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ATTESTATION_TIMEOUT` (0x60f6e440) function
        pub fn attestation_timeout(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([96, 246, 228, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CANNOT_BE_ADMIN_LESS` (0xde1f41a1) function
        pub fn cannot_be_admin_less(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([222, 31, 65, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CANNOT_BE_ZERO` (0x190ca58b) function
        pub fn cannot_be_zero(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([25, 12, 165, 139], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CANNOT_USE_MATCHING_ENGINE_ROLE` (0x6c592f25) function
        pub fn cannot_use_matching_engine_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([108, 89, 47, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CAN_N0T_BE_SLASHED` (0x1f1138e0) function
        pub fn can_n0t_be_slashed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([31, 17, 56, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CAN_NOT_ASSIGN_EXPIRED_TASKS` (0x9fc7a184) function
        pub fn can_not_assign_expired_tasks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([159, 199, 161, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CAN_NOT_BE_MORE_THAN_DECLARED_COMPUTE` (0x4fc8d17a) function
        pub fn can_not_be_more_than_declared_compute(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([79, 200, 209, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CAN_NOT_LEAVE_MARKET_WITH_ACTIVE_REQUEST` (0xb15d2811) function
        pub fn can_not_leave_market_with_active_request(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([177, 93, 40, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CAN_NOT_LEAVE_WITH_ACTIVE_MARKET` (0xab0d9bb6) function
        pub fn can_not_leave_with_active_market(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([171, 13, 155, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CAN_NOT_SLASH_USING_VALID_INPUTS` (0x5e7cd327) function
        pub fn can_not_slash_using_valid_inputs(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([94, 124, 211, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ENCLAVE_KEY_NOT_VERIFIED` (0x6efa9fcd) function
        pub fn enclave_key_not_verified(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([110, 250, 159, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EXCEEDS_ACCEPTABLE_RANGE` (0xa015a224) function
        pub fn exceeds_acceptable_range(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([160, 21, 162, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `GENERATOR_ALREADY_EXISTS` (0xbd89b3bc) function
        pub fn generator_already_exists(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([189, 137, 179, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INACTIVE_MARKET` (0xab69565e) function
        pub fn inactive_market(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([171, 105, 86, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INCORRECT_IMAGE_ID` (0x95b723c2) function
        pub fn incorrect_image_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 183, 35, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INSUFFICIENT_GENERATOR_COMPUTE_AVAILABLE` (0xfac3c099) function
        pub fn insufficient_generator_compute_available(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([250, 195, 192, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INSUFFICIENT_STAKE_TO_LOCK` (0x9378162c) function
        pub fn insufficient_stake_to_lock(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([147, 120, 22, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_CONTRACT_ADDRESS` (0x168a1ecf) function
        pub fn invalid_contract_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([22, 138, 30, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_ECIES_ACL` (0xb11baeaf) function
        pub fn invalid_ecies_acl(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([177, 27, 174, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_ENCLAVE_KEY` (0x3fc6b77e) function
        pub fn invalid_enclave_key(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([63, 198, 183, 126], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_ENCLAVE_SIGNATURE` (0xbf46f4d0) function
        pub fn invalid_enclave_signature(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([191, 70, 244, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_GENERATOR` (0x0df3560f) function
        pub fn invalid_generator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([13, 243, 86, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_GENERATOR_STATE_PER_MARKET` (0x0a2fd507) function
        pub fn invalid_generator_state_per_market(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([10, 47, 213, 7], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_INPUTS` (0x0c8d1b0a) function
        pub fn invalid_inputs(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([12, 141, 27, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_MARKET` (0xa1bb5c62) function
        pub fn invalid_market(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([161, 187, 92, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `INVALID_PROOF` (0x712eb087) function
        pub fn invalid_proof(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([113, 46, 176, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `KEY_ALREADY_EXISTS` (0xdc5da824) function
        pub fn key_already_exists(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([220, 93, 168, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MARKET_ALREADY_EXISTS` (0xc26843e6) function
        pub fn market_already_exists(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([194, 104, 67, 230], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_PARALLEL_REQUESTS_PER_MARKET_EXCEEDED` (0x692113ef) function
        pub fn max_parallel_requests_per_market_exceeded(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([105, 33, 19, 239], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ONLY_ADMIN_CAN_CALL` (0x74b16748) function
        pub fn only_admin_can_call(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([116, 177, 103, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ONLY_ASSIGNED_ASKS_CAN_BE_PROVED` (0xc5ec3fb9) function
        pub fn only_assigned_asks_can_be_proved(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([197, 236, 63, 185], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ONLY_EXPIRED_ASKS_CAN_BE_CANCELLED` (0xed1aee10) function
        pub fn only_expired_asks_can_be_cancelled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([237, 26, 238, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ONLY_GENERATOR_CAN_DISCARD_REQUEST` (0xa3929bb3) function
        pub fn only_generator_can_discard_request(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([163, 146, 155, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ONLY_MATCHING_ENGINE_CAN_ASSIGN` (0xee5492b5) function
        pub fn only_matching_engine_can_assign(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([238, 84, 146, 181], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ONLY_VALID_GENERATORS_CAN_REQUEST_EXIT` (0x06a2c02c) function
        pub fn only_valid_generators_can_request_exit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 162, 192, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ONLY_WORKING_GENERATORS` (0x9295c75b) function
        pub fn only_working_generators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([146, 149, 199, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROOF_PRICE_MISMATCH` (0x9b6ded16) function
        pub fn proof_price_mismatch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([155, 109, 237, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROOF_TIME_MISMATCH` (0xd12c4888) function
        pub fn proof_time_mismatch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([209, 44, 72, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PUBLIC_MARKETS_DONT_NEED_KEY` (0xa1f3e052) function
        pub fn public_markets_dont_need_key(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([161, 243, 224, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `REDUCE_COMPUTE_REQUEST_NOT_IN_PLACE` (0xa87901f8) function
        pub fn reduce_compute_request_not_in_place(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([168, 121, 1, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `REDUCTION_REQUEST_NOT_VALID` (0x685bb0ff) function
        pub fn reduction_request_not_valid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([104, 91, 176, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `REQUEST_ALREADY_IN_PLACE` (0x0a6ae55d) function
        pub fn request_already_in_place(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([10, 106, 229, 93], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SHOULD_BE_IN_ASSIGNED_STATE` (0xbd3085b2) function
        pub fn should_be_in_assigned_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([189, 48, 133, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SHOULD_BE_IN_CREATE_STATE` (0xddacd553) function
        pub fn should_be_in_create_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([221, 172, 213, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SHOULD_BE_IN_CROSSED_DEADLINE_STATE` (0x3c776567) function
        pub fn should_be_in_crossed_deadline_state(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([60, 119, 101, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNSTAKE_REQUEST_NOT_IN_PLACE` (0xda2e49b3) function
        pub fn unstake_request_not_in_place(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([218, 46, 73, 179], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Error<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `ALREADY_JOINED_MARKET` function with signature `ALREADY_JOINED_MARKET()` and selector `0x2d4ef04d`
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
    #[ethcall(name = "ALREADY_JOINED_MARKET", abi = "ALREADY_JOINED_MARKET()")]
    pub struct AlreadyJoinedMarketCall;
    ///Container type for all input parameters for the `ARITY_MISMATCH` function with signature `ARITY_MISMATCH()` and selector `0x1409c397`
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
    #[ethcall(name = "ARITY_MISMATCH", abi = "ARITY_MISMATCH()")]
    pub struct ArityMismatchCall;
    ///Container type for all input parameters for the `ASSIGN_ONLY_TO_IDLE_GENERATORS` function with signature `ASSIGN_ONLY_TO_IDLE_GENERATORS()` and selector `0x6db0ff1d`
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
        name = "ASSIGN_ONLY_TO_IDLE_GENERATORS",
        abi = "ASSIGN_ONLY_TO_IDLE_GENERATORS()"
    )]
    pub struct AssignOnlyToIdleGeneratorsCall;
    ///Container type for all input parameters for the `ATTESTATION_TIMEOUT` function with signature `ATTESTATION_TIMEOUT()` and selector `0x60f6e440`
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
    #[ethcall(name = "ATTESTATION_TIMEOUT", abi = "ATTESTATION_TIMEOUT()")]
    pub struct AttestationTimeoutCall;
    ///Container type for all input parameters for the `CANNOT_BE_ADMIN_LESS` function with signature `CANNOT_BE_ADMIN_LESS()` and selector `0xde1f41a1`
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
    #[ethcall(name = "CANNOT_BE_ADMIN_LESS", abi = "CANNOT_BE_ADMIN_LESS()")]
    pub struct CannotBeAdminLessCall;
    ///Container type for all input parameters for the `CANNOT_BE_ZERO` function with signature `CANNOT_BE_ZERO()` and selector `0x190ca58b`
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
    #[ethcall(name = "CANNOT_BE_ZERO", abi = "CANNOT_BE_ZERO()")]
    pub struct CannotBeZeroCall;
    ///Container type for all input parameters for the `CANNOT_USE_MATCHING_ENGINE_ROLE` function with signature `CANNOT_USE_MATCHING_ENGINE_ROLE()` and selector `0x6c592f25`
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
        name = "CANNOT_USE_MATCHING_ENGINE_ROLE",
        abi = "CANNOT_USE_MATCHING_ENGINE_ROLE()"
    )]
    pub struct CannotUseMatchingEngineRoleCall;
    ///Container type for all input parameters for the `CAN_N0T_BE_SLASHED` function with signature `CAN_N0T_BE_SLASHED()` and selector `0x1f1138e0`
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
    #[ethcall(name = "CAN_N0T_BE_SLASHED", abi = "CAN_N0T_BE_SLASHED()")]
    pub struct CanN0TBeSlashedCall;
    ///Container type for all input parameters for the `CAN_NOT_ASSIGN_EXPIRED_TASKS` function with signature `CAN_NOT_ASSIGN_EXPIRED_TASKS()` and selector `0x9fc7a184`
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
        name = "CAN_NOT_ASSIGN_EXPIRED_TASKS",
        abi = "CAN_NOT_ASSIGN_EXPIRED_TASKS()"
    )]
    pub struct CanNotAssignExpiredTasksCall;
    ///Container type for all input parameters for the `CAN_NOT_BE_MORE_THAN_DECLARED_COMPUTE` function with signature `CAN_NOT_BE_MORE_THAN_DECLARED_COMPUTE()` and selector `0x4fc8d17a`
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
        name = "CAN_NOT_BE_MORE_THAN_DECLARED_COMPUTE",
        abi = "CAN_NOT_BE_MORE_THAN_DECLARED_COMPUTE()"
    )]
    pub struct CanNotBeMoreThanDeclaredComputeCall;
    ///Container type for all input parameters for the `CAN_NOT_LEAVE_MARKET_WITH_ACTIVE_REQUEST` function with signature `CAN_NOT_LEAVE_MARKET_WITH_ACTIVE_REQUEST()` and selector `0xb15d2811`
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
        name = "CAN_NOT_LEAVE_MARKET_WITH_ACTIVE_REQUEST",
        abi = "CAN_NOT_LEAVE_MARKET_WITH_ACTIVE_REQUEST()"
    )]
    pub struct CanNotLeaveMarketWithActiveRequestCall;
    ///Container type for all input parameters for the `CAN_NOT_LEAVE_WITH_ACTIVE_MARKET` function with signature `CAN_NOT_LEAVE_WITH_ACTIVE_MARKET()` and selector `0xab0d9bb6`
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
        name = "CAN_NOT_LEAVE_WITH_ACTIVE_MARKET",
        abi = "CAN_NOT_LEAVE_WITH_ACTIVE_MARKET()"
    )]
    pub struct CanNotLeaveWithActiveMarketCall;
    ///Container type for all input parameters for the `CAN_NOT_SLASH_USING_VALID_INPUTS` function with signature `CAN_NOT_SLASH_USING_VALID_INPUTS()` and selector `0x5e7cd327`
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
        name = "CAN_NOT_SLASH_USING_VALID_INPUTS",
        abi = "CAN_NOT_SLASH_USING_VALID_INPUTS()"
    )]
    pub struct CanNotSlashUsingValidInputsCall;
    ///Container type for all input parameters for the `ENCLAVE_KEY_NOT_VERIFIED` function with signature `ENCLAVE_KEY_NOT_VERIFIED()` and selector `0x6efa9fcd`
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
    #[ethcall(name = "ENCLAVE_KEY_NOT_VERIFIED", abi = "ENCLAVE_KEY_NOT_VERIFIED()")]
    pub struct EnclaveKeyNotVerifiedCall;
    ///Container type for all input parameters for the `EXCEEDS_ACCEPTABLE_RANGE` function with signature `EXCEEDS_ACCEPTABLE_RANGE()` and selector `0xa015a224`
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
    #[ethcall(name = "EXCEEDS_ACCEPTABLE_RANGE", abi = "EXCEEDS_ACCEPTABLE_RANGE()")]
    pub struct ExceedsAcceptableRangeCall;
    ///Container type for all input parameters for the `GENERATOR_ALREADY_EXISTS` function with signature `GENERATOR_ALREADY_EXISTS()` and selector `0xbd89b3bc`
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
    #[ethcall(name = "GENERATOR_ALREADY_EXISTS", abi = "GENERATOR_ALREADY_EXISTS()")]
    pub struct GeneratorAlreadyExistsCall;
    ///Container type for all input parameters for the `INACTIVE_MARKET` function with signature `INACTIVE_MARKET()` and selector `0xab69565e`
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
    #[ethcall(name = "INACTIVE_MARKET", abi = "INACTIVE_MARKET()")]
    pub struct InactiveMarketCall;
    ///Container type for all input parameters for the `INCORRECT_IMAGE_ID` function with signature `INCORRECT_IMAGE_ID()` and selector `0x95b723c2`
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
    #[ethcall(name = "INCORRECT_IMAGE_ID", abi = "INCORRECT_IMAGE_ID()")]
    pub struct IncorrectImageIdCall;
    ///Container type for all input parameters for the `INSUFFICIENT_GENERATOR_COMPUTE_AVAILABLE` function with signature `INSUFFICIENT_GENERATOR_COMPUTE_AVAILABLE()` and selector `0xfac3c099`
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
        name = "INSUFFICIENT_GENERATOR_COMPUTE_AVAILABLE",
        abi = "INSUFFICIENT_GENERATOR_COMPUTE_AVAILABLE()"
    )]
    pub struct InsufficientGeneratorComputeAvailableCall;
    ///Container type for all input parameters for the `INSUFFICIENT_STAKE_TO_LOCK` function with signature `INSUFFICIENT_STAKE_TO_LOCK()` and selector `0x9378162c`
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
    #[ethcall(name = "INSUFFICIENT_STAKE_TO_LOCK", abi = "INSUFFICIENT_STAKE_TO_LOCK()")]
    pub struct InsufficientStakeToLockCall;
    ///Container type for all input parameters for the `INVALID_CONTRACT_ADDRESS` function with signature `INVALID_CONTRACT_ADDRESS()` and selector `0x168a1ecf`
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
    #[ethcall(name = "INVALID_CONTRACT_ADDRESS", abi = "INVALID_CONTRACT_ADDRESS()")]
    pub struct InvalidContractAddressCall;
    ///Container type for all input parameters for the `INVALID_ECIES_ACL` function with signature `INVALID_ECIES_ACL()` and selector `0xb11baeaf`
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
    #[ethcall(name = "INVALID_ECIES_ACL", abi = "INVALID_ECIES_ACL()")]
    pub struct InvalidEciesAclCall;
    ///Container type for all input parameters for the `INVALID_ENCLAVE_KEY` function with signature `INVALID_ENCLAVE_KEY()` and selector `0x3fc6b77e`
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
    #[ethcall(name = "INVALID_ENCLAVE_KEY", abi = "INVALID_ENCLAVE_KEY()")]
    pub struct InvalidEnclaveKeyCall;
    ///Container type for all input parameters for the `INVALID_ENCLAVE_SIGNATURE` function with signature `INVALID_ENCLAVE_SIGNATURE()` and selector `0xbf46f4d0`
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
    #[ethcall(name = "INVALID_ENCLAVE_SIGNATURE", abi = "INVALID_ENCLAVE_SIGNATURE()")]
    pub struct InvalidEnclaveSignatureCall;
    ///Container type for all input parameters for the `INVALID_GENERATOR` function with signature `INVALID_GENERATOR()` and selector `0x0df3560f`
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
    #[ethcall(name = "INVALID_GENERATOR", abi = "INVALID_GENERATOR()")]
    pub struct InvalidGeneratorCall;
    ///Container type for all input parameters for the `INVALID_GENERATOR_STATE_PER_MARKET` function with signature `INVALID_GENERATOR_STATE_PER_MARKET()` and selector `0x0a2fd507`
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
        name = "INVALID_GENERATOR_STATE_PER_MARKET",
        abi = "INVALID_GENERATOR_STATE_PER_MARKET()"
    )]
    pub struct InvalidGeneratorStatePerMarketCall;
    ///Container type for all input parameters for the `INVALID_INPUTS` function with signature `INVALID_INPUTS()` and selector `0x0c8d1b0a`
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
    #[ethcall(name = "INVALID_INPUTS", abi = "INVALID_INPUTS()")]
    pub struct InvalidInputsCall;
    ///Container type for all input parameters for the `INVALID_MARKET` function with signature `INVALID_MARKET()` and selector `0xa1bb5c62`
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
    #[ethcall(name = "INVALID_MARKET", abi = "INVALID_MARKET()")]
    pub struct InvalidMarketCall;
    ///Container type for all input parameters for the `INVALID_PROOF` function with signature `INVALID_PROOF()` and selector `0x712eb087`
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
    #[ethcall(name = "INVALID_PROOF", abi = "INVALID_PROOF()")]
    pub struct InvalidProofCall;
    ///Container type for all input parameters for the `KEY_ALREADY_EXISTS` function with signature `KEY_ALREADY_EXISTS()` and selector `0xdc5da824`
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
    #[ethcall(name = "KEY_ALREADY_EXISTS", abi = "KEY_ALREADY_EXISTS()")]
    pub struct KeyAlreadyExistsCall;
    ///Container type for all input parameters for the `MARKET_ALREADY_EXISTS` function with signature `MARKET_ALREADY_EXISTS()` and selector `0xc26843e6`
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
    #[ethcall(name = "MARKET_ALREADY_EXISTS", abi = "MARKET_ALREADY_EXISTS()")]
    pub struct MarketAlreadyExistsCall;
    ///Container type for all input parameters for the `MAX_PARALLEL_REQUESTS_PER_MARKET_EXCEEDED` function with signature `MAX_PARALLEL_REQUESTS_PER_MARKET_EXCEEDED()` and selector `0x692113ef`
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
        name = "MAX_PARALLEL_REQUESTS_PER_MARKET_EXCEEDED",
        abi = "MAX_PARALLEL_REQUESTS_PER_MARKET_EXCEEDED()"
    )]
    pub struct MaxParallelRequestsPerMarketExceededCall;
    ///Container type for all input parameters for the `ONLY_ADMIN_CAN_CALL` function with signature `ONLY_ADMIN_CAN_CALL()` and selector `0x74b16748`
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
    #[ethcall(name = "ONLY_ADMIN_CAN_CALL", abi = "ONLY_ADMIN_CAN_CALL()")]
    pub struct OnlyAdminCanCallCall;
    ///Container type for all input parameters for the `ONLY_ASSIGNED_ASKS_CAN_BE_PROVED` function with signature `ONLY_ASSIGNED_ASKS_CAN_BE_PROVED()` and selector `0xc5ec3fb9`
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
        name = "ONLY_ASSIGNED_ASKS_CAN_BE_PROVED",
        abi = "ONLY_ASSIGNED_ASKS_CAN_BE_PROVED()"
    )]
    pub struct OnlyAssignedAsksCanBeProvedCall;
    ///Container type for all input parameters for the `ONLY_EXPIRED_ASKS_CAN_BE_CANCELLED` function with signature `ONLY_EXPIRED_ASKS_CAN_BE_CANCELLED()` and selector `0xed1aee10`
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
        name = "ONLY_EXPIRED_ASKS_CAN_BE_CANCELLED",
        abi = "ONLY_EXPIRED_ASKS_CAN_BE_CANCELLED()"
    )]
    pub struct OnlyExpiredAsksCanBeCancelledCall;
    ///Container type for all input parameters for the `ONLY_GENERATOR_CAN_DISCARD_REQUEST` function with signature `ONLY_GENERATOR_CAN_DISCARD_REQUEST()` and selector `0xa3929bb3`
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
        name = "ONLY_GENERATOR_CAN_DISCARD_REQUEST",
        abi = "ONLY_GENERATOR_CAN_DISCARD_REQUEST()"
    )]
    pub struct OnlyGeneratorCanDiscardRequestCall;
    ///Container type for all input parameters for the `ONLY_MATCHING_ENGINE_CAN_ASSIGN` function with signature `ONLY_MATCHING_ENGINE_CAN_ASSIGN()` and selector `0xee5492b5`
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
        name = "ONLY_MATCHING_ENGINE_CAN_ASSIGN",
        abi = "ONLY_MATCHING_ENGINE_CAN_ASSIGN()"
    )]
    pub struct OnlyMatchingEngineCanAssignCall;
    ///Container type for all input parameters for the `ONLY_VALID_GENERATORS_CAN_REQUEST_EXIT` function with signature `ONLY_VALID_GENERATORS_CAN_REQUEST_EXIT()` and selector `0x06a2c02c`
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
        name = "ONLY_VALID_GENERATORS_CAN_REQUEST_EXIT",
        abi = "ONLY_VALID_GENERATORS_CAN_REQUEST_EXIT()"
    )]
    pub struct OnlyValidGeneratorsCanRequestExitCall;
    ///Container type for all input parameters for the `ONLY_WORKING_GENERATORS` function with signature `ONLY_WORKING_GENERATORS()` and selector `0x9295c75b`
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
    #[ethcall(name = "ONLY_WORKING_GENERATORS", abi = "ONLY_WORKING_GENERATORS()")]
    pub struct OnlyWorkingGeneratorsCall;
    ///Container type for all input parameters for the `PROOF_PRICE_MISMATCH` function with signature `PROOF_PRICE_MISMATCH()` and selector `0x9b6ded16`
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
    #[ethcall(name = "PROOF_PRICE_MISMATCH", abi = "PROOF_PRICE_MISMATCH()")]
    pub struct ProofPriceMismatchCall;
    ///Container type for all input parameters for the `PROOF_TIME_MISMATCH` function with signature `PROOF_TIME_MISMATCH()` and selector `0xd12c4888`
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
    #[ethcall(name = "PROOF_TIME_MISMATCH", abi = "PROOF_TIME_MISMATCH()")]
    pub struct ProofTimeMismatchCall;
    ///Container type for all input parameters for the `PUBLIC_MARKETS_DONT_NEED_KEY` function with signature `PUBLIC_MARKETS_DONT_NEED_KEY()` and selector `0xa1f3e052`
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
        name = "PUBLIC_MARKETS_DONT_NEED_KEY",
        abi = "PUBLIC_MARKETS_DONT_NEED_KEY()"
    )]
    pub struct PublicMarketsDontNeedKeyCall;
    ///Container type for all input parameters for the `REDUCE_COMPUTE_REQUEST_NOT_IN_PLACE` function with signature `REDUCE_COMPUTE_REQUEST_NOT_IN_PLACE()` and selector `0xa87901f8`
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
        name = "REDUCE_COMPUTE_REQUEST_NOT_IN_PLACE",
        abi = "REDUCE_COMPUTE_REQUEST_NOT_IN_PLACE()"
    )]
    pub struct ReduceComputeRequestNotInPlaceCall;
    ///Container type for all input parameters for the `REDUCTION_REQUEST_NOT_VALID` function with signature `REDUCTION_REQUEST_NOT_VALID()` and selector `0x685bb0ff`
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
        name = "REDUCTION_REQUEST_NOT_VALID",
        abi = "REDUCTION_REQUEST_NOT_VALID()"
    )]
    pub struct ReductionRequestNotValidCall;
    ///Container type for all input parameters for the `REQUEST_ALREADY_IN_PLACE` function with signature `REQUEST_ALREADY_IN_PLACE()` and selector `0x0a6ae55d`
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
    #[ethcall(name = "REQUEST_ALREADY_IN_PLACE", abi = "REQUEST_ALREADY_IN_PLACE()")]
    pub struct RequestAlreadyInPlaceCall;
    ///Container type for all input parameters for the `SHOULD_BE_IN_ASSIGNED_STATE` function with signature `SHOULD_BE_IN_ASSIGNED_STATE()` and selector `0xbd3085b2`
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
        name = "SHOULD_BE_IN_ASSIGNED_STATE",
        abi = "SHOULD_BE_IN_ASSIGNED_STATE()"
    )]
    pub struct ShouldBeInAssignedStateCall;
    ///Container type for all input parameters for the `SHOULD_BE_IN_CREATE_STATE` function with signature `SHOULD_BE_IN_CREATE_STATE()` and selector `0xddacd553`
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
    #[ethcall(name = "SHOULD_BE_IN_CREATE_STATE", abi = "SHOULD_BE_IN_CREATE_STATE()")]
    pub struct ShouldBeInCreateStateCall;
    ///Container type for all input parameters for the `SHOULD_BE_IN_CROSSED_DEADLINE_STATE` function with signature `SHOULD_BE_IN_CROSSED_DEADLINE_STATE()` and selector `0x3c776567`
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
        name = "SHOULD_BE_IN_CROSSED_DEADLINE_STATE",
        abi = "SHOULD_BE_IN_CROSSED_DEADLINE_STATE()"
    )]
    pub struct ShouldBeInCrossedDeadlineStateCall;
    ///Container type for all input parameters for the `UNSTAKE_REQUEST_NOT_IN_PLACE` function with signature `UNSTAKE_REQUEST_NOT_IN_PLACE()` and selector `0xda2e49b3`
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
        name = "UNSTAKE_REQUEST_NOT_IN_PLACE",
        abi = "UNSTAKE_REQUEST_NOT_IN_PLACE()"
    )]
    pub struct UnstakeRequestNotInPlaceCall;
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
    pub enum ErrorCalls {
        AlreadyJoinedMarket(AlreadyJoinedMarketCall),
        ArityMismatch(ArityMismatchCall),
        AssignOnlyToIdleGenerators(AssignOnlyToIdleGeneratorsCall),
        AttestationTimeout(AttestationTimeoutCall),
        CannotBeAdminLess(CannotBeAdminLessCall),
        CannotBeZero(CannotBeZeroCall),
        CannotUseMatchingEngineRole(CannotUseMatchingEngineRoleCall),
        CanN0TBeSlashed(CanN0TBeSlashedCall),
        CanNotAssignExpiredTasks(CanNotAssignExpiredTasksCall),
        CanNotBeMoreThanDeclaredCompute(CanNotBeMoreThanDeclaredComputeCall),
        CanNotLeaveMarketWithActiveRequest(CanNotLeaveMarketWithActiveRequestCall),
        CanNotLeaveWithActiveMarket(CanNotLeaveWithActiveMarketCall),
        CanNotSlashUsingValidInputs(CanNotSlashUsingValidInputsCall),
        EnclaveKeyNotVerified(EnclaveKeyNotVerifiedCall),
        ExceedsAcceptableRange(ExceedsAcceptableRangeCall),
        GeneratorAlreadyExists(GeneratorAlreadyExistsCall),
        InactiveMarket(InactiveMarketCall),
        IncorrectImageId(IncorrectImageIdCall),
        InsufficientGeneratorComputeAvailable(InsufficientGeneratorComputeAvailableCall),
        InsufficientStakeToLock(InsufficientStakeToLockCall),
        InvalidContractAddress(InvalidContractAddressCall),
        InvalidEciesAcl(InvalidEciesAclCall),
        InvalidEnclaveKey(InvalidEnclaveKeyCall),
        InvalidEnclaveSignature(InvalidEnclaveSignatureCall),
        InvalidGenerator(InvalidGeneratorCall),
        InvalidGeneratorStatePerMarket(InvalidGeneratorStatePerMarketCall),
        InvalidInputs(InvalidInputsCall),
        InvalidMarket(InvalidMarketCall),
        InvalidProof(InvalidProofCall),
        KeyAlreadyExists(KeyAlreadyExistsCall),
        MarketAlreadyExists(MarketAlreadyExistsCall),
        MaxParallelRequestsPerMarketExceeded(MaxParallelRequestsPerMarketExceededCall),
        OnlyAdminCanCall(OnlyAdminCanCallCall),
        OnlyAssignedAsksCanBeProved(OnlyAssignedAsksCanBeProvedCall),
        OnlyExpiredAsksCanBeCancelled(OnlyExpiredAsksCanBeCancelledCall),
        OnlyGeneratorCanDiscardRequest(OnlyGeneratorCanDiscardRequestCall),
        OnlyMatchingEngineCanAssign(OnlyMatchingEngineCanAssignCall),
        OnlyValidGeneratorsCanRequestExit(OnlyValidGeneratorsCanRequestExitCall),
        OnlyWorkingGenerators(OnlyWorkingGeneratorsCall),
        ProofPriceMismatch(ProofPriceMismatchCall),
        ProofTimeMismatch(ProofTimeMismatchCall),
        PublicMarketsDontNeedKey(PublicMarketsDontNeedKeyCall),
        ReduceComputeRequestNotInPlace(ReduceComputeRequestNotInPlaceCall),
        ReductionRequestNotValid(ReductionRequestNotValidCall),
        RequestAlreadyInPlace(RequestAlreadyInPlaceCall),
        ShouldBeInAssignedState(ShouldBeInAssignedStateCall),
        ShouldBeInCreateState(ShouldBeInCreateStateCall),
        ShouldBeInCrossedDeadlineState(ShouldBeInCrossedDeadlineStateCall),
        UnstakeRequestNotInPlace(UnstakeRequestNotInPlaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ErrorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AlreadyJoinedMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AlreadyJoinedMarket(decoded));
            }
            if let Ok(decoded) = <ArityMismatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ArityMismatch(decoded));
            }
            if let Ok(decoded) = <AssignOnlyToIdleGeneratorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssignOnlyToIdleGenerators(decoded));
            }
            if let Ok(decoded) = <AttestationTimeoutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AttestationTimeout(decoded));
            }
            if let Ok(decoded) = <CannotBeAdminLessCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotBeAdminLess(decoded));
            }
            if let Ok(decoded) = <CannotBeZeroCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotBeZero(decoded));
            }
            if let Ok(decoded) = <CannotUseMatchingEngineRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotUseMatchingEngineRole(decoded));
            }
            if let Ok(decoded) = <CanN0TBeSlashedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanN0TBeSlashed(decoded));
            }
            if let Ok(decoded) = <CanNotAssignExpiredTasksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanNotAssignExpiredTasks(decoded));
            }
            if let Ok(decoded) = <CanNotBeMoreThanDeclaredComputeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanNotBeMoreThanDeclaredCompute(decoded));
            }
            if let Ok(decoded) = <CanNotLeaveMarketWithActiveRequestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanNotLeaveMarketWithActiveRequest(decoded));
            }
            if let Ok(decoded) = <CanNotLeaveWithActiveMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanNotLeaveWithActiveMarket(decoded));
            }
            if let Ok(decoded) = <CanNotSlashUsingValidInputsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CanNotSlashUsingValidInputs(decoded));
            }
            if let Ok(decoded) = <EnclaveKeyNotVerifiedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnclaveKeyNotVerified(decoded));
            }
            if let Ok(decoded) = <ExceedsAcceptableRangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExceedsAcceptableRange(decoded));
            }
            if let Ok(decoded) = <GeneratorAlreadyExistsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GeneratorAlreadyExists(decoded));
            }
            if let Ok(decoded) = <InactiveMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InactiveMarket(decoded));
            }
            if let Ok(decoded) = <IncorrectImageIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncorrectImageId(decoded));
            }
            if let Ok(decoded) = <InsufficientGeneratorComputeAvailableCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientGeneratorComputeAvailable(decoded));
            }
            if let Ok(decoded) = <InsufficientStakeToLockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientStakeToLock(decoded));
            }
            if let Ok(decoded) = <InvalidContractAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidContractAddress(decoded));
            }
            if let Ok(decoded) = <InvalidEciesAclCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidEciesAcl(decoded));
            }
            if let Ok(decoded) = <InvalidEnclaveKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidEnclaveKey(decoded));
            }
            if let Ok(decoded) = <InvalidEnclaveSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidEnclaveSignature(decoded));
            }
            if let Ok(decoded) = <InvalidGeneratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidGenerator(decoded));
            }
            if let Ok(decoded) = <InvalidGeneratorStatePerMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidGeneratorStatePerMarket(decoded));
            }
            if let Ok(decoded) = <InvalidInputsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInputs(decoded));
            }
            if let Ok(decoded) = <InvalidMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMarket(decoded));
            }
            if let Ok(decoded) = <InvalidProofCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidProof(decoded));
            }
            if let Ok(decoded) = <KeyAlreadyExistsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KeyAlreadyExists(decoded));
            }
            if let Ok(decoded) = <MarketAlreadyExistsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MarketAlreadyExists(decoded));
            }
            if let Ok(decoded) = <MaxParallelRequestsPerMarketExceededCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxParallelRequestsPerMarketExceeded(decoded));
            }
            if let Ok(decoded) = <OnlyAdminCanCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyAdminCanCall(decoded));
            }
            if let Ok(decoded) = <OnlyAssignedAsksCanBeProvedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyAssignedAsksCanBeProved(decoded));
            }
            if let Ok(decoded) = <OnlyExpiredAsksCanBeCancelledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyExpiredAsksCanBeCancelled(decoded));
            }
            if let Ok(decoded) = <OnlyGeneratorCanDiscardRequestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyGeneratorCanDiscardRequest(decoded));
            }
            if let Ok(decoded) = <OnlyMatchingEngineCanAssignCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyMatchingEngineCanAssign(decoded));
            }
            if let Ok(decoded) = <OnlyValidGeneratorsCanRequestExitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyValidGeneratorsCanRequestExit(decoded));
            }
            if let Ok(decoded) = <OnlyWorkingGeneratorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyWorkingGenerators(decoded));
            }
            if let Ok(decoded) = <ProofPriceMismatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProofPriceMismatch(decoded));
            }
            if let Ok(decoded) = <ProofTimeMismatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProofTimeMismatch(decoded));
            }
            if let Ok(decoded) = <PublicMarketsDontNeedKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PublicMarketsDontNeedKey(decoded));
            }
            if let Ok(decoded) = <ReduceComputeRequestNotInPlaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReduceComputeRequestNotInPlace(decoded));
            }
            if let Ok(decoded) = <ReductionRequestNotValidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReductionRequestNotValid(decoded));
            }
            if let Ok(decoded) = <RequestAlreadyInPlaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestAlreadyInPlace(decoded));
            }
            if let Ok(decoded) = <ShouldBeInAssignedStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ShouldBeInAssignedState(decoded));
            }
            if let Ok(decoded) = <ShouldBeInCreateStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ShouldBeInCreateState(decoded));
            }
            if let Ok(decoded) = <ShouldBeInCrossedDeadlineStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ShouldBeInCrossedDeadlineState(decoded));
            }
            if let Ok(decoded) = <UnstakeRequestNotInPlaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnstakeRequestNotInPlace(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ErrorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AlreadyJoinedMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ArityMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssignOnlyToIdleGenerators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AttestationTimeout(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotBeAdminLess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotUseMatchingEngineRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanN0TBeSlashed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanNotAssignExpiredTasks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanNotBeMoreThanDeclaredCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanNotLeaveMarketWithActiveRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanNotLeaveWithActiveMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CanNotSlashUsingValidInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnclaveKeyNotVerified(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExceedsAcceptableRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GeneratorAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InactiveMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectImageId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientGeneratorComputeAvailable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientStakeToLock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidContractAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEciesAcl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEnclaveKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEnclaveSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidGenerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidGeneratorStatePerMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInputs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KeyAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MarketAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxParallelRequestsPerMarketExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyAdminCanCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyAssignedAsksCanBeProved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyExpiredAsksCanBeCancelled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyGeneratorCanDiscardRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyMatchingEngineCanAssign(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyValidGeneratorsCanRequestExit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyWorkingGenerators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofPriceMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofTimeMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PublicMarketsDontNeedKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReduceComputeRequestNotInPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReductionRequestNotValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestAlreadyInPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShouldBeInAssignedState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShouldBeInCreateState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShouldBeInCrossedDeadlineState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnstakeRequestNotInPlace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ErrorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadyJoinedMarket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ArityMismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignOnlyToIdleGenerators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AttestationTimeout(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotBeAdminLess(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotBeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::CannotUseMatchingEngineRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CanN0TBeSlashed(element) => ::core::fmt::Display::fmt(element, f),
                Self::CanNotAssignExpiredTasks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CanNotBeMoreThanDeclaredCompute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CanNotLeaveMarketWithActiveRequest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CanNotLeaveWithActiveMarket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CanNotSlashUsingValidInputs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveKeyNotVerified(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExceedsAcceptableRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GeneratorAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InactiveMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectImageId(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientGeneratorComputeAvailable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientStakeToLock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidContractAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidEciesAcl(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEnclaveKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidEnclaveSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidGenerator(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidGeneratorStatePerMarket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidInputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::KeyAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxParallelRequestsPerMarketExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyAdminCanCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyAssignedAsksCanBeProved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyExpiredAsksCanBeCancelled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyGeneratorCanDiscardRequest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyMatchingEngineCanAssign(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyValidGeneratorsCanRequestExit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnlyWorkingGenerators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProofPriceMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProofTimeMismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::PublicMarketsDontNeedKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReduceComputeRequestNotInPlace(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReductionRequestNotValid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestAlreadyInPlace(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ShouldBeInAssignedState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ShouldBeInCreateState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ShouldBeInCrossedDeadlineState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnstakeRequestNotInPlace(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AlreadyJoinedMarketCall> for ErrorCalls {
        fn from(value: AlreadyJoinedMarketCall) -> Self {
            Self::AlreadyJoinedMarket(value)
        }
    }
    impl ::core::convert::From<ArityMismatchCall> for ErrorCalls {
        fn from(value: ArityMismatchCall) -> Self {
            Self::ArityMismatch(value)
        }
    }
    impl ::core::convert::From<AssignOnlyToIdleGeneratorsCall> for ErrorCalls {
        fn from(value: AssignOnlyToIdleGeneratorsCall) -> Self {
            Self::AssignOnlyToIdleGenerators(value)
        }
    }
    impl ::core::convert::From<AttestationTimeoutCall> for ErrorCalls {
        fn from(value: AttestationTimeoutCall) -> Self {
            Self::AttestationTimeout(value)
        }
    }
    impl ::core::convert::From<CannotBeAdminLessCall> for ErrorCalls {
        fn from(value: CannotBeAdminLessCall) -> Self {
            Self::CannotBeAdminLess(value)
        }
    }
    impl ::core::convert::From<CannotBeZeroCall> for ErrorCalls {
        fn from(value: CannotBeZeroCall) -> Self {
            Self::CannotBeZero(value)
        }
    }
    impl ::core::convert::From<CannotUseMatchingEngineRoleCall> for ErrorCalls {
        fn from(value: CannotUseMatchingEngineRoleCall) -> Self {
            Self::CannotUseMatchingEngineRole(value)
        }
    }
    impl ::core::convert::From<CanN0TBeSlashedCall> for ErrorCalls {
        fn from(value: CanN0TBeSlashedCall) -> Self {
            Self::CanN0TBeSlashed(value)
        }
    }
    impl ::core::convert::From<CanNotAssignExpiredTasksCall> for ErrorCalls {
        fn from(value: CanNotAssignExpiredTasksCall) -> Self {
            Self::CanNotAssignExpiredTasks(value)
        }
    }
    impl ::core::convert::From<CanNotBeMoreThanDeclaredComputeCall> for ErrorCalls {
        fn from(value: CanNotBeMoreThanDeclaredComputeCall) -> Self {
            Self::CanNotBeMoreThanDeclaredCompute(value)
        }
    }
    impl ::core::convert::From<CanNotLeaveMarketWithActiveRequestCall> for ErrorCalls {
        fn from(value: CanNotLeaveMarketWithActiveRequestCall) -> Self {
            Self::CanNotLeaveMarketWithActiveRequest(value)
        }
    }
    impl ::core::convert::From<CanNotLeaveWithActiveMarketCall> for ErrorCalls {
        fn from(value: CanNotLeaveWithActiveMarketCall) -> Self {
            Self::CanNotLeaveWithActiveMarket(value)
        }
    }
    impl ::core::convert::From<CanNotSlashUsingValidInputsCall> for ErrorCalls {
        fn from(value: CanNotSlashUsingValidInputsCall) -> Self {
            Self::CanNotSlashUsingValidInputs(value)
        }
    }
    impl ::core::convert::From<EnclaveKeyNotVerifiedCall> for ErrorCalls {
        fn from(value: EnclaveKeyNotVerifiedCall) -> Self {
            Self::EnclaveKeyNotVerified(value)
        }
    }
    impl ::core::convert::From<ExceedsAcceptableRangeCall> for ErrorCalls {
        fn from(value: ExceedsAcceptableRangeCall) -> Self {
            Self::ExceedsAcceptableRange(value)
        }
    }
    impl ::core::convert::From<GeneratorAlreadyExistsCall> for ErrorCalls {
        fn from(value: GeneratorAlreadyExistsCall) -> Self {
            Self::GeneratorAlreadyExists(value)
        }
    }
    impl ::core::convert::From<InactiveMarketCall> for ErrorCalls {
        fn from(value: InactiveMarketCall) -> Self {
            Self::InactiveMarket(value)
        }
    }
    impl ::core::convert::From<IncorrectImageIdCall> for ErrorCalls {
        fn from(value: IncorrectImageIdCall) -> Self {
            Self::IncorrectImageId(value)
        }
    }
    impl ::core::convert::From<InsufficientGeneratorComputeAvailableCall>
    for ErrorCalls {
        fn from(value: InsufficientGeneratorComputeAvailableCall) -> Self {
            Self::InsufficientGeneratorComputeAvailable(value)
        }
    }
    impl ::core::convert::From<InsufficientStakeToLockCall> for ErrorCalls {
        fn from(value: InsufficientStakeToLockCall) -> Self {
            Self::InsufficientStakeToLock(value)
        }
    }
    impl ::core::convert::From<InvalidContractAddressCall> for ErrorCalls {
        fn from(value: InvalidContractAddressCall) -> Self {
            Self::InvalidContractAddress(value)
        }
    }
    impl ::core::convert::From<InvalidEciesAclCall> for ErrorCalls {
        fn from(value: InvalidEciesAclCall) -> Self {
            Self::InvalidEciesAcl(value)
        }
    }
    impl ::core::convert::From<InvalidEnclaveKeyCall> for ErrorCalls {
        fn from(value: InvalidEnclaveKeyCall) -> Self {
            Self::InvalidEnclaveKey(value)
        }
    }
    impl ::core::convert::From<InvalidEnclaveSignatureCall> for ErrorCalls {
        fn from(value: InvalidEnclaveSignatureCall) -> Self {
            Self::InvalidEnclaveSignature(value)
        }
    }
    impl ::core::convert::From<InvalidGeneratorCall> for ErrorCalls {
        fn from(value: InvalidGeneratorCall) -> Self {
            Self::InvalidGenerator(value)
        }
    }
    impl ::core::convert::From<InvalidGeneratorStatePerMarketCall> for ErrorCalls {
        fn from(value: InvalidGeneratorStatePerMarketCall) -> Self {
            Self::InvalidGeneratorStatePerMarket(value)
        }
    }
    impl ::core::convert::From<InvalidInputsCall> for ErrorCalls {
        fn from(value: InvalidInputsCall) -> Self {
            Self::InvalidInputs(value)
        }
    }
    impl ::core::convert::From<InvalidMarketCall> for ErrorCalls {
        fn from(value: InvalidMarketCall) -> Self {
            Self::InvalidMarket(value)
        }
    }
    impl ::core::convert::From<InvalidProofCall> for ErrorCalls {
        fn from(value: InvalidProofCall) -> Self {
            Self::InvalidProof(value)
        }
    }
    impl ::core::convert::From<KeyAlreadyExistsCall> for ErrorCalls {
        fn from(value: KeyAlreadyExistsCall) -> Self {
            Self::KeyAlreadyExists(value)
        }
    }
    impl ::core::convert::From<MarketAlreadyExistsCall> for ErrorCalls {
        fn from(value: MarketAlreadyExistsCall) -> Self {
            Self::MarketAlreadyExists(value)
        }
    }
    impl ::core::convert::From<MaxParallelRequestsPerMarketExceededCall> for ErrorCalls {
        fn from(value: MaxParallelRequestsPerMarketExceededCall) -> Self {
            Self::MaxParallelRequestsPerMarketExceeded(value)
        }
    }
    impl ::core::convert::From<OnlyAdminCanCallCall> for ErrorCalls {
        fn from(value: OnlyAdminCanCallCall) -> Self {
            Self::OnlyAdminCanCall(value)
        }
    }
    impl ::core::convert::From<OnlyAssignedAsksCanBeProvedCall> for ErrorCalls {
        fn from(value: OnlyAssignedAsksCanBeProvedCall) -> Self {
            Self::OnlyAssignedAsksCanBeProved(value)
        }
    }
    impl ::core::convert::From<OnlyExpiredAsksCanBeCancelledCall> for ErrorCalls {
        fn from(value: OnlyExpiredAsksCanBeCancelledCall) -> Self {
            Self::OnlyExpiredAsksCanBeCancelled(value)
        }
    }
    impl ::core::convert::From<OnlyGeneratorCanDiscardRequestCall> for ErrorCalls {
        fn from(value: OnlyGeneratorCanDiscardRequestCall) -> Self {
            Self::OnlyGeneratorCanDiscardRequest(value)
        }
    }
    impl ::core::convert::From<OnlyMatchingEngineCanAssignCall> for ErrorCalls {
        fn from(value: OnlyMatchingEngineCanAssignCall) -> Self {
            Self::OnlyMatchingEngineCanAssign(value)
        }
    }
    impl ::core::convert::From<OnlyValidGeneratorsCanRequestExitCall> for ErrorCalls {
        fn from(value: OnlyValidGeneratorsCanRequestExitCall) -> Self {
            Self::OnlyValidGeneratorsCanRequestExit(value)
        }
    }
    impl ::core::convert::From<OnlyWorkingGeneratorsCall> for ErrorCalls {
        fn from(value: OnlyWorkingGeneratorsCall) -> Self {
            Self::OnlyWorkingGenerators(value)
        }
    }
    impl ::core::convert::From<ProofPriceMismatchCall> for ErrorCalls {
        fn from(value: ProofPriceMismatchCall) -> Self {
            Self::ProofPriceMismatch(value)
        }
    }
    impl ::core::convert::From<ProofTimeMismatchCall> for ErrorCalls {
        fn from(value: ProofTimeMismatchCall) -> Self {
            Self::ProofTimeMismatch(value)
        }
    }
    impl ::core::convert::From<PublicMarketsDontNeedKeyCall> for ErrorCalls {
        fn from(value: PublicMarketsDontNeedKeyCall) -> Self {
            Self::PublicMarketsDontNeedKey(value)
        }
    }
    impl ::core::convert::From<ReduceComputeRequestNotInPlaceCall> for ErrorCalls {
        fn from(value: ReduceComputeRequestNotInPlaceCall) -> Self {
            Self::ReduceComputeRequestNotInPlace(value)
        }
    }
    impl ::core::convert::From<ReductionRequestNotValidCall> for ErrorCalls {
        fn from(value: ReductionRequestNotValidCall) -> Self {
            Self::ReductionRequestNotValid(value)
        }
    }
    impl ::core::convert::From<RequestAlreadyInPlaceCall> for ErrorCalls {
        fn from(value: RequestAlreadyInPlaceCall) -> Self {
            Self::RequestAlreadyInPlace(value)
        }
    }
    impl ::core::convert::From<ShouldBeInAssignedStateCall> for ErrorCalls {
        fn from(value: ShouldBeInAssignedStateCall) -> Self {
            Self::ShouldBeInAssignedState(value)
        }
    }
    impl ::core::convert::From<ShouldBeInCreateStateCall> for ErrorCalls {
        fn from(value: ShouldBeInCreateStateCall) -> Self {
            Self::ShouldBeInCreateState(value)
        }
    }
    impl ::core::convert::From<ShouldBeInCrossedDeadlineStateCall> for ErrorCalls {
        fn from(value: ShouldBeInCrossedDeadlineStateCall) -> Self {
            Self::ShouldBeInCrossedDeadlineState(value)
        }
    }
    impl ::core::convert::From<UnstakeRequestNotInPlaceCall> for ErrorCalls {
        fn from(value: UnstakeRequestNotInPlaceCall) -> Self {
            Self::UnstakeRequestNotInPlace(value)
        }
    }
    ///Container type for all return fields from the `ALREADY_JOINED_MARKET` function with signature `ALREADY_JOINED_MARKET()` and selector `0x2d4ef04d`
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
    pub struct AlreadyJoinedMarketReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ARITY_MISMATCH` function with signature `ARITY_MISMATCH()` and selector `0x1409c397`
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
    pub struct ArityMismatchReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ASSIGN_ONLY_TO_IDLE_GENERATORS` function with signature `ASSIGN_ONLY_TO_IDLE_GENERATORS()` and selector `0x6db0ff1d`
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
    pub struct AssignOnlyToIdleGeneratorsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ATTESTATION_TIMEOUT` function with signature `ATTESTATION_TIMEOUT()` and selector `0x60f6e440`
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
    pub struct AttestationTimeoutReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CANNOT_BE_ADMIN_LESS` function with signature `CANNOT_BE_ADMIN_LESS()` and selector `0xde1f41a1`
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
    pub struct CannotBeAdminLessReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CANNOT_BE_ZERO` function with signature `CANNOT_BE_ZERO()` and selector `0x190ca58b`
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
    pub struct CannotBeZeroReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CANNOT_USE_MATCHING_ENGINE_ROLE` function with signature `CANNOT_USE_MATCHING_ENGINE_ROLE()` and selector `0x6c592f25`
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
    pub struct CannotUseMatchingEngineRoleReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CAN_N0T_BE_SLASHED` function with signature `CAN_N0T_BE_SLASHED()` and selector `0x1f1138e0`
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
    pub struct CanN0TBeSlashedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CAN_NOT_ASSIGN_EXPIRED_TASKS` function with signature `CAN_NOT_ASSIGN_EXPIRED_TASKS()` and selector `0x9fc7a184`
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
    pub struct CanNotAssignExpiredTasksReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CAN_NOT_BE_MORE_THAN_DECLARED_COMPUTE` function with signature `CAN_NOT_BE_MORE_THAN_DECLARED_COMPUTE()` and selector `0x4fc8d17a`
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
    pub struct CanNotBeMoreThanDeclaredComputeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CAN_NOT_LEAVE_MARKET_WITH_ACTIVE_REQUEST` function with signature `CAN_NOT_LEAVE_MARKET_WITH_ACTIVE_REQUEST()` and selector `0xb15d2811`
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
    pub struct CanNotLeaveMarketWithActiveRequestReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CAN_NOT_LEAVE_WITH_ACTIVE_MARKET` function with signature `CAN_NOT_LEAVE_WITH_ACTIVE_MARKET()` and selector `0xab0d9bb6`
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
    pub struct CanNotLeaveWithActiveMarketReturn(pub ::std::string::String);
    ///Container type for all return fields from the `CAN_NOT_SLASH_USING_VALID_INPUTS` function with signature `CAN_NOT_SLASH_USING_VALID_INPUTS()` and selector `0x5e7cd327`
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
    pub struct CanNotSlashUsingValidInputsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ENCLAVE_KEY_NOT_VERIFIED` function with signature `ENCLAVE_KEY_NOT_VERIFIED()` and selector `0x6efa9fcd`
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
    pub struct EnclaveKeyNotVerifiedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `EXCEEDS_ACCEPTABLE_RANGE` function with signature `EXCEEDS_ACCEPTABLE_RANGE()` and selector `0xa015a224`
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
    pub struct ExceedsAcceptableRangeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `GENERATOR_ALREADY_EXISTS` function with signature `GENERATOR_ALREADY_EXISTS()` and selector `0xbd89b3bc`
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
    pub struct GeneratorAlreadyExistsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INACTIVE_MARKET` function with signature `INACTIVE_MARKET()` and selector `0xab69565e`
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
    pub struct InactiveMarketReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INCORRECT_IMAGE_ID` function with signature `INCORRECT_IMAGE_ID()` and selector `0x95b723c2`
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
    pub struct IncorrectImageIdReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INSUFFICIENT_GENERATOR_COMPUTE_AVAILABLE` function with signature `INSUFFICIENT_GENERATOR_COMPUTE_AVAILABLE()` and selector `0xfac3c099`
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
    pub struct InsufficientGeneratorComputeAvailableReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INSUFFICIENT_STAKE_TO_LOCK` function with signature `INSUFFICIENT_STAKE_TO_LOCK()` and selector `0x9378162c`
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
    pub struct InsufficientStakeToLockReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_CONTRACT_ADDRESS` function with signature `INVALID_CONTRACT_ADDRESS()` and selector `0x168a1ecf`
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
    pub struct InvalidContractAddressReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_ECIES_ACL` function with signature `INVALID_ECIES_ACL()` and selector `0xb11baeaf`
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
    pub struct InvalidEciesAclReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_ENCLAVE_KEY` function with signature `INVALID_ENCLAVE_KEY()` and selector `0x3fc6b77e`
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
    pub struct InvalidEnclaveKeyReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_ENCLAVE_SIGNATURE` function with signature `INVALID_ENCLAVE_SIGNATURE()` and selector `0xbf46f4d0`
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
    pub struct InvalidEnclaveSignatureReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_GENERATOR` function with signature `INVALID_GENERATOR()` and selector `0x0df3560f`
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
    pub struct InvalidGeneratorReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_GENERATOR_STATE_PER_MARKET` function with signature `INVALID_GENERATOR_STATE_PER_MARKET()` and selector `0x0a2fd507`
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
    pub struct InvalidGeneratorStatePerMarketReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_INPUTS` function with signature `INVALID_INPUTS()` and selector `0x0c8d1b0a`
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
    pub struct InvalidInputsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_MARKET` function with signature `INVALID_MARKET()` and selector `0xa1bb5c62`
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
    pub struct InvalidMarketReturn(pub ::std::string::String);
    ///Container type for all return fields from the `INVALID_PROOF` function with signature `INVALID_PROOF()` and selector `0x712eb087`
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
    pub struct InvalidProofReturn(pub ::std::string::String);
    ///Container type for all return fields from the `KEY_ALREADY_EXISTS` function with signature `KEY_ALREADY_EXISTS()` and selector `0xdc5da824`
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
    pub struct KeyAlreadyExistsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `MARKET_ALREADY_EXISTS` function with signature `MARKET_ALREADY_EXISTS()` and selector `0xc26843e6`
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
    pub struct MarketAlreadyExistsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `MAX_PARALLEL_REQUESTS_PER_MARKET_EXCEEDED` function with signature `MAX_PARALLEL_REQUESTS_PER_MARKET_EXCEEDED()` and selector `0x692113ef`
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
    pub struct MaxParallelRequestsPerMarketExceededReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ONLY_ADMIN_CAN_CALL` function with signature `ONLY_ADMIN_CAN_CALL()` and selector `0x74b16748`
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
    pub struct OnlyAdminCanCallReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ONLY_ASSIGNED_ASKS_CAN_BE_PROVED` function with signature `ONLY_ASSIGNED_ASKS_CAN_BE_PROVED()` and selector `0xc5ec3fb9`
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
    pub struct OnlyAssignedAsksCanBeProvedReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ONLY_EXPIRED_ASKS_CAN_BE_CANCELLED` function with signature `ONLY_EXPIRED_ASKS_CAN_BE_CANCELLED()` and selector `0xed1aee10`
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
    pub struct OnlyExpiredAsksCanBeCancelledReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ONLY_GENERATOR_CAN_DISCARD_REQUEST` function with signature `ONLY_GENERATOR_CAN_DISCARD_REQUEST()` and selector `0xa3929bb3`
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
    pub struct OnlyGeneratorCanDiscardRequestReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ONLY_MATCHING_ENGINE_CAN_ASSIGN` function with signature `ONLY_MATCHING_ENGINE_CAN_ASSIGN()` and selector `0xee5492b5`
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
    pub struct OnlyMatchingEngineCanAssignReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ONLY_VALID_GENERATORS_CAN_REQUEST_EXIT` function with signature `ONLY_VALID_GENERATORS_CAN_REQUEST_EXIT()` and selector `0x06a2c02c`
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
    pub struct OnlyValidGeneratorsCanRequestExitReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ONLY_WORKING_GENERATORS` function with signature `ONLY_WORKING_GENERATORS()` and selector `0x9295c75b`
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
    pub struct OnlyWorkingGeneratorsReturn(pub ::std::string::String);
    ///Container type for all return fields from the `PROOF_PRICE_MISMATCH` function with signature `PROOF_PRICE_MISMATCH()` and selector `0x9b6ded16`
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
    pub struct ProofPriceMismatchReturn(pub ::std::string::String);
    ///Container type for all return fields from the `PROOF_TIME_MISMATCH` function with signature `PROOF_TIME_MISMATCH()` and selector `0xd12c4888`
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
    pub struct ProofTimeMismatchReturn(pub ::std::string::String);
    ///Container type for all return fields from the `PUBLIC_MARKETS_DONT_NEED_KEY` function with signature `PUBLIC_MARKETS_DONT_NEED_KEY()` and selector `0xa1f3e052`
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
    pub struct PublicMarketsDontNeedKeyReturn(pub ::std::string::String);
    ///Container type for all return fields from the `REDUCE_COMPUTE_REQUEST_NOT_IN_PLACE` function with signature `REDUCE_COMPUTE_REQUEST_NOT_IN_PLACE()` and selector `0xa87901f8`
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
    pub struct ReduceComputeRequestNotInPlaceReturn(pub ::std::string::String);
    ///Container type for all return fields from the `REDUCTION_REQUEST_NOT_VALID` function with signature `REDUCTION_REQUEST_NOT_VALID()` and selector `0x685bb0ff`
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
    pub struct ReductionRequestNotValidReturn(pub ::std::string::String);
    ///Container type for all return fields from the `REQUEST_ALREADY_IN_PLACE` function with signature `REQUEST_ALREADY_IN_PLACE()` and selector `0x0a6ae55d`
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
    pub struct RequestAlreadyInPlaceReturn(pub ::std::string::String);
    ///Container type for all return fields from the `SHOULD_BE_IN_ASSIGNED_STATE` function with signature `SHOULD_BE_IN_ASSIGNED_STATE()` and selector `0xbd3085b2`
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
    pub struct ShouldBeInAssignedStateReturn(pub ::std::string::String);
    ///Container type for all return fields from the `SHOULD_BE_IN_CREATE_STATE` function with signature `SHOULD_BE_IN_CREATE_STATE()` and selector `0xddacd553`
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
    pub struct ShouldBeInCreateStateReturn(pub ::std::string::String);
    ///Container type for all return fields from the `SHOULD_BE_IN_CROSSED_DEADLINE_STATE` function with signature `SHOULD_BE_IN_CROSSED_DEADLINE_STATE()` and selector `0x3c776567`
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
    pub struct ShouldBeInCrossedDeadlineStateReturn(pub ::std::string::String);
    ///Container type for all return fields from the `UNSTAKE_REQUEST_NOT_IN_PLACE` function with signature `UNSTAKE_REQUEST_NOT_IN_PLACE()` and selector `0xda2e49b3`
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
    pub struct UnstakeRequestNotInPlaceReturn(pub ::std::string::String);
}
