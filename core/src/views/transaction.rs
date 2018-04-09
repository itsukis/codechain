// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use cbytes::Bytes;
use ccrypto::blake256;
use ctypes::{H256, U256};

use rlp::Rlp;

/// View onto transaction rlp.
pub struct TransactionView<'a> {
    rlp: Rlp<'a>,
}

impl<'a> TransactionView<'a> {
    /// Creates new view onto block from raw bytes.
    pub fn new(bytes: &'a [u8]) -> TransactionView<'a> {
        TransactionView {
            rlp: Rlp::new(bytes),
        }
    }

    /// Creates new view onto block from rlp.
    pub fn new_from_rlp(rlp: Rlp<'a>) -> TransactionView<'a> {
        TransactionView {
            rlp,
        }
    }

    /// Return reference to underlaying rlp.
    pub fn rlp(&self) -> &Rlp<'a> {
        &self.rlp
    }

    /// Returns transaction hash.
    pub fn hash(&self) -> H256 {
        blake256(self.rlp.as_raw())
    }

    /// Get the nonce field of the transaction.
    pub fn nonce(&self) -> U256 {
        self.rlp.val_at(0)
    }

    /// Get the fee field of the transaction.
    pub fn fee(&self) -> U256 {
        self.rlp.val_at(1)
    }

    /// Get the data field of the transaction.
    pub fn data(&self) -> Bytes {
        self.rlp.val_at(2)
    }

    /// Get the v field of the transaction.
    pub fn v(&self) -> u8 {
        let r: u16 = self.rlp.val_at(3);
        r as u8
    }

    /// Get the r field of the transaction.
    pub fn r(&self) -> U256 {
        self.rlp.val_at(4)
    }

    /// Get the s field of the transaction.
    pub fn s(&self) -> U256 {
        self.rlp.val_at(5)
    }
}
