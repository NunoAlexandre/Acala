// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_transaction_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/release/acala
// benchmark
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=module_transaction_payment
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_transaction_payment.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_transaction_payment::WeightInfo for WeightInfo<T> {
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: TransactionPayment AlternativeFeeSwapPath (r:0 w:1)
	fn set_alternative_fee_swap_path() -> Weight {
		(34_312_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: TransactionPayment GlobalFeeSwapPath (r:1 w:1)
	fn set_global_fee_swap_path() -> Weight {
		(19_420_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TransactionPayment GlobalFeeSwapPath (r:1 w:1)
	fn remove_global_fee_swap_path() -> Weight {
		(19_280_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TransactionPayment PoolSize (r:1 w:1)
	// Storage: TransactionPayment AlternativeFeeSwapPath (r:1 w:0)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: TransactionPayment TokenExchangeRate (r:0 w:1)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	fn enable_charge_fee_pool() -> Weight {
		(98_681_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: TransactionPayment TokenExchangeRate (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	// Storage: TransactionPayment PoolSize (r:0 w:1)
	// Storage: EvmAccounts Accounts (r:0 w:1)
	fn disable_charge_fee_pool() -> Weight {
		(105_817_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:0)
	fn on_finalize() -> Weight {
		(10_840_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn with_fee_path() -> Weight {
		(156_000_000 as Weight)
	}

	fn with_fee_currency() -> Weight {
		(193_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}
