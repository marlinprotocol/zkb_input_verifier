pub use generator_registry::*;
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
pub mod generator_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stakingToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IERC20Upgradeable",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_entityRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract EntityKeyRegistry",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("ENTITY_KEY_REGISTRY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ENTITY_KEY_REGISTRY",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract EntityKeyRegistry",
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
                    ::std::borrow::ToOwned::to_owned("PARALLEL_REQUESTS_UPPER_LIMIT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PARALLEL_REQUESTS_UPPER_LIMIT",
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
                    ::std::borrow::ToOwned::to_owned("PROOF_MARKET_PLACE_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PROOF_MARKET_PLACE_ROLE",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("STAKING_TOKEN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("STAKING_TOKEN"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IERC20Upgradeable",
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
                    ::std::borrow::ToOwned::to_owned("UNLOCK_WAIT_BLOCKS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("UNLOCK_WAIT_BLOCKS"),
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
                    ::std::borrow::ToOwned::to_owned("addIvsKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addIvsKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestationData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveSignature"),
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
                    ::std::borrow::ToOwned::to_owned("assignGeneratorTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "assignGeneratorTask",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakeToLock"),
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
                    ::std::borrow::ToOwned::to_owned("changeRewardAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeRewardAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newRewardAddress"),
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
                    ::std::borrow::ToOwned::to_owned("completeGeneratorTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "completeGeneratorTask",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakeToRelease"),
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
                    ::std::borrow::ToOwned::to_owned("decreaseDeclaredCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "decreaseDeclaredCompute",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregister"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deregister"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundAddress"),
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
                    ::std::borrow::ToOwned::to_owned("generatorInfoPerMarket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "generatorInfoPerMarket",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum GeneratorRegistry.GeneratorState",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "computePerRequestRequired",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "proofGenerationCost",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposedTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("activeRequests"),
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
                    ::std::borrow::ToOwned::to_owned("generatorRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("generatorRegistry"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewardAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sumOfComputeAllocations",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("computeConsumed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stakeLocked"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "activeMarketplaces",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("declaredCompute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "intendedStakeUtilization",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "intendedComputeUtilization",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorData"),
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
                    ::std::borrow::ToOwned::to_owned("getGeneratorAssignmentDetails"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getGeneratorAssignmentDetails",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGeneratorRewardDetails"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getGeneratorRewardDetails",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("getGeneratorState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGeneratorState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum GeneratorRegistry.GeneratorState",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("increaseDeclaredCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "increaseDeclaredCompute",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("computeToIncrease"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_proofMarketplace"),
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
                    ::std::borrow::ToOwned::to_owned("intendToReduceCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "intendToReduceCompute",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newUtilization"),
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
                    ::std::borrow::ToOwned::to_owned("intendToReduceStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "intendToReduceStake",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newUtilization"),
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
                    ::std::borrow::ToOwned::to_owned("joinMarketplace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("joinMarketplace"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "computePerRequestRequired",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "proofGenerationCost",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposedTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "updateMarketDedicatedKey",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestationData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveSignature"),
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
                    ::std::borrow::ToOwned::to_owned("leaveMarketplace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("leaveMarketplace"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("leaveMarketplaces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("leaveMarketplaces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketIds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("proofMarketplace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proofMarketplace"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ProofMarketplace",
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
                    ::std::borrow::ToOwned::to_owned("register"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("register"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewardAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("declaredCompute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorData"),
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
                    ::std::borrow::ToOwned::to_owned("removeEncryptionKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeEncryptionKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("requestForExitMarketplace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requestForExitMarketplace",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("requestForExitMarketplaces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requestForExitMarketplaces",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketIds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("slashGenerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slashGenerator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slashingAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewardAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("unstake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unstake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("updateEncryptionKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateEncryptionKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestationData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveSignature"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddIvsKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddIvsKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddedStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddedStake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("ChangedGeneratorRewardAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChangedGeneratorRewardAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newRewardAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DecreaseCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DecreaseCompute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("compute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DeregisteredGenerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeregisteredGenerator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncreasedCompute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IncreasedCompute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("compute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("JoinedMarketplace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("JoinedMarketplace"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("computeAllocation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LeftMarketplace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LeftMarketplace"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("RegisteredGenerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RegisteredGenerator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initialCompute"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initialStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemovedStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemovedStake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RequestComputeDecrease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RequestComputeDecrease",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "intendedUtilization",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RequestExitMarketplace"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RequestExitMarketplace",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("marketId"),
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
                    ::std::borrow::ToOwned::to_owned("RequestStakeDecrease"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RequestStakeDecrease",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "intendedUtilization",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
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
    pub static GENERATORREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R0`\x80R4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\0]\x878\x03\x80b\0]\x87\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\x01}V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\0YWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\0uWP0;\x15\x80\x15b\0\0uWP`\0T`\xFF\x16`\x01\x14[b\0\0\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x01\x01W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\xA0R\x82\x16`\xC0R\x80\x15b\0\x01[W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPb\0\x01\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01zW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01\x91W`\0\x80\xFD[\x82Qb\0\x01\x9E\x81b\0\x01dV[` \x84\x01Q\x90\x92Pb\0\x01\xB1\x81b\0\x01dV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Qa[@b\0\x02G`\09`\0\x81\x81a\x05u\x01R\x81\x81a\x0Ea\x01R\x81\x81a\x19\0\x01R\x81\x81a#|\x01Ra/\x10\x01R`\0\x81\x81a\x02\xB3\x01R\x81\x81a\x0C]\x01R\x81\x81a\x1E\xB6\x01R\x81\x81a(`\x01R\x81\x81a3K\x01Ra4\xFE\x01R`\0\x81\x81a\x148\x01R\x81\x81a\x14x\x01R\x81\x81a\x17\x81\x01R\x81\x81a\x17\xC1\x01Ra\x18P\x01Ra[@`\0\xF3\xFE`\x80`@R`\x046\x10a\x02gW`\x005`\xE0\x1C\x80cz\x14\xC4c\x11a\x01DW\x80c\xA2\x17\xFD\xDF\x11a\0\xB6W\x80c\xD5Gt\x1F\x11a\0zW\x80c\xD5Gt\x1F\x14a\x07\xEEW\x80c\xE2\xFA3\xCE\x14a\x08\x0EW\x80c\xE7\xBC\x96\0\x14a\x08.W\x80c\xE9\xE94\xA0\x14a\x08NW\x80c\xEA\xAC\xAE\x94\x14a\x08nW\x80c\xF2\x88\x8D\xBB\x14a\x08\x8EW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x07YW\x80c\xAD\xC9w.\x14a\x07nW\x80c\xC4\x92\xEE9\x14a\x07\x8EW\x80c\xCA\x15\xC8s\x14a\x07\xAEW\x80c\xD0n\x1F{\x14a\x07\xCEW`\0\x80\xFD[\x80c\x91\xD1HT\x11a\x01\x08W\x80c\x91\xD1HT\x14a\x06NW\x80c\x92\xEB\x91\xE2\x14a\x06nW\x80c\x96\xDE\x0E\xEF\x14a\x06\x8EW\x80c\x98*A]\x14a\x06\xAEW\x80c\x9A\x7F\xCA\x8E\x14a\x06\xCEW\x80c\x9F]\xB9\x86\x14a\x079W`\0\x80\xFD[\x80cz\x14\xC4c\x14a\x04\x98W\x80c\x81\xC4\\p\x14a\x05\xB7W\x80c\x84\xAC3\xEC\x14a\x05\xD8W\x80c\x8C\xFCV\xD8\x14a\x05\xF8W\x80c\x90\x10\xD0|\x14a\x06.W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01\xDDW\x80cO\x1E\xF2\x86\x11a\x01\xA1W\x80cO\x1E\xF2\x86\x14a\x04\xEDW\x80cR\xD1\x90-\x14a\x05\0W\x80cT\x1A\x8C\x18\x14a\x05\x15W\x80cdmQ\xB4\x14a\x055W\x80cf\x1D\xE5\xAC\x14a\x05cW\x80cm@Xw\x14a\x05\x97W`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x04XW\x80c6Y\xCF\xE6\x14a\x04xW\x80c<^\xB5|\x14a\x04\x98W\x80cH\\\xC9U\x14a\x04\xADW\x80cM*\xAB\x9A\x14a\x04\xCDW`\0\x80\xFD[\x80c!\x80\xDE]\x11a\x02/W\x80c!\x80\xDE]\x14a\x03dW\x80c$\x8A\x9C\xA3\x14a\x03\x84W\x80c+a\x0C-\x14a\x03\xC2W\x80c,\x1F\xBD\x03\x14a\x04\x01W\x80c//\xF1]\x14a\x04#W\x80c/\x8FJ;\x14a\x04CW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02lW\x80c\x04y\xD6D\x14a\x02\xA1W\x80c\x08\xBEk\xAD\x14a\x02\xEDW\x80c\x13m\xFB\xF5\x14a\x03\x0FW\x80c\x1C~\xAEe\x14a\x03/W[`\0\x80\xFD[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04aN\x9FV[a\x08\xAEV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x98V[4\x80\x15a\x02\xF9W`\0\x80\xFD[Pa\x03\ra\x03\x086`\x04aN\xC9V[a\x08\xBFV[\0[4\x80\x15a\x03\x1BW`\0\x80\xFD[Pa\x03\ra\x03*6`\x04aP\x18V[a\t\x03V[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x03Oa\x03J6`\x04aP{V[a\x0C\xDCV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x98V[4\x80\x15a\x03pW`\0\x80\xFD[Pa\x03\ra\x03\x7F6`\x04aP\xE9V[a\r~V[4\x80\x15a\x03\x90W`\0\x80\xFD[Pa\x03\xB4a\x03\x9F6`\x04aQYV[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x02\x98V[4\x80\x15a\x03\xCEW`\0\x80\xFD[Pa\x03\xE2a\x03\xDD6`\x04aP{V[a\x0F\x0EV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x98V[4\x80\x15a\x04\rW`\0\x80\xFD[Pa\x03\xB4`\0\x80Q` aZ\xA4\x839\x81Q\x91R\x81V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x03\ra\x04>6`\x04aQrV[a\x10\xF5V[4\x80\x15a\x04OW`\0\x80\xFD[Pa\x03\ra\x11\x1AV[4\x80\x15a\x04dW`\0\x80\xFD[Pa\x03\ra\x04s6`\x04aQrV[a\x13\xB0V[4\x80\x15a\x04\x84W`\0\x80\xFD[Pa\x03\ra\x04\x936`\x04aQ\xA2V[a\x14.V[4\x80\x15a\x04\xA4W`\0\x80\xFD[Pa\x03\xB4`d\x81V[4\x80\x15a\x04\xB9W`\0\x80\xFD[Pa\x03\ra\x04\xC86`\x04aQ\xBFV[a\x15\rV[4\x80\x15a\x04\xD9W`\0\x80\xFD[Pa\x03\ra\x04\xE86`\x04aQ\xA2V[a\x16\x86V[a\x03\ra\x04\xFB6`\x04aQ\xEDV[a\x17wV[4\x80\x15a\x05\x0CW`\0\x80\xFD[Pa\x03\xB4a\x18CV[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x03\ra\x0506`\x04aQYV[a\x18\xF6V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x05Ua\x05P6`\x04aP{V[a\x19\x89V[`@Qa\x02\x98\x92\x91\x90aRuV[4\x80\x15a\x05oW`\0\x80\xFD[Pa\x02\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xA3W`\0\x80\xFD[Pa\x03\ra\x05\xB26`\x04aQYV[a\x1CMV[4\x80\x15a\x05\xC3W`\0\x80\xFD[Pa\x03WTa\x02\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\xE4W`\0\x80\xFD[Pa\x03\ra\x05\xF36`\x04aQ\xA2V[a\x1DAV[4\x80\x15a\x06\x04W`\0\x80\xFD[Pa\x06\x18a\x06\x136`\x04aQ\xA2V[a\x1F\x8EV[`@Qa\x02\x98\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aR\xE0V[4\x80\x15a\x06:W`\0\x80\xFD[Pa\x02\xD5a\x06I6`\x04aSEV[a vV[4\x80\x15a\x06ZW`\0\x80\xFD[Pa\x02\x8Ca\x06i6`\x04aQrV[a \x95V[4\x80\x15a\x06zW`\0\x80\xFD[Pa\x03\ra\x06\x896`\x04aP\xE9V[a \xC0V[4\x80\x15a\x06\x9AW`\0\x80\xFD[Pa\x03\ra\x06\xA96`\x04aQYV[a#\xF3V[4\x80\x15a\x06\xBAW`\0\x80\xFD[Pa\x03\ra\x06\xC96`\x04aSgV[a&2V[4\x80\x15a\x06\xDAW`\0\x80\xFD[Pa\x07(a\x06\xE96`\x04aP{V[a\x03T` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\xFF\x90\x93\x16\x93\x91\x92\x90\x91\x90\x85V[`@Qa\x02\x98\x95\x94\x93\x92\x91\x90aS\x9CV[4\x80\x15a\x07EW`\0\x80\xFD[Pa\x03\ra\x07T6`\x04aQYV[a'kV[4\x80\x15a\x07eW`\0\x80\xFD[Pa\x03\xB4`\0\x81V[4\x80\x15a\x07zW`\0\x80\xFD[Pa\x03\xB4a\x07\x896`\x04aP{V[a'uV[4\x80\x15a\x07\x9AW`\0\x80\xFD[Pa\x03\ra\x07\xA96`\x04aSgV[a(\xF8V[4\x80\x15a\x07\xBAW`\0\x80\xFD[Pa\x03\xB4a\x07\xC96`\x04aQYV[a*\xF0V[4\x80\x15a\x07\xDAW`\0\x80\xFD[Pa\x03\ra\x07\xE96`\x04aN\xC9V[a+\x07V[4\x80\x15a\x07\xFAW`\0\x80\xFD[Pa\x03\ra\x08\t6`\x04aQrV[a+FV[4\x80\x15a\x08\x1AW`\0\x80\xFD[Pa\x03\ra\x08)6`\x04aS\xDAV[a+kV[4\x80\x15a\x08:W`\0\x80\xFD[Pa\x03\ra\x08I6`\x04aQYV[a/\xEEV[4\x80\x15a\x08ZW`\0\x80\xFD[Pa\x03\ra\x08i6`\x04aQYV[a/\xF8V[4\x80\x15a\x08zW`\0\x80\xFD[Pa\x03\xB4a\x08\x896`\x04aT}V[a1\xF6V[4\x80\x15a\x08\x9AW`\0\x80\xFD[Pa\x03\ra\x08\xA96`\x04aQ\xA2V[a3\x82V[`\0a\x08\xB9\x82a6\x1BV[\x92\x91PPV[`\0[\x81\x81\x10\x15a\x08\xFEWa\x08\xEC3\x84\x84\x84\x81\x81\x10a\x08\xE0Wa\x08\xE0aT\xC7V[\x90P` \x02\x015a6@V[\x80a\x08\xF6\x81aT\xF3V[\x91PPa\x08\xC2V[PPPV[a\t\x0Ba8\xE9V[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\t\x9F\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xCB\x90aU\x0CV[\x80\x15a\n\x18W\x80`\x1F\x10a\t\xEDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x18V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xFBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x82Q`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA3`\xF0\x1B\x81RP\x90a\nkW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16a\n\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x85a\n\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaG1`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@Q\x80a\x01@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01\x86\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01\x84\x81RPa\x03S`\0\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01U`\xE0\x82\x01Q\x81`\x07\x01Ua\x01\0\x82\x01Q\x81`\x08\x01Ua\x01 \x82\x01Q\x81`\t\x01\x90\x81a\x0CF\x91\x90aU\xA7V[PP\x84\x15\x90Pa\x0C\x85Wa\x0C\x85`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x830\x87a9DV[`@\x80Q\x86\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x7FI\x1B\x0FF\x1D\x05\xB7\x9B\xFCi\x04\xA9\xBA\x9D\xF5#\xD7fP?\x11\x04k+^n\xDD\xAA\xE3\xA7#\x1C\x91\x01`@Q\x80\x91\x03\x90\xA2PPa\x0C\xD6`\x01a\x01-UV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03T` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\r(Wa\r(aR=V[`\x04\x81\x11\x15a\r9Wa\r9aR=V[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP[\x92P\x92\x90PV[a\x03WT`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xF1\x91\x90\x81\x01\x90aV\xACV[P\x94PPPPPa\x0E\x01\x84a9\xAFV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a\x0E=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa\x0EJ\x84\x84\x843a9\xE8V[`@Qc\x07\x07Y\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x07\x07Y\x1F\x90a\x0E\x96\x90\x87\x90`\x04\x01aUFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xB0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xC4W=`\0\x80>=`\0\xFD[PPPPa\x0E\xD1\x84a;\x15V[`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F\xC3\xFB\xBD\xB6\xAA\x8D\x99\xF6\xEF\xE2J:\"\xE9\xA9\x9F\xFE\xF2J/9\x9C\x0B\x1ET\x99F\xF9\x1B\xF06\xE4`@Q`@Q\x80\x91\x03\x90\xA3PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03T` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x0FZWa\x0FZaR=V[`\x04\x81\x11\x15a\x0FkWa\x0FkaR=V[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x03S`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01\x80Ta\x10\\\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\x88\x90aU\x0CV[\x80\x15a\x10\xD5W\x80`\x1F\x10a\x10\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xD5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x90Q`@\x90\x93\x01Q\x92\x94P\x91\x92PPP\x92P\x92\x90PV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x11\x10\x81a;>V[a\x08\xFE\x83\x83a;HV[3`\0\x81\x81Ra\x03S` R`@\x90 `\t\x81\x01\x80Ta\x119\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa\x11qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a\x11\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x08\x01T\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG11`\xE8\x1B\x81RP\x90a\x11\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x06\x01T\x83`\x08\x01Ta\x12\x1F\x91\x90aW#V[a\x12)\x91\x90aW:V[\x90P`\0\x81\x83`\x06\x01Ta\x12=\x91\x90aW\\V[\x90P\x82`\x03\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x90a\x12\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x82`\x02\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x90a\x12\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x06\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x08\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03V` R`@\x90 TC\x10\x80\x15\x90a\x13\x19WP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03V` R`@\x90 T\x15\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x90a\x13SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x03V` R`@\x80\x82 \x91\x90\x91UQ\x7F\x11\xDE\xAE(\x9Epx\xFC\xE2\x88\xF4\xE9sN\x9C=\xF6{U\xEA\xC4\xFFA\xB8\x90\xB6\xD68\x1A\xCE\xE7b\x90a\x13\xA2\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x14 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\nbV[a\x14*\x82\x82a;RV[PPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x14vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x90aWoV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x14\xBF`\0\x80Q` aZ\xC4\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x90aW\xBBV[a\x14\xEE\x81a;\x9DV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\x15\n\x91\x83\x91\x90a;\xA8V[PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x15-WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x15GWP0;\x15\x80\x15a\x15GWP`\0T`\xFF\x16`\x01\x14[a\x15\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x15\xCDW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x15\xD5a=\x13V[a\x15\xDDa=\x13V[a\x15\xE5a=\x13V[a\x15\xEDa=\x13V[a\x15\xF5a=\x13V[a\x15\xFDa=\x13V[a\x16\x08`\0\x84a=\x80V[a\x16 `\0\x80Q` aZ\xA4\x839\x81Q\x91R\x83a;HV[a\x03W\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x08\xFEW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16a\x16\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P3`\0\x81\x81Ra\x03S` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x17\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x83U`@Q\x91\x82R\x83\x16\x90\x7F\xB1\x95\xA9K\xEC\xD3\x88\xC9\xA0\x7F\xE8\x18qh3\xBD\xF9\x8Bu\\x\xC9\xB48\xF4\xC8\xF0o5O\xA3h\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x17\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x90aWoV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x18\x08`\0\x80Q` aZ\xC4\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x90aW\xBBV[a\x187\x82a;\x9DV[a\x14*\x82\x82`\x01a;\xA8V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nbV[P`\0\x80Q` aZ\xC4\x839\x81Q\x91R\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xED8\r\x033`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x82W=`\0\x80>=`\0\xFD[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03T` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x19\xD5Wa\x19\xD5aR=V[`\x04\x81\x11\x15a\x19\xE6Wa\x19\xE6aR=V[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x03S`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01\x80Ta\x1A\xD7\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B\x03\x90aU\x0CV[\x80\x15a\x1BPW\x80`\x1F\x10a\x1B%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1BPV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0`\x04\x81\x11\x15a\x1BnWa\x1BnaR=V[\x82Q`\x04\x81\x11\x15a\x1B\x81Wa\x1B\x81aR=V[\x03a\x1B\x94W`\0\x80\x93P\x93PPPa\rwV[`\x04\x82Q`\x04\x81\x11\x15a\x1B\xA9Wa\x1B\xA9aR=V[\x03a\x1B\xBDW`\x04`\0\x93P\x93PPPa\rwV[`\0a\x1B\xC8\x87a=\x8AV[\x90P`\0\x83Q`\x04\x81\x11\x15a\x1B\xDFWa\x1B\xDFaR=V[\x14\x15\x80\x15a\x1B\xEBWP\x80\x15[\x15a\x1C\0W`\x02`\0\x94P\x94PPPPa\rwV[\x81`\xC0\x01Q\x81\x03a\x1C\x19W`\x01\x94P\x92Pa\rw\x91PPV[\x80\x15\x80\x15\x90a\x1C+WP\x81`\xC0\x01Q\x81\x10[\x15a\x1C>W`\x03\x94P\x92Pa\rw\x91PPV[P`\0\x96\x87\x96P\x94PPPPPV[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1C\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80`\t\x01\x80Ta\x1C\xB4\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa\x1C\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x82\x81`\x06\x01`\0\x82\x82Ta\x1D\x01\x91\x90aX\x07V[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F \x98X\xCE\xC2\x8CQ\x8A\xEAD\xAA?\xA7\n\x93\"S\xF8\xBD\xF7j\x1C\x9A\x0B\x08\"j\x13\xE6\x975\xA3\x90` \x01a\x17jV[a\x1DIa8\xE9V[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\x1D\xDD\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\t\x90aU\x0CV[\x80\x15a\x1EVW\x80`\x1F\x10a\x1E+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1EVV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`@\x01Q`\0\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG3`\xF0\x1B\x81RP\x90a\x1E\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P` \x81\x01Qa\x1E\xDE\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x85\x90a?\x02V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03S` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x82\x90U`\x05\x81\x01\x82\x90U`\x06\x81\x01\x82\x90U`\x07\x81\x01\x82\x90U`\x08\x81\x01\x82\x90U\x90a\x1FK`\t\x83\x01\x82aNQV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x9F70\xAD\xE9K\xE5\xCE?\xAD\x97;\x88\x8At\x86j:\x91]\0\x8E\x8C\xBD\xE2\x13\x82\xB91\xB6|c\x90`\0\x90\xA2PPa\x15\n`\x01a\x01-UV[a\x03S` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x88\x01T`\x08\x89\x01T`\t\x8A\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x9A\x16\x9A\x98\x99\x97\x98\x96\x97\x95\x96\x94\x95\x93\x94\x92\x93\x91\x92\x91a\x1F\xF3\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \x1F\x90aU\x0CV[\x80\x15a lW\x80`\x1F\x10a AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a lV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a OW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x8AV[`\0\x82\x81R`\x97` R`@\x81 a \x8E\x90\x83a?2V[\x93\x92PPPV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a!T\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!\x80\x90aU\x0CV[\x80\x15a!\xCDW\x80`\x1F\x10a!\xA2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xCDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\xB0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPa\x03WT`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R\x92\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"O\x91\x90\x81\x01\x90aV\xACV[PPPP\x91PP`\0\x80\x1B\x81\x14\x15\x80a\"\x88WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x81\x14\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG15`\xE8\x1B\x81RP\x90a\"\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa\"\xCC\x86a9\xAFV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a#\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x81Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a#KW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0a#W\x87a?>V[\x90Pa#e\x87\x87\x873a9\xE8V[`@Qc4\xFE\xDEe`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\xFD\xBC\xCA\x90a#\xB7\x90\x87\x90\x8C\x90\x86\x90\x8D\x90`\x04\x01aX\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\xE5W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a$JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80`\t\x01\x80Ta$Z\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa$\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x08\x01T\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x90a$\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA7`\xF0\x1B` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a%\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x06\x01T\x85a%:\x91\x90aW#V[a%D\x91\x90aW:V[\x90P\x81`\x02\x01T\x81\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA7`\xF0\x1B\x81RP\x90a%\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80\x82`\x06\x01Ta%\x98\x91\x90aW\\V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90a%\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x08\x82\x01\x84\x90Ua%\xE2`\x01CaX\x07V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x03V` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7Fv\x12,\xFD_<h\0\xA2#\xE6\x0C\xC6$G\xC6I6\x03}x\xD0-\x91\x92\x93\xF6U\xBA}T\xCB\x90a\x13\xA2\x90\x87\x81R` \x01\x90V[`\0\x80Q` aZ\xA4\x839\x81Q\x91Ra&J\x81a;>V[`\0a&V\x85\x85a\x19\x89V[P\x90P`\x03\x81`\x04\x81\x11\x15a&mWa&maR=V[\x14\x80a&\x8AWP`\x04\x81`\x04\x81\x11\x15a&\x88Wa&\x88aR=V[\x14[\x80a&\xA6WP`\x02\x81`\x04\x81\x11\x15a&\xA4Wa&\xA4aR=V[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a#\x9B`\xF1\x1B\x81RP\x90a&\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 a\x03T\x83R\x81\x84 \x88\x85R\x90\x92R\x82 `\x01\x81\x01T`\x03\x83\x01\x80T\x93\x94\x92\x93\x91\x92\x83\x92a'*\x90\x84\x90aW\\V[\x92PP\x81\x90UP\x85\x83`\x04\x01`\0\x82\x82Ta'E\x91\x90aW\\V[\x90\x91UPP`\x04\x82\x01\x80T\x90`\0a'\\\x83aXSV[\x91\x90PUPPPPPPPPPV[a\x15\n3\x82a6@V[`\0a'\x7Fa8\xE9V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03S` R`@\x90 `\t\x81\x01\x80Ta'\xA7\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa'\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a(\"W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x83a(ZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa(\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x1630\x86a9DV[\x82\x81`\x01\x01`\0\x82\x82Ta(\xA4\x91\x90aX\x07V[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xB6\xBE#\x16\x85\x06\xA1\xCEb\xCCo\x933\x9Fg\x10\xE0\x98De\x17\n\xB5-\xF8\xEC\xF7\xDA8\xB3E\x84\x90` \x01`@Q\x80\x91\x03\x90\xA2`\x01\x01T\x90Pa\x08\xB9`\x01a\x01-UV[a)\0a8\xE9V[`\0\x80Q` aZ\xA4\x839\x81Q\x91Ra)\x18\x81a;>V[`\0\x80a)%\x86\x86a\x19\x89V[\x90\x92P\x90P`\x01\x82`\x04\x81\x11\x15a)>Wa)>aR=V[\x14\x80a)[WP`\x03\x82`\x04\x81\x11\x15a)YWa)YaR=V[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x11\xCD`\xF2\x1B\x81RP\x90a)\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 a\x03T\x83R\x81\x84 \x89\x85R\x83R\x92\x81\x90 `\x01\x81\x01T\x82Q\x80\x84\x01\x90\x93R`\x02\x83RaG5`\xF0\x1B\x93\x83\x01\x93\x90\x93R\x91\x84\x10\x15a*\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`d\x81`\x04\x01T\x11\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b#\x98\x99`\xE9\x1B\x81RP\x90a*EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0a*Q\x89a?eV[\x90P\x86\x81\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x90a*\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x86\x83`\x04\x01`\0\x82\x82Ta*\xA5\x91\x90aX\x07V[\x90\x91UPP`\x01\x82\x01T`\x03\x84\x01\x80T`\0\x90a*\xC3\x90\x84\x90aX\x07V[\x90\x91UPP`\x04\x82\x01\x80T\x90`\0a*\xDA\x83aT\xF3V[\x91\x90PUPPPPPPPa\x08\xFE`\x01a\x01-UV[`\0\x81\x81R`\x97` R`@\x81 a\x08\xB9\x90a@\xD4V[`\0[\x81\x81\x10\x15a\x08\xFEWa+43\x84\x84\x84\x81\x81\x10a+(Wa+(aT\xC7V[\x90P` \x02\x015a@\xDEV[\x80a+>\x81aT\xF3V[\x91PPa+\nV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta+a\x81a;>V[a\x08\xFE\x83\x83a;RV[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x80\x83 a\x03T\x83R\x81\x84 \x8D\x85R\x90\x92R\x80\x83 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x92\x93\x92\x82\x90`\xFF\x16`\x04\x81\x11\x15a+\xB3Wa+\xB3aR=V[`\x04\x81\x11\x15a+\xC4Wa+\xC4aR=V[\x81R`\x01\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x02\x80\x84\x01T`@\x80\x85\x01\x91\x90\x91R`\x03\x85\x01T``\x85\x01R`\x04\x90\x94\x01T`\x80\x90\x93\x01\x92\x90\x92R\x85T\x83Q\x80\x85\x01\x90\x94R\x91\x83Ra#\x99`\xF1\x1B\x90\x83\x01R\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16a,=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0\x80a,J\x8DaA\xCDV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaM1`\xF0\x1B` \x82\x01R\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16a,\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0\x83Q`\x04\x81\x11\x15a,\xA5Wa,\xA5aR=V[\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a&\x99`\xF1\x1B\x81RP\x90a,\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x8Aa-\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x8Ca-OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x8B\x84`\x02\x01`\0\x82\x82Ta-d\x91\x90aX\x07V[\x92PP\x81\x90UP\x83`\x06\x01T\x84`\x02\x01T\x11\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM3`\xF0\x1B\x81RP\x90a-\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x05\x84\x01\x80T\x90`\0a-\xC3\x83aT\xF3V[\x90\x91UPP`@\x80Q`\xA0\x81\x01\x90\x91R\x80`\x01\x81R` \x01\x8D\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01`\0\x81RPa\x03T`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8F\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x04\x81\x11\x15a.SWa.SaR=V[\x02\x17\x90UP` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01U``\x82\x01Q`\x03\x82\x01U`\x80\x90\x91\x01Q`\x04\x90\x91\x01U\x80\x15\x80\x15\x90a.\xB1WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x81\x14\x15[\x15a/\x9BWa.\xBF\x88a9\xAFV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a.\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x88\x15a/\x9BWa/\x0E\x88\x88\x88\x88a9\xE8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ci\xFD\xBC\xCA\x86\x8Fa/H\x8Ca?>V[\x8C`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/h\x94\x93\x92\x91\x90aX\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a/\x96W=`\0\x80>=`\0\xFD[PPPP[\x8C\x85`\x01`\x01`\xA0\x1B\x03\x16\x7F\x04Y\xABWU\x08\xB4\xA5\x89O{\x13\x87\xBF6-\x03!;;\xF81\xDAE&!\x8C3\xE4\xA9\x06\xD8\x8E`@Qa/\xD7\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPPV[a\x15\n3\x82a@\xDEV[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a0OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80`\t\x01\x80Ta0_\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa0\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x07\x01T\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x90a0\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA7`\xF0\x1B` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a1#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x07\x81\x01\x83\x90U`\x01\x81\x01T`\0\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90a1G\x90\x86aW#V[a1Q\x91\x90aW:V[\x90P\x80\x82`\x01\x01Ta1c\x91\x90aW\\V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90a1\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa1\xA6`\x01CaX\x07V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x03U` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7F\x13\x9A\xD7\xA0\xC3\xF6\xC0\xAD\x0F\x0F\xC5+vx`jq\xFF\x08\x1B\x99&\xA5do\xE5:\x8Eg\xC6\x8Ar\x90a\x13\xA2\x90\x87\x81R` \x01\x90V[`\0`\0\x80Q` aZ\xA4\x839\x81Q\x91Ra2\x10\x81a;>V[`\0a2\x1C\x87\x87a\x19\x89V[P\x90P`\x03\x81`\x04\x81\x11\x15a23Wa23aR=V[\x14\x80a2PWP`\x04\x81`\x04\x81\x11\x15a2NWa2NaR=V[\x14[\x80a2lWP`\x02\x81`\x04\x81\x11\x15a2jWa2jaR=V[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x10M`\xF2\x1B\x81RP\x90a2\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 a\x03T\x83R\x81\x84 \x8A\x85R\x90\x92R\x82 `\x04\x81\x01\x80T\x92\x93\x91\x92\x91a2\xE6\x83aXSV[\x91\x90PUP\x86\x82`\x01\x01`\0\x82\x82Ta2\xFF\x91\x90aW\\V[\x92PP\x81\x90UP\x86\x82`\x04\x01`\0\x82\x82Ta3\x1A\x91\x90aW\\V[\x90\x91UPP`\x01\x81\x01T`\x03\x83\x01\x80T`\0\x90a38\x90\x84\x90aW\\V[\x90\x91UPa3r\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x89a?\x02V[P`\x01\x01T\x97\x96PPPPPPPV[a3\x8Aa8\xE9V[3`\0\x81\x81Ra\x03S` R`@\x90 `\t\x81\x01\x80Ta3\xA9\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa3\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a4$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x07\x01T\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04s\x13`\xEC\x1B\x81RP\x90a4oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x01\x01T\x83`\x07\x01Ta4\x8F\x91\x90aW#V[a4\x99\x91\x90aW:V[\x90P`\0\x81\x83`\x01\x01Ta4\xAD\x91\x90aW\\V[\x90P\x82`\x04\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x90a4\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa5%`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x83a?\x02V[`\x01\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x07\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03U` R`@\x90 TC\x10\x80\x15\x90a5{WP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03U` R`@\x90 T\x15\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x90a5\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x03U` R`@\x80\x82 \x91\x90\x91UQ\x7F\xE5+<X\xA1\x16\xC1\xF0\x12\xC9\x9D\x11 \xC0T\xE7?Q\xB7\xA2\x9CEq\x92E_\xD2\xC2\x0E\x03\xA53\x90a6\x04\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPa\x15\n`\x01a\x01-UV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x08\xB9WPa\x08\xB9\x82aBYV[a\x03WT`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6\xB3\x91\x90\x81\x01\x90aV\xACV[PP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaM1`\xF0\x1B` \x82\x01R\x93\x94PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16a6\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03T` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x04\x81\x11\x15a7BWa7BaR=V[`\x04\x81\x11\x15a7SWa7SaR=V[\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x90\x91\x01R\x90P`\0\x81Q`\x04\x81\x11\x15a7\x96Wa7\x96aR=V[\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG9`\xF0\x1B\x81RP\x90a7\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x80\x81\x01Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x13M`\xF2\x1B` \x82\x01R\x90\x15a8\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0a\x03S`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x90P\x81` \x01Q\x81`\x02\x01`\0\x82\x82Ta8R\x91\x90aW\\V[\x92PP\x81\x90UP`\x01\x81`\x05\x01`\0\x82\x82Ta8n\x91\x90aW\\V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x81\x81Ra\x03T` \x90\x81R`@\x80\x83 \x88\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x81\x01\x83\x90U`\x03\x81\x01\x83\x90U`\x04\x01\x82\x90UQ\x86\x92\x91\x7F\x83\x1C\xD5\xB7S\x83\x80\xB0\xA2\xA3\x14\x14\xD64\xECBq\x16\x07V\x84\xA2v\x82\x8A\xB4\xD2ut\xA2\xDF\xDF\x91\xA3PPPPPV[`\x02a\x01-T\x03a9<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\nbV[`\x02a\x01-UV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x0C\xD6\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91RaB\x8EV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a9\xC9\x91\x90aXjV[PPP\x94P\x94P\x94PPPa9\xDF\x83\x83\x83aCcV[\x95\x94PPPPPV[`\0\x84\x82`@Q` \x01a9\xFD\x92\x91\x90aYYV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a:n\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P`\0a:\xB2\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaC\x9C\x92PPPV[\x90Pa:\xBD\x87a;\x15V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x13\x13`\xEC\x1B\x81RP\x90a;\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[PPPPPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a;,\x91\x90aXjV[PPPPPP\x91PPa \x8E\x81aC\xC0V[a\x15\n\x813aD\rV[a\x14*\x82\x82aDfV[a;\\\x82\x82aD\x88V[a;f`\0a*\xF0V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra \x99`\xF1\x1B` \x82\x01R\x90a\x08\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[`\0a\x14*\x81a;>V[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a;\xDBWa\x08\xFE\x83aD\xAAV[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a<5WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra<2\x91\x81\x01\x90aY\x83V[`\x01[a<\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80Q` aZ\xC4\x839\x81Q\x91R\x81\x14a=\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01a\nbV[Pa\x08\xFE\x83\x83\x83aEFV[`\0Ta\x01\0\x90\x04`\xFF\x16a=~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\nbV[V[a\x14*\x82\x82a;HV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01T`\xC0\x84\x01R`\x07\x81\x01T`\xE0\x84\x01R`\x08\x81\x01Ta\x01\0\x84\x01R`\t\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91a\x01 \x84\x01\x91a>%\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta>Q\x90aU\x0CV[\x80\x15a>\x9EW\x80`\x1F\x10a>sWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a>\x9EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a>\x81W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x01\0\x01Q\x83`\xC0\x01Qa>\xC9\x91\x90aW#V[a>\xD3\x91\x90aW:V[\x90P\x81``\x01Q\x81\x10\x15a>\xEBWP`\0\x93\x92PPPV[``\x82\x01Qa>\xFA\x90\x82aW\\V[\x94\x93PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x08\xFE\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a9xV[`\0a \x8E\x83\x83aEkV[```\0\x82\x80` \x01\x90Q\x81\x01\x90a?V\x91\x90aXjV[P\x94\x99\x98PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01T`\xC0\x84\x01R`\x07\x81\x01T`\xE0\x84\x01R`\x08\x81\x01Ta\x01\0\x84\x01R`\t\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91a\x01 \x84\x01\x91a@\0\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta@,\x90aU\x0CV[\x80\x15a@yW\x80`\x1F\x10a@NWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a@yV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a@\\W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\xE0\x01Q\x83` \x01Qa@\xA3\x91\x90aW#V[a@\xAD\x91\x90aW:V[\x90P\x81`\x80\x01Q\x81\x10\x15a@\xC5WP`\0\x93\x92PPPV[`\x80\x82\x01Qa>\xFA\x90\x82aW\\V[`\0a\x08\xB9\x82T\x90V[`\0a@\xEA\x83\x83a\x19\x89V[P\x90P`\0\x81`\x04\x81\x11\x15aA\x01WaA\x01aR=V[\x14\x15\x80\x15aA!WP`\x04\x81`\x04\x81\x11\x15aA\x1EWaA\x1EaR=V[\x14\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x08\xE7`\xF3\x1B\x81RP\x90aAZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03T` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16`\x04\x17\x81U\x90Q\x90\x92\x85\x92\x90\x91\x7F\xF9\xE1yp\xDFW\xA6\xA8E}\xCB\xB5\xC2\x91gkF1\xD37\xCFv\xB0\xC8\x01\xF6\xB8\xADj|_\x92\x91\x90\xA3\x80`\x04\x01T`\0\x03a\x0C\xD6Wa\x0C\xD6\x84\x84a6@V[a\x03WT`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaBH\x91\x90\x81\x01\x90aV\xACV[P\x93\x99\x92\x98P\x91\x96PPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x08\xB9WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x08\xB9V[`\0aB\xE3\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aE\x95\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80aC\x04WP\x80\x80` \x01\x90Q\x81\x01\x90aC\x04\x91\x90aY\x9CV[a\x08\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80\x84\x84\x84`@Q` \x01aC{\x93\x92\x91\x90aY\xB9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x80`\0aC\xAB\x85\x85aE\xA4V[\x91P\x91PaC\xB8\x81aE\xE6V[P\x93\x92PPPV[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90aD\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[PP\x80Q` \x90\x91\x01 \x90V[aD\x17\x82\x82a \x95V[a\x14*WaD$\x81aG0V[aD/\x83` aGBV[`@Q` \x01aD@\x92\x91\x90aY\xFCV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\nb\x91`\x04\x01aUFV[aDp\x82\x82aH\xDEV[`\0\x82\x81R`\x97` R`@\x90 a\x08\xFE\x90\x82aIdV[aD\x92\x82\x82aIyV[`\0\x82\x81R`\x97` R`@\x90 a\x08\xFE\x90\x82aI\xE0V[`\x01`\x01`\xA0\x1B\x03\x81\x16;aE\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80Q` aZ\xC4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[aEO\x83aI\xF5V[`\0\x82Q\x11\x80aE\\WP\x80[\x15a\x08\xFEWa\x0C\xD6\x83\x83aJ5V[`\0\x82`\0\x01\x82\x81T\x81\x10aE\x82WaE\x82aT\xC7V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``a>\xFA\x84\x84`\0\x85aJZV[`\0\x80\x82Q`A\x03aE\xDAW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaE\xCE\x87\x82\x85\x85aK5V[\x94P\x94PPPPa\rwV[P`\0\x90P`\x02a\rwV[`\0\x81`\x04\x81\x11\x15aE\xFAWaE\xFAaR=V[\x03aF\x02WPV[`\x01\x81`\x04\x81\x11\x15aF\x16WaF\x16aR=V[\x03aFcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nbV[`\x02\x81`\x04\x81\x11\x15aFwWaFwaR=V[\x03aF\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\nbV[`\x03\x81`\x04\x81\x11\x15aF\xD8WaF\xD8aR=V[\x03a\x15\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\nbV[``a\x08\xB9`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0aGQ\x83`\x02aW#V[aG\\\x90`\x02aX\x07V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aGtWaGtaOSV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aG\x9EW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10aG\xB9WaG\xB9aT\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10aG\xE8WaG\xE8aT\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0aH\x0C\x84`\x02aW#V[aH\x17\x90`\x01aX\x07V[\x90P[`\x01\x81\x11\x15aH\x8FWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10aHKWaHKaT\xC7V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10aHaWaHaaT\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93aH\x88\x81aXSV[\x90PaH\x1AV[P\x83\x15a \x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\nbV[aH\xE8\x82\x82a \x95V[a\x14*W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UaI 3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a \x8E\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aK\xF9V[aI\x83\x82\x82a \x95V[\x15a\x14*W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a \x8E\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aLHV[aI\xFE\x81aD\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a \x8E\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01aZ\xE4`'\x919aM;V[``\x82G\x10\x15aJ\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaJ\xD7\x91\x90aZqV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aK\x14W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aK\x19V[``\x91P[P\x91P\x91PaK*\x87\x83\x83\x87aM\xB3V[\x97\x96PPPPPPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aKlWP`\0\x90P`\x03aK\xF0V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aK\xC0W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aK\xE9W`\0`\x01\x92P\x92PPaK\xF0V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 TaL@WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x08\xB9V[P`\0a\x08\xB9V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aM1W`\0aLl`\x01\x83aW\\V[\x85T\x90\x91P`\0\x90aL\x80\x90`\x01\x90aW\\V[\x90P\x81\x81\x14aL\xE5W`\0\x86`\0\x01\x82\x81T\x81\x10aL\xA0WaL\xA0aT\xC7V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10aL\xC3WaL\xC3aT\xC7V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aL\xF6WaL\xF6aZ\x8DV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x08\xB9V[`\0\x91PPa\x08\xB9V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@QaMX\x91\x90aZqV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14aM\x93W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aM\x98V[``\x91P[P\x91P\x91PaM\xA9\x86\x83\x83\x87aM\xB3V[\x96\x95PPPPPPV[``\x83\x15aN\"W\x82Q`\0\x03aN\x1BW`\x01`\x01`\xA0\x1B\x03\x85\x16;aN\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\nbV[P\x81a>\xFAV[a>\xFA\x83\x83\x81Q\x15aN7W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80TaN]\x90aU\x0CV[`\0\x82U\x80`\x1F\x10aNmWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x15\n\x91\x90[\x80\x82\x11\x15aN\x9BW`\0\x81U`\x01\x01aN\x87V[P\x90V[`\0` \x82\x84\x03\x12\x15aN\xB1W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a \x8EW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aN\xDCW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aN\xF4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aO\x08W`\0\x80\xFD[\x815\x81\x81\x11\x15aO\x17W`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15aO,W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\nW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aO\x92WaO\x92aOSV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aO\xB4WaO\xB4aOSV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aO\xD3W`\0\x80\xFD[\x815aO\xE6aO\xE1\x82aO\x9AV[aOiV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aO\xFBW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aP.W`\0\x80\xFD[\x845aP9\x81aO>V[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aPcW`\0\x80\xFD[aPo\x87\x82\x88\x01aO\xC2V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15aP\x8EW`\0\x80\xFD[\x825aP\x99\x81aO>V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x83`\x1F\x84\x01\x12aP\xB9W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\xD1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\rwW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aP\xFFW`\0\x80\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aQ\x1EW`\0\x80\xFD[aQ*\x88\x83\x89\x01aO\xC2V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15aQ@W`\0\x80\xFD[PaQM\x87\x82\x88\x01aP\xA7V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15aQkW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aQ\x85W`\0\x80\xFD[\x825\x91P` \x83\x015aQ\x97\x81aO>V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aQ\xB4W`\0\x80\xFD[\x815a \x8E\x81aO>V[`\0\x80`@\x83\x85\x03\x12\x15aQ\xD2W`\0\x80\xFD[\x825aQ\xDD\x81aO>V[\x91P` \x83\x015aQ\x97\x81aO>V[`\0\x80`@\x83\x85\x03\x12\x15aR\0W`\0\x80\xFD[\x825aR\x0B\x81aO>V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR'W`\0\x80\xFD[aR3\x85\x82\x86\x01aO\xC2V[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x05\x81\x10aRqWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01aR\x83\x82\x85aRSV[\x82` \x83\x01R\x93\x92PPPV[`\0[\x83\x81\x10\x15aR\xABW\x81\x81\x01Q\x83\x82\x01R` \x01aR\x93V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaR\xCC\x81` \x86\x01` \x86\x01aR\x90V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x8D\x16\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x88`\x80\x84\x01R\x87`\xA0\x84\x01R\x86`\xC0\x84\x01R\x85`\xE0\x84\x01R\x84a\x01\0\x84\x01R\x80a\x01 \x84\x01RaS4\x81\x84\x01\x85aR\xB4V[\x9D\x9CPPPPPPPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aSXW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15aS|W`\0\x80\xFD[\x835aS\x87\x81aO>V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xA0\x81\x01aS\xAA\x82\x88aRSV[\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01R\x82`\x80\x83\x01R\x96\x95PPPPPPV[\x80\x15\x15\x81\x14a\x15\nW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15aS\xF6W`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015aT\x1D\x81aS\xCCV[\x93P`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aT:W`\0\x80\xFD[aTF\x8C\x83\x8D\x01aO\xC2V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15aT\\W`\0\x80\xFD[PaTi\x8B\x82\x8C\x01aP\xA7V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aT\x93W`\0\x80\xFD[\x845aT\x9E\x81aO>V[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aT\xBC\x81aO>V[\x93\x96\x92\x95P\x90\x93PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aU\x05WaU\x05aT\xDDV[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aU W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aU@WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x81R`\0a \x8E` \x83\x01\x84aR\xB4V[`\x1F\x82\x11\x15a\x08\xFEW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aU\x80WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aU\x9FW\x82\x81U`\x01\x01aU\x8CV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xC1WaU\xC1aOSV[aU\xD5\x81aU\xCF\x84TaU\x0CV[\x84aUYV[` \x80`\x1F\x83\x11`\x01\x81\x14aV\nW`\0\x84\x15aU\xF2WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85UaU\x9FV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aV9W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aV\x1AV[P\x85\x82\x10\x15aVWW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82`\x1F\x83\x01\x12aVxW`\0\x80\xFD[\x81QaV\x86aO\xE1\x82aO\x9AV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aV\x9BW`\0\x80\xFD[a>\xFA\x82` \x83\x01` \x87\x01aR\x90V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aV\xC5W`\0\x80\xFD[\x86QaV\xD0\x81aO>V[\x80\x96PP` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\nW`\0\x80\xFD[aW\x16\x89\x82\x8A\x01aVgV[\x91PP\x92\x95P\x92\x95P\x92\x95V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08\xB9Wa\x08\xB9aT\xDDV[`\0\x82aWWWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x08\xB9Wa\x08\xB9aT\xDDV[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x08\xB9Wa\x08\xB9aT\xDDV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0aXA`\x80\x83\x01\x85aR\xB4V[\x82\x81\x03``\x84\x01RaK*\x81\x85aR\xB4V[`\0\x81aXbWaXbaT\xDDV[P`\0\x19\x01\x90V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aX\x87W`\0\x80\xFD[\x88Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aX\x9FW`\0\x80\xFD[aX\xAB\x8C\x83\x8D\x01aVgV[\x99P` \x8B\x01Q\x91P\x80\x82\x11\x15aX\xC1W`\0\x80\xFD[aX\xCD\x8C\x83\x8D\x01aVgV[\x98P`@\x8B\x01Q\x91P\x80\x82\x11\x15aX\xE3W`\0\x80\xFD[aX\xEF\x8C\x83\x8D\x01aVgV[\x97P``\x8B\x01Q\x91P\x80\x82\x11\x15aY\x05W`\0\x80\xFD[aY\x11\x8C\x83\x8D\x01aVgV[\x96P`\x80\x8B\x01Q\x91P\x80\x82\x11\x15aY'W`\0\x80\xFD[PaY4\x8B\x82\x8C\x01aVgV[\x94PP`\xA0\x89\x01Q\x92P`\xC0\x89\x01Q\x91P`\xE0\x89\x01Q\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`@\x81R`\0aYl`@\x83\x01\x85aR\xB4V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aY\x95W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15aY\xAEW`\0\x80\xFD[\x81Qa \x8E\x81aS\xCCV[`\0\x84QaY\xCB\x81\x84` \x89\x01aR\x90V[\x84Q\x90\x83\x01\x90aY\xDF\x81\x83` \x89\x01aR\x90V[\x84Q\x91\x01\x90aY\xF2\x81\x83` \x88\x01aR\x90V[\x01\x95\x94PPPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83QaZ4\x81`\x17\x85\x01` \x88\x01aR\x90V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83QaZe\x81`(\x84\x01` \x88\x01aR\x90V[\x01`(\x01\x94\x93PPPPV[`\0\x82QaZ\x83\x81\x84` \x87\x01aR\x90V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xC7\x9BP*\x85%\xF5\x83\xD1)\xC1Ep\xE1|\xE9\xBC\xA2a\x10\xA5\xC4\x1A\xB7\xE2Uo\x95\xE0\x81\xFE\xC56\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCAddress: low-level delegate call failed\xA2dipfsX\"\x12 \x9B\xD1n\\\x99|\xD2\xF5\xD0\xB8\xBF\x9A\xB3a\xF5\x9Cw\xA2\xDBV\xE2\x8E\x01\x1F\xD2`\xEFr\xEB\x0B\x0B\xECdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static GENERATORREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x02gW`\x005`\xE0\x1C\x80cz\x14\xC4c\x11a\x01DW\x80c\xA2\x17\xFD\xDF\x11a\0\xB6W\x80c\xD5Gt\x1F\x11a\0zW\x80c\xD5Gt\x1F\x14a\x07\xEEW\x80c\xE2\xFA3\xCE\x14a\x08\x0EW\x80c\xE7\xBC\x96\0\x14a\x08.W\x80c\xE9\xE94\xA0\x14a\x08NW\x80c\xEA\xAC\xAE\x94\x14a\x08nW\x80c\xF2\x88\x8D\xBB\x14a\x08\x8EW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x07YW\x80c\xAD\xC9w.\x14a\x07nW\x80c\xC4\x92\xEE9\x14a\x07\x8EW\x80c\xCA\x15\xC8s\x14a\x07\xAEW\x80c\xD0n\x1F{\x14a\x07\xCEW`\0\x80\xFD[\x80c\x91\xD1HT\x11a\x01\x08W\x80c\x91\xD1HT\x14a\x06NW\x80c\x92\xEB\x91\xE2\x14a\x06nW\x80c\x96\xDE\x0E\xEF\x14a\x06\x8EW\x80c\x98*A]\x14a\x06\xAEW\x80c\x9A\x7F\xCA\x8E\x14a\x06\xCEW\x80c\x9F]\xB9\x86\x14a\x079W`\0\x80\xFD[\x80cz\x14\xC4c\x14a\x04\x98W\x80c\x81\xC4\\p\x14a\x05\xB7W\x80c\x84\xAC3\xEC\x14a\x05\xD8W\x80c\x8C\xFCV\xD8\x14a\x05\xF8W\x80c\x90\x10\xD0|\x14a\x06.W`\0\x80\xFD[\x80c6V\x8A\xBE\x11a\x01\xDDW\x80cO\x1E\xF2\x86\x11a\x01\xA1W\x80cO\x1E\xF2\x86\x14a\x04\xEDW\x80cR\xD1\x90-\x14a\x05\0W\x80cT\x1A\x8C\x18\x14a\x05\x15W\x80cdmQ\xB4\x14a\x055W\x80cf\x1D\xE5\xAC\x14a\x05cW\x80cm@Xw\x14a\x05\x97W`\0\x80\xFD[\x80c6V\x8A\xBE\x14a\x04XW\x80c6Y\xCF\xE6\x14a\x04xW\x80c<^\xB5|\x14a\x04\x98W\x80cH\\\xC9U\x14a\x04\xADW\x80cM*\xAB\x9A\x14a\x04\xCDW`\0\x80\xFD[\x80c!\x80\xDE]\x11a\x02/W\x80c!\x80\xDE]\x14a\x03dW\x80c$\x8A\x9C\xA3\x14a\x03\x84W\x80c+a\x0C-\x14a\x03\xC2W\x80c,\x1F\xBD\x03\x14a\x04\x01W\x80c//\xF1]\x14a\x04#W\x80c/\x8FJ;\x14a\x04CW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02lW\x80c\x04y\xD6D\x14a\x02\xA1W\x80c\x08\xBEk\xAD\x14a\x02\xEDW\x80c\x13m\xFB\xF5\x14a\x03\x0FW\x80c\x1C~\xAEe\x14a\x03/W[`\0\x80\xFD[4\x80\x15a\x02xW`\0\x80\xFD[Pa\x02\x8Ca\x02\x876`\x04aN\x9FV[a\x08\xAEV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xADW`\0\x80\xFD[Pa\x02\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x98V[4\x80\x15a\x02\xF9W`\0\x80\xFD[Pa\x03\ra\x03\x086`\x04aN\xC9V[a\x08\xBFV[\0[4\x80\x15a\x03\x1BW`\0\x80\xFD[Pa\x03\ra\x03*6`\x04aP\x18V[a\t\x03V[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x03Oa\x03J6`\x04aP{V[a\x0C\xDCV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x98V[4\x80\x15a\x03pW`\0\x80\xFD[Pa\x03\ra\x03\x7F6`\x04aP\xE9V[a\r~V[4\x80\x15a\x03\x90W`\0\x80\xFD[Pa\x03\xB4a\x03\x9F6`\x04aQYV[`\0\x90\x81R`e` R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x02\x98V[4\x80\x15a\x03\xCEW`\0\x80\xFD[Pa\x03\xE2a\x03\xDD6`\x04aP{V[a\x0F\x0EV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x98V[4\x80\x15a\x04\rW`\0\x80\xFD[Pa\x03\xB4`\0\x80Q` aZ\xA4\x839\x81Q\x91R\x81V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x03\ra\x04>6`\x04aQrV[a\x10\xF5V[4\x80\x15a\x04OW`\0\x80\xFD[Pa\x03\ra\x11\x1AV[4\x80\x15a\x04dW`\0\x80\xFD[Pa\x03\ra\x04s6`\x04aQrV[a\x13\xB0V[4\x80\x15a\x04\x84W`\0\x80\xFD[Pa\x03\ra\x04\x936`\x04aQ\xA2V[a\x14.V[4\x80\x15a\x04\xA4W`\0\x80\xFD[Pa\x03\xB4`d\x81V[4\x80\x15a\x04\xB9W`\0\x80\xFD[Pa\x03\ra\x04\xC86`\x04aQ\xBFV[a\x15\rV[4\x80\x15a\x04\xD9W`\0\x80\xFD[Pa\x03\ra\x04\xE86`\x04aQ\xA2V[a\x16\x86V[a\x03\ra\x04\xFB6`\x04aQ\xEDV[a\x17wV[4\x80\x15a\x05\x0CW`\0\x80\xFD[Pa\x03\xB4a\x18CV[4\x80\x15a\x05!W`\0\x80\xFD[Pa\x03\ra\x0506`\x04aQYV[a\x18\xF6V[4\x80\x15a\x05AW`\0\x80\xFD[Pa\x05Ua\x05P6`\x04aP{V[a\x19\x89V[`@Qa\x02\x98\x92\x91\x90aRuV[4\x80\x15a\x05oW`\0\x80\xFD[Pa\x02\xD5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xA3W`\0\x80\xFD[Pa\x03\ra\x05\xB26`\x04aQYV[a\x1CMV[4\x80\x15a\x05\xC3W`\0\x80\xFD[Pa\x03WTa\x02\xD5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x05\xE4W`\0\x80\xFD[Pa\x03\ra\x05\xF36`\x04aQ\xA2V[a\x1DAV[4\x80\x15a\x06\x04W`\0\x80\xFD[Pa\x06\x18a\x06\x136`\x04aQ\xA2V[a\x1F\x8EV[`@Qa\x02\x98\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aR\xE0V[4\x80\x15a\x06:W`\0\x80\xFD[Pa\x02\xD5a\x06I6`\x04aSEV[a vV[4\x80\x15a\x06ZW`\0\x80\xFD[Pa\x02\x8Ca\x06i6`\x04aQrV[a \x95V[4\x80\x15a\x06zW`\0\x80\xFD[Pa\x03\ra\x06\x896`\x04aP\xE9V[a \xC0V[4\x80\x15a\x06\x9AW`\0\x80\xFD[Pa\x03\ra\x06\xA96`\x04aQYV[a#\xF3V[4\x80\x15a\x06\xBAW`\0\x80\xFD[Pa\x03\ra\x06\xC96`\x04aSgV[a&2V[4\x80\x15a\x06\xDAW`\0\x80\xFD[Pa\x07(a\x06\xE96`\x04aP{V[a\x03T` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T`\xFF\x90\x93\x16\x93\x91\x92\x90\x91\x90\x85V[`@Qa\x02\x98\x95\x94\x93\x92\x91\x90aS\x9CV[4\x80\x15a\x07EW`\0\x80\xFD[Pa\x03\ra\x07T6`\x04aQYV[a'kV[4\x80\x15a\x07eW`\0\x80\xFD[Pa\x03\xB4`\0\x81V[4\x80\x15a\x07zW`\0\x80\xFD[Pa\x03\xB4a\x07\x896`\x04aP{V[a'uV[4\x80\x15a\x07\x9AW`\0\x80\xFD[Pa\x03\ra\x07\xA96`\x04aSgV[a(\xF8V[4\x80\x15a\x07\xBAW`\0\x80\xFD[Pa\x03\xB4a\x07\xC96`\x04aQYV[a*\xF0V[4\x80\x15a\x07\xDAW`\0\x80\xFD[Pa\x03\ra\x07\xE96`\x04aN\xC9V[a+\x07V[4\x80\x15a\x07\xFAW`\0\x80\xFD[Pa\x03\ra\x08\t6`\x04aQrV[a+FV[4\x80\x15a\x08\x1AW`\0\x80\xFD[Pa\x03\ra\x08)6`\x04aS\xDAV[a+kV[4\x80\x15a\x08:W`\0\x80\xFD[Pa\x03\ra\x08I6`\x04aQYV[a/\xEEV[4\x80\x15a\x08ZW`\0\x80\xFD[Pa\x03\ra\x08i6`\x04aQYV[a/\xF8V[4\x80\x15a\x08zW`\0\x80\xFD[Pa\x03\xB4a\x08\x896`\x04aT}V[a1\xF6V[4\x80\x15a\x08\x9AW`\0\x80\xFD[Pa\x03\ra\x08\xA96`\x04aQ\xA2V[a3\x82V[`\0a\x08\xB9\x82a6\x1BV[\x92\x91PPV[`\0[\x81\x81\x10\x15a\x08\xFEWa\x08\xEC3\x84\x84\x84\x81\x81\x10a\x08\xE0Wa\x08\xE0aT\xC7V[\x90P` \x02\x015a6@V[\x80a\x08\xF6\x81aT\xF3V[\x91PPa\x08\xC2V[PPPV[a\t\x0Ba8\xE9V[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\t\x9F\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xCB\x90aU\x0CV[\x80\x15a\n\x18W\x80`\x1F\x10a\t\xEDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x18V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xFBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x82Q`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA3`\xF0\x1B\x81RP\x90a\nkW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[`@Q\x80\x91\x03\x90\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16a\n\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x85a\n\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaG1`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@Q\x80a\x01@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01\x86\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01g\r\xE0\xB6\xB3\xA7d\0\0\x81R` \x01\x84\x81RPa\x03S`\0\x84`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U`@\x82\x01Q\x81`\x02\x01U``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01U`\xE0\x82\x01Q\x81`\x07\x01Ua\x01\0\x82\x01Q\x81`\x08\x01Ua\x01 \x82\x01Q\x81`\t\x01\x90\x81a\x0CF\x91\x90aU\xA7V[PP\x84\x15\x90Pa\x0C\x85Wa\x0C\x85`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x830\x87a9DV[`@\x80Q\x86\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x91\x7FI\x1B\x0FF\x1D\x05\xB7\x9B\xFCi\x04\xA9\xBA\x9D\xF5#\xD7fP?\x11\x04k+^n\xDD\xAA\xE3\xA7#\x1C\x91\x01`@Q\x80\x91\x03\x90\xA2PPa\x0C\xD6`\x01a\x01-UV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03T` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\r(Wa\r(aR=V[`\x04\x81\x11\x15a\r9Wa\r9aR=V[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP[\x92P\x92\x90PV[a\x03WT`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xF1\x91\x90\x81\x01\x90aV\xACV[P\x94PPPPPa\x0E\x01\x84a9\xAFV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a\x0E=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa\x0EJ\x84\x84\x843a9\xE8V[`@Qc\x07\x07Y\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x07\x07Y\x1F\x90a\x0E\x96\x90\x87\x90`\x04\x01aUFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xB0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xC4W=`\0\x80>=`\0\xFD[PPPPa\x0E\xD1\x84a;\x15V[`\x01`\x01`\xA0\x1B\x03\x16\x85\x7F\xC3\xFB\xBD\xB6\xAA\x8D\x99\xF6\xEF\xE2J:\"\xE9\xA9\x9F\xFE\xF2J/9\x9C\x0B\x1ET\x99F\xF9\x1B\xF06\xE4`@Q`@Q\x80\x91\x03\x90\xA3PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03T` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x0FZWa\x0FZaR=V[`\x04\x81\x11\x15a\x0FkWa\x0FkaR=V[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x03S`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01\x80Ta\x10\\\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\x88\x90aU\x0CV[\x80\x15a\x10\xD5W\x80`\x1F\x10a\x10\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xD5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP\x90Q`@\x90\x93\x01Q\x92\x94P\x91\x92PPP\x92P\x92\x90PV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta\x11\x10\x81a;>V[a\x08\xFE\x83\x83a;HV[3`\0\x81\x81Ra\x03S` R`@\x90 `\t\x81\x01\x80Ta\x119\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa\x11qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a\x11\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x08\x01T\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG11`\xE8\x1B\x81RP\x90a\x11\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x06\x01T\x83`\x08\x01Ta\x12\x1F\x91\x90aW#V[a\x12)\x91\x90aW:V[\x90P`\0\x81\x83`\x06\x01Ta\x12=\x91\x90aW\\V[\x90P\x82`\x03\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x90a\x12\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x82`\x02\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG5`\xF0\x1B\x81RP\x90a\x12\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x06\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x08\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03V` R`@\x90 TC\x10\x80\x15\x90a\x13\x19WP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03V` R`@\x90 T\x15\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x90a\x13SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x03V` R`@\x80\x82 \x91\x90\x91UQ\x7F\x11\xDE\xAE(\x9Epx\xFC\xE2\x88\xF4\xE9sN\x9C=\xF6{U\xEA\xC4\xFFA\xB8\x90\xB6\xD68\x1A\xCE\xE7b\x90a\x13\xA2\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x14 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\nbV[a\x14*\x82\x82a;RV[PPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x14vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x90aWoV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x14\xBF`\0\x80Q` aZ\xC4\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x90aW\xBBV[a\x14\xEE\x81a;\x9DV[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92Ra\x15\n\x91\x83\x91\x90a;\xA8V[PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x15-WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x15GWP0;\x15\x80\x15a\x15GWP`\0T`\xFF\x16`\x01\x14[a\x15\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x15\xCDW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x15\xD5a=\x13V[a\x15\xDDa=\x13V[a\x15\xE5a=\x13V[a\x15\xEDa=\x13V[a\x15\xF5a=\x13V[a\x15\xFDa=\x13V[a\x16\x08`\0\x84a=\x80V[a\x16 `\0\x80Q` aZ\xA4\x839\x81Q\x91R\x83a;HV[a\x03W\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x08\xFEW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16a\x16\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P3`\0\x81\x81Ra\x03S` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x17\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x83U`@Q\x91\x82R\x83\x16\x90\x7F\xB1\x95\xA9K\xEC\xD3\x88\xC9\xA0\x7F\xE8\x18qh3\xBD\xF9\x8Bu\\x\xC9\xB48\xF4\xC8\xF0o5O\xA3h\x90` \x01[`@Q\x80\x91\x03\x90\xA2PPPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a\x17\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x90aWoV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x18\x08`\0\x80Q` aZ\xC4\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x90aW\xBBV[a\x187\x82a;\x9DV[a\x14*\x82\x82`\x01a;\xA8V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FUUPSUpgradeable: must not be cal`D\x82\x01R\x7Fled through delegatecall\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nbV[P`\0\x80Q` aZ\xC4\x839\x81Q\x91R\x90V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xED8\r\x033`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x82W=`\0\x80>=`\0\xFD[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03T` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x83\x92\x83\x92\x90\x91\x82\x90`\xFF\x16`\x04\x81\x11\x15a\x19\xD5Wa\x19\xD5aR=V[`\x04\x81\x11\x15a\x19\xE6Wa\x19\xE6aR=V[\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81RPP\x90P`\0a\x03S`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80a\x01@\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81R` \x01`\x04\x82\x01T\x81R` \x01`\x05\x82\x01T\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81R` \x01`\x08\x82\x01T\x81R` \x01`\t\x82\x01\x80Ta\x1A\xD7\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B\x03\x90aU\x0CV[\x80\x15a\x1BPW\x80`\x1F\x10a\x1B%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1BPV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0`\x04\x81\x11\x15a\x1BnWa\x1BnaR=V[\x82Q`\x04\x81\x11\x15a\x1B\x81Wa\x1B\x81aR=V[\x03a\x1B\x94W`\0\x80\x93P\x93PPPa\rwV[`\x04\x82Q`\x04\x81\x11\x15a\x1B\xA9Wa\x1B\xA9aR=V[\x03a\x1B\xBDW`\x04`\0\x93P\x93PPPa\rwV[`\0a\x1B\xC8\x87a=\x8AV[\x90P`\0\x83Q`\x04\x81\x11\x15a\x1B\xDFWa\x1B\xDFaR=V[\x14\x15\x80\x15a\x1B\xEBWP\x80\x15[\x15a\x1C\0W`\x02`\0\x94P\x94PPPPa\rwV[\x81`\xC0\x01Q\x81\x03a\x1C\x19W`\x01\x94P\x92Pa\rw\x91PPV[\x80\x15\x80\x15\x90a\x1C+WP\x81`\xC0\x01Q\x81\x10[\x15a\x1C>W`\x03\x94P\x92Pa\rw\x91PPV[P`\0\x96\x87\x96P\x94PPPPPV[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1C\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80`\t\x01\x80Ta\x1C\xB4\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa\x1C\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x82\x81`\x06\x01`\0\x82\x82Ta\x1D\x01\x91\x90aX\x07V[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F \x98X\xCE\xC2\x8CQ\x8A\xEAD\xAA?\xA7\n\x93\"S\xF8\xBD\xF7j\x1C\x9A\x0B\x08\"j\x13\xE6\x975\xA3\x90` \x01a\x17jV[a\x1DIa8\xE9V[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a\x1D\xDD\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\t\x90aU\x0CV[\x80\x15a\x1EVW\x80`\x1F\x10a\x1E+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1EVV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80`@\x01Q`\0\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG3`\xF0\x1B\x81RP\x90a\x1E\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P` \x81\x01Qa\x1E\xDE\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x85\x90a?\x02V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x03S` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U`\x02\x81\x01\x82\x90U`\x03\x81\x01\x82\x90U`\x04\x81\x01\x82\x90U`\x05\x81\x01\x82\x90U`\x06\x81\x01\x82\x90U`\x07\x81\x01\x82\x90U`\x08\x81\x01\x82\x90U\x90a\x1FK`\t\x83\x01\x82aNQV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x9F70\xAD\xE9K\xE5\xCE?\xAD\x97;\x88\x8At\x86j:\x91]\0\x8E\x8C\xBD\xE2\x13\x82\xB91\xB6|c\x90`\0\x90\xA2PPa\x15\n`\x01a\x01-UV[a\x03S` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x87\x01T`\x07\x88\x01T`\x08\x89\x01T`\t\x8A\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x9A\x16\x9A\x98\x99\x97\x98\x96\x97\x95\x96\x94\x95\x93\x94\x92\x93\x91\x92\x91a\x1F\xF3\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \x1F\x90aU\x0CV[\x80\x15a lW\x80`\x1F\x10a AWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a lV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a OW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x8AV[`\0\x82\x81R`\x97` R`@\x81 a \x8E\x90\x83a?2V[\x93\x92PPPV[`\0\x91\x82R`e` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x81\x01T`\xC0\x83\x01R`\x07\x81\x01T`\xE0\x83\x01R`\x08\x81\x01Ta\x01\0\x83\x01R`\t\x81\x01\x80Ta\x01 \x84\x01\x91\x90a!T\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!\x80\x90aU\x0CV[\x80\x15a!\xCDW\x80`\x1F\x10a!\xA2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xCDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\xB0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPa\x03WT`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R\x92\x93P`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"O\x91\x90\x81\x01\x90aV\xACV[PPPP\x91PP`\0\x80\x1B\x81\x14\x15\x80a\"\x88WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x81\x14\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bG15`\xE8\x1B\x81RP\x90a\"\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa\"\xCC\x86a9\xAFV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a#\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x81Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a#KW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0a#W\x87a?>V[\x90Pa#e\x87\x87\x873a9\xE8V[`@Qc4\xFE\xDEe`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\xFD\xBC\xCA\x90a#\xB7\x90\x87\x90\x8C\x90\x86\x90\x8D\x90`\x04\x01aX\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\xE5W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a$JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80`\t\x01\x80Ta$Z\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa$\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x08\x01T\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x90a$\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA7`\xF0\x1B` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a%\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x06\x01T\x85a%:\x91\x90aW#V[a%D\x91\x90aW:V[\x90P\x81`\x02\x01T\x81\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA7`\xF0\x1B\x81RP\x90a%\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80\x82`\x06\x01Ta%\x98\x91\x90aW\\V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90a%\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x08\x82\x01\x84\x90Ua%\xE2`\x01CaX\x07V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x03V` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7Fv\x12,\xFD_<h\0\xA2#\xE6\x0C\xC6$G\xC6I6\x03}x\xD0-\x91\x92\x93\xF6U\xBA}T\xCB\x90a\x13\xA2\x90\x87\x81R` \x01\x90V[`\0\x80Q` aZ\xA4\x839\x81Q\x91Ra&J\x81a;>V[`\0a&V\x85\x85a\x19\x89V[P\x90P`\x03\x81`\x04\x81\x11\x15a&mWa&maR=V[\x14\x80a&\x8AWP`\x04\x81`\x04\x81\x11\x15a&\x88Wa&\x88aR=V[\x14[\x80a&\xA6WP`\x02\x81`\x04\x81\x11\x15a&\xA4Wa&\xA4aR=V[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a#\x9B`\xF1\x1B\x81RP\x90a&\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 a\x03T\x83R\x81\x84 \x88\x85R\x90\x92R\x82 `\x01\x81\x01T`\x03\x83\x01\x80T\x93\x94\x92\x93\x91\x92\x83\x92a'*\x90\x84\x90aW\\V[\x92PP\x81\x90UP\x85\x83`\x04\x01`\0\x82\x82Ta'E\x91\x90aW\\V[\x90\x91UPP`\x04\x82\x01\x80T\x90`\0a'\\\x83aXSV[\x91\x90PUPPPPPPPPPV[a\x15\n3\x82a6@V[`\0a'\x7Fa8\xE9V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03S` R`@\x90 `\t\x81\x01\x80Ta'\xA7\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa'\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a(\"W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x83a(ZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa(\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x1630\x86a9DV[\x82\x81`\x01\x01`\0\x82\x82Ta(\xA4\x91\x90aX\x07V[\x90\x91UPP`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xB6\xBE#\x16\x85\x06\xA1\xCEb\xCCo\x933\x9Fg\x10\xE0\x98De\x17\n\xB5-\xF8\xEC\xF7\xDA8\xB3E\x84\x90` \x01`@Q\x80\x91\x03\x90\xA2`\x01\x01T\x90Pa\x08\xB9`\x01a\x01-UV[a)\0a8\xE9V[`\0\x80Q` aZ\xA4\x839\x81Q\x91Ra)\x18\x81a;>V[`\0\x80a)%\x86\x86a\x19\x89V[\x90\x92P\x90P`\x01\x82`\x04\x81\x11\x15a)>Wa)>aR=V[\x14\x80a)[WP`\x03\x82`\x04\x81\x11\x15a)YWa)YaR=V[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x11\xCD`\xF2\x1B\x81RP\x90a)\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 a\x03T\x83R\x81\x84 \x89\x85R\x83R\x92\x81\x90 `\x01\x81\x01T\x82Q\x80\x84\x01\x90\x93R`\x02\x83RaG5`\xF0\x1B\x93\x83\x01\x93\x90\x93R\x91\x84\x10\x15a*\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`d\x81`\x04\x01T\x11\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b#\x98\x99`\xE9\x1B\x81RP\x90a*EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0a*Q\x89a?eV[\x90P\x86\x81\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x90a*\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x86\x83`\x04\x01`\0\x82\x82Ta*\xA5\x91\x90aX\x07V[\x90\x91UPP`\x01\x82\x01T`\x03\x84\x01\x80T`\0\x90a*\xC3\x90\x84\x90aX\x07V[\x90\x91UPP`\x04\x82\x01\x80T\x90`\0a*\xDA\x83aT\xF3V[\x91\x90PUPPPPPPPa\x08\xFE`\x01a\x01-UV[`\0\x81\x81R`\x97` R`@\x81 a\x08\xB9\x90a@\xD4V[`\0[\x81\x81\x10\x15a\x08\xFEWa+43\x84\x84\x84\x81\x81\x10a+(Wa+(aT\xC7V[\x90P` \x02\x015a@\xDEV[\x80a+>\x81aT\xF3V[\x91PPa+\nV[`\0\x82\x81R`e` R`@\x90 `\x01\x01Ta+a\x81a;>V[a\x08\xFE\x83\x83a;RV[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x80\x83 a\x03T\x83R\x81\x84 \x8D\x85R\x90\x92R\x80\x83 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x92\x93\x92\x82\x90`\xFF\x16`\x04\x81\x11\x15a+\xB3Wa+\xB3aR=V[`\x04\x81\x11\x15a+\xC4Wa+\xC4aR=V[\x81R`\x01\x82\x01T` \x80\x83\x01\x91\x90\x91R`\x02\x80\x84\x01T`@\x80\x85\x01\x91\x90\x91R`\x03\x85\x01T``\x85\x01R`\x04\x90\x94\x01T`\x80\x90\x93\x01\x92\x90\x92R\x85T\x83Q\x80\x85\x01\x90\x94R\x91\x83Ra#\x99`\xF1\x1B\x90\x83\x01R\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16a,=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0\x80a,J\x8DaA\xCDV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaM1`\xF0\x1B` \x82\x01R\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16a,\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0\x83Q`\x04\x81\x11\x15a,\xA5Wa,\xA5aR=V[\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a&\x99`\xF1\x1B\x81RP\x90a,\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x8Aa-\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x8Ca-OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x8B\x84`\x02\x01`\0\x82\x82Ta-d\x91\x90aX\x07V[\x92PP\x81\x90UP\x83`\x06\x01T\x84`\x02\x01T\x11\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aM3`\xF0\x1B\x81RP\x90a-\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x05\x84\x01\x80T\x90`\0a-\xC3\x83aT\xF3V[\x90\x91UPP`@\x80Q`\xA0\x81\x01\x90\x91R\x80`\x01\x81R` \x01\x8D\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01`\0\x81RPa\x03T`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8F\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x04\x81\x11\x15a.SWa.SaR=V[\x02\x17\x90UP` \x82\x01Q`\x01\x82\x01U`@\x82\x01Q`\x02\x82\x01U``\x82\x01Q`\x03\x82\x01U`\x80\x90\x91\x01Q`\x04\x90\x91\x01U\x80\x15\x80\x15\x90a.\xB1WP\x7F\xCD.f\xBF\x0B\x91\xEE\xED\xC6\xC6H\xAE\x935\xA7\x8D|\x9AJ\xB0\xEF3a*\x82M\x91\xCD\xC6\x8AO!\x81\x14\x15[\x15a/\x9BWa.\xBF\x88a9\xAFV[\x81\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bA11`\xE8\x1B\x81RP\x90a.\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x88\x15a/\x9BWa/\x0E\x88\x88\x88\x88a9\xE8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ci\xFD\xBC\xCA\x86\x8Fa/H\x8Ca?>V[\x8C`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/h\x94\x93\x92\x91\x90aX\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a/\x96W=`\0\x80>=`\0\xFD[PPPP[\x8C\x85`\x01`\x01`\xA0\x1B\x03\x16\x7F\x04Y\xABWU\x08\xB4\xA5\x89O{\x13\x87\xBF6-\x03!;;\xF81\xDAE&!\x8C3\xE4\xA9\x06\xD8\x8E`@Qa/\xD7\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPPV[a\x15\n3\x82a@\xDEV[3`\0\x81\x81Ra\x03S` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x80\x85\x01\x90\x94R`\x02\x84RaA3`\xF0\x1B\x92\x84\x01\x92\x90\x92R\x91\x90`\x01`\x01`\xA0\x1B\x03\x16a0OW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80`\t\x01\x80Ta0_\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x91Pa0\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x07\x01T\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bTR5`\xE8\x1B\x81RP\x90a0\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA7`\xF0\x1B` \x82\x01Rg\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a1#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x07\x81\x01\x83\x90U`\x01\x81\x01T`\0\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90a1G\x90\x86aW#V[a1Q\x91\x90aW:V[\x90P\x80\x82`\x01\x01Ta1c\x91\x90aW\\V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaA3`\xF0\x1B` \x82\x01R\x90a1\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa1\xA6`\x01CaX\x07V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x03U` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x7F\x13\x9A\xD7\xA0\xC3\xF6\xC0\xAD\x0F\x0F\xC5+vx`jq\xFF\x08\x1B\x99&\xA5do\xE5:\x8Eg\xC6\x8Ar\x90a\x13\xA2\x90\x87\x81R` \x01\x90V[`\0`\0\x80Q` aZ\xA4\x839\x81Q\x91Ra2\x10\x81a;>V[`\0a2\x1C\x87\x87a\x19\x89V[P\x90P`\x03\x81`\x04\x81\x11\x15a23Wa23aR=V[\x14\x80a2PWP`\x04\x81`\x04\x81\x11\x15a2NWa2NaR=V[\x14[\x80a2lWP`\x02\x81`\x04\x81\x11\x15a2jWa2jaR=V[\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x10M`\xF2\x1B\x81RP\x90a2\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 a\x03T\x83R\x81\x84 \x8A\x85R\x90\x92R\x82 `\x04\x81\x01\x80T\x92\x93\x91\x92\x91a2\xE6\x83aXSV[\x91\x90PUP\x86\x82`\x01\x01`\0\x82\x82Ta2\xFF\x91\x90aW\\V[\x92PP\x81\x90UP\x86\x82`\x04\x01`\0\x82\x82Ta3\x1A\x91\x90aW\\V[\x90\x91UPP`\x01\x81\x01T`\x03\x83\x01\x80T`\0\x90a38\x90\x84\x90aW\\V[\x90\x91UPa3r\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x89a?\x02V[P`\x01\x01T\x97\x96PPPPPPPV[a3\x8Aa8\xE9V[3`\0\x81\x81Ra\x03S` R`@\x90 `\t\x81\x01\x80Ta3\xA9\x90aU\x0CV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x91Pa3\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra#\x99`\xF1\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x16a4$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x07\x01T\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04s\x13`\xEC\x1B\x81RP\x90a4oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\x01\x01T\x83`\x07\x01Ta4\x8F\x91\x90aW#V[a4\x99\x91\x90aW:V[\x90P`\0\x81\x83`\x01\x01Ta4\xAD\x91\x90aW\\V[\x90P\x82`\x04\x01T\x82\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aA5`\xF0\x1B\x81RP\x90a4\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[Pa5%`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86\x83a?\x02V[`\x01\x83\x01\x82\x90Ug\r\xE0\xB6\xB3\xA7d\0\0`\x07\x84\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03U` R`@\x90 TC\x10\x80\x15\x90a5{WP`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81Ra\x03U` R`@\x90 T\x15\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x11\xCCM`\xEA\x1B\x81RP\x90a5\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x81\x81Ra\x03U` R`@\x80\x82 \x91\x90\x91UQ\x7F\xE5+<X\xA1\x16\xC1\xF0\x12\xC9\x9D\x11 \xC0T\xE7?Q\xB7\xA2\x9CEq\x92E_\xD2\xC2\x0E\x03\xA53\x90a6\x04\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPa\x15\n`\x01a\x01-UV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x08\xB9WPa\x08\xB9\x82aBYV[a\x03WT`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6\xB3\x91\x90\x81\x01\x90aV\xACV[PP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaM1`\xF0\x1B` \x82\x01R\x93\x94PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16a6\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x03T` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x80\x82 \x81Q`\xA0\x81\x01\x90\x92R\x80T\x82\x90`\xFF\x16`\x04\x81\x11\x15a7BWa7BaR=V[`\x04\x81\x11\x15a7SWa7SaR=V[\x81R`\x01\x82\x01T` \x82\x01R`\x02\x82\x01T`@\x82\x01R`\x03\x82\x01T``\x82\x01R`\x04\x90\x91\x01T`\x80\x90\x91\x01R\x90P`\0\x81Q`\x04\x81\x11\x15a7\x96Wa7\x96aR=V[\x14\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG9`\xF0\x1B\x81RP\x90a7\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x80\x81\x01Q`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x13M`\xF2\x1B` \x82\x01R\x90\x15a8\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\0a\x03S`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x90P\x81` \x01Q\x81`\x02\x01`\0\x82\x82Ta8R\x91\x90aW\\V[\x92PP\x81\x90UP`\x01\x81`\x05\x01`\0\x82\x82Ta8n\x91\x90aW\\V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x81\x81Ra\x03T` \x90\x81R`@\x80\x83 \x88\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x81\x01\x83\x90U`\x03\x81\x01\x83\x90U`\x04\x01\x82\x90UQ\x86\x92\x91\x7F\x83\x1C\xD5\xB7S\x83\x80\xB0\xA2\xA3\x14\x14\xD64\xECBq\x16\x07V\x84\xA2v\x82\x8A\xB4\xD2ut\xA2\xDF\xDF\x91\xA3PPPPPV[`\x02a\x01-T\x03a9<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\nbV[`\x02a\x01-UV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x0C\xD6\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91RaB\x8EV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a9\xC9\x91\x90aXjV[PPP\x94P\x94P\x94PPPa9\xDF\x83\x83\x83aCcV[\x95\x94PPPPPV[`\0\x84\x82`@Q` \x01a9\xFD\x92\x91\x90aYYV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a:n\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90P`\0a:\xB2\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaC\x9C\x92PPPV[\x90Pa:\xBD\x87a;\x15V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x13\x13`\xEC\x1B\x81RP\x90a;\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[PPPPPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a;,\x91\x90aXjV[PPPPPP\x91PPa \x8E\x81aC\xC0V[a\x15\n\x813aD\rV[a\x14*\x82\x82aDfV[a;\\\x82\x82aD\x88V[a;f`\0a*\xF0V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra \x99`\xF1\x1B` \x82\x01R\x90a\x08\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[`\0a\x14*\x81a;>V[\x7FI\x10\xFD\xFA\x16\xFE\xD3&\x0E\xD0\xE7\x14\x7F|\xC6\xDA\x11\xA6\x02\x08\xB5\xB9@m\x12\xA65aO\xFD\x91CT`\xFF\x16\x15a;\xDBWa\x08\xFE\x83aD\xAAV[\x82`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a<5WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra<2\x91\x81\x01\x90aY\x83V[`\x01[a<\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC1967Upgrade: new implementati`D\x82\x01Rmon is not UUPS`\x90\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80Q` aZ\xC4\x839\x81Q\x91R\x81\x14a=\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FERC1967Upgrade: unsupported prox`D\x82\x01Rh\x1AXX\x9B\x19UURQ`\xBA\x1B`d\x82\x01R`\x84\x01a\nbV[Pa\x08\xFE\x83\x83\x83aEFV[`\0Ta\x01\0\x90\x04`\xFF\x16a=~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\nbV[V[a\x14*\x82\x82a;HV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01T`\xC0\x84\x01R`\x07\x81\x01T`\xE0\x84\x01R`\x08\x81\x01Ta\x01\0\x84\x01R`\t\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91a\x01 \x84\x01\x91a>%\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta>Q\x90aU\x0CV[\x80\x15a>\x9EW\x80`\x1F\x10a>sWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a>\x9EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a>\x81W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x01\0\x01Q\x83`\xC0\x01Qa>\xC9\x91\x90aW#V[a>\xD3\x91\x90aW:V[\x90P\x81``\x01Q\x81\x10\x15a>\xEBWP`\0\x93\x92PPPV[``\x82\x01Qa>\xFA\x90\x82aW\\V[\x94\x93PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x08\xFE\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a9xV[`\0a \x8E\x83\x83aEkV[```\0\x82\x80` \x01\x90Q\x81\x01\x90a?V\x91\x90aXjV[P\x94\x99\x98PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81Ra\x03S` \x90\x81R`@\x80\x83 \x81Qa\x01@\x81\x01\x83R\x81T\x90\x95\x16\x85R`\x01\x81\x01T\x92\x85\x01\x92\x90\x92R`\x02\x82\x01T\x90\x84\x01R`\x03\x81\x01T``\x84\x01R`\x04\x81\x01T`\x80\x84\x01R`\x05\x81\x01T`\xA0\x84\x01R`\x06\x81\x01T`\xC0\x84\x01R`\x07\x81\x01T`\xE0\x84\x01R`\x08\x81\x01Ta\x01\0\x84\x01R`\t\x81\x01\x80T\x92\x93\x84\x93\x90\x92\x91a\x01 \x84\x01\x91a@\0\x90aU\x0CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta@,\x90aU\x0CV[\x80\x15a@yW\x80`\x1F\x10a@NWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a@yV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a@\\W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0g\r\xE0\xB6\xB3\xA7d\0\0\x82`\xE0\x01Q\x83` \x01Qa@\xA3\x91\x90aW#V[a@\xAD\x91\x90aW:V[\x90P\x81`\x80\x01Q\x81\x10\x15a@\xC5WP`\0\x93\x92PPPV[`\x80\x82\x01Qa>\xFA\x90\x82aW\\V[`\0a\x08\xB9\x82T\x90V[`\0a@\xEA\x83\x83a\x19\x89V[P\x90P`\0\x81`\x04\x81\x11\x15aA\x01WaA\x01aR=V[\x14\x15\x80\x15aA!WP`\x04\x81`\x04\x81\x11\x15aA\x1EWaA\x1EaR=V[\x14\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x08\xE7`\xF3\x1B\x81RP\x90aAZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81Ra\x03T` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x80\x82 \x80T`\xFF\x19\x16`\x04\x17\x81U\x90Q\x90\x92\x85\x92\x90\x91\x7F\xF9\xE1yp\xDFW\xA6\xA8E}\xCB\xB5\xC2\x91gkF1\xD37\xCFv\xB0\xC8\x01\xF6\xB8\xADj|_\x92\x91\x90\xA3\x80`\x04\x01T`\0\x03a\x0C\xD6Wa\x0C\xD6\x84\x84a6@V[a\x03WT`@Qc\xF8\xA9H/`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA9H/\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaBH\x91\x90\x81\x01\x90aV\xACV[P\x93\x99\x92\x98P\x91\x96PPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x08\xB9WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x08\xB9V[`\0aB\xE3\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aE\x95\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80aC\x04WP\x80\x80` \x01\x90Q\x81\x01\x90aC\x04\x91\x90aY\x9CV[a\x08\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80\x84\x84\x84`@Q` \x01aC{\x93\x92\x91\x90aY\xB9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x95\x94PPPPPV[`\0\x80`\0aC\xAB\x85\x85aE\xA4V[\x91P\x91PaC\xB8\x81aE\xE6V[P\x93\x92PPPV[`\0\x81Q`@\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aG7`\xF0\x1B\x81RP\x90aD\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[PP\x80Q` \x90\x91\x01 \x90V[aD\x17\x82\x82a \x95V[a\x14*WaD$\x81aG0V[aD/\x83` aGBV[`@Q` \x01aD@\x92\x91\x90aY\xFCV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\nb\x91`\x04\x01aUFV[aDp\x82\x82aH\xDEV[`\0\x82\x81R`\x97` R`@\x90 a\x08\xFE\x90\x82aIdV[aD\x92\x82\x82aIyV[`\0\x82\x81R`\x97` R`@\x90 a\x08\xFE\x90\x82aI\xE0V[`\x01`\x01`\xA0\x1B\x03\x81\x16;aE\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC1967: new implementation is n`D\x82\x01Rl\x1B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80Q` aZ\xC4\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[aEO\x83aI\xF5V[`\0\x82Q\x11\x80aE\\WP\x80[\x15a\x08\xFEWa\x0C\xD6\x83\x83aJ5V[`\0\x82`\0\x01\x82\x81T\x81\x10aE\x82WaE\x82aT\xC7V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``a>\xFA\x84\x84`\0\x85aJZV[`\0\x80\x82Q`A\x03aE\xDAW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaE\xCE\x87\x82\x85\x85aK5V[\x94P\x94PPPPa\rwV[P`\0\x90P`\x02a\rwV[`\0\x81`\x04\x81\x11\x15aE\xFAWaE\xFAaR=V[\x03aF\x02WPV[`\x01\x81`\x04\x81\x11\x15aF\x16WaF\x16aR=V[\x03aFcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\nbV[`\x02\x81`\x04\x81\x11\x15aFwWaFwaR=V[\x03aF\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\nbV[`\x03\x81`\x04\x81\x11\x15aF\xD8WaF\xD8aR=V[\x03a\x15\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\nbV[``a\x08\xB9`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0aGQ\x83`\x02aW#V[aG\\\x90`\x02aX\x07V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aGtWaGtaOSV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aG\x9EW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10aG\xB9WaG\xB9aT\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10aG\xE8WaG\xE8aT\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0aH\x0C\x84`\x02aW#V[aH\x17\x90`\x01aX\x07V[\x90P[`\x01\x81\x11\x15aH\x8FWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10aHKWaHKaT\xC7V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10aHaWaHaaT\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93aH\x88\x81aXSV[\x90PaH\x1AV[P\x83\x15a \x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\nbV[aH\xE8\x82\x82a \x95V[a\x14*W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UaI 3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a \x8E\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aK\xF9V[aI\x83\x82\x82a \x95V[\x15a\x14*W`\0\x82\x81R`e` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a \x8E\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aLHV[aI\xFE\x81aD\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[``a \x8E\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01aZ\xE4`'\x919aM;V[``\x82G\x10\x15aJ\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\nbV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaJ\xD7\x91\x90aZqV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aK\x14W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aK\x19V[``\x91P[P\x91P\x91PaK*\x87\x83\x83\x87aM\xB3V[\x97\x96PPPPPPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aKlWP`\0\x90P`\x03aK\xF0V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aK\xC0W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aK\xE9W`\0`\x01\x92P\x92PPaK\xF0V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 TaL@WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x08\xB9V[P`\0a\x08\xB9V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aM1W`\0aLl`\x01\x83aW\\V[\x85T\x90\x91P`\0\x90aL\x80\x90`\x01\x90aW\\V[\x90P\x81\x81\x14aL\xE5W`\0\x86`\0\x01\x82\x81T\x81\x10aL\xA0WaL\xA0aT\xC7V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10aL\xC3WaL\xC3aT\xC7V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aL\xF6WaL\xF6aZ\x8DV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x08\xB9V[`\0\x91PPa\x08\xB9V[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@QaMX\x91\x90aZqV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14aM\x93W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aM\x98V[``\x91P[P\x91P\x91PaM\xA9\x86\x83\x83\x87aM\xB3V[\x96\x95PPPPPPV[``\x83\x15aN\"W\x82Q`\0\x03aN\x1BW`\x01`\x01`\xA0\x1B\x03\x85\x16;aN\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\nbV[P\x81a>\xFAV[a>\xFA\x83\x83\x81Q\x15aN7W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nb\x91\x90aUFV[P\x80TaN]\x90aU\x0CV[`\0\x82U\x80`\x1F\x10aNmWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x15\n\x91\x90[\x80\x82\x11\x15aN\x9BW`\0\x81U`\x01\x01aN\x87V[P\x90V[`\0` \x82\x84\x03\x12\x15aN\xB1W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a \x8EW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aN\xDCW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aN\xF4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aO\x08W`\0\x80\xFD[\x815\x81\x81\x11\x15aO\x17W`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15aO,W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\nW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aO\x92WaO\x92aOSV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aO\xB4WaO\xB4aOSV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aO\xD3W`\0\x80\xFD[\x815aO\xE6aO\xE1\x82aO\x9AV[aOiV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aO\xFBW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aP.W`\0\x80\xFD[\x845aP9\x81aO>V[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aPcW`\0\x80\xFD[aPo\x87\x82\x88\x01aO\xC2V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15aP\x8EW`\0\x80\xFD[\x825aP\x99\x81aO>V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x83`\x1F\x84\x01\x12aP\xB9W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\xD1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\rwW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aP\xFFW`\0\x80\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aQ\x1EW`\0\x80\xFD[aQ*\x88\x83\x89\x01aO\xC2V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15aQ@W`\0\x80\xFD[PaQM\x87\x82\x88\x01aP\xA7V[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15aQkW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aQ\x85W`\0\x80\xFD[\x825\x91P` \x83\x015aQ\x97\x81aO>V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aQ\xB4W`\0\x80\xFD[\x815a \x8E\x81aO>V[`\0\x80`@\x83\x85\x03\x12\x15aQ\xD2W`\0\x80\xFD[\x825aQ\xDD\x81aO>V[\x91P` \x83\x015aQ\x97\x81aO>V[`\0\x80`@\x83\x85\x03\x12\x15aR\0W`\0\x80\xFD[\x825aR\x0B\x81aO>V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR'W`\0\x80\xFD[aR3\x85\x82\x86\x01aO\xC2V[\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x05\x81\x10aRqWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01aR\x83\x82\x85aRSV[\x82` \x83\x01R\x93\x92PPPV[`\0[\x83\x81\x10\x15aR\xABW\x81\x81\x01Q\x83\x82\x01R` \x01aR\x93V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaR\xCC\x81` \x86\x01` \x86\x01aR\x90V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x8D\x16\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x88`\x80\x84\x01R\x87`\xA0\x84\x01R\x86`\xC0\x84\x01R\x85`\xE0\x84\x01R\x84a\x01\0\x84\x01R\x80a\x01 \x84\x01RaS4\x81\x84\x01\x85aR\xB4V[\x9D\x9CPPPPPPPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aSXW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15aS|W`\0\x80\xFD[\x835aS\x87\x81aO>V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xA0\x81\x01aS\xAA\x82\x88aRSV[\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01R\x82`\x80\x83\x01R\x96\x95PPPPPPV[\x80\x15\x15\x81\x14a\x15\nW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15aS\xF6W`\0\x80\xFD[\x885\x97P` \x89\x015\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015aT\x1D\x81aS\xCCV[\x93P`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aT:W`\0\x80\xFD[aTF\x8C\x83\x8D\x01aO\xC2V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15aT\\W`\0\x80\xFD[PaTi\x8B\x82\x8C\x01aP\xA7V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aT\x93W`\0\x80\xFD[\x845aT\x9E\x81aO>V[\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015aT\xBC\x81aO>V[\x93\x96\x92\x95P\x90\x93PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aU\x05WaU\x05aT\xDDV[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aU W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aU@WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x81R`\0a \x8E` \x83\x01\x84aR\xB4V[`\x1F\x82\x11\x15a\x08\xFEW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aU\x80WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aU\x9FW\x82\x81U`\x01\x01aU\x8CV[PPPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xC1WaU\xC1aOSV[aU\xD5\x81aU\xCF\x84TaU\x0CV[\x84aUYV[` \x80`\x1F\x83\x11`\x01\x81\x14aV\nW`\0\x84\x15aU\xF2WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85UaU\x9FV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aV9W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aV\x1AV[P\x85\x82\x10\x15aVWW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82`\x1F\x83\x01\x12aVxW`\0\x80\xFD[\x81QaV\x86aO\xE1\x82aO\x9AV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aV\x9BW`\0\x80\xFD[a>\xFA\x82` \x83\x01` \x87\x01aR\x90V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aV\xC5W`\0\x80\xFD[\x86QaV\xD0\x81aO>V[\x80\x96PP` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\nW`\0\x80\xFD[aW\x16\x89\x82\x8A\x01aVgV[\x91PP\x92\x95P\x92\x95P\x92\x95V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08\xB9Wa\x08\xB9aT\xDDV[`\0\x82aWWWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x08\xB9Wa\x08\xB9aT\xDDV[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rk\x19\x19[\x19Y\xD8]\x19X\xD8[\x1B`\xA2\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FFunction must be called through `@\x82\x01Rkactive proxy`\xA0\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x08\xB9Wa\x08\xB9aT\xDDV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0aXA`\x80\x83\x01\x85aR\xB4V[\x82\x81\x03``\x84\x01RaK*\x81\x85aR\xB4V[`\0\x81aXbWaXbaT\xDDV[P`\0\x19\x01\x90V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aX\x87W`\0\x80\xFD[\x88Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aX\x9FW`\0\x80\xFD[aX\xAB\x8C\x83\x8D\x01aVgV[\x99P` \x8B\x01Q\x91P\x80\x82\x11\x15aX\xC1W`\0\x80\xFD[aX\xCD\x8C\x83\x8D\x01aVgV[\x98P`@\x8B\x01Q\x91P\x80\x82\x11\x15aX\xE3W`\0\x80\xFD[aX\xEF\x8C\x83\x8D\x01aVgV[\x97P``\x8B\x01Q\x91P\x80\x82\x11\x15aY\x05W`\0\x80\xFD[aY\x11\x8C\x83\x8D\x01aVgV[\x96P`\x80\x8B\x01Q\x91P\x80\x82\x11\x15aY'W`\0\x80\xFD[PaY4\x8B\x82\x8C\x01aVgV[\x94PP`\xA0\x89\x01Q\x92P`\xC0\x89\x01Q\x91P`\xE0\x89\x01Q\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`@\x81R`\0aYl`@\x83\x01\x85aR\xB4V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aY\x95W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15aY\xAEW`\0\x80\xFD[\x81Qa \x8E\x81aS\xCCV[`\0\x84QaY\xCB\x81\x84` \x89\x01aR\x90V[\x84Q\x90\x83\x01\x90aY\xDF\x81\x83` \x89\x01aR\x90V[\x84Q\x91\x01\x90aY\xF2\x81\x83` \x88\x01aR\x90V[\x01\x95\x94PPPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83QaZ4\x81`\x17\x85\x01` \x88\x01aR\x90V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83QaZe\x81`(\x84\x01` \x88\x01aR\x90V[\x01`(\x01\x94\x93PPPPV[`\0\x82QaZ\x83\x81\x84` \x87\x01aR\x90V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xC7\x9BP*\x85%\xF5\x83\xD1)\xC1Ep\xE1|\xE9\xBC\xA2a\x10\xA5\xC4\x1A\xB7\xE2Uo\x95\xE0\x81\xFE\xC56\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCAddress: low-level delegate call failed\xA2dipfsX\"\x12 \x9B\xD1n\\\x99|\xD2\xF5\xD0\xB8\xBF\x9A\xB3a\xF5\x9Cw\xA2\xDBV\xE2\x8E\x01\x1F\xD2`\xEFr\xEB\x0B\x0B\xECdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static GENERATORREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GeneratorRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GeneratorRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GeneratorRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GeneratorRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GeneratorRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GeneratorRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GeneratorRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GENERATORREGISTRY_ABI.clone(),
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
                GENERATORREGISTRY_ABI.clone(),
                GENERATORREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ENTITY_KEY_REGISTRY` (0x661de5ac) function
        pub fn entity_key_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([102, 29, 229, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PARALLEL_REQUESTS_UPPER_LIMIT` (0x7a14c463) function
        pub fn parallel_requests_upper_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 20, 196, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROOF_MARKET_PLACE_ROLE` (0x2c1fbd03) function
        pub fn proof_market_place_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([44, 31, 189, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKING_TOKEN` (0x0479d644) function
        pub fn staking_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([4, 121, 214, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNLOCK_WAIT_BLOCKS` (0x3c5eb57c) function
        pub fn unlock_wait_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 94, 181, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addIvsKey` (0x2180de5d) function
        pub fn add_ivs_key(
            &self,
            market_id: ::ethers::core::types::U256,
            attestation_data: ::ethers::core::types::Bytes,
            enclave_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [33, 128, 222, 93],
                    (market_id, attestation_data, enclave_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assignGeneratorTask` (0xc492ee39) function
        pub fn assign_generator_task(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
            stake_to_lock: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [196, 146, 238, 57],
                    (generator_address, market_id, stake_to_lock),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeRewardAddress` (0x4d2aab9a) function
        pub fn change_reward_address(
            &self,
            new_reward_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 42, 171, 154], new_reward_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeGeneratorTask` (0x982a415d) function
        pub fn complete_generator_task(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
            stake_to_release: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [152, 42, 65, 93],
                    (generator_address, market_id, stake_to_release),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseDeclaredCompute` (0x2f8f4a3b) function
        pub fn decrease_declared_compute(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 143, 74, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregister` (0x84ac33ec) function
        pub fn deregister(
            &self,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 172, 51, 236], refund_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generatorInfoPerMarket` (0x9a7fca8e) function
        pub fn generator_info_per_market(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u8,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([154, 127, 202, 142], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generatorRegistry` (0x8cfc56d8) function
        pub fn generator_registry(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([140, 252, 86, 216], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGeneratorAssignmentDetails` (0x1c7eae65) function
        pub fn get_generator_assignment_details(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([28, 126, 174, 101], (generator_address, market_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGeneratorRewardDetails` (0x2b610c2d) function
        pub fn get_generator_reward_details(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([43, 97, 12, 45], (generator_address, market_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGeneratorState` (0x646d51b4) function
        pub fn get_generator_state(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([100, 109, 81, 180], (generator_address, market_id))
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
        ///Calls the contract's `increaseDeclaredCompute` (0x6d405877) function
        pub fn increase_declared_compute(
            &self,
            compute_to_increase: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 64, 88, 119], compute_to_increase)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            admin: ::ethers::core::types::Address,
            proof_marketplace: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (admin, proof_marketplace))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intendToReduceCompute` (0x96de0eef) function
        pub fn intend_to_reduce_compute(
            &self,
            new_utilization: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 222, 14, 239], new_utilization)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intendToReduceStake` (0xe9e934a0) function
        pub fn intend_to_reduce_stake(
            &self,
            new_utilization: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 233, 52, 160], new_utilization)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `joinMarketplace` (0xe2fa33ce) function
        pub fn join_marketplace(
            &self,
            market_id: ::ethers::core::types::U256,
            compute_per_request_required: ::ethers::core::types::U256,
            proof_generation_cost: ::ethers::core::types::U256,
            proposed_time: ::ethers::core::types::U256,
            update_market_dedicated_key: bool,
            attestation_data: ::ethers::core::types::Bytes,
            enclave_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [226, 250, 51, 206],
                    (
                        market_id,
                        compute_per_request_required,
                        proof_generation_cost,
                        proposed_time,
                        update_market_dedicated_key,
                        attestation_data,
                        enclave_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `leaveMarketplace` (0x9f5db986) function
        pub fn leave_marketplace(
            &self,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 93, 185, 134], market_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `leaveMarketplaces` (0x08be6bad) function
        pub fn leave_marketplaces(
            &self,
            market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 190, 107, 173], market_ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proofMarketplace` (0x81c45c70) function
        pub fn proof_marketplace(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([129, 196, 92, 112], ())
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
        ///Calls the contract's `register` (0x136dfbf5) function
        pub fn register(
            &self,
            reward_address: ::ethers::core::types::Address,
            declared_compute: ::ethers::core::types::U256,
            initial_stake: ::ethers::core::types::U256,
            generator_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [19, 109, 251, 245],
                    (reward_address, declared_compute, initial_stake, generator_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeEncryptionKey` (0x541a8c18) function
        pub fn remove_encryption_key(
            &self,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 26, 140, 24], market_id)
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
        ///Calls the contract's `requestForExitMarketplace` (0xe7bc9600) function
        pub fn request_for_exit_marketplace(
            &self,
            market_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 188, 150, 0], market_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestForExitMarketplaces` (0xd06e1f7b) function
        pub fn request_for_exit_marketplaces(
            &self,
            market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 110, 31, 123], market_ids)
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
        ///Calls the contract's `slashGenerator` (0xeaacae94) function
        pub fn slash_generator(
            &self,
            generator_address: ::ethers::core::types::Address,
            market_id: ::ethers::core::types::U256,
            slashing_amount: ::ethers::core::types::U256,
            reward_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [234, 172, 174, 148],
                    (generator_address, market_id, slashing_amount, reward_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0xadc9772e) function
        pub fn stake(
            &self,
            generator_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 201, 119, 46], (generator_address, amount))
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
        ///Calls the contract's `unstake` (0xf2888dbb) function
        pub fn unstake(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 136, 141, 187], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateEncryptionKey` (0x92eb91e2) function
        pub fn update_encryption_key(
            &self,
            market_id: ::ethers::core::types::U256,
            attestation_data: ::ethers::core::types::Bytes,
            enclave_signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [146, 235, 145, 226],
                    (market_id, attestation_data, enclave_signature),
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
        ///Gets the contract's `AddIvsKey` event
        pub fn add_ivs_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddIvsKeyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AddedStake` event
        pub fn added_stake_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddedStakeFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `ChangedGeneratorRewardAddress` event
        pub fn changed_generator_reward_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedGeneratorRewardAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DecreaseCompute` event
        pub fn decrease_compute_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DecreaseComputeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DeregisteredGenerator` event
        pub fn deregistered_generator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeregisteredGeneratorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IncreasedCompute` event
        pub fn increased_compute_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IncreasedComputeFilter,
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
        ///Gets the contract's `JoinedMarketplace` event
        pub fn joined_marketplace_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            JoinedMarketplaceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LeftMarketplace` event
        pub fn left_marketplace_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LeftMarketplaceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RegisteredGenerator` event
        pub fn registered_generator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RegisteredGeneratorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemovedStake` event
        pub fn removed_stake_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemovedStakeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RequestComputeDecrease` event
        pub fn request_compute_decrease_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequestComputeDecreaseFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RequestExitMarketplace` event
        pub fn request_exit_marketplace_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequestExitMarketplaceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RequestStakeDecrease` event
        pub fn request_stake_decrease_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequestStakeDecreaseFilter,
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
            GeneratorRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GeneratorRegistry<M> {
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
    #[ethevent(name = "AddIvsKey", abi = "AddIvsKey(uint256,address)")]
    pub struct AddIvsKeyFilter {
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub signer: ::ethers::core::types::Address,
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
    #[ethevent(name = "AddedStake", abi = "AddedStake(address,uint256)")]
    pub struct AddedStakeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "ChangedGeneratorRewardAddress",
        abi = "ChangedGeneratorRewardAddress(address,address)"
    )]
    pub struct ChangedGeneratorRewardAddressFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub new_reward_address: ::ethers::core::types::Address,
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
    #[ethevent(name = "DecreaseCompute", abi = "DecreaseCompute(address,uint256)")]
    pub struct DecreaseComputeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub compute: ::ethers::core::types::U256,
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
    #[ethevent(name = "DeregisteredGenerator", abi = "DeregisteredGenerator(address)")]
    pub struct DeregisteredGeneratorFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
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
    #[ethevent(name = "IncreasedCompute", abi = "IncreasedCompute(address,uint256)")]
    pub struct IncreasedComputeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub compute: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "JoinedMarketplace",
        abi = "JoinedMarketplace(address,uint256,uint256)"
    )]
    pub struct JoinedMarketplaceFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
        pub compute_allocation: ::ethers::core::types::U256,
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
    #[ethevent(name = "LeftMarketplace", abi = "LeftMarketplace(address,uint256)")]
    pub struct LeftMarketplaceFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
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
        name = "RegisteredGenerator",
        abi = "RegisteredGenerator(address,uint256,uint256)"
    )]
    pub struct RegisteredGeneratorFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub initial_compute: ::ethers::core::types::U256,
        pub initial_stake: ::ethers::core::types::U256,
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
    #[ethevent(name = "RemovedStake", abi = "RemovedStake(address,uint256)")]
    pub struct RemovedStakeFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "RequestComputeDecrease",
        abi = "RequestComputeDecrease(address,uint256)"
    )]
    pub struct RequestComputeDecreaseFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub intended_utilization: ::ethers::core::types::U256,
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
        name = "RequestExitMarketplace",
        abi = "RequestExitMarketplace(address,uint256)"
    )]
    pub struct RequestExitMarketplaceFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub market_id: ::ethers::core::types::U256,
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
        name = "RequestStakeDecrease",
        abi = "RequestStakeDecrease(address,uint256)"
    )]
    pub struct RequestStakeDecreaseFilter {
        #[ethevent(indexed)]
        pub generator: ::ethers::core::types::Address,
        pub intended_utilization: ::ethers::core::types::U256,
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
    pub enum GeneratorRegistryEvents {
        AddIvsKeyFilter(AddIvsKeyFilter),
        AddedStakeFilter(AddedStakeFilter),
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        ChangedGeneratorRewardAddressFilter(ChangedGeneratorRewardAddressFilter),
        DecreaseComputeFilter(DecreaseComputeFilter),
        DeregisteredGeneratorFilter(DeregisteredGeneratorFilter),
        IncreasedComputeFilter(IncreasedComputeFilter),
        InitializedFilter(InitializedFilter),
        JoinedMarketplaceFilter(JoinedMarketplaceFilter),
        LeftMarketplaceFilter(LeftMarketplaceFilter),
        RegisteredGeneratorFilter(RegisteredGeneratorFilter),
        RemovedStakeFilter(RemovedStakeFilter),
        RequestComputeDecreaseFilter(RequestComputeDecreaseFilter),
        RequestExitMarketplaceFilter(RequestExitMarketplaceFilter),
        RequestStakeDecreaseFilter(RequestStakeDecreaseFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for GeneratorRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddIvsKeyFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::AddIvsKeyFilter(decoded));
            }
            if let Ok(decoded) = AddedStakeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::AddedStakeFilter(decoded));
            }
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::BeaconUpgradedFilter(decoded));
            }
            if let Ok(decoded) = ChangedGeneratorRewardAddressFilter::decode_log(log) {
                return Ok(
                    GeneratorRegistryEvents::ChangedGeneratorRewardAddressFilter(decoded),
                );
            }
            if let Ok(decoded) = DecreaseComputeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::DecreaseComputeFilter(decoded));
            }
            if let Ok(decoded) = DeregisteredGeneratorFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::DeregisteredGeneratorFilter(decoded));
            }
            if let Ok(decoded) = IncreasedComputeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::IncreasedComputeFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = JoinedMarketplaceFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::JoinedMarketplaceFilter(decoded));
            }
            if let Ok(decoded) = LeftMarketplaceFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::LeftMarketplaceFilter(decoded));
            }
            if let Ok(decoded) = RegisteredGeneratorFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RegisteredGeneratorFilter(decoded));
            }
            if let Ok(decoded) = RemovedStakeFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RemovedStakeFilter(decoded));
            }
            if let Ok(decoded) = RequestComputeDecreaseFilter::decode_log(log) {
                return Ok(
                    GeneratorRegistryEvents::RequestComputeDecreaseFilter(decoded),
                );
            }
            if let Ok(decoded) = RequestExitMarketplaceFilter::decode_log(log) {
                return Ok(
                    GeneratorRegistryEvents::RequestExitMarketplaceFilter(decoded),
                );
            }
            if let Ok(decoded) = RequestStakeDecreaseFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RequestStakeDecreaseFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(GeneratorRegistryEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GeneratorRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddIvsKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddedStakeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedGeneratorRewardAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseComputeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeregisteredGeneratorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreasedComputeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::JoinedMarketplaceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LeftMarketplaceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisteredGeneratorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovedStakeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestComputeDecreaseFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestExitMarketplaceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestStakeDecreaseFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddIvsKeyFilter> for GeneratorRegistryEvents {
        fn from(value: AddIvsKeyFilter) -> Self {
            Self::AddIvsKeyFilter(value)
        }
    }
    impl ::core::convert::From<AddedStakeFilter> for GeneratorRegistryEvents {
        fn from(value: AddedStakeFilter) -> Self {
            Self::AddedStakeFilter(value)
        }
    }
    impl ::core::convert::From<AdminChangedFilter> for GeneratorRegistryEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<BeaconUpgradedFilter> for GeneratorRegistryEvents {
        fn from(value: BeaconUpgradedFilter) -> Self {
            Self::BeaconUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<ChangedGeneratorRewardAddressFilter>
    for GeneratorRegistryEvents {
        fn from(value: ChangedGeneratorRewardAddressFilter) -> Self {
            Self::ChangedGeneratorRewardAddressFilter(value)
        }
    }
    impl ::core::convert::From<DecreaseComputeFilter> for GeneratorRegistryEvents {
        fn from(value: DecreaseComputeFilter) -> Self {
            Self::DecreaseComputeFilter(value)
        }
    }
    impl ::core::convert::From<DeregisteredGeneratorFilter> for GeneratorRegistryEvents {
        fn from(value: DeregisteredGeneratorFilter) -> Self {
            Self::DeregisteredGeneratorFilter(value)
        }
    }
    impl ::core::convert::From<IncreasedComputeFilter> for GeneratorRegistryEvents {
        fn from(value: IncreasedComputeFilter) -> Self {
            Self::IncreasedComputeFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for GeneratorRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<JoinedMarketplaceFilter> for GeneratorRegistryEvents {
        fn from(value: JoinedMarketplaceFilter) -> Self {
            Self::JoinedMarketplaceFilter(value)
        }
    }
    impl ::core::convert::From<LeftMarketplaceFilter> for GeneratorRegistryEvents {
        fn from(value: LeftMarketplaceFilter) -> Self {
            Self::LeftMarketplaceFilter(value)
        }
    }
    impl ::core::convert::From<RegisteredGeneratorFilter> for GeneratorRegistryEvents {
        fn from(value: RegisteredGeneratorFilter) -> Self {
            Self::RegisteredGeneratorFilter(value)
        }
    }
    impl ::core::convert::From<RemovedStakeFilter> for GeneratorRegistryEvents {
        fn from(value: RemovedStakeFilter) -> Self {
            Self::RemovedStakeFilter(value)
        }
    }
    impl ::core::convert::From<RequestComputeDecreaseFilter>
    for GeneratorRegistryEvents {
        fn from(value: RequestComputeDecreaseFilter) -> Self {
            Self::RequestComputeDecreaseFilter(value)
        }
    }
    impl ::core::convert::From<RequestExitMarketplaceFilter>
    for GeneratorRegistryEvents {
        fn from(value: RequestExitMarketplaceFilter) -> Self {
            Self::RequestExitMarketplaceFilter(value)
        }
    }
    impl ::core::convert::From<RequestStakeDecreaseFilter> for GeneratorRegistryEvents {
        fn from(value: RequestStakeDecreaseFilter) -> Self {
            Self::RequestStakeDecreaseFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for GeneratorRegistryEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for GeneratorRegistryEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for GeneratorRegistryEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for GeneratorRegistryEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `ENTITY_KEY_REGISTRY` function with signature `ENTITY_KEY_REGISTRY()` and selector `0x661de5ac`
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
    #[ethcall(name = "ENTITY_KEY_REGISTRY", abi = "ENTITY_KEY_REGISTRY()")]
    pub struct EntityKeyRegistryCall;
    ///Container type for all input parameters for the `PARALLEL_REQUESTS_UPPER_LIMIT` function with signature `PARALLEL_REQUESTS_UPPER_LIMIT()` and selector `0x7a14c463`
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
        name = "PARALLEL_REQUESTS_UPPER_LIMIT",
        abi = "PARALLEL_REQUESTS_UPPER_LIMIT()"
    )]
    pub struct ParallelRequestsUpperLimitCall;
    ///Container type for all input parameters for the `PROOF_MARKET_PLACE_ROLE` function with signature `PROOF_MARKET_PLACE_ROLE()` and selector `0x2c1fbd03`
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
    #[ethcall(name = "PROOF_MARKET_PLACE_ROLE", abi = "PROOF_MARKET_PLACE_ROLE()")]
    pub struct ProofMarketPlaceRoleCall;
    ///Container type for all input parameters for the `STAKING_TOKEN` function with signature `STAKING_TOKEN()` and selector `0x0479d644`
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
    #[ethcall(name = "STAKING_TOKEN", abi = "STAKING_TOKEN()")]
    pub struct StakingTokenCall;
    ///Container type for all input parameters for the `UNLOCK_WAIT_BLOCKS` function with signature `UNLOCK_WAIT_BLOCKS()` and selector `0x3c5eb57c`
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
    #[ethcall(name = "UNLOCK_WAIT_BLOCKS", abi = "UNLOCK_WAIT_BLOCKS()")]
    pub struct UnlockWaitBlocksCall;
    ///Container type for all input parameters for the `addIvsKey` function with signature `addIvsKey(uint256,bytes,bytes)` and selector `0x2180de5d`
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
    #[ethcall(name = "addIvsKey", abi = "addIvsKey(uint256,bytes,bytes)")]
    pub struct AddIvsKeyCall {
        pub market_id: ::ethers::core::types::U256,
        pub attestation_data: ::ethers::core::types::Bytes,
        pub enclave_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `assignGeneratorTask` function with signature `assignGeneratorTask(address,uint256,uint256)` and selector `0xc492ee39`
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
        name = "assignGeneratorTask",
        abi = "assignGeneratorTask(address,uint256,uint256)"
    )]
    pub struct AssignGeneratorTaskCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
        pub stake_to_lock: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `changeRewardAddress` function with signature `changeRewardAddress(address)` and selector `0x4d2aab9a`
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
    #[ethcall(name = "changeRewardAddress", abi = "changeRewardAddress(address)")]
    pub struct ChangeRewardAddressCall {
        pub new_reward_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `completeGeneratorTask` function with signature `completeGeneratorTask(address,uint256,uint256)` and selector `0x982a415d`
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
        name = "completeGeneratorTask",
        abi = "completeGeneratorTask(address,uint256,uint256)"
    )]
    pub struct CompleteGeneratorTaskCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
        pub stake_to_release: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decreaseDeclaredCompute` function with signature `decreaseDeclaredCompute()` and selector `0x2f8f4a3b`
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
    #[ethcall(name = "decreaseDeclaredCompute", abi = "decreaseDeclaredCompute()")]
    pub struct DecreaseDeclaredComputeCall;
    ///Container type for all input parameters for the `deregister` function with signature `deregister(address)` and selector `0x84ac33ec`
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
    #[ethcall(name = "deregister", abi = "deregister(address)")]
    pub struct DeregisterCall {
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `generatorInfoPerMarket` function with signature `generatorInfoPerMarket(address,uint256)` and selector `0x9a7fca8e`
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
        name = "generatorInfoPerMarket",
        abi = "generatorInfoPerMarket(address,uint256)"
    )]
    pub struct GeneratorInfoPerMarketCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `generatorRegistry` function with signature `generatorRegistry(address)` and selector `0x8cfc56d8`
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
    #[ethcall(name = "generatorRegistry", abi = "generatorRegistry(address)")]
    pub struct GeneratorRegistryCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getGeneratorAssignmentDetails` function with signature `getGeneratorAssignmentDetails(address,uint256)` and selector `0x1c7eae65`
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
        name = "getGeneratorAssignmentDetails",
        abi = "getGeneratorAssignmentDetails(address,uint256)"
    )]
    pub struct GetGeneratorAssignmentDetailsCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getGeneratorRewardDetails` function with signature `getGeneratorRewardDetails(address,uint256)` and selector `0x2b610c2d`
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
        name = "getGeneratorRewardDetails",
        abi = "getGeneratorRewardDetails(address,uint256)"
    )]
    pub struct GetGeneratorRewardDetailsCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getGeneratorState` function with signature `getGeneratorState(address,uint256)` and selector `0x646d51b4`
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
    #[ethcall(name = "getGeneratorState", abi = "getGeneratorState(address,uint256)")]
    pub struct GetGeneratorStateCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `increaseDeclaredCompute` function with signature `increaseDeclaredCompute(uint256)` and selector `0x6d405877`
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
        name = "increaseDeclaredCompute",
        abi = "increaseDeclaredCompute(uint256)"
    )]
    pub struct IncreaseDeclaredComputeCall {
        pub compute_to_increase: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub admin: ::ethers::core::types::Address,
        pub proof_marketplace: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `intendToReduceCompute` function with signature `intendToReduceCompute(uint256)` and selector `0x96de0eef`
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
    #[ethcall(name = "intendToReduceCompute", abi = "intendToReduceCompute(uint256)")]
    pub struct IntendToReduceComputeCall {
        pub new_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `intendToReduceStake` function with signature `intendToReduceStake(uint256)` and selector `0xe9e934a0`
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
    #[ethcall(name = "intendToReduceStake", abi = "intendToReduceStake(uint256)")]
    pub struct IntendToReduceStakeCall {
        pub new_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `joinMarketplace` function with signature `joinMarketplace(uint256,uint256,uint256,uint256,bool,bytes,bytes)` and selector `0xe2fa33ce`
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
        name = "joinMarketplace",
        abi = "joinMarketplace(uint256,uint256,uint256,uint256,bool,bytes,bytes)"
    )]
    pub struct JoinMarketplaceCall {
        pub market_id: ::ethers::core::types::U256,
        pub compute_per_request_required: ::ethers::core::types::U256,
        pub proof_generation_cost: ::ethers::core::types::U256,
        pub proposed_time: ::ethers::core::types::U256,
        pub update_market_dedicated_key: bool,
        pub attestation_data: ::ethers::core::types::Bytes,
        pub enclave_signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `leaveMarketplace` function with signature `leaveMarketplace(uint256)` and selector `0x9f5db986`
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
    #[ethcall(name = "leaveMarketplace", abi = "leaveMarketplace(uint256)")]
    pub struct LeaveMarketplaceCall {
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `leaveMarketplaces` function with signature `leaveMarketplaces(uint256[])` and selector `0x08be6bad`
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
    #[ethcall(name = "leaveMarketplaces", abi = "leaveMarketplaces(uint256[])")]
    pub struct LeaveMarketplacesCall {
        pub market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `proofMarketplace` function with signature `proofMarketplace()` and selector `0x81c45c70`
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
    #[ethcall(name = "proofMarketplace", abi = "proofMarketplace()")]
    pub struct ProofMarketplaceCall;
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
    ///Container type for all input parameters for the `register` function with signature `register(address,uint256,uint256,bytes)` and selector `0x136dfbf5`
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
    #[ethcall(name = "register", abi = "register(address,uint256,uint256,bytes)")]
    pub struct RegisterCall {
        pub reward_address: ::ethers::core::types::Address,
        pub declared_compute: ::ethers::core::types::U256,
        pub initial_stake: ::ethers::core::types::U256,
        pub generator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `removeEncryptionKey` function with signature `removeEncryptionKey(uint256)` and selector `0x541a8c18`
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
    #[ethcall(name = "removeEncryptionKey", abi = "removeEncryptionKey(uint256)")]
    pub struct RemoveEncryptionKeyCall {
        pub market_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `requestForExitMarketplace` function with signature `requestForExitMarketplace(uint256)` and selector `0xe7bc9600`
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
        name = "requestForExitMarketplace",
        abi = "requestForExitMarketplace(uint256)"
    )]
    pub struct RequestForExitMarketplaceCall {
        pub market_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requestForExitMarketplaces` function with signature `requestForExitMarketplaces(uint256[])` and selector `0xd06e1f7b`
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
        name = "requestForExitMarketplaces",
        abi = "requestForExitMarketplaces(uint256[])"
    )]
    pub struct RequestForExitMarketplacesCall {
        pub market_ids: ::std::vec::Vec<::ethers::core::types::U256>,
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
    ///Container type for all input parameters for the `slashGenerator` function with signature `slashGenerator(address,uint256,uint256,address)` and selector `0xeaacae94`
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
        name = "slashGenerator",
        abi = "slashGenerator(address,uint256,uint256,address)"
    )]
    pub struct SlashGeneratorCall {
        pub generator_address: ::ethers::core::types::Address,
        pub market_id: ::ethers::core::types::U256,
        pub slashing_amount: ::ethers::core::types::U256,
        pub reward_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stake` function with signature `stake(address,uint256)` and selector `0xadc9772e`
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
    #[ethcall(name = "stake", abi = "stake(address,uint256)")]
    pub struct StakeCall {
        pub generator_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `unstake` function with signature `unstake(address)` and selector `0xf2888dbb`
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
    #[ethcall(name = "unstake", abi = "unstake(address)")]
    pub struct UnstakeCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateEncryptionKey` function with signature `updateEncryptionKey(uint256,bytes,bytes)` and selector `0x92eb91e2`
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
        name = "updateEncryptionKey",
        abi = "updateEncryptionKey(uint256,bytes,bytes)"
    )]
    pub struct UpdateEncryptionKeyCall {
        pub market_id: ::ethers::core::types::U256,
        pub attestation_data: ::ethers::core::types::Bytes,
        pub enclave_signature: ::ethers::core::types::Bytes,
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
    pub enum GeneratorRegistryCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        EntityKeyRegistry(EntityKeyRegistryCall),
        ParallelRequestsUpperLimit(ParallelRequestsUpperLimitCall),
        ProofMarketPlaceRole(ProofMarketPlaceRoleCall),
        StakingToken(StakingTokenCall),
        UnlockWaitBlocks(UnlockWaitBlocksCall),
        AddIvsKey(AddIvsKeyCall),
        AssignGeneratorTask(AssignGeneratorTaskCall),
        ChangeRewardAddress(ChangeRewardAddressCall),
        CompleteGeneratorTask(CompleteGeneratorTaskCall),
        DecreaseDeclaredCompute(DecreaseDeclaredComputeCall),
        Deregister(DeregisterCall),
        GeneratorInfoPerMarket(GeneratorInfoPerMarketCall),
        GeneratorRegistry(GeneratorRegistryCall),
        GetGeneratorAssignmentDetails(GetGeneratorAssignmentDetailsCall),
        GetGeneratorRewardDetails(GetGeneratorRewardDetailsCall),
        GetGeneratorState(GetGeneratorStateCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IncreaseDeclaredCompute(IncreaseDeclaredComputeCall),
        Initialize(InitializeCall),
        IntendToReduceCompute(IntendToReduceComputeCall),
        IntendToReduceStake(IntendToReduceStakeCall),
        JoinMarketplace(JoinMarketplaceCall),
        LeaveMarketplace(LeaveMarketplaceCall),
        LeaveMarketplaces(LeaveMarketplacesCall),
        ProofMarketplace(ProofMarketplaceCall),
        ProxiableUUID(ProxiableUUIDCall),
        Register(RegisterCall),
        RemoveEncryptionKey(RemoveEncryptionKeyCall),
        RenounceRole(RenounceRoleCall),
        RequestForExitMarketplace(RequestForExitMarketplaceCall),
        RequestForExitMarketplaces(RequestForExitMarketplacesCall),
        RevokeRole(RevokeRoleCall),
        SlashGenerator(SlashGeneratorCall),
        Stake(StakeCall),
        SupportsInterface(SupportsInterfaceCall),
        Unstake(UnstakeCall),
        UpdateEncryptionKey(UpdateEncryptionKeyCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ::ethers::core::abi::AbiDecode for GeneratorRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <EntityKeyRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EntityKeyRegistry(decoded));
            }
            if let Ok(decoded) = <ParallelRequestsUpperLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParallelRequestsUpperLimit(decoded));
            }
            if let Ok(decoded) = <ProofMarketPlaceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProofMarketPlaceRole(decoded));
            }
            if let Ok(decoded) = <StakingTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakingToken(decoded));
            }
            if let Ok(decoded) = <UnlockWaitBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnlockWaitBlocks(decoded));
            }
            if let Ok(decoded) = <AddIvsKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddIvsKey(decoded));
            }
            if let Ok(decoded) = <AssignGeneratorTaskCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssignGeneratorTask(decoded));
            }
            if let Ok(decoded) = <ChangeRewardAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeRewardAddress(decoded));
            }
            if let Ok(decoded) = <CompleteGeneratorTaskCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CompleteGeneratorTask(decoded));
            }
            if let Ok(decoded) = <DecreaseDeclaredComputeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseDeclaredCompute(decoded));
            }
            if let Ok(decoded) = <DeregisterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deregister(decoded));
            }
            if let Ok(decoded) = <GeneratorInfoPerMarketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GeneratorInfoPerMarket(decoded));
            }
            if let Ok(decoded) = <GeneratorRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GeneratorRegistry(decoded));
            }
            if let Ok(decoded) = <GetGeneratorAssignmentDetailsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetGeneratorAssignmentDetails(decoded));
            }
            if let Ok(decoded) = <GetGeneratorRewardDetailsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetGeneratorRewardDetails(decoded));
            }
            if let Ok(decoded) = <GetGeneratorStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetGeneratorState(decoded));
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
            if let Ok(decoded) = <IncreaseDeclaredComputeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseDeclaredCompute(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IntendToReduceComputeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntendToReduceCompute(decoded));
            }
            if let Ok(decoded) = <IntendToReduceStakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntendToReduceStake(decoded));
            }
            if let Ok(decoded) = <JoinMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::JoinMarketplace(decoded));
            }
            if let Ok(decoded) = <LeaveMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LeaveMarketplace(decoded));
            }
            if let Ok(decoded) = <LeaveMarketplacesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LeaveMarketplaces(decoded));
            }
            if let Ok(decoded) = <ProofMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProofMarketplace(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) = <RemoveEncryptionKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveEncryptionKey(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RequestForExitMarketplaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestForExitMarketplace(decoded));
            }
            if let Ok(decoded) = <RequestForExitMarketplacesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RequestForExitMarketplaces(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <SlashGeneratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SlashGenerator(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UnstakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unstake(decoded));
            }
            if let Ok(decoded) = <UpdateEncryptionKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateEncryptionKey(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GeneratorRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EntityKeyRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParallelRequestsUpperLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofMarketPlaceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakingToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnlockWaitBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddIvsKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssignGeneratorTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeRewardAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteGeneratorTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseDeclaredCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deregister(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GeneratorInfoPerMarket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GeneratorRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGeneratorAssignmentDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGeneratorRewardDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGeneratorState(element) => {
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
                Self::IncreaseDeclaredCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntendToReduceCompute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntendToReduceStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::JoinMarketplace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LeaveMarketplace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LeaveMarketplaces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProofMarketplace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveEncryptionKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestForExitMarketplace(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestForExitMarketplaces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SlashGenerator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unstake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateEncryptionKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GeneratorRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::EntityKeyRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParallelRequestsUpperLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProofMarketPlaceRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakingToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnlockWaitBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddIvsKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignGeneratorTask(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeRewardAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteGeneratorTask(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseDeclaredCompute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deregister(element) => ::core::fmt::Display::fmt(element, f),
                Self::GeneratorInfoPerMarket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GeneratorRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGeneratorAssignmentDetails(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGeneratorRewardDetails(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGeneratorState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseDeclaredCompute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntendToReduceCompute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntendToReduceStake(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::JoinMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::LeaveMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::LeaveMarketplaces(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofMarketplace(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::Register(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveEncryptionKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestForExitMarketplace(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestForExitMarketplaces(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlashGenerator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unstake(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateEncryptionKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for GeneratorRegistryCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<EntityKeyRegistryCall> for GeneratorRegistryCalls {
        fn from(value: EntityKeyRegistryCall) -> Self {
            Self::EntityKeyRegistry(value)
        }
    }
    impl ::core::convert::From<ParallelRequestsUpperLimitCall>
    for GeneratorRegistryCalls {
        fn from(value: ParallelRequestsUpperLimitCall) -> Self {
            Self::ParallelRequestsUpperLimit(value)
        }
    }
    impl ::core::convert::From<ProofMarketPlaceRoleCall> for GeneratorRegistryCalls {
        fn from(value: ProofMarketPlaceRoleCall) -> Self {
            Self::ProofMarketPlaceRole(value)
        }
    }
    impl ::core::convert::From<StakingTokenCall> for GeneratorRegistryCalls {
        fn from(value: StakingTokenCall) -> Self {
            Self::StakingToken(value)
        }
    }
    impl ::core::convert::From<UnlockWaitBlocksCall> for GeneratorRegistryCalls {
        fn from(value: UnlockWaitBlocksCall) -> Self {
            Self::UnlockWaitBlocks(value)
        }
    }
    impl ::core::convert::From<AddIvsKeyCall> for GeneratorRegistryCalls {
        fn from(value: AddIvsKeyCall) -> Self {
            Self::AddIvsKey(value)
        }
    }
    impl ::core::convert::From<AssignGeneratorTaskCall> for GeneratorRegistryCalls {
        fn from(value: AssignGeneratorTaskCall) -> Self {
            Self::AssignGeneratorTask(value)
        }
    }
    impl ::core::convert::From<ChangeRewardAddressCall> for GeneratorRegistryCalls {
        fn from(value: ChangeRewardAddressCall) -> Self {
            Self::ChangeRewardAddress(value)
        }
    }
    impl ::core::convert::From<CompleteGeneratorTaskCall> for GeneratorRegistryCalls {
        fn from(value: CompleteGeneratorTaskCall) -> Self {
            Self::CompleteGeneratorTask(value)
        }
    }
    impl ::core::convert::From<DecreaseDeclaredComputeCall> for GeneratorRegistryCalls {
        fn from(value: DecreaseDeclaredComputeCall) -> Self {
            Self::DecreaseDeclaredCompute(value)
        }
    }
    impl ::core::convert::From<DeregisterCall> for GeneratorRegistryCalls {
        fn from(value: DeregisterCall) -> Self {
            Self::Deregister(value)
        }
    }
    impl ::core::convert::From<GeneratorInfoPerMarketCall> for GeneratorRegistryCalls {
        fn from(value: GeneratorInfoPerMarketCall) -> Self {
            Self::GeneratorInfoPerMarket(value)
        }
    }
    impl ::core::convert::From<GeneratorRegistryCall> for GeneratorRegistryCalls {
        fn from(value: GeneratorRegistryCall) -> Self {
            Self::GeneratorRegistry(value)
        }
    }
    impl ::core::convert::From<GetGeneratorAssignmentDetailsCall>
    for GeneratorRegistryCalls {
        fn from(value: GetGeneratorAssignmentDetailsCall) -> Self {
            Self::GetGeneratorAssignmentDetails(value)
        }
    }
    impl ::core::convert::From<GetGeneratorRewardDetailsCall>
    for GeneratorRegistryCalls {
        fn from(value: GetGeneratorRewardDetailsCall) -> Self {
            Self::GetGeneratorRewardDetails(value)
        }
    }
    impl ::core::convert::From<GetGeneratorStateCall> for GeneratorRegistryCalls {
        fn from(value: GetGeneratorStateCall) -> Self {
            Self::GetGeneratorState(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for GeneratorRegistryCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for GeneratorRegistryCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for GeneratorRegistryCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for GeneratorRegistryCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for GeneratorRegistryCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IncreaseDeclaredComputeCall> for GeneratorRegistryCalls {
        fn from(value: IncreaseDeclaredComputeCall) -> Self {
            Self::IncreaseDeclaredCompute(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for GeneratorRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IntendToReduceComputeCall> for GeneratorRegistryCalls {
        fn from(value: IntendToReduceComputeCall) -> Self {
            Self::IntendToReduceCompute(value)
        }
    }
    impl ::core::convert::From<IntendToReduceStakeCall> for GeneratorRegistryCalls {
        fn from(value: IntendToReduceStakeCall) -> Self {
            Self::IntendToReduceStake(value)
        }
    }
    impl ::core::convert::From<JoinMarketplaceCall> for GeneratorRegistryCalls {
        fn from(value: JoinMarketplaceCall) -> Self {
            Self::JoinMarketplace(value)
        }
    }
    impl ::core::convert::From<LeaveMarketplaceCall> for GeneratorRegistryCalls {
        fn from(value: LeaveMarketplaceCall) -> Self {
            Self::LeaveMarketplace(value)
        }
    }
    impl ::core::convert::From<LeaveMarketplacesCall> for GeneratorRegistryCalls {
        fn from(value: LeaveMarketplacesCall) -> Self {
            Self::LeaveMarketplaces(value)
        }
    }
    impl ::core::convert::From<ProofMarketplaceCall> for GeneratorRegistryCalls {
        fn from(value: ProofMarketplaceCall) -> Self {
            Self::ProofMarketplace(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for GeneratorRegistryCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for GeneratorRegistryCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RemoveEncryptionKeyCall> for GeneratorRegistryCalls {
        fn from(value: RemoveEncryptionKeyCall) -> Self {
            Self::RemoveEncryptionKey(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for GeneratorRegistryCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RequestForExitMarketplaceCall>
    for GeneratorRegistryCalls {
        fn from(value: RequestForExitMarketplaceCall) -> Self {
            Self::RequestForExitMarketplace(value)
        }
    }
    impl ::core::convert::From<RequestForExitMarketplacesCall>
    for GeneratorRegistryCalls {
        fn from(value: RequestForExitMarketplacesCall) -> Self {
            Self::RequestForExitMarketplaces(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for GeneratorRegistryCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SlashGeneratorCall> for GeneratorRegistryCalls {
        fn from(value: SlashGeneratorCall) -> Self {
            Self::SlashGenerator(value)
        }
    }
    impl ::core::convert::From<StakeCall> for GeneratorRegistryCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for GeneratorRegistryCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UnstakeCall> for GeneratorRegistryCalls {
        fn from(value: UnstakeCall) -> Self {
            Self::Unstake(value)
        }
    }
    impl ::core::convert::From<UpdateEncryptionKeyCall> for GeneratorRegistryCalls {
        fn from(value: UpdateEncryptionKeyCall) -> Self {
            Self::UpdateEncryptionKey(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for GeneratorRegistryCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for GeneratorRegistryCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
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
    ///Container type for all return fields from the `ENTITY_KEY_REGISTRY` function with signature `ENTITY_KEY_REGISTRY()` and selector `0x661de5ac`
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
    pub struct EntityKeyRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `PARALLEL_REQUESTS_UPPER_LIMIT` function with signature `PARALLEL_REQUESTS_UPPER_LIMIT()` and selector `0x7a14c463`
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
    pub struct ParallelRequestsUpperLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PROOF_MARKET_PLACE_ROLE` function with signature `PROOF_MARKET_PLACE_ROLE()` and selector `0x2c1fbd03`
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
    pub struct ProofMarketPlaceRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKING_TOKEN` function with signature `STAKING_TOKEN()` and selector `0x0479d644`
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
    pub struct StakingTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `UNLOCK_WAIT_BLOCKS` function with signature `UNLOCK_WAIT_BLOCKS()` and selector `0x3c5eb57c`
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
    pub struct UnlockWaitBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `generatorInfoPerMarket` function with signature `generatorInfoPerMarket(address,uint256)` and selector `0x9a7fca8e`
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
    pub struct GeneratorInfoPerMarketReturn {
        pub state: u8,
        pub compute_per_request_required: ::ethers::core::types::U256,
        pub proof_generation_cost: ::ethers::core::types::U256,
        pub proposed_time: ::ethers::core::types::U256,
        pub active_requests: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `generatorRegistry` function with signature `generatorRegistry(address)` and selector `0x8cfc56d8`
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
    pub struct GeneratorRegistryReturn {
        pub reward_address: ::ethers::core::types::Address,
        pub total_stake: ::ethers::core::types::U256,
        pub sum_of_compute_allocations: ::ethers::core::types::U256,
        pub compute_consumed: ::ethers::core::types::U256,
        pub stake_locked: ::ethers::core::types::U256,
        pub active_marketplaces: ::ethers::core::types::U256,
        pub declared_compute: ::ethers::core::types::U256,
        pub intended_stake_utilization: ::ethers::core::types::U256,
        pub intended_compute_utilization: ::ethers::core::types::U256,
        pub generator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getGeneratorAssignmentDetails` function with signature `getGeneratorAssignmentDetails(address,uint256)` and selector `0x1c7eae65`
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
    pub struct GetGeneratorAssignmentDetailsReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getGeneratorRewardDetails` function with signature `getGeneratorRewardDetails(address,uint256)` and selector `0x2b610c2d`
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
    pub struct GetGeneratorRewardDetailsReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getGeneratorState` function with signature `getGeneratorState(address,uint256)` and selector `0x646d51b4`
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
    pub struct GetGeneratorStateReturn(pub u8, pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `proofMarketplace` function with signature `proofMarketplace()` and selector `0x81c45c70`
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
    pub struct ProofMarketplaceReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `slashGenerator` function with signature `slashGenerator(address,uint256,uint256,address)` and selector `0xeaacae94`
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
    pub struct SlashGeneratorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stake` function with signature `stake(address,uint256)` and selector `0xadc9772e`
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
    pub struct StakeReturn(pub ::ethers::core::types::U256);
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
}
