// Rust Bitcoin Library
// Written in 2014 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Rust Bitcoin Library
//!
//! This is a library for which supports the Bitcoin network protocol and associated
//! primitives. It is designed for Rust programs built to work with the Bitcoin
//! network.
//!
//! It is also written entirely in Rust to illustrate the benefits of strong type
//! safety, including ownership and lifetime, for financial and/or cryptographic
//! software.
//!

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

// Experimental features we need
#![cfg_attr(all(test, feature = "unstable"), feature(test))]

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![deny(broken_intra_doc_links)]

#[cfg(not(any(feature = "std", feature = "no-std")))]
compile_error!("at least one of the `std` or `no-std` features must be enabled");

#[cfg(feature = "no-std")]
#[macro_use]
extern crate alloc;
#[cfg(feature = "no-std")]
extern crate core2;

#[cfg(any(feature = "std", test))]
extern crate core; // for Rust 1.29 and no-std tests

// Re-exported dependencies.
#[macro_use] pub extern crate bitcoin_hashes as hashes;
pub extern crate secp256k1;
pub extern crate bech32;

#[cfg(feature = "no-std")]
extern crate hashbrown;

#[cfg(feature = "base64")] pub extern crate base64;

#[cfg(feature="bitcoinconsensus")] extern crate bitcoinconsensus;
#[cfg(feature = "serde")] #[macro_use] extern crate serde;
#[cfg(all(test, feature = "serde"))] extern crate serde_json;
#[cfg(all(test, feature = "serde"))] extern crate serde_test;
#[cfg(all(test, feature = "serde"))] extern crate bincode;
#[cfg(all(test, feature = "unstable"))] extern crate test;

#[cfg(target_pointer_width = "16")]
compile_error!("rust-bitcoin cannot be used on 16-bit architectures");

#[cfg(test)]
#[macro_use]
mod test_macros;
#[macro_use]
mod internal_macros;
#[cfg(feature = "serde")]
mod serde_utils;

#[macro_use]
pub mod network;
pub mod blockdata;
pub mod util;
pub mod consensus;
pub mod hash_types;
pub mod policy;

pub use hash_types::*;
pub use blockdata::block::Block;
pub use blockdata::block::BlockHeader;
pub use blockdata::script::Script;
pub use blockdata::transaction::Transaction;
pub use blockdata::transaction::TxIn;
pub use blockdata::transaction::TxOut;
pub use blockdata::transaction::OutPoint;
pub use blockdata::transaction::SigHashType;
pub use consensus::encode::VarInt;
pub use network::constants::Network;
pub use util::Error;
pub use util::address::Address;
pub use util::address::AddressType;
pub use util::amount::Amount;
pub use util::amount::Denomination;
pub use util::amount::SignedAmount;
pub use util::merkleblock::MerkleBlock;

pub use util::ecdsa;
pub use util::schnorr;
#[deprecated(since = "0.26.1", note = "Please use `ecdsa::PrivateKey` instead")]
pub use util::ecdsa::PrivateKey;
#[deprecated(since = "0.26.1", note = "Please use `ecdsa::PublicKey` instead")]
pub use util::ecdsa::PublicKey;

#[cfg(feature = "std")]
use std::io;
#[cfg(not(feature = "std"))]
use core2::io;

#[cfg(not(feature = "std"))]
mod io_extras {
    /// A writer which will move data into the void.
    pub struct Sink {
        _priv: (),
    }

    /// Creates an instance of a writer which will successfully consume all data.
    pub const fn sink() -> Sink {
        Sink { _priv: () }
    }

    impl core2::io::Write for Sink {
        #[inline]
        fn write(&mut self, buf: &[u8]) -> core2::io::Result<usize> {
            Ok(buf.len())
        }

        #[inline]
        fn flush(&mut self) -> core2::io::Result<()> {
            Ok(())
        }
    }
}

mod prelude {
    #[cfg(all(not(feature = "std"), not(test)))]
    pub use alloc::{string::{String, ToString}, vec::Vec, boxed::Box, borrow::{Cow, ToOwned}, slice, rc, sync};

    #[cfg(any(feature = "std", test))]
    pub use std::{string::{String, ToString}, vec::Vec, boxed::Box, borrow::{Cow, ToOwned}, slice, rc, sync};

    #[cfg(all(not(feature = "std"), not(test)))]
    pub use alloc::collections::{BTreeMap, btree_map};

    #[cfg(any(feature = "std", test))]
    pub use std::collections::{BTreeMap, btree_map};

    #[cfg(feature = "std")]
    pub use std::io::sink;

    #[cfg(not(feature = "std"))]
    pub use io_extras::sink;

    #[cfg(feature = "hashbrown")]
    pub use hashbrown::HashSet;

    #[cfg(not(feature = "hashbrown"))]
    pub use std::collections::HashSet;
}

#[cfg(all(test, feature = "unstable"))] use tests::EmptyWrite;

#[cfg(all(test, feature = "unstable"))]
mod tests {
    use core::fmt::Arguments;
    use io::{IoSlice, Result, Write};

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
    pub struct EmptyWrite;

    impl Write for EmptyWrite {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_vectored(&mut self, bufs: &[IoSlice]) -> Result<usize> {
            Ok(bufs.iter().map(|s| s.len()).sum())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }

        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }
        fn write_fmt(&mut self, _: Arguments) -> Result<()> {
            Ok(())
        }
    }
}
