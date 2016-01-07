//! Contract execution environment.

use std::collections::HashSet;
use util::hash::*;
use util::uint::*;
use util::bytes::*;
use state::*;
use env_info::*;
use evm::LogEntry;

struct SubState {
	// any accounts that have suicided
	suicides: HashSet<Address>,
	// any logs
	logs: Vec<LogEntry>,
	// refund counter of SSTORE nonzero->zero
	refunds: U256
}

impl SubState {
	fn new() -> SubState {
		SubState {
			suicides: HashSet::new(),
			logs: vec![],
			refunds: U256::zero()
		}
	}
}

/// Externality interface for the Virtual Machine providing access to 
/// world state.
/// 
/// ```markdown
/// extern crate ethcore_util as util;
/// extern crate ethcore;
/// use util::hash::*;
/// use ethcore::state::*;
/// use ethcore::env_info::*;
/// use ethcore::evm::*;
///
/// fn main() {
/// 	let address = Address::from_str("0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6").unwrap();
/// 	let mut data = RuntimeData::new();
/// 	let mut ext = Ext::new(EnvInfo::new(), State::new_temp(), address);
/// }	
/// ```
pub struct Ext {
	info: EnvInfo,
	state: State,
	address: Address,
	substate: SubState
}

impl Ext {
	/// Creates new evm environment object with backing state.
	pub fn new(info: EnvInfo, state: State, address: Address) -> Ext {
		Ext {
			info: info,
			state: state,
			address: address,
			substate: SubState::new()
		}
	}

	/// Returns a value for given key.
	pub fn sload(&self, key: &H256) -> H256 {
		self.state.storage_at(&self.address, key)
	}

	/// Stores a value for given key.
	pub fn sstore(&mut self, key: H256, value: H256) {
		self.state.set_storage(&self.address, key, value)
	}

	/// Returns address balance.
	pub fn balance(&self, address: &Address) -> U256 {
		self.state.balance(address)
	}

	/// Returns the hash of one of the 256 most recent complete blocks.
	pub fn blockhash(&self, number: &U256) -> H256 {
		match *number < self.info.number {
			false => H256::from(&U256::zero()),
			true => {
				let index = self.info.number - *number - U256::one();
				self.info.last_hashes[index.low_u32() as usize].clone()
			}
		}
	}

	/// Creates new contract.
	/// Returns new contract address and gas used.
	pub fn create(&self, _gas: u64, _endowment: &U256, _code: &[u8]) -> (Address, u64) {
		unimplemented!();
	}

	/// Calls existing contract.
	/// Returns call output and gas used.
	pub fn call(&self, _gas: u64, _call_gas: u64, _receive_address: &Address, _value: &U256, _data: &[u8], _code_address: &Address) -> Option<(Vec<u8>, u64)>{
		unimplemented!();
	}

	/// Returns code at given address
	pub fn extcode(&self, address: &Address) -> Vec<u8> {
		self.state.code(address).unwrap_or(vec![])
	}

	/// Creates log entry with given topics and data
	pub fn log(&mut self, topics: Vec<H256>, data: Bytes) {
		let address = self.address.clone();
		self.substate.logs.push(LogEntry::new(address, topics, data));
	}

	/// Returns state
	// not sure if this is the best solution, but seems to be the easiest one, mk
	pub fn state(&self) -> &State {
		&self.state
	}

	/// Returns substate
	pub fn logs(&self) -> &[LogEntry] {
		&self.substate.logs
	}
}


