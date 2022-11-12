use std::str::FromStr;

use consensus_types::compute_epoch_at_slot;
use eth_types::eth2::{ForkVersion, Epoch, Slot};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Kiln,
    Ropsten,
    Goerli,
}

impl FromStr for Network {
    type Err = String;
    fn from_str(input: &str) -> Result<Network, Self::Err> {
        match input {
            "mainnet" => Ok(Network::Mainnet),
            "kiln" => Ok(Network::Kiln),
            "ropsten" => Ok(Network::Ropsten),
            "goerli" => Ok(Network::Goerli),
            _ => Err(format!("Unknown network {}", input)),
        }
    }
}

pub struct NetworkConfig {
    pub genesis_validators_root: [u8; 32],
    pub bellatrix_fork_version: ForkVersion,
    pub bellatrix_fork_epoch: u64,
}

impl NetworkConfig {
    pub fn new(network: &Network) -> Self {
        match network {
            Network::Mainnet => Self {
                genesis_validators_root: [
                    0x4b, 0x36, 0x3d, 0xb9, 0x4e, 0x28, 0x61, 0x20, 0xd7, 0x6e, 0xb9, 0x05, 0x34,
                    0x0f, 0xdd, 0x4e, 0x54, 0xbf, 0xe9, 0xf0, 0x6b, 0xf3, 0x3f, 0xf6, 0xcf, 0x5a,
                    0xd2, 0x7f, 0x51, 0x1b, 0xfe, 0x95,
                ],
                bellatrix_fork_version: [0x02, 0x00, 0x00, 0x00],
                bellatrix_fork_epoch: 144896,
            },
            Network::Goerli => Self {
                genesis_validators_root: [
                    0x04, 0x3d, 0xb0, 0xd9, 0xa8, 0x38, 0x13, 0x55, 0x1e, 0xe2, 0xf3, 0x34, 0x50,
                    0xd2, 0x37, 0x97, 0x75, 0x7d, 0x43, 0x09, 0x11, 0xa9, 0x32, 0x05, 0x30, 0xad,
                    0x8a, 0x0e, 0xab, 0xc4, 0x3e, 0xfb,
                ],
                bellatrix_fork_version: [0x02, 0x00, 0x10, 0x20],
                bellatrix_fork_epoch: 112260,
            },
            Network::Kiln => Self {
                genesis_validators_root: [
                    0x99, 0xb0, 0x9f, 0xcd, 0x43, 0xe5, 0x90, 0x52, 0x36, 0xc3, 0x70, 0xf1, 0x84,
                    0x05, 0x6b, 0xec, 0x6e, 0x66, 0x38, 0xcf, 0xc3, 0x1a, 0x32, 0x3b, 0x30, 0x4f,
                    0xc4, 0xaa, 0x78, 0x9c, 0xb4, 0xad,
                ],
                bellatrix_fork_version: [0x70, 0x00, 0x00, 0x71],
                bellatrix_fork_epoch: 150,
            },
            Network::Ropsten => Self {
                genesis_validators_root: [
                    0x44, 0xf1, 0xe5, 0x62, 0x83, 0xca, 0x88, 0xb3, 0x5c, 0x78, 0x9f, 0x7f, 0x44,
                    0x9e, 0x52, 0x33, 0x9b, 0xc1, 0xfe, 0xfe, 0x3a, 0x45, 0x91, 0x3a, 0x43, 0xa6,
                    0xd1, 0x6e, 0xdc, 0xd3, 0x3c, 0xf1,
                ],
                bellatrix_fork_version: [0x80, 0x00, 0x00, 0x71],
                bellatrix_fork_epoch: 750,
            },
        }
    }

    pub fn compute_fork_version(&self, epoch: Epoch) -> Option<ForkVersion> {
        if epoch >= self.bellatrix_fork_epoch {
            return Some(self.bellatrix_fork_version);
        }

        None
    }

    pub fn compute_fork_version_by_slot(&self, slot: Slot) -> Option<ForkVersion> {
        self.compute_fork_version(compute_epoch_at_slot(slot))
    }
}