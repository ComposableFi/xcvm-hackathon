mod abi {
    pub use bridge_mod::*;
    use ethers::contract::abigen;
    #[allow(clippy::too_many_arguments)]
    mod bridge_mod {
        #![allow(clippy::enum_variant_names)]
        #![allow(dead_code)]
        #![allow(clippy::type_complexity)]
        #![allow(unused_imports)]
        use ethers::contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        };
        use ethers::core::{
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        };
        use ethers::providers::Middleware;
        #[doc = "Bridge was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
        use std::sync::Arc;
        pub static BRIDGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
            ethers::contract::Lazy::new(|| {
                serde_json :: from_str ("{\n  \"abi\": [\n    {\n      \"inputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"constructor\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"assetId\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"string\",\n          \"name\": \"symbol\",\n          \"type\": \"string\"\n        }\n      ],\n      \"name\": \"AssetCreated\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"payload\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"Call\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"string\",\n          \"name\": \"reason\",\n          \"type\": \"string\"\n        }\n      ],\n      \"name\": \"Failed\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint8\",\n          \"name\": \"version\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"name\": \"Initialized\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"networkId\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"account\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"indexed\": false,\n          \"internalType\": \"struct IRouter.Origin\",\n          \"name\": \"origin\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"instance\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"InstanceCreated\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"previousAdminRole\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"newAdminRole\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"RoleAdminChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"RoleGranted\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"RoleRevoked\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"network\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"salt\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"id\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint128\",\n              \"name\": \"amount\",\n              \"type\": \"uint128\"\n            }\n          ],\n          \"indexed\": false,\n          \"internalType\": \"struct IRouter.AssetsAmount[]\",\n          \"name\": \"assets\",\n          \"type\": \"tuple[]\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"string\",\n          \"name\": \"program\",\n          \"type\": \"string\"\n        }\n      ],\n      \"name\": \"Spawn\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"Succeeded\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"id\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint128\",\n              \"name\": \"amount\",\n              \"type\": \"uint128\"\n            }\n          ],\n          \"indexed\": false,\n          \"internalType\": \"struct IRouter.AssetsAmount\",\n          \"name\": \"asset\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"name\": \"Transfer\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"BRIDGE_ADMIN_ROLE\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"DEFAULT_ADMIN_ROLE\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"XCVMs\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"bridgedAssets\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"getRoleAdmin\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"grantRole\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"hasRole\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"relayerAddress\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"renounceRole\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"role\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"revokeRole\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"interfaceId\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"name\": \"supportsInterface\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"upgraderAccount_\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"__Bridge_init\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"networkId\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"account\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct IRouter.Origin\",\n          \"name\": \"origin\",\n          \"type\": \"tuple\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"id\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint128\",\n              \"name\": \"amount\",\n              \"type\": \"uint128\"\n            }\n          ],\n          \"internalType\": \"struct IRouter.AssetsAmount[]\",\n          \"name\": \"assets\",\n          \"type\": \"tuple[]\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"program\",\n          \"type\": \"string\"\n        }\n      ],\n      \"name\": \"bridge\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"network\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"salt\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"id\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint128\",\n              \"name\": \"amount\",\n              \"type\": \"uint128\"\n            }\n          ],\n          \"internalType\": \"struct IRouter.AssetsAmount[]\",\n          \"name\": \"assets\",\n          \"type\": \"tuple[]\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"program\",\n          \"type\": \"string\"\n        }\n      ],\n      \"name\": \"spawn\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"id\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint128\",\n              \"name\": \"amount\",\n              \"type\": \"uint128\"\n            }\n          ],\n          \"internalType\": \"struct IRouter.AssetsAmount\",\n          \"name\": \"asset\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"name\": \"transfer\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"payload\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"call\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"succeeded\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"tag\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"reason\",\n          \"type\": \"string\"\n        }\n      ],\n      \"name\": \"failed\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"assetId\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint128\",\n          \"name\": \"amount\",\n          \"type\": \"uint128\"\n        }\n      ],\n      \"name\": \"burn\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"assetId\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint128\",\n          \"name\": \"amount\",\n          \"type\": \"uint128\"\n        }\n      ],\n      \"name\": \"mint\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    }\n  ]\n}") . expect ("invalid abi")
            });
        pub struct Bridge<M>(ethers::contract::Contract<M>);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<M: ::core::clone::Clone> ::core::clone::Clone for Bridge<M> {
            #[inline]
            fn clone(&self) -> Bridge<M> {
                match *self {
                    Self(ref __self_0_0) => Bridge(::core::clone::Clone::clone(&(*__self_0_0))),
                }
            }
        }
        impl<M> std::ops::Deref for Bridge<M> {
            type Target = ethers::contract::Contract<M>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<M: ethers::providers::Middleware> std::fmt::Debug for Bridge<M> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_tuple("Bridge").field(&self.address()).finish()
            }
        }
        impl<'a, M: ethers::providers::Middleware> Bridge<M> {
            #[doc = r" Creates a new contract instance with the specified `ethers`"]
            #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
            #[doc = r" object"]
            pub fn new<T: Into<ethers::core::types::Address>>(
                address: T,
                client: ::std::sync::Arc<M>,
            ) -> Self {
                let contract =
                    ethers::contract::Contract::new(address.into(), BRIDGE_ABI.clone(), client);
                Self(contract)
            }
            #[doc = "Calls the contract's `BRIDGE_ADMIN_ROLE` (0x118c38c7) function"]
            pub fn bridge_admin_role(
                &self,
            ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
                self.0
                    .method_hash([17, 140, 56, 199], ())
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function"]
            pub fn default_admin_role(
                &self,
            ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
                self.0
                    .method_hash([162, 23, 253, 223], ())
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `XCVMs` (0xb6b95716) function"]
            pub fn xcv_ms(
                &self,
                p0: ethers::core::types::U256,
                p1: ethers::core::types::Bytes,
            ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address>
            {
                self.0
                    .method_hash([182, 185, 87, 22], (p0, p1))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `__Bridge_init` (0xdb1e9776) function"]
            pub fn bridge_init(
                &self,
                upgrader_account: ethers::core::types::Address,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([219, 30, 151, 118], upgrader_account)
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `bridge` (0x26f3ed1e) function"]
            pub fn bridge(
                &self,
                origin: Origin,
                assets: ::std::vec::Vec<AssetsAmount>,
                program: String,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([38, 243, 237, 30], (origin, assets, program))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `bridgedAssets` (0x96776363) function"]
            pub fn bridged_assets(
                &self,
                p0: u32,
            ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address>
            {
                self.0
                    .method_hash([150, 119, 99, 99], p0)
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `burn` (0x3f74e718) function"]
            pub fn burn(
                &self,
                asset_id: u32,
                amount: u128,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([63, 116, 231, 24], (asset_id, amount))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `call` (0x2dcda6f2) function"]
            pub fn call(
                &self,
                tag: ethers::core::types::Bytes,
                to: ethers::core::types::Address,
                payload: ethers::core::types::Bytes,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([45, 205, 166, 242], (tag, to, payload))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `failed` (0x9d6a9246) function"]
            pub fn failed(
                &self,
                tag: ethers::core::types::Bytes,
                reason: String,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([157, 106, 146, 70], (tag, reason))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `getRoleAdmin` (0x248a9ca3) function"]
            pub fn get_role_admin(
                &self,
                role: [u8; 32],
            ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
                self.0
                    .method_hash([36, 138, 156, 163], role)
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `grantRole` (0x2f2ff15d) function"]
            pub fn grant_role(
                &self,
                role: [u8; 32],
                account: ethers::core::types::Address,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([47, 47, 241, 93], (role, account))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `hasRole` (0x91d14854) function"]
            pub fn has_role(
                &self,
                role: [u8; 32],
                account: ethers::core::types::Address,
            ) -> ethers::contract::builders::ContractCall<M, bool> {
                self.0
                    .method_hash([145, 209, 72, 84], (role, account))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `mint` (0x2cb58bb7) function"]
            pub fn mint(
                &self,
                account: ethers::core::types::Address,
                asset_id: u32,
                amount: u128,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([44, 181, 139, 183], (account, asset_id, amount))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `relayerAddress` (0x18a7cca8) function"]
            pub fn relayer_address(
                &self,
            ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address>
            {
                self.0
                    .method_hash([24, 167, 204, 168], ())
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `renounceRole` (0x36568abe) function"]
            pub fn renounce_role(
                &self,
                role: [u8; 32],
                account: ethers::core::types::Address,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([54, 86, 138, 190], (role, account))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `revokeRole` (0xd547741f) function"]
            pub fn revoke_role(
                &self,
                role: [u8; 32],
                account: ethers::core::types::Address,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([213, 71, 116, 31], (role, account))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `spawn` (0x326c9a2d) function"]
            pub fn spawn(
                &self,
                tag: ethers::core::types::Bytes,
                network: u32,
                salt: ethers::core::types::Bytes,
                assets: ::std::vec::Vec<AssetsAmount>,
                program: String,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([50, 108, 154, 45], (tag, network, salt, assets, program))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `succeeded` (0xea3aaa34) function"]
            pub fn succeeded(
                &self,
                tag: ethers::core::types::Bytes,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([234, 58, 170, 52], tag)
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
            pub fn supports_interface(
                &self,
                interface_id: [u8; 4],
            ) -> ethers::contract::builders::ContractCall<M, bool> {
                self.0
                    .method_hash([1, 255, 201, 167], interface_id)
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Calls the contract's `transfer` (0xe731842f) function"]
            pub fn transfer(
                &self,
                tag: ethers::core::types::Bytes,
                to: ethers::core::types::Address,
                asset: AssetsAmount,
            ) -> ethers::contract::builders::ContractCall<M, ()> {
                self.0
                    .method_hash([231, 49, 132, 47], (tag, to, asset))
                    .expect("method not found (this should never happen)")
            }
            #[doc = "Gets the contract's `AssetCreated` event"]
            pub fn asset_created_filter(
                &self,
            ) -> ethers::contract::builders::Event<M, AssetCreatedFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `Call` event"]
            pub fn call_filter(&self) -> ethers::contract::builders::Event<M, CallFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `Failed` event"]
            pub fn failed_filter(&self) -> ethers::contract::builders::Event<M, FailedFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `Initialized` event"]
            pub fn initialized_filter(
                &self,
            ) -> ethers::contract::builders::Event<M, InitializedFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `InstanceCreated` event"]
            pub fn instance_created_filter(
                &self,
            ) -> ethers::contract::builders::Event<M, InstanceCreatedFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `RoleAdminChanged` event"]
            pub fn role_admin_changed_filter(
                &self,
            ) -> ethers::contract::builders::Event<M, RoleAdminChangedFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `RoleGranted` event"]
            pub fn role_granted_filter(
                &self,
            ) -> ethers::contract::builders::Event<M, RoleGrantedFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `RoleRevoked` event"]
            pub fn role_revoked_filter(
                &self,
            ) -> ethers::contract::builders::Event<M, RoleRevokedFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `Spawn` event"]
            pub fn spawn_filter(&self) -> ethers::contract::builders::Event<M, SpawnFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `Succeeded` event"]
            pub fn succeeded_filter(
                &self,
            ) -> ethers::contract::builders::Event<M, SucceededFilter> {
                self.0.event()
            }
            #[doc = "Gets the contract's `Transfer` event"]
            pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
                self.0.event()
            }
            #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
            pub fn events(&self) -> ethers::contract::builders::Event<M, BridgeEvents> {
                self.0.event_with_filter(Default::default())
            }
        }
        #[ethevent(name = "AssetCreated", abi = "AssetCreated(uint32,string)")]
        pub struct AssetCreatedFilter {
            pub asset_id: u32,
            pub symbol: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for AssetCreatedFilter {
            #[inline]
            fn clone(&self) -> AssetCreatedFilter {
                match *self {
                    Self {
                        asset_id: ref __self_0_0,
                        symbol: ref __self_0_1,
                    } => AssetCreatedFilter {
                        asset_id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        symbol: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for AssetCreatedFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        asset_id: ref __self_0_0,
                        symbol: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "AssetCreatedFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "asset_id",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "symbol",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for AssetCreatedFilter {
            #[inline]
            fn default() -> AssetCreatedFilter {
                AssetCreatedFilter {
                    asset_id: ::core::default::Default::default(),
                    symbol: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for AssetCreatedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for AssetCreatedFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                    let _: ::core::cmp::AssertParamIsEq<String>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for AssetCreatedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for AssetCreatedFilter {
            #[inline]
            fn eq(&self, other: &AssetCreatedFilter) -> bool {
                match *other {
                    Self {
                        asset_id: ref __self_1_0,
                        symbol: ref __self_1_1,
                    } => match *self {
                        Self {
                            asset_id: ref __self_0_0,
                            symbol: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &AssetCreatedFilter) -> bool {
                match *other {
                    Self {
                        asset_id: ref __self_1_0,
                        symbol: ref __self_1_1,
                    } => match *self {
                        Self {
                            asset_id: ref __self_0_0,
                            symbol: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for AssetCreatedFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <u32 as ethers::core::abi::AbiType>::param_type(),
                        <String as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for AssetCreatedFilter {}
        impl ethers::core::abi::Tokenizable for AssetCreatedFilter
        where
            u32: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        asset_id: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        symbol: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.asset_id.into_token(),
                        self.symbol.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for AssetCreatedFilter
        where
            u32: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthEvent for AssetCreatedFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "AssetCreated".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    139, 237, 202, 52, 29, 232, 199, 3, 141, 108, 205, 167, 231, 165, 47, 207, 35,
                    90, 160, 51, 118, 123, 245, 31, 201, 161, 216, 135, 155, 235, 213, 147,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "AssetCreated(uint32,string)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = ::alloc::vec::Vec::new();
                let data_types = [
                    ethers::core::abi::ParamType::Uint(32usize),
                    ethers::core::abi::ParamType::String,
                ];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for AssetCreatedFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&self.asset_id)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                self.symbol.fmt(f)?;
                Ok(())
            }
        }
        #[ethevent(name = "Call", abi = "Call(bytes,address,bytes)")]
        pub struct CallFilter {
            pub tag: ethers::core::types::Bytes, // str
            pub to: ethers::core::types::Address,
            pub payload: ethers::core::types::Bytes,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for CallFilter {
            #[inline]
            fn clone(&self) -> CallFilter {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        to: ref __self_0_1,
                        payload: ref __self_0_2,
                    } => CallFilter {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                        to: ::core::clone::Clone::clone(&(*__self_0_1)),
                        payload: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for CallFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        to: ref __self_0_1,
                        payload: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "CallFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "to",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "payload",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for CallFilter {
            #[inline]
            fn default() -> CallFilter {
                CallFilter {
                    tag: ::core::default::Default::default(),
                    to: ::core::default::Default::default(),
                    payload: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for CallFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for CallFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for CallFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for CallFilter {
            #[inline]
            fn eq(&self, other: &CallFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        to: ref __self_1_1,
                        payload: ref __self_1_2,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            to: ref __self_0_1,
                            payload: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &CallFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        to: ref __self_1_1,
                        payload: ref __self_1_2,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            to: ref __self_0_1,
                            payload: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for CallFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for CallFilter {}
        impl ethers::core::abi::Tokenizable for CallFilter
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 3usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&3usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        to: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        payload: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.tag.into_token(),
                        self.to.into_token(),
                        self.payload.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for CallFilter
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthEvent for CallFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "Call".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    178, 105, 221, 57, 63, 94, 2, 255, 26, 251, 105, 55, 205, 60, 72, 180, 138, 32,
                    163, 83, 66, 194, 26, 157, 76, 148, 246, 235, 166, 247, 6, 44,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "Call(bytes,address,bytes)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = ::alloc::vec::Vec::new();
                let data_types = [
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::Address,
                    ethers::core::abi::ParamType::Bytes,
                ];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for CallFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.to)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.payload)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[ethevent(name = "Failed", abi = "Failed(bytes,string)")]
        pub struct FailedFilter {
            pub tag: ethers::core::types::Bytes,
            pub reason: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for FailedFilter {
            #[inline]
            fn clone(&self) -> FailedFilter {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        reason: ref __self_0_1,
                    } => FailedFilter {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                        reason: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for FailedFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        reason: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "FailedFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "reason",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for FailedFilter {
            #[inline]
            fn default() -> FailedFilter {
                FailedFilter {
                    tag: ::core::default::Default::default(),
                    reason: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for FailedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for FailedFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<String>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for FailedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for FailedFilter {
            #[inline]
            fn eq(&self, other: &FailedFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        reason: ref __self_1_1,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            reason: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &FailedFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        reason: ref __self_1_1,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            reason: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for FailedFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <String as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for FailedFilter {}
        impl ethers::core::abi::Tokenizable for FailedFilter
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        reason: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.tag.into_token(), self.reason.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for FailedFilter
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthEvent for FailedFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "Failed".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    178, 192, 167, 176, 37, 105, 51, 210, 17, 127, 236, 46, 14, 231, 26, 158, 24,
                    91, 193, 134, 66, 61, 1, 202, 42, 193, 158, 46, 220, 11, 225, 172,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "Failed(bytes,string)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = ::alloc::vec::Vec::new();
                let data_types = [
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::String,
                ];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for FailedFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                self.reason.fmt(f)?;
                Ok(())
            }
        }
        #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
        pub struct InitializedFilter {
            pub version: u8,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for InitializedFilter {
            #[inline]
            fn clone(&self) -> InitializedFilter {
                match *self {
                    Self {
                        version: ref __self_0_0,
                    } => InitializedFilter {
                        version: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InitializedFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        version: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "InitializedFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "version",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for InitializedFilter {
            #[inline]
            fn default() -> InitializedFilter {
                InitializedFilter {
                    version: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for InitializedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for InitializedFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u8>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for InitializedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for InitializedFilter {
            #[inline]
            fn eq(&self, other: &InitializedFilter) -> bool {
                match *other {
                    Self {
                        version: ref __self_1_0,
                    } => match *self {
                        Self {
                            version: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &InitializedFilter) -> bool {
                match *other {
                    Self {
                        version: ref __self_1_0,
                    } => match *self {
                        Self {
                            version: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for InitializedFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([<u8 as ethers::core::abi::AbiType>::param_type()]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for InitializedFilter {}
        impl ethers::core::abi::Tokenizable for InitializedFilter
        where
            u8: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 1usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&1usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        version: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.version.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for InitializedFilter where u8: ethers::core::abi::Tokenize {}
        impl ethers::contract::EthEvent for InitializedFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "Initialized".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    127, 38, 184, 63, 249, 110, 31, 43, 106, 104, 47, 19, 56, 82, 246, 121, 138, 9,
                    196, 101, 218, 149, 146, 20, 96, 206, 251, 56, 71, 64, 36, 152,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "Initialized(uint8)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = ::alloc::vec::Vec::new();
                let data_types = [ethers::core::abi::ParamType::Uint(8usize)];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for InitializedFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&self.version)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[ethevent(
            name = "InstanceCreated",
            abi = "InstanceCreated((uint32,bytes),address)"
        )]
        pub struct InstanceCreatedFilter {
            pub origin: (u32, ethers::core::types::Bytes),
            pub instance: ethers::core::types::Address,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for InstanceCreatedFilter {
            #[inline]
            fn clone(&self) -> InstanceCreatedFilter {
                match *self {
                    Self {
                        origin: ref __self_0_0,
                        instance: ref __self_0_1,
                    } => InstanceCreatedFilter {
                        origin: ::core::clone::Clone::clone(&(*__self_0_0)),
                        instance: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InstanceCreatedFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        origin: ref __self_0_0,
                        instance: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "InstanceCreatedFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "origin",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "instance",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for InstanceCreatedFilter {
            #[inline]
            fn default() -> InstanceCreatedFilter {
                InstanceCreatedFilter {
                    origin: ::core::default::Default::default(),
                    instance: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for InstanceCreatedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for InstanceCreatedFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<(u32, ethers::core::types::Bytes)>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for InstanceCreatedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for InstanceCreatedFilter {
            #[inline]
            fn eq(&self, other: &InstanceCreatedFilter) -> bool {
                match *other {
                    Self {
                        origin: ref __self_1_0,
                        instance: ref __self_1_1,
                    } => match *self {
                        Self {
                            origin: ref __self_0_0,
                            instance: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &InstanceCreatedFilter) -> bool {
                match *other {
                    Self {
                        origin: ref __self_1_0,
                        instance: ref __self_1_1,
                    } => match *self {
                        Self {
                            origin: ref __self_0_0,
                            instance: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for InstanceCreatedFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers :: core :: abi :: ParamType :: Tuple (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([< (u32 , ethers :: core :: types :: Bytes) as ethers :: core :: abi :: AbiType > :: param_type () , < ethers :: core :: types :: Address as ethers :: core :: abi :: AbiType > :: param_type ()])))
            }
        }
        impl ethers::core::abi::AbiArrayType for InstanceCreatedFilter {}
        impl ethers::core::abi::Tokenizable for InstanceCreatedFilter
        where
            (u32, ethers::core::types::Bytes): ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        origin: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        instance: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.origin.into_token(),
                        self.instance.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for InstanceCreatedFilter
        where
            (u32, ethers::core::types::Bytes): ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthEvent for InstanceCreatedFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "InstanceCreated".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    85, 93, 184, 89, 230, 110, 219, 89, 24, 160, 205, 85, 11, 24, 168, 214, 52,
                    167, 72, 246, 43, 218, 48, 127, 77, 219, 137, 143, 68, 198, 160, 255,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "InstanceCreated((uint32,bytes),address)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = ::alloc::vec::Vec::new();
                let data_types = [
                    ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ethers::core::abi::ParamType::Uint(32usize),
                            ethers::core::abi::ParamType::Bytes,
                        ]),
                    )),
                    ethers::core::abi::ParamType::Address,
                ];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for InstanceCreatedFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.origin)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.instance)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
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
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for RoleAdminChangedFilter {
            #[inline]
            fn clone(&self) -> RoleAdminChangedFilter {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        previous_admin_role: ref __self_0_1,
                        new_admin_role: ref __self_0_2,
                    } => RoleAdminChangedFilter {
                        role: ::core::clone::Clone::clone(&(*__self_0_0)),
                        previous_admin_role: ::core::clone::Clone::clone(&(*__self_0_1)),
                        new_admin_role: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for RoleAdminChangedFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        previous_admin_role: ref __self_0_1,
                        new_admin_role: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "RoleAdminChangedFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "role",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "previous_admin_role",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "new_admin_role",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for RoleAdminChangedFilter {
            #[inline]
            fn default() -> RoleAdminChangedFilter {
                RoleAdminChangedFilter {
                    role: ::core::default::Default::default(),
                    previous_admin_role: ::core::default::Default::default(),
                    new_admin_role: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for RoleAdminChangedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for RoleAdminChangedFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for RoleAdminChangedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for RoleAdminChangedFilter {
            #[inline]
            fn eq(&self, other: &RoleAdminChangedFilter) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        previous_admin_role: ref __self_1_1,
                        new_admin_role: ref __self_1_2,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            previous_admin_role: ref __self_0_1,
                            new_admin_role: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RoleAdminChangedFilter) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        previous_admin_role: ref __self_1_1,
                        new_admin_role: ref __self_1_2,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            previous_admin_role: ref __self_0_1,
                            new_admin_role: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for RoleAdminChangedFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for RoleAdminChangedFilter {}
        impl ethers::core::abi::Tokenizable for RoleAdminChangedFilter
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            [u8; 32]: ethers::core::abi::Tokenize,
            [u8; 32]: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 3usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&3usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        role: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        previous_admin_role: ethers::core::abi::Tokenizable::from_token(
                            iter.next().unwrap(),
                        )?,
                        new_admin_role: ethers::core::abi::Tokenizable::from_token(
                            iter.next().unwrap(),
                        )?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.role.into_token(),
                        self.previous_admin_role.into_token(),
                        self.new_admin_role.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for RoleAdminChangedFilter
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            [u8; 32]: ethers::core::abi::Tokenize,
            [u8; 32]: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthEvent for RoleAdminChangedFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "RoleAdminChanged".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    189, 121, 184, 111, 254, 10, 184, 232, 119, 97, 81, 81, 66, 23, 205, 124, 172,
                    213, 44, 144, 159, 102, 71, 92, 58, 244, 78, 18, 159, 11, 0, 255,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "RoleAdminChanged(bytes32,bytes32,bytes32)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ethers::core::abi::ParamType::FixedBytes(32usize),
                        ethers::core::abi::ParamType::FixedBytes(32usize),
                        ethers::core::abi::ParamType::FixedBytes(32usize),
                    ]),
                );
                let data_types = [];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for RoleAdminChangedFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.role[..]),
                        )],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.previous_admin_role[..]),
                        )],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.new_admin_role[..]),
                        )],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
        pub struct RoleGrantedFilter {
            #[ethevent(indexed)]
            pub role: [u8; 32],
            #[ethevent(indexed)]
            pub account: ethers::core::types::Address,
            #[ethevent(indexed)]
            pub sender: ethers::core::types::Address,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for RoleGrantedFilter {
            #[inline]
            fn clone(&self) -> RoleGrantedFilter {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                        sender: ref __self_0_2,
                    } => RoleGrantedFilter {
                        role: ::core::clone::Clone::clone(&(*__self_0_0)),
                        account: ::core::clone::Clone::clone(&(*__self_0_1)),
                        sender: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for RoleGrantedFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                        sender: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "RoleGrantedFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "role",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "account",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "sender",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for RoleGrantedFilter {
            #[inline]
            fn default() -> RoleGrantedFilter {
                RoleGrantedFilter {
                    role: ::core::default::Default::default(),
                    account: ::core::default::Default::default(),
                    sender: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for RoleGrantedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for RoleGrantedFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for RoleGrantedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for RoleGrantedFilter {
            #[inline]
            fn eq(&self, other: &RoleGrantedFilter) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                        sender: ref __self_1_2,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                            sender: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RoleGrantedFilter) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                        sender: ref __self_1_2,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                            sender: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for RoleGrantedFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for RoleGrantedFilter {}
        impl ethers::core::abi::Tokenizable for RoleGrantedFilter
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 3usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&3usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        role: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        account: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        sender: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.role.into_token(),
                        self.account.into_token(),
                        self.sender.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for RoleGrantedFilter
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthEvent for RoleGrantedFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "RoleGranted".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    47, 135, 136, 17, 126, 126, 255, 29, 130, 233, 38, 236, 121, 73, 1, 209, 124,
                    120, 2, 74, 80, 39, 9, 64, 48, 69, 64, 167, 51, 101, 111, 13,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "RoleGranted(bytes32,address,address)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ethers::core::abi::ParamType::FixedBytes(32usize),
                        ethers::core::abi::ParamType::Address,
                        ethers::core::abi::ParamType::Address,
                    ]),
                );
                let data_types = [];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for RoleGrantedFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.role[..]),
                        )],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.account)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.sender)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
        pub struct RoleRevokedFilter {
            #[ethevent(indexed)]
            pub role: [u8; 32],
            #[ethevent(indexed)]
            pub account: ethers::core::types::Address,
            #[ethevent(indexed)]
            pub sender: ethers::core::types::Address,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for RoleRevokedFilter {
            #[inline]
            fn clone(&self) -> RoleRevokedFilter {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                        sender: ref __self_0_2,
                    } => RoleRevokedFilter {
                        role: ::core::clone::Clone::clone(&(*__self_0_0)),
                        account: ::core::clone::Clone::clone(&(*__self_0_1)),
                        sender: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for RoleRevokedFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                        sender: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "RoleRevokedFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "role",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "account",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "sender",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for RoleRevokedFilter {
            #[inline]
            fn default() -> RoleRevokedFilter {
                RoleRevokedFilter {
                    role: ::core::default::Default::default(),
                    account: ::core::default::Default::default(),
                    sender: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for RoleRevokedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for RoleRevokedFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for RoleRevokedFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for RoleRevokedFilter {
            #[inline]
            fn eq(&self, other: &RoleRevokedFilter) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                        sender: ref __self_1_2,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                            sender: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RoleRevokedFilter) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                        sender: ref __self_1_2,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                            sender: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for RoleRevokedFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for RoleRevokedFilter {}
        impl ethers::core::abi::Tokenizable for RoleRevokedFilter
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 3usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&3usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        role: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        account: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        sender: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.role.into_token(),
                        self.account.into_token(),
                        self.sender.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for RoleRevokedFilter
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthEvent for RoleRevokedFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "RoleRevoked".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    246, 57, 31, 92, 50, 217, 198, 157, 42, 71, 234, 103, 11, 68, 41, 116, 181, 57,
                    53, 209, 237, 199, 253, 100, 235, 33, 224, 71, 168, 57, 23, 27,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "RoleRevoked(bytes32,address,address)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        ethers::core::abi::ParamType::FixedBytes(32usize),
                        ethers::core::abi::ParamType::Address,
                        ethers::core::abi::ParamType::Address,
                    ]),
                );
                let data_types = [];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for RoleRevokedFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.role[..]),
                        )],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.account)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.sender)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[ethevent(
            name = "Spawn",
            abi = "Spawn(bytes,uint256,bytes,(uint32,uint128)[],string)"
        )]
        pub struct SpawnFilter {
            pub tag: ethers::core::types::Bytes,
            pub network: ethers::core::types::U256,
            pub salt: ethers::core::types::Bytes,
            pub assets: Vec<(u32, u128)>,
            pub program: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for SpawnFilter {
            #[inline]
            fn clone(&self) -> SpawnFilter {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        network: ref __self_0_1,
                        salt: ref __self_0_2,
                        assets: ref __self_0_3,
                        program: ref __self_0_4,
                    } => SpawnFilter {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                        network: ::core::clone::Clone::clone(&(*__self_0_1)),
                        salt: ::core::clone::Clone::clone(&(*__self_0_2)),
                        assets: ::core::clone::Clone::clone(&(*__self_0_3)),
                        program: ::core::clone::Clone::clone(&(*__self_0_4)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for SpawnFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        network: ref __self_0_1,
                        salt: ref __self_0_2,
                        assets: ref __self_0_3,
                        program: ref __self_0_4,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "SpawnFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "network",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "salt",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "assets",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "program",
                            &&(*__self_0_4),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for SpawnFilter {
            #[inline]
            fn default() -> SpawnFilter {
                SpawnFilter {
                    tag: ::core::default::Default::default(),
                    network: ::core::default::Default::default(),
                    salt: ::core::default::Default::default(),
                    assets: ::core::default::Default::default(),
                    program: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for SpawnFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for SpawnFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::U256>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<Vec<(u32, u128)>>;
                    let _: ::core::cmp::AssertParamIsEq<String>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for SpawnFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for SpawnFilter {
            #[inline]
            fn eq(&self, other: &SpawnFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        network: ref __self_1_1,
                        salt: ref __self_1_2,
                        assets: ref __self_1_3,
                        program: ref __self_1_4,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            network: ref __self_0_1,
                            salt: ref __self_0_2,
                            assets: ref __self_0_3,
                            program: ref __self_0_4,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &SpawnFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        network: ref __self_1_1,
                        salt: ref __self_1_2,
                        assets: ref __self_1_3,
                        program: ref __self_1_4,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            network: ref __self_0_1,
                            salt: ref __self_0_2,
                            assets: ref __self_0_3,
                            program: ref __self_0_4,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for SpawnFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::U256 as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <Vec<(u32, u128)> as ethers::core::abi::AbiType>::param_type(),
                        <String as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for SpawnFilter {}
        impl ethers::core::abi::Tokenizable for SpawnFilter
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::U256: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            Vec<(u32, u128)>: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 5usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&5usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        network: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        salt: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        assets: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        program: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.tag.into_token(),
                        self.network.into_token(),
                        self.salt.into_token(),
                        self.assets.into_token(),
                        self.program.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for SpawnFilter
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::U256: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            Vec<(u32, u128)>: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthEvent for SpawnFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "Spawn".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    207, 35, 101, 240, 121, 204, 99, 144, 23, 168, 233, 78, 50, 10, 135, 219, 51,
                    60, 151, 230, 217, 83, 220, 0, 15, 82, 1, 183, 226, 233, 39, 94,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "Spawn(bytes,uint256,bytes,(uint32,uint128)[],string)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = ::alloc::vec::Vec::new();
                let data_types = [
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::Uint(256usize),
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::Array(Box::new(
                        ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ethers::core::abi::ParamType::Uint(32usize),
                                ethers::core::abi::ParamType::Uint(128usize),
                            ]),
                        )),
                    )),
                    ethers::core::abi::ParamType::String,
                ];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for SpawnFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.network)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.salt)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&["["], &[]));
                    result
                }?;
                for (idx, val) in self.assets.iter().enumerate() {
                    {
                        let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_debug(&val)],
                        ));
                        result
                    }?;
                    if idx < self.assets.len() - 1 {
                        {
                            let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                            result
                        }?;
                    }
                }
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&["]"], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                self.program.fmt(f)?;
                Ok(())
            }
        }
        #[ethevent(name = "Succeeded", abi = "Succeeded(bytes)")]
        pub struct SucceededFilter {
            pub tag: ethers::core::types::Bytes,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for SucceededFilter {
            #[inline]
            fn clone(&self) -> SucceededFilter {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                    } => SucceededFilter {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for SucceededFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "SucceededFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for SucceededFilter {
            #[inline]
            fn default() -> SucceededFilter {
                SucceededFilter {
                    tag: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for SucceededFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for SucceededFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for SucceededFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for SucceededFilter {
            #[inline]
            fn eq(&self, other: &SucceededFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &SucceededFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for SucceededFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for SucceededFilter {}
        impl ethers::core::abi::Tokenizable for SucceededFilter
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 1usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&1usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.tag.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for SucceededFilter where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize
        {
        }
        impl ethers::contract::EthEvent for SucceededFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "Succeeded".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    103, 152, 38, 233, 237, 216, 49, 229, 111, 62, 137, 136, 56, 127, 97, 151, 60,
                    43, 97, 192, 76, 137, 111, 43, 20, 235, 241, 67, 111, 218, 73, 159,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "Succeeded(bytes)".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = ::alloc::vec::Vec::new();
                let data_types = [ethers::core::abi::ParamType::Bytes];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for SucceededFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[ethevent(name = "Transfer", abi = "Transfer(bytes,address,(uint32,uint128))")]
        pub struct TransferFilter {
            pub tag: ethers::core::types::Bytes,  // str
            pub to: ethers::core::types::Address, // str
            pub asset: (u32, u128),               // id, amnt
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for TransferFilter {
            #[inline]
            fn clone(&self) -> TransferFilter {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        to: ref __self_0_1,
                        asset: ref __self_0_2,
                    } => TransferFilter {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                        to: ::core::clone::Clone::clone(&(*__self_0_1)),
                        asset: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for TransferFilter {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        to: ref __self_0_1,
                        asset: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "TransferFilter");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "to",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "asset",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for TransferFilter {
            #[inline]
            fn default() -> TransferFilter {
                TransferFilter {
                    tag: ::core::default::Default::default(),
                    to: ::core::default::Default::default(),
                    asset: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for TransferFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for TransferFilter {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                    let _: ::core::cmp::AssertParamIsEq<(u32, u128)>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for TransferFilter {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for TransferFilter {
            #[inline]
            fn eq(&self, other: &TransferFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        to: ref __self_1_1,
                        asset: ref __self_1_2,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            to: ref __self_0_1,
                            asset: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &TransferFilter) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        to: ref __self_1_1,
                        asset: ref __self_1_2,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            to: ref __self_0_1,
                            asset: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for TransferFilter {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                        <(u32, u128) as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for TransferFilter {}
        impl ethers::core::abi::Tokenizable for TransferFilter
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            (u32, u128): ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 3usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&3usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        to: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        asset: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.tag.into_token(),
                        self.to.into_token(),
                        self.asset.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for TransferFilter
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            (u32, u128): ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthEvent for TransferFilter {
            fn name() -> ::std::borrow::Cow<'static, str> {
                "Transfer".into()
            }
            fn signature() -> ethers::core::types::H256 {
                ethers::core::types::H256([
                    42, 123, 183, 150, 161, 138, 212, 165, 19, 208, 109, 112, 112, 152, 152, 189,
                    160, 236, 151, 132, 22, 255, 190, 164, 1, 39, 198, 45, 11, 72, 234, 51,
                ])
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "Transfer(bytes,address,(uint32,uint128))".into()
            }
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                let ethers::core::abi::RawLog { data, topics } = log;
                let event_signature = topics.get(0).ok_or(ethers::core::abi::Error::InvalidData)?;
                if event_signature != &Self::signature() {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let topic_types = ::alloc::vec::Vec::new();
                let data_types = [
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::Address,
                    ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ethers::core::abi::ParamType::Uint(32usize),
                            ethers::core::abi::ParamType::Uint(128usize),
                        ]),
                    )),
                ];
                let flat_topics = topics
                    .iter()
                    .skip(1)
                    .flat_map(|t| t.as_ref().to_vec())
                    .collect::<Vec<u8>>();
                let topic_tokens = ethers::core::abi::decode(&topic_types, &flat_topics)?;
                if topic_tokens.len() != topics.len() - 1 {
                    return Err(ethers::core::abi::Error::InvalidData);
                }
                let data_tokens = ethers::core::abi::decode(&data_types, data)?;
                let tokens: Vec<_> = topic_tokens
                    .into_iter()
                    .chain(data_tokens.into_iter())
                    .collect();
                ethers::core::abi::Tokenizable::from_token(ethers::core::abi::Token::Tuple(tokens))
                    .map_err(|_| ethers::core::abi::Error::InvalidData)
            }
            fn is_anonymous() -> bool {
                false
            }
        }
        impl ::std::fmt::Display for TransferFilter {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.to)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.asset)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        pub enum BridgeEvents {
            AssetCreatedFilter(AssetCreatedFilter),
            FailedFilter(FailedFilter),
            InitializedFilter(InitializedFilter),
            InstanceCreatedFilter(InstanceCreatedFilter),
            RoleAdminChangedFilter(RoleAdminChangedFilter),
            RoleGrantedFilter(RoleGrantedFilter),
            RoleRevokedFilter(RoleRevokedFilter),
            CallFilter(CallFilter), // table
            SpawnFilter(SpawnFilter),         // table
            SucceededFilter(SucceededFilter), // toggle
            TransferFilter(TransferFilter),   // table
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BridgeEvents {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&BridgeEvents::AssetCreatedFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "AssetCreatedFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::CallFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "CallFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::FailedFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "FailedFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::InitializedFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "InitializedFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::InstanceCreatedFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "InstanceCreatedFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::RoleAdminChangedFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "RoleAdminChangedFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::RoleGrantedFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "RoleGrantedFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::RoleRevokedFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "RoleRevokedFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::SpawnFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "SpawnFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::SucceededFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "SucceededFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeEvents::TransferFilter(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "TransferFilter");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BridgeEvents {
            #[inline]
            fn clone(&self) -> BridgeEvents {
                match (&*self,) {
                    (&BridgeEvents::AssetCreatedFilter(ref __self_0),) => {
                        BridgeEvents::AssetCreatedFilter(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeEvents::CallFilter(ref __self_0),) => {
                        BridgeEvents::CallFilter(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeEvents::FailedFilter(ref __self_0),) => {
                        BridgeEvents::FailedFilter(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeEvents::InitializedFilter(ref __self_0),) => {
                        BridgeEvents::InitializedFilter(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeEvents::InstanceCreatedFilter(ref __self_0),) => {
                        BridgeEvents::InstanceCreatedFilter(::core::clone::Clone::clone(
                            &(*__self_0),
                        ))
                    }
                    (&BridgeEvents::RoleAdminChangedFilter(ref __self_0),) => {
                        BridgeEvents::RoleAdminChangedFilter(::core::clone::Clone::clone(
                            &(*__self_0),
                        ))
                    }
                    (&BridgeEvents::RoleGrantedFilter(ref __self_0),) => {
                        BridgeEvents::RoleGrantedFilter(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeEvents::RoleRevokedFilter(ref __self_0),) => {
                        BridgeEvents::RoleRevokedFilter(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeEvents::SpawnFilter(ref __self_0),) => {
                        BridgeEvents::SpawnFilter(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeEvents::SucceededFilter(ref __self_0),) => {
                        BridgeEvents::SucceededFilter(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeEvents::TransferFilter(ref __self_0),) => {
                        BridgeEvents::TransferFilter(::core::clone::Clone::clone(&(*__self_0)))
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for BridgeEvents {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for BridgeEvents {
            #[inline]
            fn eq(&self, other: &BridgeEvents) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (
                                &BridgeEvents::AssetCreatedFilter(ref __self_0),
                                &BridgeEvents::AssetCreatedFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::CallFilter(ref __self_0),
                                &BridgeEvents::CallFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::FailedFilter(ref __self_0),
                                &BridgeEvents::FailedFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::InitializedFilter(ref __self_0),
                                &BridgeEvents::InitializedFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::InstanceCreatedFilter(ref __self_0),
                                &BridgeEvents::InstanceCreatedFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::RoleAdminChangedFilter(ref __self_0),
                                &BridgeEvents::RoleAdminChangedFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::RoleGrantedFilter(ref __self_0),
                                &BridgeEvents::RoleGrantedFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::RoleRevokedFilter(ref __self_0),
                                &BridgeEvents::RoleRevokedFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::SpawnFilter(ref __self_0),
                                &BridgeEvents::SpawnFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::SucceededFilter(ref __self_0),
                                &BridgeEvents::SucceededFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeEvents::TransferFilter(ref __self_0),
                                &BridgeEvents::TransferFilter(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        false
                    }
                }
            }
            #[inline]
            fn ne(&self, other: &BridgeEvents) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (
                                &BridgeEvents::AssetCreatedFilter(ref __self_0),
                                &BridgeEvents::AssetCreatedFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::CallFilter(ref __self_0),
                                &BridgeEvents::CallFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::FailedFilter(ref __self_0),
                                &BridgeEvents::FailedFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::InitializedFilter(ref __self_0),
                                &BridgeEvents::InitializedFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::InstanceCreatedFilter(ref __self_0),
                                &BridgeEvents::InstanceCreatedFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::RoleAdminChangedFilter(ref __self_0),
                                &BridgeEvents::RoleAdminChangedFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::RoleGrantedFilter(ref __self_0),
                                &BridgeEvents::RoleGrantedFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::RoleRevokedFilter(ref __self_0),
                                &BridgeEvents::RoleRevokedFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::SpawnFilter(ref __self_0),
                                &BridgeEvents::SpawnFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::SucceededFilter(ref __self_0),
                                &BridgeEvents::SucceededFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeEvents::TransferFilter(ref __self_0),
                                &BridgeEvents::TransferFilter(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        true
                    }
                }
            }
        }
        impl ::core::marker::StructuralEq for BridgeEvents {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for BridgeEvents {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<AssetCreatedFilter>;
                    let _: ::core::cmp::AssertParamIsEq<CallFilter>;
                    let _: ::core::cmp::AssertParamIsEq<FailedFilter>;
                    let _: ::core::cmp::AssertParamIsEq<InitializedFilter>;
                    let _: ::core::cmp::AssertParamIsEq<InstanceCreatedFilter>;
                    let _: ::core::cmp::AssertParamIsEq<RoleAdminChangedFilter>;
                    let _: ::core::cmp::AssertParamIsEq<RoleGrantedFilter>;
                    let _: ::core::cmp::AssertParamIsEq<RoleRevokedFilter>;
                    let _: ::core::cmp::AssertParamIsEq<SpawnFilter>;
                    let _: ::core::cmp::AssertParamIsEq<SucceededFilter>;
                    let _: ::core::cmp::AssertParamIsEq<TransferFilter>;
                }
            }
        }
        impl ethers::core::abi::Tokenizable for BridgeEvents {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let Ok(decoded) = AssetCreatedFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::AssetCreatedFilter(decoded));
                }
                if let Ok(decoded) = CallFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::CallFilter(decoded));
                }
                if let Ok(decoded) = FailedFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::FailedFilter(decoded));
                }
                if let Ok(decoded) = InitializedFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::InitializedFilter(decoded));
                }
                if let Ok(decoded) = InstanceCreatedFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::InstanceCreatedFilter(decoded));
                }
                if let Ok(decoded) = RoleAdminChangedFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::RoleAdminChangedFilter(decoded));
                }
                if let Ok(decoded) = RoleGrantedFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::RoleGrantedFilter(decoded));
                }
                if let Ok(decoded) = RoleRevokedFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::RoleRevokedFilter(decoded));
                }
                if let Ok(decoded) = SpawnFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::SpawnFilter(decoded));
                }
                if let Ok(decoded) = SucceededFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::SucceededFilter(decoded));
                }
                if let Ok(decoded) = TransferFilter::from_token(token.clone()) {
                    return Ok(BridgeEvents::TransferFilter(decoded));
                }
                Err(ethers::core::abi::InvalidOutputType(
                    "Failed to decode all type variants".to_string(),
                ))
            }
            fn into_token(self) -> ethers::core::abi::Token {
                match self {
                    BridgeEvents::AssetCreatedFilter(element) => element.into_token(),
                    BridgeEvents::CallFilter(element) => element.into_token(),
                    BridgeEvents::FailedFilter(element) => element.into_token(),
                    BridgeEvents::InitializedFilter(element) => element.into_token(),
                    BridgeEvents::InstanceCreatedFilter(element) => element.into_token(),
                    BridgeEvents::RoleAdminChangedFilter(element) => element.into_token(),
                    BridgeEvents::RoleGrantedFilter(element) => element.into_token(),
                    BridgeEvents::RoleRevokedFilter(element) => element.into_token(),
                    BridgeEvents::SpawnFilter(element) => element.into_token(),
                    BridgeEvents::SucceededFilter(element) => element.into_token(),
                    BridgeEvents::TransferFilter(element) => element.into_token(),
                }
            }
        }
        impl ethers::core::abi::TokenizableItem for BridgeEvents {}
        impl ethers::contract::EthLogDecode for BridgeEvents {
            fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
            where
                Self: Sized,
            {
                if let Ok(decoded) = AssetCreatedFilter::decode_log(log) {
                    return Ok(BridgeEvents::AssetCreatedFilter(decoded));
                }
                if let Ok(decoded) = CallFilter::decode_log(log) {
                    return Ok(BridgeEvents::CallFilter(decoded));
                }
                if let Ok(decoded) = FailedFilter::decode_log(log) {
                    return Ok(BridgeEvents::FailedFilter(decoded));
                }
                if let Ok(decoded) = InitializedFilter::decode_log(log) {
                    return Ok(BridgeEvents::InitializedFilter(decoded));
                }
                if let Ok(decoded) = InstanceCreatedFilter::decode_log(log) {
                    return Ok(BridgeEvents::InstanceCreatedFilter(decoded));
                }
                if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                    return Ok(BridgeEvents::RoleAdminChangedFilter(decoded));
                }
                if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                    return Ok(BridgeEvents::RoleGrantedFilter(decoded));
                }
                if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                    return Ok(BridgeEvents::RoleRevokedFilter(decoded));
                }
                if let Ok(decoded) = SpawnFilter::decode_log(log) {
                    return Ok(BridgeEvents::SpawnFilter(decoded));
                }
                if let Ok(decoded) = SucceededFilter::decode_log(log) {
                    return Ok(BridgeEvents::SucceededFilter(decoded));
                }
                if let Ok(decoded) = TransferFilter::decode_log(log) {
                    return Ok(BridgeEvents::TransferFilter(decoded));
                }
                Err(ethers::core::abi::Error::InvalidData)
            }
        }
        impl ::std::fmt::Display for BridgeEvents {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    BridgeEvents::AssetCreatedFilter(element) => element.fmt(f),
                    BridgeEvents::CallFilter(element) => element.fmt(f),
                    BridgeEvents::FailedFilter(element) => element.fmt(f),
                    BridgeEvents::InitializedFilter(element) => element.fmt(f),
                    BridgeEvents::InstanceCreatedFilter(element) => element.fmt(f),
                    BridgeEvents::RoleAdminChangedFilter(element) => element.fmt(f),
                    BridgeEvents::RoleGrantedFilter(element) => element.fmt(f),
                    BridgeEvents::RoleRevokedFilter(element) => element.fmt(f),
                    BridgeEvents::SpawnFilter(element) => element.fmt(f),
                    BridgeEvents::SucceededFilter(element) => element.fmt(f),
                    BridgeEvents::TransferFilter(element) => element.fmt(f),
                }
            }
        }
        #[doc = "Container type for all input parameters for the `BRIDGE_ADMIN_ROLE`function with signature `BRIDGE_ADMIN_ROLE()` and selector `[17, 140, 56, 199]`"]
        #[ethcall(name = "BRIDGE_ADMIN_ROLE", abi = "BRIDGE_ADMIN_ROLE()")]
        pub struct BridgeAdminRoleCall;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BridgeAdminRoleCall {
            #[inline]
            fn clone(&self) -> BridgeAdminRoleCall {
                match *self {
                    Self => BridgeAdminRoleCall,
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BridgeAdminRoleCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self => ::core::fmt::Formatter::write_str(f, "BridgeAdminRoleCall"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BridgeAdminRoleCall {
            #[inline]
            fn default() -> BridgeAdminRoleCall {
                BridgeAdminRoleCall {}
            }
        }
        impl ::core::marker::StructuralEq for BridgeAdminRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for BridgeAdminRoleCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        impl ::core::marker::StructuralPartialEq for BridgeAdminRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for BridgeAdminRoleCall {
            #[inline]
            fn eq(&self, other: &BridgeAdminRoleCall) -> bool {
                match *other {
                    Self => match *self {
                        Self => true,
                    },
                }
            }
        }
        impl ethers::core::abi::Tokenizable for BridgeAdminRoleCall {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if !tokens.is_empty() {
                        Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected empty tuple, got "],
                                &[::core::fmt::ArgumentV1::new_debug(&tokens)],
                            ));
                            res
                        }))
                    } else {
                        Ok(BridgeAdminRoleCall {})
                    }
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(::std::vec::Vec::new())
            }
        }
        impl ethers::core::abi::TokenizableItem for BridgeAdminRoleCall {}
        impl ethers::contract::EthCall for BridgeAdminRoleCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "BRIDGE_ADMIN_ROLE".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [17, 140, 56, 199]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "BRIDGE_ADMIN_ROLE()".into()
            }
        }
        impl ethers::core::abi::AbiDecode for BridgeAdminRoleCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for BridgeAdminRoleCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for BridgeAdminRoleCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `DEFAULT_ADMIN_ROLE`function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
        #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
        pub struct DefaultAdminRoleCall;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for DefaultAdminRoleCall {
            #[inline]
            fn clone(&self) -> DefaultAdminRoleCall {
                match *self {
                    Self => DefaultAdminRoleCall,
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for DefaultAdminRoleCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self => ::core::fmt::Formatter::write_str(f, "DefaultAdminRoleCall"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for DefaultAdminRoleCall {
            #[inline]
            fn default() -> DefaultAdminRoleCall {
                DefaultAdminRoleCall {}
            }
        }
        impl ::core::marker::StructuralEq for DefaultAdminRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for DefaultAdminRoleCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        impl ::core::marker::StructuralPartialEq for DefaultAdminRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for DefaultAdminRoleCall {
            #[inline]
            fn eq(&self, other: &DefaultAdminRoleCall) -> bool {
                match *other {
                    Self => match *self {
                        Self => true,
                    },
                }
            }
        }
        impl ethers::core::abi::Tokenizable for DefaultAdminRoleCall {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if !tokens.is_empty() {
                        Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected empty tuple, got "],
                                &[::core::fmt::ArgumentV1::new_debug(&tokens)],
                            ));
                            res
                        }))
                    } else {
                        Ok(DefaultAdminRoleCall {})
                    }
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(::std::vec::Vec::new())
            }
        }
        impl ethers::core::abi::TokenizableItem for DefaultAdminRoleCall {}
        impl ethers::contract::EthCall for DefaultAdminRoleCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "DEFAULT_ADMIN_ROLE".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [162, 23, 253, 223]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "DEFAULT_ADMIN_ROLE()".into()
            }
        }
        impl ethers::core::abi::AbiDecode for DefaultAdminRoleCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for DefaultAdminRoleCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for DefaultAdminRoleCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `XCVMs`function with signature `XCVMs(uint256,bytes)` and selector `[182, 185, 87, 22]`"]
        #[ethcall(name = "XCVMs", abi = "XCVMs(uint256,bytes)")]
        pub struct XcvmsCall(
            pub ethers::core::types::U256,
            pub ethers::core::types::Bytes,
        );
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for XcvmsCall {
            #[inline]
            fn clone(&self) -> XcvmsCall {
                match *self {
                    Self(ref __self_0_0, ref __self_0_1) => XcvmsCall(
                        ::core::clone::Clone::clone(&(*__self_0_0)),
                        ::core::clone::Clone::clone(&(*__self_0_1)),
                    ),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for XcvmsCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self(ref __self_0_0, ref __self_0_1) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "XcvmsCall");
                        let _ =
                            ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                        let _ =
                            ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_1));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for XcvmsCall {
            #[inline]
            fn default() -> XcvmsCall {
                XcvmsCall(
                    ::core::default::Default::default(),
                    ::core::default::Default::default(),
                )
            }
        }
        impl ::core::marker::StructuralEq for XcvmsCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for XcvmsCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::U256>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for XcvmsCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for XcvmsCall {
            #[inline]
            fn eq(&self, other: &XcvmsCall) -> bool {
                match *other {
                    Self(ref __self_1_0, ref __self_1_1) => match *self {
                        Self(ref __self_0_0, ref __self_0_1) => {
                            (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &XcvmsCall) -> bool {
                match *other {
                    Self(ref __self_1_0, ref __self_1_1) => match *self {
                        Self(ref __self_0_0, ref __self_0_1) => {
                            (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for XcvmsCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::U256 as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for XcvmsCall {}
        impl ethers::core::abi::Tokenizable for XcvmsCall
        where
            ethers::core::types::U256: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self(
                        ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    ))
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token(), self.1.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for XcvmsCall
        where
            ethers::core::types::U256: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for XcvmsCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "XCVMs".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [182, 185, 87, 22]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "XCVMs(uint256,bytes)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for XcvmsCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::Uint(256usize),
                    ethers::core::abi::ParamType::Bytes,
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for XcvmsCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for XcvmsCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.0)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.1)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `__Bridge_init`function with signature `__Bridge_init(address)` and selector `[219, 30, 151, 118]`"]
        #[ethcall(name = "__Bridge_init", abi = "__Bridge_init(address)")]
        pub struct BridgeInitCall {
            pub upgrader_account: ethers::core::types::Address,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BridgeInitCall {
            #[inline]
            fn clone(&self) -> BridgeInitCall {
                match *self {
                    Self {
                        upgrader_account: ref __self_0_0,
                    } => BridgeInitCall {
                        upgrader_account: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BridgeInitCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        upgrader_account: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "BridgeInitCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "upgrader_account",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BridgeInitCall {
            #[inline]
            fn default() -> BridgeInitCall {
                BridgeInitCall {
                    upgrader_account: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for BridgeInitCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for BridgeInitCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for BridgeInitCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for BridgeInitCall {
            #[inline]
            fn eq(&self, other: &BridgeInitCall) -> bool {
                match *other {
                    Self {
                        upgrader_account: ref __self_1_0,
                    } => match *self {
                        Self {
                            upgrader_account: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &BridgeInitCall) -> bool {
                match *other {
                    Self {
                        upgrader_account: ref __self_1_0,
                    } => match *self {
                        Self {
                            upgrader_account: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for BridgeInitCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for BridgeInitCall {}
        impl ethers::core::abi::Tokenizable for BridgeInitCall
        where
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 1usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&1usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        upgrader_account: ethers::core::abi::Tokenizable::from_token(
                            iter.next().unwrap(),
                        )?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.upgrader_account.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for BridgeInitCall where
            ethers::core::types::Address: ethers::core::abi::Tokenize
        {
        }
        impl ethers::contract::EthCall for BridgeInitCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "__Bridge_init".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [219, 30, 151, 118]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "__Bridge_init(address)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for BridgeInitCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [ethers::core::abi::ParamType::Address];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for BridgeInitCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for BridgeInitCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.upgrader_account)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `bridge`function with signature `bridge((uint32,bytes),(uint32,uint128)[],string)` and selector `[38, 243, 237, 30]`"]
        #[ethcall(
            name = "bridge",
            abi = "bridge((uint32,bytes),(uint32,uint128)[],string)"
        )]
        pub struct BridgeCall {
            pub origin: Origin,
            pub assets: ::std::vec::Vec<AssetsAmount>,
            pub program: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BridgeCall {
            #[inline]
            fn clone(&self) -> BridgeCall {
                match *self {
                    Self {
                        origin: ref __self_0_0,
                        assets: ref __self_0_1,
                        program: ref __self_0_2,
                    } => BridgeCall {
                        origin: ::core::clone::Clone::clone(&(*__self_0_0)),
                        assets: ::core::clone::Clone::clone(&(*__self_0_1)),
                        program: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BridgeCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        origin: ref __self_0_0,
                        assets: ref __self_0_1,
                        program: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "BridgeCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "origin",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "assets",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "program",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BridgeCall {
            #[inline]
            fn default() -> BridgeCall {
                BridgeCall {
                    origin: ::core::default::Default::default(),
                    assets: ::core::default::Default::default(),
                    program: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for BridgeCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for BridgeCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<Origin>;
                    let _: ::core::cmp::AssertParamIsEq<::std::vec::Vec<AssetsAmount>>;
                    let _: ::core::cmp::AssertParamIsEq<String>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for BridgeCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for BridgeCall {
            #[inline]
            fn eq(&self, other: &BridgeCall) -> bool {
                match *other {
                    Self {
                        origin: ref __self_1_0,
                        assets: ref __self_1_1,
                        program: ref __self_1_2,
                    } => match *self {
                        Self {
                            origin: ref __self_0_0,
                            assets: ref __self_0_1,
                            program: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &BridgeCall) -> bool {
                match *other {
                    Self {
                        origin: ref __self_1_0,
                        assets: ref __self_1_1,
                        program: ref __self_1_2,
                    } => match *self {
                        Self {
                            origin: ref __self_0_0,
                            assets: ref __self_0_1,
                            program: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for BridgeCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <Origin as ethers::core::abi::AbiType>::param_type(),
                        <::std::vec::Vec<AssetsAmount> as ethers::core::abi::AbiType>::param_type(),
                        <String as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for BridgeCall {}
        impl ethers::core::abi::Tokenizable for BridgeCall
        where
            Origin: ethers::core::abi::Tokenize,
            ::std::vec::Vec<AssetsAmount>: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 3usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&3usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        origin: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        assets: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        program: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.origin.into_token(),
                        self.assets.into_token(),
                        self.program.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for BridgeCall
        where
            Origin: ethers::core::abi::Tokenize,
            ::std::vec::Vec<AssetsAmount>: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for BridgeCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "bridge".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [38, 243, 237, 30]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "bridge((uint32,bytes),(uint32,uint128)[],string)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for BridgeCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ethers::core::abi::ParamType::Uint(32usize),
                            ethers::core::abi::ParamType::Bytes,
                        ]),
                    )),
                    ethers::core::abi::ParamType::Array(Box::new(
                        ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ethers::core::abi::ParamType::Uint(32usize),
                                ethers::core::abi::ParamType::Uint(128usize),
                            ]),
                        )),
                    )),
                    ethers::core::abi::ParamType::String,
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for BridgeCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for BridgeCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.origin)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.assets)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                self.program.fmt(f)?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `bridgedAssets`function with signature `bridgedAssets(uint32)` and selector `[150, 119, 99, 99]`"]
        #[ethcall(name = "bridgedAssets", abi = "bridgedAssets(uint32)")]
        pub struct BridgedAssetsCall(pub u32);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BridgedAssetsCall {
            #[inline]
            fn clone(&self) -> BridgedAssetsCall {
                match *self {
                    Self(ref __self_0_0) => {
                        BridgedAssetsCall(::core::clone::Clone::clone(&(*__self_0_0)))
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BridgedAssetsCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self(ref __self_0_0) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "BridgedAssetsCall");
                        let _ =
                            ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BridgedAssetsCall {
            #[inline]
            fn default() -> BridgedAssetsCall {
                BridgedAssetsCall(::core::default::Default::default())
            }
        }
        impl ::core::marker::StructuralEq for BridgedAssetsCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for BridgedAssetsCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for BridgedAssetsCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for BridgedAssetsCall {
            #[inline]
            fn eq(&self, other: &BridgedAssetsCall) -> bool {
                match *other {
                    Self(ref __self_1_0) => match *self {
                        Self(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &BridgedAssetsCall) -> bool {
                match *other {
                    Self(ref __self_1_0) => match *self {
                        Self(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for BridgedAssetsCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([<u32 as ethers::core::abi::AbiType>::param_type()]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for BridgedAssetsCall {}
        impl ethers::core::abi::Tokenizable for BridgedAssetsCall
        where
            u32: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 1usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&1usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self(ethers::core::abi::Tokenizable::from_token(
                        iter.next().unwrap(),
                    )?))
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for BridgedAssetsCall where u32: ethers::core::abi::Tokenize {}
        impl ethers::contract::EthCall for BridgedAssetsCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "bridgedAssets".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [150, 119, 99, 99]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "bridgedAssets(uint32)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for BridgedAssetsCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [ethers::core::abi::ParamType::Uint(32usize)];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for BridgedAssetsCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for BridgedAssetsCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&self.0)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `burn`function with signature `burn(uint32,uint128)` and selector `[63, 116, 231, 24]`"]
        #[ethcall(name = "burn", abi = "burn(uint32,uint128)")]
        pub struct BurnCall {
            pub asset_id: u32,
            pub amount: u128,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BurnCall {
            #[inline]
            fn clone(&self) -> BurnCall {
                match *self {
                    Self {
                        asset_id: ref __self_0_0,
                        amount: ref __self_0_1,
                    } => BurnCall {
                        asset_id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        amount: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BurnCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        asset_id: ref __self_0_0,
                        amount: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "BurnCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "asset_id",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "amount",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for BurnCall {
            #[inline]
            fn default() -> BurnCall {
                BurnCall {
                    asset_id: ::core::default::Default::default(),
                    amount: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for BurnCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for BurnCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                    let _: ::core::cmp::AssertParamIsEq<u128>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for BurnCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for BurnCall {
            #[inline]
            fn eq(&self, other: &BurnCall) -> bool {
                match *other {
                    Self {
                        asset_id: ref __self_1_0,
                        amount: ref __self_1_1,
                    } => match *self {
                        Self {
                            asset_id: ref __self_0_0,
                            amount: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &BurnCall) -> bool {
                match *other {
                    Self {
                        asset_id: ref __self_1_0,
                        amount: ref __self_1_1,
                    } => match *self {
                        Self {
                            asset_id: ref __self_0_0,
                            amount: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for BurnCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <u32 as ethers::core::abi::AbiType>::param_type(),
                        <u128 as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for BurnCall {}
        impl ethers::core::abi::Tokenizable for BurnCall
        where
            u32: ethers::core::abi::Tokenize,
            u128: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        asset_id: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        amount: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.asset_id.into_token(),
                        self.amount.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for BurnCall
        where
            u32: ethers::core::abi::Tokenize,
            u128: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for BurnCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "burn".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [63, 116, 231, 24]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "burn(uint32,uint128)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for BurnCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::Uint(32usize),
                    ethers::core::abi::ParamType::Uint(128usize),
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for BurnCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for BurnCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&self.asset_id)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&self.amount)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `call`function with signature `call(bytes,address,bytes)` and selector `[45, 205, 166, 242]`"]
        #[ethcall(name = "call", abi = "call(bytes,address,bytes)")]
        pub struct CallCall {
            pub tag: ethers::core::types::Bytes,
            pub to: ethers::core::types::Address,
            pub payload: ethers::core::types::Bytes,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for CallCall {
            #[inline]
            fn clone(&self) -> CallCall {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        to: ref __self_0_1,
                        payload: ref __self_0_2,
                    } => CallCall {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                        to: ::core::clone::Clone::clone(&(*__self_0_1)),
                        payload: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for CallCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        to: ref __self_0_1,
                        payload: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "CallCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "to",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "payload",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for CallCall {
            #[inline]
            fn default() -> CallCall {
                CallCall {
                    tag: ::core::default::Default::default(),
                    to: ::core::default::Default::default(),
                    payload: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for CallCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for CallCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for CallCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for CallCall {
            #[inline]
            fn eq(&self, other: &CallCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        to: ref __self_1_1,
                        payload: ref __self_1_2,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            to: ref __self_0_1,
                            payload: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &CallCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        to: ref __self_1_1,
                        payload: ref __self_1_2,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            to: ref __self_0_1,
                            payload: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for CallCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for CallCall {}
        impl ethers::core::abi::Tokenizable for CallCall
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 3usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&3usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        to: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        payload: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.tag.into_token(),
                        self.to.into_token(),
                        self.payload.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for CallCall
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for CallCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "call".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [45, 205, 166, 242]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "call(bytes,address,bytes)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for CallCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::Address,
                    ethers::core::abi::ParamType::Bytes,
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for CallCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for CallCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.to)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.payload)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `failed`function with signature `failed(bytes,string)` and selector `[157, 106, 146, 70]`"]
        #[ethcall(name = "failed", abi = "failed(bytes,string)")]
        pub struct FailedCall {
            pub tag: ethers::core::types::Bytes,
            pub reason: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for FailedCall {
            #[inline]
            fn clone(&self) -> FailedCall {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        reason: ref __self_0_1,
                    } => FailedCall {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                        reason: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for FailedCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        reason: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "FailedCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "reason",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for FailedCall {
            #[inline]
            fn default() -> FailedCall {
                FailedCall {
                    tag: ::core::default::Default::default(),
                    reason: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for FailedCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for FailedCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<String>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for FailedCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for FailedCall {
            #[inline]
            fn eq(&self, other: &FailedCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        reason: ref __self_1_1,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            reason: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &FailedCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        reason: ref __self_1_1,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            reason: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for FailedCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <String as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for FailedCall {}
        impl ethers::core::abi::Tokenizable for FailedCall
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        reason: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.tag.into_token(), self.reason.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for FailedCall
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for FailedCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "failed".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [157, 106, 146, 70]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "failed(bytes,string)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for FailedCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::String,
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for FailedCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for FailedCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                self.reason.fmt(f)?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `getRoleAdmin`function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
        #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
        pub struct GetRoleAdminCall {
            pub role: [u8; 32],
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for GetRoleAdminCall {
            #[inline]
            fn clone(&self) -> GetRoleAdminCall {
                match *self {
                    Self {
                        role: ref __self_0_0,
                    } => GetRoleAdminCall {
                        role: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for GetRoleAdminCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        role: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "GetRoleAdminCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "role",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for GetRoleAdminCall {
            #[inline]
            fn default() -> GetRoleAdminCall {
                GetRoleAdminCall {
                    role: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for GetRoleAdminCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for GetRoleAdminCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for GetRoleAdminCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for GetRoleAdminCall {
            #[inline]
            fn eq(&self, other: &GetRoleAdminCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &GetRoleAdminCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for GetRoleAdminCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for GetRoleAdminCall {}
        impl ethers::core::abi::Tokenizable for GetRoleAdminCall
        where
            [u8; 32]: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 1usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&1usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        role: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.role.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for GetRoleAdminCall where
            [u8; 32]: ethers::core::abi::Tokenize
        {
        }
        impl ethers::contract::EthCall for GetRoleAdminCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "getRoleAdmin".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [36, 138, 156, 163]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "getRoleAdmin(bytes32)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for GetRoleAdminCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [ethers::core::abi::ParamType::FixedBytes(32usize)];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for GetRoleAdminCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for GetRoleAdminCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.role[..]),
                        )],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `grantRole`function with signature `grantRole(bytes32,address)` and selector `[47, 47, 241, 93]`"]
        #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
        pub struct GrantRoleCall {
            pub role: [u8; 32],
            pub account: ethers::core::types::Address,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for GrantRoleCall {
            #[inline]
            fn clone(&self) -> GrantRoleCall {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                    } => GrantRoleCall {
                        role: ::core::clone::Clone::clone(&(*__self_0_0)),
                        account: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for GrantRoleCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "GrantRoleCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "role",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "account",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for GrantRoleCall {
            #[inline]
            fn default() -> GrantRoleCall {
                GrantRoleCall {
                    role: ::core::default::Default::default(),
                    account: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for GrantRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for GrantRoleCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for GrantRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for GrantRoleCall {
            #[inline]
            fn eq(&self, other: &GrantRoleCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &GrantRoleCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for GrantRoleCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for GrantRoleCall {}
        impl ethers::core::abi::Tokenizable for GrantRoleCall
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        role: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        account: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.role.into_token(), self.account.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for GrantRoleCall
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for GrantRoleCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "grantRole".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [47, 47, 241, 93]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "grantRole(bytes32,address)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for GrantRoleCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::FixedBytes(32usize),
                    ethers::core::abi::ParamType::Address,
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for GrantRoleCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for GrantRoleCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.role[..]),
                        )],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.account)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `hasRole`function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
        #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
        pub struct HasRoleCall {
            pub role: [u8; 32],
            pub account: ethers::core::types::Address,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for HasRoleCall {
            #[inline]
            fn clone(&self) -> HasRoleCall {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                    } => HasRoleCall {
                        role: ::core::clone::Clone::clone(&(*__self_0_0)),
                        account: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for HasRoleCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "HasRoleCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "role",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "account",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for HasRoleCall {
            #[inline]
            fn default() -> HasRoleCall {
                HasRoleCall {
                    role: ::core::default::Default::default(),
                    account: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for HasRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for HasRoleCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for HasRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for HasRoleCall {
            #[inline]
            fn eq(&self, other: &HasRoleCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &HasRoleCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for HasRoleCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for HasRoleCall {}
        impl ethers::core::abi::Tokenizable for HasRoleCall
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        role: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        account: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.role.into_token(), self.account.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for HasRoleCall
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for HasRoleCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "hasRole".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [145, 209, 72, 84]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "hasRole(bytes32,address)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for HasRoleCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::FixedBytes(32usize),
                    ethers::core::abi::ParamType::Address,
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for HasRoleCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for HasRoleCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.role[..]),
                        )],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.account)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address,uint32,uint128)` and selector `[44, 181, 139, 183]`"]
        #[ethcall(name = "mint", abi = "mint(address,uint32,uint128)")]
        pub struct MintCall {
            pub account: ethers::core::types::Address,
            pub asset_id: u32,
            pub amount: u128,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for MintCall {
            #[inline]
            fn clone(&self) -> MintCall {
                match *self {
                    Self {
                        account: ref __self_0_0,
                        asset_id: ref __self_0_1,
                        amount: ref __self_0_2,
                    } => MintCall {
                        account: ::core::clone::Clone::clone(&(*__self_0_0)),
                        asset_id: ::core::clone::Clone::clone(&(*__self_0_1)),
                        amount: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for MintCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        account: ref __self_0_0,
                        asset_id: ref __self_0_1,
                        amount: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "MintCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "account",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "asset_id",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "amount",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for MintCall {
            #[inline]
            fn default() -> MintCall {
                MintCall {
                    account: ::core::default::Default::default(),
                    asset_id: ::core::default::Default::default(),
                    amount: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for MintCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for MintCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                    let _: ::core::cmp::AssertParamIsEq<u128>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for MintCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for MintCall {
            #[inline]
            fn eq(&self, other: &MintCall) -> bool {
                match *other {
                    Self {
                        account: ref __self_1_0,
                        asset_id: ref __self_1_1,
                        amount: ref __self_1_2,
                    } => match *self {
                        Self {
                            account: ref __self_0_0,
                            asset_id: ref __self_0_1,
                            amount: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &MintCall) -> bool {
                match *other {
                    Self {
                        account: ref __self_1_0,
                        asset_id: ref __self_1_1,
                        amount: ref __self_1_2,
                    } => match *self {
                        Self {
                            account: ref __self_0_0,
                            asset_id: ref __self_0_1,
                            amount: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for MintCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                        <u32 as ethers::core::abi::AbiType>::param_type(),
                        <u128 as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for MintCall {}
        impl ethers::core::abi::Tokenizable for MintCall
        where
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            u32: ethers::core::abi::Tokenize,
            u128: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 3usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&3usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        account: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        asset_id: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        amount: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.account.into_token(),
                        self.asset_id.into_token(),
                        self.amount.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for MintCall
        where
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            u32: ethers::core::abi::Tokenize,
            u128: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for MintCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "mint".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [44, 181, 139, 183]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "mint(address,uint32,uint128)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for MintCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::Address,
                    ethers::core::abi::ParamType::Uint(32usize),
                    ethers::core::abi::ParamType::Uint(128usize),
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for MintCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for MintCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.account)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&self.asset_id)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&self.amount)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `relayerAddress`function with signature `relayerAddress()` and selector `[24, 167, 204, 168]`"]
        #[ethcall(name = "relayerAddress", abi = "relayerAddress()")]
        pub struct RelayerAddressCall;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for RelayerAddressCall {
            #[inline]
            fn clone(&self) -> RelayerAddressCall {
                match *self {
                    Self => RelayerAddressCall,
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for RelayerAddressCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self => ::core::fmt::Formatter::write_str(f, "RelayerAddressCall"),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for RelayerAddressCall {
            #[inline]
            fn default() -> RelayerAddressCall {
                RelayerAddressCall {}
            }
        }
        impl ::core::marker::StructuralEq for RelayerAddressCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for RelayerAddressCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        impl ::core::marker::StructuralPartialEq for RelayerAddressCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for RelayerAddressCall {
            #[inline]
            fn eq(&self, other: &RelayerAddressCall) -> bool {
                match *other {
                    Self => match *self {
                        Self => true,
                    },
                }
            }
        }
        impl ethers::core::abi::Tokenizable for RelayerAddressCall {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if !tokens.is_empty() {
                        Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected empty tuple, got "],
                                &[::core::fmt::ArgumentV1::new_debug(&tokens)],
                            ));
                            res
                        }))
                    } else {
                        Ok(RelayerAddressCall {})
                    }
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(::std::vec::Vec::new())
            }
        }
        impl ethers::core::abi::TokenizableItem for RelayerAddressCall {}
        impl ethers::contract::EthCall for RelayerAddressCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "relayerAddress".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [24, 167, 204, 168]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "relayerAddress()".into()
            }
        }
        impl ethers::core::abi::AbiDecode for RelayerAddressCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for RelayerAddressCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for RelayerAddressCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `renounceRole`function with signature `renounceRole(bytes32,address)` and selector `[54, 86, 138, 190]`"]
        #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
        pub struct RenounceRoleCall {
            pub role: [u8; 32],
            pub account: ethers::core::types::Address,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for RenounceRoleCall {
            #[inline]
            fn clone(&self) -> RenounceRoleCall {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                    } => RenounceRoleCall {
                        role: ::core::clone::Clone::clone(&(*__self_0_0)),
                        account: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for RenounceRoleCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "RenounceRoleCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "role",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "account",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for RenounceRoleCall {
            #[inline]
            fn default() -> RenounceRoleCall {
                RenounceRoleCall {
                    role: ::core::default::Default::default(),
                    account: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for RenounceRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for RenounceRoleCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for RenounceRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for RenounceRoleCall {
            #[inline]
            fn eq(&self, other: &RenounceRoleCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RenounceRoleCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for RenounceRoleCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for RenounceRoleCall {}
        impl ethers::core::abi::Tokenizable for RenounceRoleCall
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        role: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        account: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.role.into_token(), self.account.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for RenounceRoleCall
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for RenounceRoleCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "renounceRole".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [54, 86, 138, 190]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "renounceRole(bytes32,address)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for RenounceRoleCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::FixedBytes(32usize),
                    ethers::core::abi::ParamType::Address,
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for RenounceRoleCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for RenounceRoleCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.role[..]),
                        )],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.account)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `revokeRole`function with signature `revokeRole(bytes32,address)` and selector `[213, 71, 116, 31]`"]
        #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
        pub struct RevokeRoleCall {
            pub role: [u8; 32],
            pub account: ethers::core::types::Address,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for RevokeRoleCall {
            #[inline]
            fn clone(&self) -> RevokeRoleCall {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                    } => RevokeRoleCall {
                        role: ::core::clone::Clone::clone(&(*__self_0_0)),
                        account: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for RevokeRoleCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        role: ref __self_0_0,
                        account: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "RevokeRoleCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "role",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "account",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for RevokeRoleCall {
            #[inline]
            fn default() -> RevokeRoleCall {
                RevokeRoleCall {
                    role: ::core::default::Default::default(),
                    account: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for RevokeRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for RevokeRoleCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<[u8; 32]>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for RevokeRoleCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for RevokeRoleCall {
            #[inline]
            fn eq(&self, other: &RevokeRoleCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RevokeRoleCall) -> bool {
                match *other {
                    Self {
                        role: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            role: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for RevokeRoleCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <[u8; 32] as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for RevokeRoleCall {}
        impl ethers::core::abi::Tokenizable for RevokeRoleCall
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        role: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        account: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.role.into_token(), self.account.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for RevokeRoleCall
        where
            [u8; 32]: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for RevokeRoleCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "revokeRole".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [213, 71, 116, 31]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "revokeRole(bytes32,address)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for RevokeRoleCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::FixedBytes(32usize),
                    ethers::core::abi::ParamType::Address,
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for RevokeRoleCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for RevokeRoleCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.role[..]),
                        )],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.account)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `spawn`function with signature `spawn(bytes,uint32,bytes,(uint32,uint128)[],string)` and selector `[50, 108, 154, 45]`"]
        #[ethcall(
            name = "spawn",
            abi = "spawn(bytes,uint32,bytes,(uint32,uint128)[],string)"
        )]
        pub struct SpawnCall {
            pub tag: ethers::core::types::Bytes,
            pub network: u32,
            pub salt: ethers::core::types::Bytes,
            pub assets: ::std::vec::Vec<AssetsAmount>,
            pub program: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for SpawnCall {
            #[inline]
            fn clone(&self) -> SpawnCall {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        network: ref __self_0_1,
                        salt: ref __self_0_2,
                        assets: ref __self_0_3,
                        program: ref __self_0_4,
                    } => SpawnCall {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                        network: ::core::clone::Clone::clone(&(*__self_0_1)),
                        salt: ::core::clone::Clone::clone(&(*__self_0_2)),
                        assets: ::core::clone::Clone::clone(&(*__self_0_3)),
                        program: ::core::clone::Clone::clone(&(*__self_0_4)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for SpawnCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        network: ref __self_0_1,
                        salt: ref __self_0_2,
                        assets: ref __self_0_3,
                        program: ref __self_0_4,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "SpawnCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "network",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "salt",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "assets",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "program",
                            &&(*__self_0_4),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for SpawnCall {
            #[inline]
            fn default() -> SpawnCall {
                SpawnCall {
                    tag: ::core::default::Default::default(),
                    network: ::core::default::Default::default(),
                    salt: ::core::default::Default::default(),
                    assets: ::core::default::Default::default(),
                    program: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for SpawnCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for SpawnCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<::std::vec::Vec<AssetsAmount>>;
                    let _: ::core::cmp::AssertParamIsEq<String>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for SpawnCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for SpawnCall {
            #[inline]
            fn eq(&self, other: &SpawnCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        network: ref __self_1_1,
                        salt: ref __self_1_2,
                        assets: ref __self_1_3,
                        program: ref __self_1_4,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            network: ref __self_0_1,
                            salt: ref __self_0_2,
                            assets: ref __self_0_3,
                            program: ref __self_0_4,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &SpawnCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        network: ref __self_1_1,
                        salt: ref __self_1_2,
                        assets: ref __self_1_3,
                        program: ref __self_1_4,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            network: ref __self_0_1,
                            salt: ref __self_0_2,
                            assets: ref __self_0_3,
                            program: ref __self_0_4,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for SpawnCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <u32 as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <::std::vec::Vec<AssetsAmount> as ethers::core::abi::AbiType>::param_type(),
                        <String as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for SpawnCall {}
        impl ethers::core::abi::Tokenizable for SpawnCall
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            u32: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ::std::vec::Vec<AssetsAmount>: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 5usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&5usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        network: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        salt: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        assets: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        program: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.tag.into_token(),
                        self.network.into_token(),
                        self.salt.into_token(),
                        self.assets.into_token(),
                        self.program.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for SpawnCall
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            u32: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ::std::vec::Vec<AssetsAmount>: ethers::core::abi::Tokenize,
            String: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for SpawnCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "spawn".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [50, 108, 154, 45]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "spawn(bytes,uint32,bytes,(uint32,uint128)[],string)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for SpawnCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::Uint(32usize),
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::Array(Box::new(
                        ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ethers::core::abi::ParamType::Uint(32usize),
                                ethers::core::abi::ParamType::Uint(128usize),
                            ]),
                        )),
                    )),
                    ethers::core::abi::ParamType::String,
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for SpawnCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for SpawnCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&self.network)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.salt)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.assets)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                self.program.fmt(f)?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `succeeded`function with signature `succeeded(bytes)` and selector `[234, 58, 170, 52]`"]
        #[ethcall(name = "succeeded", abi = "succeeded(bytes)")]
        pub struct SucceededCall {
            pub tag: ethers::core::types::Bytes,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for SucceededCall {
            #[inline]
            fn clone(&self) -> SucceededCall {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                    } => SucceededCall {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for SucceededCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "SucceededCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for SucceededCall {
            #[inline]
            fn default() -> SucceededCall {
                SucceededCall {
                    tag: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for SucceededCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for SucceededCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for SucceededCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for SucceededCall {
            #[inline]
            fn eq(&self, other: &SucceededCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &SucceededCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for SucceededCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for SucceededCall {}
        impl ethers::core::abi::Tokenizable for SucceededCall
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 1usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&1usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.tag.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for SucceededCall where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize
        {
        }
        impl ethers::contract::EthCall for SucceededCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "succeeded".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [234, 58, 170, 52]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "succeeded(bytes)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for SucceededCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [ethers::core::abi::ParamType::Bytes];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for SucceededCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for SucceededCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `supportsInterface`function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
        #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
        pub struct SupportsInterfaceCall {
            pub interface_id: [u8; 4],
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for SupportsInterfaceCall {
            #[inline]
            fn clone(&self) -> SupportsInterfaceCall {
                match *self {
                    Self {
                        interface_id: ref __self_0_0,
                    } => SupportsInterfaceCall {
                        interface_id: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for SupportsInterfaceCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        interface_id: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "SupportsInterfaceCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "interface_id",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for SupportsInterfaceCall {
            #[inline]
            fn default() -> SupportsInterfaceCall {
                SupportsInterfaceCall {
                    interface_id: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for SupportsInterfaceCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for SupportsInterfaceCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<[u8; 4]>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for SupportsInterfaceCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for SupportsInterfaceCall {
            #[inline]
            fn eq(&self, other: &SupportsInterfaceCall) -> bool {
                match *other {
                    Self {
                        interface_id: ref __self_1_0,
                    } => match *self {
                        Self {
                            interface_id: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &SupportsInterfaceCall) -> bool {
                match *other {
                    Self {
                        interface_id: ref __self_1_0,
                    } => match *self {
                        Self {
                            interface_id: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for SupportsInterfaceCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <[u8; 4] as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for SupportsInterfaceCall {}
        impl ethers::core::abi::Tokenizable for SupportsInterfaceCall
        where
            [u8; 4]: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 1usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&1usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        interface_id: ethers::core::abi::Tokenizable::from_token(
                            iter.next().unwrap(),
                        )?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.interface_id.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for SupportsInterfaceCall where
            [u8; 4]: ethers::core::abi::Tokenize
        {
        }
        impl ethers::contract::EthCall for SupportsInterfaceCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "supportsInterface".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [1, 255, 201, 167]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "supportsInterface(bytes4)".into()
            }
        }
        impl ethers::core::abi::AbiDecode for SupportsInterfaceCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [ethers::core::abi::ParamType::FixedBytes(4usize)];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for SupportsInterfaceCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for SupportsInterfaceCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["0x"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &ethers::core::utils::hex::encode(&self.interface_id[..]),
                        )],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(bytes,address,(uint32,uint128))` and selector `[231, 49, 132, 47]`"]
        #[ethcall(name = "transfer", abi = "transfer(bytes,address,(uint32,uint128))")]
        pub struct TransferCall {
            pub tag: ethers::core::types::Bytes,
            pub to: ethers::core::types::Address,
            pub asset: AssetsAmount,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for TransferCall {
            #[inline]
            fn clone(&self) -> TransferCall {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        to: ref __self_0_1,
                        asset: ref __self_0_2,
                    } => TransferCall {
                        tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                        to: ::core::clone::Clone::clone(&(*__self_0_1)),
                        asset: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for TransferCall {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        tag: ref __self_0_0,
                        to: ref __self_0_1,
                        asset: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "TransferCall");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "tag",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "to",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "asset",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for TransferCall {
            #[inline]
            fn default() -> TransferCall {
                TransferCall {
                    tag: ::core::default::Default::default(),
                    to: ::core::default::Default::default(),
                    asset: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for TransferCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for TransferCall {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Address>;
                    let _: ::core::cmp::AssertParamIsEq<AssetsAmount>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for TransferCall {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for TransferCall {
            #[inline]
            fn eq(&self, other: &TransferCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        to: ref __self_1_1,
                        asset: ref __self_1_2,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            to: ref __self_0_1,
                            asset: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &TransferCall) -> bool {
                match *other {
                    Self {
                        tag: ref __self_1_0,
                        to: ref __self_1_1,
                        asset: ref __self_1_2,
                    } => match *self {
                        Self {
                            tag: ref __self_0_0,
                            to: ref __self_0_1,
                            asset: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for TransferCall {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Address as ethers::core::abi::AbiType>::param_type(),
                        <AssetsAmount as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for TransferCall {}
        impl ethers::core::abi::Tokenizable for TransferCall
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            AssetsAmount: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 3usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&3usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        tag: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        to: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        asset: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.tag.into_token(),
                        self.to.into_token(),
                        self.asset.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for TransferCall
        where
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
            ethers::core::types::Address: ethers::core::abi::Tokenize,
            AssetsAmount: ethers::core::abi::Tokenize,
        {
        }
        impl ethers::contract::EthCall for TransferCall {
            fn function_name() -> ::std::borrow::Cow<'static, str> {
                "transfer".into()
            }
            fn selector() -> ethers::core::types::Selector {
                [231, 49, 132, 47]
            }
            fn abi_signature() -> ::std::borrow::Cow<'static, str> {
                "transfer(bytes,address,(uint32,uint128))".into()
            }
        }
        impl ethers::core::abi::AbiDecode for TransferCall {
            fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                let bytes = bytes.as_ref();
                if bytes.len() < 4 || bytes[..4] != <Self as ethers::contract::EthCall>::selector()
                {
                    return Err(ethers::contract::AbiError::WrongSelector);
                }
                let data_types = [
                    ethers::core::abi::ParamType::Bytes,
                    ethers::core::abi::ParamType::Address,
                    ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ethers::core::abi::ParamType::Uint(32usize),
                            ethers::core::abi::ParamType::Uint(128usize),
                        ]),
                    )),
                ];
                let data_tokens = ethers::core::abi::decode(&data_types, &bytes[4..])?;
                Ok(<Self as ethers::core::abi::Tokenizable>::from_token(
                    ethers::core::abi::Token::Tuple(data_tokens),
                )?)
            }
        }
        impl ethers::core::abi::AbiEncode for TransferCall {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = ethers::core::abi::Tokenize::into_tokens(self);
                let selector = <Self as ethers::contract::EthCall>::selector();
                let encoded = ethers::core::abi::encode(&tokens);
                selector
                    .iter()
                    .copied()
                    .chain(encoded.into_iter())
                    .collect()
            }
        }
        impl ::std::fmt::Display for TransferCall {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.tag)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.to)],
                    ));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(&[", "], &[]));
                    result
                }?;
                {
                    let result = f.write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&&self.asset)],
                    ));
                    result
                }?;
                Ok(())
            }
        }
        pub enum BridgeCalls {
            BridgeAdminRole(BridgeAdminRoleCall),
            DefaultAdminRole(DefaultAdminRoleCall),
            Xcvms(XcvmsCall),
            BridgeInit(BridgeInitCall),
            Bridge(BridgeCall),
            BridgedAssets(BridgedAssetsCall),
            Burn(BurnCall),
            Call(CallCall),
            Failed(FailedCall),
            GetRoleAdmin(GetRoleAdminCall),
            GrantRole(GrantRoleCall),
            HasRole(HasRoleCall),
            Mint(MintCall),
            RelayerAddress(RelayerAddressCall),
            RenounceRole(RenounceRoleCall),
            RevokeRole(RevokeRoleCall),
            Spawn(SpawnCall),
            Succeeded(SucceededCall),
            SupportsInterface(SupportsInterfaceCall),
            Transfer(TransferCall),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for BridgeCalls {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&BridgeCalls::BridgeAdminRole(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "BridgeAdminRole");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::DefaultAdminRole(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "DefaultAdminRole");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::Xcvms(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Xcvms");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::BridgeInit(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "BridgeInit");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::Bridge(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Bridge");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::BridgedAssets(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "BridgedAssets");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::Burn(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Burn");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::Call(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Call");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::Failed(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Failed");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::GetRoleAdmin(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "GetRoleAdmin");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::GrantRole(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "GrantRole");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::HasRole(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "HasRole");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::Mint(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Mint");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::RelayerAddress(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "RelayerAddress");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::RenounceRole(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "RenounceRole");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::RevokeRole(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "RevokeRole");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::Spawn(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Spawn");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::Succeeded(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Succeeded");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::SupportsInterface(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "SupportsInterface");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                    (&BridgeCalls::Transfer(ref __self_0),) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "Transfer");
                        let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for BridgeCalls {
            #[inline]
            fn clone(&self) -> BridgeCalls {
                match (&*self,) {
                    (&BridgeCalls::BridgeAdminRole(ref __self_0),) => {
                        BridgeCalls::BridgeAdminRole(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::DefaultAdminRole(ref __self_0),) => {
                        BridgeCalls::DefaultAdminRole(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::Xcvms(ref __self_0),) => {
                        BridgeCalls::Xcvms(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::BridgeInit(ref __self_0),) => {
                        BridgeCalls::BridgeInit(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::Bridge(ref __self_0),) => {
                        BridgeCalls::Bridge(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::BridgedAssets(ref __self_0),) => {
                        BridgeCalls::BridgedAssets(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::Burn(ref __self_0),) => {
                        BridgeCalls::Burn(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::Call(ref __self_0),) => {
                        BridgeCalls::Call(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::Failed(ref __self_0),) => {
                        BridgeCalls::Failed(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::GetRoleAdmin(ref __self_0),) => {
                        BridgeCalls::GetRoleAdmin(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::GrantRole(ref __self_0),) => {
                        BridgeCalls::GrantRole(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::HasRole(ref __self_0),) => {
                        BridgeCalls::HasRole(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::Mint(ref __self_0),) => {
                        BridgeCalls::Mint(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::RelayerAddress(ref __self_0),) => {
                        BridgeCalls::RelayerAddress(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::RenounceRole(ref __self_0),) => {
                        BridgeCalls::RenounceRole(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::RevokeRole(ref __self_0),) => {
                        BridgeCalls::RevokeRole(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::Spawn(ref __self_0),) => {
                        BridgeCalls::Spawn(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::Succeeded(ref __self_0),) => {
                        BridgeCalls::Succeeded(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::SupportsInterface(ref __self_0),) => {
                        BridgeCalls::SupportsInterface(::core::clone::Clone::clone(&(*__self_0)))
                    }
                    (&BridgeCalls::Transfer(ref __self_0),) => {
                        BridgeCalls::Transfer(::core::clone::Clone::clone(&(*__self_0)))
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for BridgeCalls {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for BridgeCalls {
            #[inline]
            fn eq(&self, other: &BridgeCalls) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (
                                &BridgeCalls::BridgeAdminRole(ref __self_0),
                                &BridgeCalls::BridgeAdminRole(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::DefaultAdminRole(ref __self_0),
                                &BridgeCalls::DefaultAdminRole(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::Xcvms(ref __self_0),
                                &BridgeCalls::Xcvms(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::BridgeInit(ref __self_0),
                                &BridgeCalls::BridgeInit(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::Bridge(ref __self_0),
                                &BridgeCalls::Bridge(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::BridgedAssets(ref __self_0),
                                &BridgeCalls::BridgedAssets(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::Burn(ref __self_0),
                                &BridgeCalls::Burn(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::Call(ref __self_0),
                                &BridgeCalls::Call(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::Failed(ref __self_0),
                                &BridgeCalls::Failed(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::GetRoleAdmin(ref __self_0),
                                &BridgeCalls::GetRoleAdmin(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::GrantRole(ref __self_0),
                                &BridgeCalls::GrantRole(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::HasRole(ref __self_0),
                                &BridgeCalls::HasRole(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::Mint(ref __self_0),
                                &BridgeCalls::Mint(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::RelayerAddress(ref __self_0),
                                &BridgeCalls::RelayerAddress(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::RenounceRole(ref __self_0),
                                &BridgeCalls::RenounceRole(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::RevokeRole(ref __self_0),
                                &BridgeCalls::RevokeRole(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::Spawn(ref __self_0),
                                &BridgeCalls::Spawn(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::Succeeded(ref __self_0),
                                &BridgeCalls::Succeeded(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::SupportsInterface(ref __self_0),
                                &BridgeCalls::SupportsInterface(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            (
                                &BridgeCalls::Transfer(ref __self_0),
                                &BridgeCalls::Transfer(ref __arg_1_0),
                            ) => (*__self_0) == (*__arg_1_0),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        false
                    }
                }
            }
            #[inline]
            fn ne(&self, other: &BridgeCalls) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (
                                &BridgeCalls::BridgeAdminRole(ref __self_0),
                                &BridgeCalls::BridgeAdminRole(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::DefaultAdminRole(ref __self_0),
                                &BridgeCalls::DefaultAdminRole(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::Xcvms(ref __self_0),
                                &BridgeCalls::Xcvms(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::BridgeInit(ref __self_0),
                                &BridgeCalls::BridgeInit(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::Bridge(ref __self_0),
                                &BridgeCalls::Bridge(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::BridgedAssets(ref __self_0),
                                &BridgeCalls::BridgedAssets(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::Burn(ref __self_0),
                                &BridgeCalls::Burn(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::Call(ref __self_0),
                                &BridgeCalls::Call(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::Failed(ref __self_0),
                                &BridgeCalls::Failed(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::GetRoleAdmin(ref __self_0),
                                &BridgeCalls::GetRoleAdmin(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::GrantRole(ref __self_0),
                                &BridgeCalls::GrantRole(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::HasRole(ref __self_0),
                                &BridgeCalls::HasRole(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::Mint(ref __self_0),
                                &BridgeCalls::Mint(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::RelayerAddress(ref __self_0),
                                &BridgeCalls::RelayerAddress(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::RenounceRole(ref __self_0),
                                &BridgeCalls::RenounceRole(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::RevokeRole(ref __self_0),
                                &BridgeCalls::RevokeRole(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::Spawn(ref __self_0),
                                &BridgeCalls::Spawn(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::Succeeded(ref __self_0),
                                &BridgeCalls::Succeeded(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::SupportsInterface(ref __self_0),
                                &BridgeCalls::SupportsInterface(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            (
                                &BridgeCalls::Transfer(ref __self_0),
                                &BridgeCalls::Transfer(ref __arg_1_0),
                            ) => (*__self_0) != (*__arg_1_0),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        true
                    }
                }
            }
        }
        impl ::core::marker::StructuralEq for BridgeCalls {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for BridgeCalls {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<BridgeAdminRoleCall>;
                    let _: ::core::cmp::AssertParamIsEq<DefaultAdminRoleCall>;
                    let _: ::core::cmp::AssertParamIsEq<XcvmsCall>;
                    let _: ::core::cmp::AssertParamIsEq<BridgeInitCall>;
                    let _: ::core::cmp::AssertParamIsEq<BridgeCall>;
                    let _: ::core::cmp::AssertParamIsEq<BridgedAssetsCall>;
                    let _: ::core::cmp::AssertParamIsEq<BurnCall>;
                    let _: ::core::cmp::AssertParamIsEq<CallCall>;
                    let _: ::core::cmp::AssertParamIsEq<FailedCall>;
                    let _: ::core::cmp::AssertParamIsEq<GetRoleAdminCall>;
                    let _: ::core::cmp::AssertParamIsEq<GrantRoleCall>;
                    let _: ::core::cmp::AssertParamIsEq<HasRoleCall>;
                    let _: ::core::cmp::AssertParamIsEq<MintCall>;
                    let _: ::core::cmp::AssertParamIsEq<RelayerAddressCall>;
                    let _: ::core::cmp::AssertParamIsEq<RenounceRoleCall>;
                    let _: ::core::cmp::AssertParamIsEq<RevokeRoleCall>;
                    let _: ::core::cmp::AssertParamIsEq<SpawnCall>;
                    let _: ::core::cmp::AssertParamIsEq<SucceededCall>;
                    let _: ::core::cmp::AssertParamIsEq<SupportsInterfaceCall>;
                    let _: ::core::cmp::AssertParamIsEq<TransferCall>;
                }
            }
        }
        impl ethers::core::abi::Tokenizable for BridgeCalls {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let Ok(decoded) = BridgeAdminRoleCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::BridgeAdminRole(decoded));
                }
                if let Ok(decoded) = DefaultAdminRoleCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::DefaultAdminRole(decoded));
                }
                if let Ok(decoded) = XcvmsCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::Xcvms(decoded));
                }
                if let Ok(decoded) = BridgeInitCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::BridgeInit(decoded));
                }
                if let Ok(decoded) = BridgeCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::Bridge(decoded));
                }
                if let Ok(decoded) = BridgedAssetsCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::BridgedAssets(decoded));
                }
                if let Ok(decoded) = BurnCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::Burn(decoded));
                }
                if let Ok(decoded) = CallCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::Call(decoded));
                }
                if let Ok(decoded) = FailedCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::Failed(decoded));
                }
                if let Ok(decoded) = GetRoleAdminCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::GetRoleAdmin(decoded));
                }
                if let Ok(decoded) = GrantRoleCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::GrantRole(decoded));
                }
                if let Ok(decoded) = HasRoleCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::HasRole(decoded));
                }
                if let Ok(decoded) = MintCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::Mint(decoded));
                }
                if let Ok(decoded) = RelayerAddressCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::RelayerAddress(decoded));
                }
                if let Ok(decoded) = RenounceRoleCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::RenounceRole(decoded));
                }
                if let Ok(decoded) = RevokeRoleCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::RevokeRole(decoded));
                }
                if let Ok(decoded) = SpawnCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::Spawn(decoded));
                }
                if let Ok(decoded) = SucceededCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::Succeeded(decoded));
                }
                if let Ok(decoded) = SupportsInterfaceCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::SupportsInterface(decoded));
                }
                if let Ok(decoded) = TransferCall::from_token(token.clone()) {
                    return Ok(BridgeCalls::Transfer(decoded));
                }
                Err(ethers::core::abi::InvalidOutputType(
                    "Failed to decode all type variants".to_string(),
                ))
            }
            fn into_token(self) -> ethers::core::abi::Token {
                match self {
                    BridgeCalls::BridgeAdminRole(element) => element.into_token(),
                    BridgeCalls::DefaultAdminRole(element) => element.into_token(),
                    BridgeCalls::Xcvms(element) => element.into_token(),
                    BridgeCalls::BridgeInit(element) => element.into_token(),
                    BridgeCalls::Bridge(element) => element.into_token(),
                    BridgeCalls::BridgedAssets(element) => element.into_token(),
                    BridgeCalls::Burn(element) => element.into_token(),
                    BridgeCalls::Call(element) => element.into_token(),
                    BridgeCalls::Failed(element) => element.into_token(),
                    BridgeCalls::GetRoleAdmin(element) => element.into_token(),
                    BridgeCalls::GrantRole(element) => element.into_token(),
                    BridgeCalls::HasRole(element) => element.into_token(),
                    BridgeCalls::Mint(element) => element.into_token(),
                    BridgeCalls::RelayerAddress(element) => element.into_token(),
                    BridgeCalls::RenounceRole(element) => element.into_token(),
                    BridgeCalls::RevokeRole(element) => element.into_token(),
                    BridgeCalls::Spawn(element) => element.into_token(),
                    BridgeCalls::Succeeded(element) => element.into_token(),
                    BridgeCalls::SupportsInterface(element) => element.into_token(),
                    BridgeCalls::Transfer(element) => element.into_token(),
                }
            }
        }
        impl ethers::core::abi::TokenizableItem for BridgeCalls {}
        impl ethers::core::abi::AbiDecode for BridgeCalls {
            fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
                if let Ok(decoded) =
                    <BridgeAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::BridgeAdminRole(decoded));
                }
                if let Ok(decoded) =
                    <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::DefaultAdminRole(decoded));
                }
                if let Ok(decoded) =
                    <XcvmsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::Xcvms(decoded));
                }
                if let Ok(decoded) =
                    <BridgeInitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::BridgeInit(decoded));
                }
                if let Ok(decoded) =
                    <BridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::Bridge(decoded));
                }
                if let Ok(decoded) =
                    <BridgedAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::BridgedAssets(decoded));
                }
                if let Ok(decoded) =
                    <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::Burn(decoded));
                }
                if let Ok(decoded) =
                    <CallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::Call(decoded));
                }
                if let Ok(decoded) =
                    <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::Failed(decoded));
                }
                if let Ok(decoded) =
                    <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::GetRoleAdmin(decoded));
                }
                if let Ok(decoded) =
                    <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::GrantRole(decoded));
                }
                if let Ok(decoded) =
                    <HasRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::HasRole(decoded));
                }
                if let Ok(decoded) =
                    <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::Mint(decoded));
                }
                if let Ok(decoded) =
                    <RelayerAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::RelayerAddress(decoded));
                }
                if let Ok(decoded) =
                    <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::RenounceRole(decoded));
                }
                if let Ok(decoded) =
                    <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::RevokeRole(decoded));
                }
                if let Ok(decoded) =
                    <SpawnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::Spawn(decoded));
                }
                if let Ok(decoded) =
                    <SucceededCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::Succeeded(decoded));
                }
                if let Ok(decoded) =
                    <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::SupportsInterface(decoded));
                }
                if let Ok(decoded) =
                    <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
                {
                    return Ok(BridgeCalls::Transfer(decoded));
                }
                Err(ethers::core::abi::Error::InvalidData.into())
            }
        }
        impl ethers::core::abi::AbiEncode for BridgeCalls {
            fn encode(self) -> Vec<u8> {
                match self {
                    BridgeCalls::BridgeAdminRole(element) => element.encode(),
                    BridgeCalls::DefaultAdminRole(element) => element.encode(),
                    BridgeCalls::Xcvms(element) => element.encode(),
                    BridgeCalls::BridgeInit(element) => element.encode(),
                    BridgeCalls::Bridge(element) => element.encode(),
                    BridgeCalls::BridgedAssets(element) => element.encode(),
                    BridgeCalls::Burn(element) => element.encode(),
                    BridgeCalls::Call(element) => element.encode(),
                    BridgeCalls::Failed(element) => element.encode(),
                    BridgeCalls::GetRoleAdmin(element) => element.encode(),
                    BridgeCalls::GrantRole(element) => element.encode(),
                    BridgeCalls::HasRole(element) => element.encode(),
                    BridgeCalls::Mint(element) => element.encode(),
                    BridgeCalls::RelayerAddress(element) => element.encode(),
                    BridgeCalls::RenounceRole(element) => element.encode(),
                    BridgeCalls::RevokeRole(element) => element.encode(),
                    BridgeCalls::Spawn(element) => element.encode(),
                    BridgeCalls::Succeeded(element) => element.encode(),
                    BridgeCalls::SupportsInterface(element) => element.encode(),
                    BridgeCalls::Transfer(element) => element.encode(),
                }
            }
        }
        impl ::std::fmt::Display for BridgeCalls {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    BridgeCalls::BridgeAdminRole(element) => element.fmt(f),
                    BridgeCalls::DefaultAdminRole(element) => element.fmt(f),
                    BridgeCalls::Xcvms(element) => element.fmt(f),
                    BridgeCalls::BridgeInit(element) => element.fmt(f),
                    BridgeCalls::Bridge(element) => element.fmt(f),
                    BridgeCalls::BridgedAssets(element) => element.fmt(f),
                    BridgeCalls::Burn(element) => element.fmt(f),
                    BridgeCalls::Call(element) => element.fmt(f),
                    BridgeCalls::Failed(element) => element.fmt(f),
                    BridgeCalls::GetRoleAdmin(element) => element.fmt(f),
                    BridgeCalls::GrantRole(element) => element.fmt(f),
                    BridgeCalls::HasRole(element) => element.fmt(f),
                    BridgeCalls::Mint(element) => element.fmt(f),
                    BridgeCalls::RelayerAddress(element) => element.fmt(f),
                    BridgeCalls::RenounceRole(element) => element.fmt(f),
                    BridgeCalls::RevokeRole(element) => element.fmt(f),
                    BridgeCalls::Spawn(element) => element.fmt(f),
                    BridgeCalls::Succeeded(element) => element.fmt(f),
                    BridgeCalls::SupportsInterface(element) => element.fmt(f),
                    BridgeCalls::Transfer(element) => element.fmt(f),
                }
            }
        }
        impl ::std::convert::From<BridgeAdminRoleCall> for BridgeCalls {
            fn from(var: BridgeAdminRoleCall) -> Self {
                BridgeCalls::BridgeAdminRole(var)
            }
        }
        impl ::std::convert::From<DefaultAdminRoleCall> for BridgeCalls {
            fn from(var: DefaultAdminRoleCall) -> Self {
                BridgeCalls::DefaultAdminRole(var)
            }
        }
        impl ::std::convert::From<XcvmsCall> for BridgeCalls {
            fn from(var: XcvmsCall) -> Self {
                BridgeCalls::Xcvms(var)
            }
        }
        impl ::std::convert::From<BridgeInitCall> for BridgeCalls {
            fn from(var: BridgeInitCall) -> Self {
                BridgeCalls::BridgeInit(var)
            }
        }
        impl ::std::convert::From<BridgeCall> for BridgeCalls {
            fn from(var: BridgeCall) -> Self {
                BridgeCalls::Bridge(var)
            }
        }
        impl ::std::convert::From<BridgedAssetsCall> for BridgeCalls {
            fn from(var: BridgedAssetsCall) -> Self {
                BridgeCalls::BridgedAssets(var)
            }
        }
        impl ::std::convert::From<BurnCall> for BridgeCalls {
            fn from(var: BurnCall) -> Self {
                BridgeCalls::Burn(var)
            }
        }
        impl ::std::convert::From<CallCall> for BridgeCalls {
            fn from(var: CallCall) -> Self {
                BridgeCalls::Call(var)
            }
        }
        impl ::std::convert::From<FailedCall> for BridgeCalls {
            fn from(var: FailedCall) -> Self {
                BridgeCalls::Failed(var)
            }
        }
        impl ::std::convert::From<GetRoleAdminCall> for BridgeCalls {
            fn from(var: GetRoleAdminCall) -> Self {
                BridgeCalls::GetRoleAdmin(var)
            }
        }
        impl ::std::convert::From<GrantRoleCall> for BridgeCalls {
            fn from(var: GrantRoleCall) -> Self {
                BridgeCalls::GrantRole(var)
            }
        }
        impl ::std::convert::From<HasRoleCall> for BridgeCalls {
            fn from(var: HasRoleCall) -> Self {
                BridgeCalls::HasRole(var)
            }
        }
        impl ::std::convert::From<MintCall> for BridgeCalls {
            fn from(var: MintCall) -> Self {
                BridgeCalls::Mint(var)
            }
        }
        impl ::std::convert::From<RelayerAddressCall> for BridgeCalls {
            fn from(var: RelayerAddressCall) -> Self {
                BridgeCalls::RelayerAddress(var)
            }
        }
        impl ::std::convert::From<RenounceRoleCall> for BridgeCalls {
            fn from(var: RenounceRoleCall) -> Self {
                BridgeCalls::RenounceRole(var)
            }
        }
        impl ::std::convert::From<RevokeRoleCall> for BridgeCalls {
            fn from(var: RevokeRoleCall) -> Self {
                BridgeCalls::RevokeRole(var)
            }
        }
        impl ::std::convert::From<SpawnCall> for BridgeCalls {
            fn from(var: SpawnCall) -> Self {
                BridgeCalls::Spawn(var)
            }
        }
        impl ::std::convert::From<SucceededCall> for BridgeCalls {
            fn from(var: SucceededCall) -> Self {
                BridgeCalls::Succeeded(var)
            }
        }
        impl ::std::convert::From<SupportsInterfaceCall> for BridgeCalls {
            fn from(var: SupportsInterfaceCall) -> Self {
                BridgeCalls::SupportsInterface(var)
            }
        }
        impl ::std::convert::From<TransferCall> for BridgeCalls {
            fn from(var: TransferCall) -> Self {
                BridgeCalls::Transfer(var)
            }
        }
        #[doc = "`AssetsAmount(uint32,uint128)`"]
        pub struct AssetsAmount {
            pub id: u32,
            pub amount: u128,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for AssetsAmount {
            #[inline]
            fn clone(&self) -> AssetsAmount {
                match *self {
                    Self {
                        id: ref __self_0_0,
                        amount: ref __self_0_1,
                    } => AssetsAmount {
                        id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        amount: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for AssetsAmount {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        id: ref __self_0_0,
                        amount: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "AssetsAmount");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "id",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "amount",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for AssetsAmount {
            #[inline]
            fn default() -> AssetsAmount {
                AssetsAmount {
                    id: ::core::default::Default::default(),
                    amount: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for AssetsAmount {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for AssetsAmount {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                    let _: ::core::cmp::AssertParamIsEq<u128>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for AssetsAmount {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for AssetsAmount {
            #[inline]
            fn eq(&self, other: &AssetsAmount) -> bool {
                match *other {
                    Self {
                        id: ref __self_1_0,
                        amount: ref __self_1_1,
                    } => match *self {
                        Self {
                            id: ref __self_0_0,
                            amount: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &AssetsAmount) -> bool {
                match *other {
                    Self {
                        id: ref __self_1_0,
                        amount: ref __self_1_1,
                    } => match *self {
                        Self {
                            id: ref __self_0_0,
                            amount: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for AssetsAmount {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <u32 as ethers::core::abi::AbiType>::param_type(),
                        <u128 as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for AssetsAmount {}
        impl ethers::core::abi::Tokenizable for AssetsAmount
        where
            u32: ethers::core::abi::Tokenize,
            u128: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        id: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                        amount: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.id.into_token(), self.amount.into_token()]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for AssetsAmount
        where
            u32: ethers::core::abi::Tokenize,
            u128: ethers::core::abi::Tokenize,
        {
        }
        #[doc = "`Origin(uint32,bytes)`"]
        pub struct Origin {
            pub network_id: u32,
            pub account: ethers::core::types::Bytes,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Origin {
            #[inline]
            fn clone(&self) -> Origin {
                match *self {
                    Self {
                        network_id: ref __self_0_0,
                        account: ref __self_0_1,
                    } => Origin {
                        network_id: ::core::clone::Clone::clone(&(*__self_0_0)),
                        account: ::core::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Origin {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Self {
                        network_id: ref __self_0_0,
                        account: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "Origin");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "network_id",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "account",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::default::Default for Origin {
            #[inline]
            fn default() -> Origin {
                Origin {
                    network_id: ::core::default::Default::default(),
                    account: ::core::default::Default::default(),
                }
            }
        }
        impl ::core::marker::StructuralEq for Origin {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for Origin {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                    let _: ::core::cmp::AssertParamIsEq<ethers::core::types::Bytes>;
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for Origin {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for Origin {
            #[inline]
            fn eq(&self, other: &Origin) -> bool {
                match *other {
                    Self {
                        network_id: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            network_id: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &Origin) -> bool {
                match *other {
                    Self {
                        network_id: ref __self_1_0,
                        account: ref __self_1_1,
                    } => match *self {
                        Self {
                            network_id: ref __self_0_0,
                            account: ref __self_0_1,
                        } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        impl ethers::core::abi::AbiType for Origin {
            fn param_type() -> ethers::core::abi::ParamType {
                ethers::core::abi::ParamType::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <u32 as ethers::core::abi::AbiType>::param_type(),
                        <ethers::core::types::Bytes as ethers::core::abi::AbiType>::param_type(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::AbiArrayType for Origin {}
        impl ethers::core::abi::Tokenizable for Origin
        where
            u32: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
            fn from_token(
                token: ethers::core::abi::Token,
            ) -> Result<Self, ethers::core::abi::InvalidOutputType>
            where
                Self: Sized,
            {
                if let ethers::core::abi::Token::Tuple(tokens) = token {
                    if tokens.len() != 2usize {
                        return Err(ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Expected ", " tokens, got ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&2usize),
                                    ::core::fmt::ArgumentV1::new_display(&tokens.len()),
                                    ::core::fmt::ArgumentV1::new_debug(&tokens),
                                ],
                            ));
                            res
                        }));
                    }
                    let mut iter = tokens.into_iter();
                    Ok(Self {
                        network_id: ethers::core::abi::Tokenizable::from_token(
                            iter.next().unwrap(),
                        )?,
                        account: ethers::core::abi::Tokenizable::from_token(iter.next().unwrap())?,
                    })
                } else {
                    Err(ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Expected Tuple, got "],
                            &[::core::fmt::ArgumentV1::new_debug(&token)],
                        ));
                        res
                    }))
                }
            }
            fn into_token(self) -> ethers::core::abi::Token {
                ethers::core::abi::Token::Tuple(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.network_id.into_token(),
                        self.account.into_token(),
                    ]),
                ))
            }
        }
        impl ethers::core::abi::TokenizableItem for Origin
        where
            u32: ethers::core::abi::Tokenize,
            ethers::core::types::Bytes: ethers::core::abi::Tokenize,
        {
        }
    }
}
