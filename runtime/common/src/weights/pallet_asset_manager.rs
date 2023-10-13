// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_asset_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_asset_manager
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_asset_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_manager::WeightInfo for WeightInfo<T> {
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(174), added: 2649, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetTypeId (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	fn register_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `82`
		//  Estimated: `3639`
		// Minimum execution time: 18_825_000 picoseconds.
		Weight::from_parts(19_383_000, 0)
			.saturating_add(Weight::from_parts(0, 3639))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: AssetManager AssetTypeId (r:1 w:0)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn set_asset_units_per_second(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `611 + x * (9 ±0)`
		//  Estimated: `4000 + x * (10 ±0)`
		// Minimum execution time: 13_957_000 picoseconds.
		Weight::from_parts(15_527_752, 0)
			.saturating_add(Weight::from_parts(0, 4000))
			// Standard Error: 2_293
			.saturating_add(Weight::from_parts(285_056, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:1 w:2)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeId (r:0 w:2)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn change_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `926 + x * (13 ±0)`
		//  Estimated: `4309 + x * (15 ±0)`
		// Minimum execution time: 19_724_000 picoseconds.
		Weight::from_parts(22_299_450, 0)
			.saturating_add(Weight::from_parts(0, 4309))
			// Standard Error: 3_815
			.saturating_add(Weight::from_parts(335_242, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(Weight::from_parts(0, 15).saturating_mul(x.into()))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn remove_supported_asset(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + x * (5 ±0)`
		//  Estimated: `1678 + x * (5 ±0)`
		// Minimum execution time: 10_516_000 picoseconds.
		Weight::from_parts(10_939_301, 0)
			.saturating_add(Weight::from_parts(0, 1678))
			// Standard Error: 1_969
			.saturating_add(Weight::from_parts(279_966, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 5).saturating_mul(x.into()))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: AssetManager LocalAssetCounter (r:1 w:1)
	/// Proof Skipped: AssetManager LocalAssetCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: LocalAssets Asset (r:1 w:1)
	/// Proof: LocalAssets Asset (max_values: None, max_size: Some(174), added: 2649, mode: MaxEncodedLen)
	/// Storage: EVM AccountCodes (r:0 w:1)
	/// Proof Skipped: EVM AccountCodes (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager LocalAssetDeposit (r:0 w:1)
	/// Proof Skipped: AssetManager LocalAssetDeposit (max_values: None, max_size: None, mode: Measured)
	fn register_local_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `310`
		//  Estimated: `3639`
		// Minimum execution time: 25_860_000 picoseconds.
		Weight::from_parts(26_303_000, 0)
			.saturating_add(Weight::from_parts(0, 3639))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: AssetManager SupportedFeePaymentAssets (r:1 w:1)
	/// Proof Skipped: AssetManager SupportedFeePaymentAssets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdType (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdType (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeUnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeUnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetTypeId (r:0 w:1)
	/// Proof Skipped: AssetManager AssetTypeId (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[5, 100]`.
	fn remove_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + x * (10 ±0)`
		//  Estimated: `3955 + x * (10 ±0)`
		// Minimum execution time: 14_838_000 picoseconds.
		Weight::from_parts(16_794_374, 0)
			.saturating_add(Weight::from_parts(0, 3955))
			// Standard Error: 2_454
			.saturating_add(Weight::from_parts(282_694, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
}
