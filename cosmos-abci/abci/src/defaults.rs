/// Method for getting RPC url form active env.
pub fn get_server_url() -> String {
    match std::env::var("ABCI_SERVER_URL") {
        Ok(val) => val,
        Err(_) => DEFAULT_ABCI_URL.to_owned(),
    }
}

/// Default ABCI RPC url.
pub const DEFAULT_ABCI_URL: &str = "tcp://localhost:26658";

/// Genesis initial state.
pub const TEST_GENESIS: &str = r#"
{
    "genesis_time": "2020-09-10T18:03:56.233701Z",
    "chain_id": "namechain",
    "consensus_params": {
        "block": {
            "max_bytes": "22020096",
            "max_gas": "-1",
            "time_iota_ms": "1000"
        },
        "evidence": {
            "max_age_num_blocks": "100000",
            "max_age_duration": "172800000000000"
        },
        "validator": {
            "pub_key_types": ["ed25519"]
        }
    },
    "app_hash": "",
    "app_state": {
        "genutil": {
            "gentxs": [
                {
                    "body": {
                        "messages": [
                            {
                                "@type": "/cosmos.staking.MsgCreateValidator",
                                "description": {
                                    "moniker": "test"
                                },
                                "commission": {
                                    "rate": "0.100000000000000000",
                                    "max_rate": "0.200000000000000000",
                                    "max_change_rate": "0.010000000000000000"
                                },
                                "min_self_delegation": "1",
                                "delegator_address": "cosmos14gczfcwzmqgjgjkzpjegqexx380kj6dp2uwcyp",
                                "validator_address": "cosmosvaloper14gczfcwzmqgjgjkzpjegqexx380kj6dp0g6dgj",
                                "pubkey": "cosmosvalconspub1zcjduepq4lf5h7jd6j28rvfx9wnuds22agex45gdvgmt4qdj86z9cvugp8sqwg3v6q",
                                "value": {
                                    "denom": "stake",
                                    "amount": "100000000"
                                }
                            }
                        ],
                        "memo": "d74ee60d8acbf6d1c047b9e9bc67d703006f2179@192.168.88.151:26656"
                    },
                    "auth_info": {
                        "signer_infos": [
                            {
                                "public_key": {
                                    "secp256k1": "AqJ6MR19i/LciIYnEs/lJ6/6P/3eBPNxHnlbN7Mj/PyR"
                                },
                                "mode_info": {
                                    "single": {
                                        "mode": "SIGN_MODE_DIRECT"
                                    }
                                }
                            }
                        ],
                        "fee": {
                            "gas_limit": "200000"
                        }
                    },
                    "signatures": [
                        "HI4713AZYCpKlu818Iv/+2Gu9T7scF9EyqEao2lg5f5nG14o3n/ONtoDqLt5eFRpdEwIi/ICQoLwvb4KZVJnKQ=="
                    ]
                }
            ]
        },
        "mint": {
            "minter": {
                "inflation": "0.130000000000000000",
                "annual_provisions": "0.000000000000000000"
            },
            "params": {
                "mint_denom": "stake",
                "inflation_rate_change": "0.130000000000000000",
                "inflation_max": "0.200000000000000000",
                "inflation_min": "0.070000000000000000",
                "goal_bonded": "0.670000000000000000",
                "blocks_per_year": "6311520"
            }
        },
        "capability": {
            "index": "1",
            "owners": []
        },
        "crisis": {
            "constant_fee": {
                "denom": "stake",
                "amount": "1000"
            }
        },
        "slashing": {
            "params": {
                "signed_blocks_window": "100",
                "min_signed_per_window": "0.500000000000000000",
                "downtime_jail_duration": "600000000000",
                "slash_fraction_double_sign": "0.050000000000000000",
                "slash_fraction_downtime": "0.010000000000000000"
            },
            "signing_infos": [],
            "missed_blocks": []
        },
        "staking": {
            "params": {
                "unbonding_time": "1814400000000000",
                "max_validators": 100,
                "max_entries": 7,
                "historical_entries": 100,
                "bond_denom": "stake"
            },
            "last_total_power": "0",
            "last_validator_powers": null,
            "validators": null,
            "delegations": null,
            "unbonding_delegations": null,
            "redelegations": null
        },
        "transfer": {
            "port_id": "transfer"
        },
        "upgrade": {},
        "evidence": {},
        "auth": {
            "params": {
                "max_memo_characters": "256",
                "tx_sig_limit": "7",
                "tx_size_cost_per_byte": "10",
                "sig_verify_cost_ed25519": "590",
                "sig_verify_cost_secp256k1": "1000"
            },
            "accounts": [
                {
                    "type": "cosmos-sdk/BaseAccount",
                    "value": {
                        "address": "cosmos14gczfcwzmqgjgjkzpjegqexx380kj6dp2uwcyp"
                    }
                },
                {
                    "type": "cosmos-sdk/BaseAccount",
                    "value": {
                        "address": "cosmos1gzvxnlmzhds0zn3epe973dfjjafe7ndrtvm6f9"
                    }
                }
            ]
        },
        "nameservice": {},
        "distribution": {
            "params": {
                "community_tax": "0.020000000000000000",
                "base_proposer_reward": "0.010000000000000000",
                "bonus_proposer_reward": "0.040000000000000000",
                "withdraw_addr_enabled": true
            },
            "fee_pool": {
                "community_pool": []
            },
            "delegator_withdraw_infos": [],
            "outstanding_rewards": [],
            "validator_accumulated_commissions": [],
            "validator_historical_rewards": [],
            "validator_current_rewards": [],
            "delegator_starting_infos": [],
            "validator_slash_events": []
        },
        "params": null,
        "gov": {
            "starting_proposal_id": "1",
            "deposits": null,
            "votes": null,
            "proposals": null,
            "deposit_params": {
                "min_deposit": [
                    {
                        "denom": "stake",
                        "amount": "10000000"
                    }
                ],
                "max_deposit_period": "172800000000000"
            },
            "voting_params": {
                "voting_period": "172800000000000"
            },
            "tally_params": {
                "quorum": "0.334000000000000000",
                "threshold": "0.500000000000000000",
                "veto": "0.334000000000000000"
            }
        },
        "ibc": {
            "client_genesis": {
                "clients": [],
                "clients_consensus": [],
                "create_localhost": true
            },
            "connection_genesis": {
                "connections": [],
                "client_connection_paths": []
            },
            "channel_genesis": {
                "channels": [],
                "acknowledgements": [],
                "commitments": [],
                "send_sequences": [],
                "recv_sequences": [],
                "ack_sequences": []
            }
        },
        "bank": {
            "params": {
                "default_send_enabled": true
            },
            "balances": [
                {
                    "address": "cosmos1gzvxnlmzhds0zn3epe973dfjjafe7ndrtvm6f9",
                    "coins": [
                        {
                            "denom": "nametoken",
                            "amount": "1000"
                        },
                        {
                            "denom": "stake",
                            "amount": "100000000"
                        }
                    ]
                },
                {
                    "address": "cosmos14gczfcwzmqgjgjkzpjegqexx380kj6dp2uwcyp",
                    "coins": [
                        {
                            "denom": "nametoken",
                            "amount": "1000"
                        },
                        {
                            "denom": "stake",
                            "amount": "100000000"
                        }
                    ]
                }
            ],
            "supply": [],
            "denom_metadata": null
        }
    },
    "validators": [
        {
          "address": "B547AB87E79F75A4A3198C57A8C2FDAF8628CB47",
          "pub_key": {
            "type": "substrate/PubKeyEd25519",
            "value": "P/V6GHuZrb8rs/k1oBorxc6vyXMlnzhJmv7LmjELDys="
          },
          "power": "10",
          "name": "Alice"
        }
    ]
}"#;

/// App version type.
pub type AppVersion = String;
/// App block version type.
pub type BlockVersion = u64;
/// App P2P version type.
pub type P2PVersion = u64;

/// VersionConfigs struct that represent app version confuration.
pub struct VersionConfigs {
    pub app_version: String,
    pub block_version: u64,
    pub p2p_version: u64,
}

/// Implementation for VersionConfigs struct.
impl VersionConfigs {
    fn log_info(&self) {
        println!("BlockVersion is {}", self.block_version);
        println!("AppVersion is {}", self.app_version);
        println!("P2PVersion is {}", self.p2p_version);
    }
}

/// Method for getting app version configs.
pub fn get_app_configs() -> VersionConfigs {
    let version_configs = VersionConfigs {
        app_version: "0.1.0".to_string(), // version specified at Cargo.toml of `abci` pallet.
        block_version: 0,
        p2p_version: 0,
    };
    version_configs.log_info();
    version_configs
}
