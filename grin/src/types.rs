// Copyright 2018 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Server types

use std::convert::From;

use api;
use chain;
use core::core;
use p2p;
use pool;
use store;
use pow;
use wallet;
use core::global::ChainTypes;

/// Error type wrapping underlying module errors.
#[derive(Debug)]
pub enum Error {
	/// Error originating from the core implementation.
	Core(core::block::Error),
	/// Error originating from the db storage.
	Store(store::Error),
	/// Error originating from the blockchain implementation.
	Chain(chain::Error),
	/// Error originating from the peer-to-peer network.
	P2P(p2p::Error),
	/// Error originating from HTTP API calls.
	API(api::Error),
	/// Error originating from wallet API.
	Wallet(wallet::Error),
	/// Error originating from the cuckoo miner
	Cuckoo(pow::cuckoo::Error),
}

impl From<core::block::Error> for Error {
	fn from(e: core::block::Error) -> Error {
		Error::Core(e)
	}
}
impl From<chain::Error> for Error {
	fn from(e: chain::Error) -> Error {
		Error::Chain(e)
	}
}

impl From<p2p::Error> for Error {
	fn from(e: p2p::Error) -> Error {
		Error::P2P(e)
	}
}

impl From<pow::cuckoo::Error> for Error {
	fn from(e: pow::cuckoo::Error) -> Error {
		Error::Cuckoo(e)
	}
}

impl From<store::Error> for Error {
	fn from(e: store::Error) -> Error {
		Error::Store(e)
	}
}

impl From<api::Error> for Error {
	fn from(e: api::Error) -> Error {
		Error::API(e)
	}
}

impl From<wallet::Error> for Error {
	fn from(e: wallet::Error) -> Error {
		Error::Wallet(e)
	}
}

/// Type of seeding the server will use to find other peers on the network.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChainValidationMode {
	/// Run full chain validation after processing every block.
	EveryBlock,
	/// Do not automatically run chain validation during normal block
	/// processing.
	Disabled,
}

impl Default for ChainValidationMode {
	fn default() -> ChainValidationMode {
		ChainValidationMode::Disabled
	}
}

/// Type of seeding the server will use to find other peers on the network.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Seeding {
	/// No seeding, mostly for tests that programmatically connect
	None,
	/// A list of seed addresses provided to the server
	List,
	/// Automatically download a text file with a list of server addresses
	WebStatic,
	/// Automatically get a list of seeds from mutliple DNS
	DNSSeed,
	/// Mostly for tests, where connections are initiated programmatically
	Programmatic,
}

impl Default for Seeding {
	fn default() -> Seeding {
		Seeding::DNSSeed
	}
}

/// Full server configuration, aggregating configurations required for the
/// different components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
	/// Directory under which the rocksdb stores will be created
	pub db_root: String,

	/// Network address for the Rest API HTTP server.
	pub api_http_addr: String,

	/// Setup the server for tests, testnet or mainnet
	#[serde(default)]
	pub chain_type: ChainTypes,

	/// Whether this node is a full archival node or a fast-sync, pruned node
	pub archive_mode: Option<bool>,

	/// Automatically run full chain validation during normal block processing?
	#[serde(default)]
	pub chain_validation_mode: ChainValidationMode,

	/// Method used to get the list of seed nodes for initial bootstrap.
	#[serde(default)]
	pub seeding_type: Seeding,

	/// TODO - move this into p2p_config?
	/// The list of seed nodes, if using Seeding as a seed type
	pub seeds: Option<Vec<String>>,

	/// TODO - move this into p2p_config?
	/// Capabilities expose by this node, also conditions which other peers this
	/// node will have an affinity toward when connection.
	pub capabilities: p2p::Capabilities,

	/// Configuration for the peer-to-peer server
	pub p2p_config: p2p::P2PConfig,

	/// Configuration for the mining daemon
	pub mining_config: Option<pow::types::MinerConfig>,

	/// Transaction pool configuration
	#[serde(default)]
	pub pool_config: pool::PoolConfig,

	/// Whether to skip the sync timeout on startup
	/// (To assist testing on solo chains)
	pub skip_sync_wait: Option<bool>,

	/// Whether to run the TUI
	/// if enabled, this will disable logging to stdout
	pub run_tui: Option<bool>,

	/// Whether to run the wallet listener with the server by default
	pub run_wallet_listener: Option<bool>,
}

impl Default for ServerConfig {
	fn default() -> ServerConfig {
		ServerConfig {
			db_root: ".grin".to_string(),
			api_http_addr: "0.0.0.0:13413".to_string(),
			capabilities: p2p::Capabilities::FULL_NODE,
			seeding_type: Seeding::default(),
			seeds: None,
			p2p_config: p2p::P2PConfig::default(),
			mining_config: Some(pow::types::MinerConfig::default()),
			chain_type: ChainTypes::default(),
			archive_mode: None,
			chain_validation_mode: ChainValidationMode::default(),
			pool_config: pool::PoolConfig::default(),
			skip_sync_wait: None,
			run_tui: None,
			run_wallet_listener: Some(false),
		}
	}
}
