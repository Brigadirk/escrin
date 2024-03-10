use ethers::{
    middleware::contract::{Eip712, EthAbiType},
    types::{Address, H256, U256},
};
use serde::{Deserialize, Serialize};

pub type ChainId = u64;

pub type ShareVersion = u64;
pub type KeyVersion = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IdentityId(pub H256);

impl From<H256> for IdentityId {
    fn from(h: H256) -> Self {
        Self(h)
    }
}

#[cfg(test)]
impl rand::distributions::Distribution<IdentityId> for rand::distributions::Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> IdentityId {
        IdentityId(rng.gen())
    }
}

#[cfg(test)]
impl IdentityId {
    pub fn random() -> Self {
        rand::random()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentityLocator {
    pub chain: u64,
    pub registry: Address,
    pub id: IdentityId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShareId {
    pub identity: IdentityLocator,
    pub version: ShareVersion,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyId {
    pub name: String,
    pub identity: IdentityLocator,
    pub version: KeyVersion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PermitterLocator {
    pub chain: u64,
    pub permitter: Address,
}

impl PermitterLocator {
    pub fn new(chain: u64, permitter: Address) -> Self {
        Self { chain, permitter }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permit {
    pub expiry: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChainState {
    pub block: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChainStateUpdate {
    pub block: Option<u64>,
}

#[derive(Clone, Serialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct SecretShare {
    pub index: u64,
    // TODO: encrypt then zeroize. there's no point of zeroizing before serde w/o encryption
    #[serde(with = "hex::serde")]
    pub share: zeroize::Zeroizing<Vec<u8>>,
    pub commitment: (U256, U256),
}

impl std::fmt::Debug for SecretShare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecretShare")
            .field("commitment", &self.commitment)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Deserialize, zeroize::Zeroize)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct WrappedKey(Vec<u8>);

impl WrappedKey {
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for WrappedKey {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventIndex {
    pub block: u64,
    pub log_index: u64,
}

#[derive(Clone, Default, EthAbiType, Eip712)]
#[eip712(
    name = "SsssRequest",
    version = "1",
    chain_id = 0,
    verifying_contract = "0x0000000000000000000000000000000000000000"
)]
pub struct SsssRequest {
    pub method: String,
    pub uri: String,
    pub body: H256,
}
