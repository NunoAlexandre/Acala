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

use super::input::{Input, InputT, Output};
use crate::WeightToGas;
use frame_support::{
	log,
	traits::{Currency, Get},
};
use module_currencies::WeightInfo;
use module_evm::{
	precompiles::Precompile,
	runner::state::{PrecompileFailure, PrecompileOutput, PrecompileResult},
	Context, ExitError, ExitRevert, ExitSucceed,
};
use module_support::Erc20InfoMapping as Erc20InfoMappingT;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use orml_traits::MultiCurrency as MultiCurrencyT;
use primitives::{currency::DexShare, Balance, CurrencyId};
use sp_runtime::{traits::Convert, RuntimeDebug};
use sp_std::{marker::PhantomData, prelude::*};

/// The `MultiCurrency` impl precompile.
///
///
/// `input` data starts with `action` and `currency_id`.
///
/// Actions:
/// - Query total issuance.
/// - Query balance. Rest `input` bytes: `account_id`.
/// - Transfer. Rest `input` bytes: `from`, `to`, `amount`.
pub struct MultiCurrencyPrecompile<R>(PhantomData<R>);

#[module_evm_utility_macro::generate_function_selector]
#[derive(RuntimeDebug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Action {
	QueryName = "name()",
	QuerySymbol = "symbol()",
	QueryDecimals = "decimals()",
	QueryTotalIssuance = "totalSupply()",
	QueryBalance = "balanceOf(address)",
	Transfer = "transfer(address,address,uint256)",
}

impl<Runtime> Precompile for MultiCurrencyPrecompile<Runtime>
where
	Runtime:
		module_currencies::Config + module_evm::Config + module_prices::Config + module_transaction_payment::Config,
	module_currencies::Pallet<Runtime>: MultiCurrencyT<Runtime::AccountId, CurrencyId = CurrencyId, Balance = Balance>,
{
	fn execute(input: &[u8], target_gas: Option<u64>, context: &Context, _is_static: bool) -> PrecompileResult {
		let input = Input::<
			Action,
			Runtime::AccountId,
			<Runtime as module_evm::Config>::AddressMapping,
			Runtime::Erc20InfoMapping,
		>::new(input, target_gas);

		let currency_id =
			Runtime::Erc20InfoMapping::decode_evm_address(context.caller).ok_or_else(|| PrecompileFailure::Revert {
				exit_status: ExitRevert::Reverted,
				output: "invalid currency id".into(),
				cost: target_gas.unwrap_or_default(),
			})?;

		let gas_cost = Pricer::<Runtime>::cost(&input, currency_id)?;

		if let Some(gas_limit) = target_gas {
			if gas_limit < gas_cost {
				return Err(PrecompileFailure::Error {
					exit_status: ExitError::OutOfGas,
				});
			}
		}

		let action = input.action()?;

		log::debug!(target: "evm", "multicurrency: currency id: {:?}", currency_id);

		match action {
			Action::QueryName => {
				let name = Runtime::Erc20InfoMapping::name(currency_id).ok_or_else(|| PrecompileFailure::Revert {
					exit_status: ExitRevert::Reverted,
					output: "Get name failed".into(),
					cost: target_gas.unwrap_or_default(),
				})?;
				log::debug!(target: "evm", "multicurrency: name: {:?}", name);

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::default().encode_bytes(&name),
					logs: Default::default(),
				})
			}
			Action::QuerySymbol => {
				let symbol =
					Runtime::Erc20InfoMapping::symbol(currency_id).ok_or_else(|| PrecompileFailure::Revert {
						exit_status: ExitRevert::Reverted,
						output: "Get symbol failed".into(),
						cost: target_gas.unwrap_or_default(),
					})?;
				log::debug!(target: "evm", "multicurrency: symbol: {:?}", symbol);

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::default().encode_bytes(&symbol),
					logs: Default::default(),
				})
			}
			Action::QueryDecimals => {
				let decimals =
					Runtime::Erc20InfoMapping::decimals(currency_id).ok_or_else(|| PrecompileFailure::Revert {
						exit_status: ExitRevert::Reverted,
						output: "Get decimals failed".into(),
						cost: target_gas.unwrap_or_default(),
					})?;
				log::debug!(target: "evm", "multicurrency: decimals: {:?}", decimals);

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::default().encode_u8(decimals),
					logs: Default::default(),
				})
			}
			Action::QueryTotalIssuance => {
				let total_issuance =
					<Runtime as module_transaction_payment::Config>::MultiCurrency::total_issuance(currency_id);
				log::debug!(target: "evm", "multicurrency: total issuance: {:?}", total_issuance);

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::default().encode_u128(total_issuance),
					logs: Default::default(),
				})
			}
			Action::QueryBalance => {
				let who = input.account_id_at(1)?;
				let balance = if currency_id == <Runtime as module_transaction_payment::Config>::NativeCurrencyId::get()
				{
					<Runtime as module_evm::Config>::Currency::free_balance(&who)
				} else {
					<Runtime as module_transaction_payment::Config>::MultiCurrency::total_balance(currency_id, &who)
				};
				log::debug!(target: "evm", "multicurrency: who: {:?}, balance: {:?}", who, balance);

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::default().encode_u128(balance),
					logs: Default::default(),
				})
			}
			Action::Transfer => {
				let from = input.account_id_at(1)?;
				let to = input.account_id_at(2)?;
				let amount = input.balance_at(3)?;
				log::debug!(target: "evm", "multicurrency: transfer from: {:?}, to: {:?}, amount: {:?}", from, to, amount);

				<module_currencies::Pallet<Runtime> as MultiCurrencyT<Runtime::AccountId>>::transfer(
					currency_id,
					&from,
					&to,
					amount,
				)
				.map_err(|e| PrecompileFailure::Revert {
					exit_status: ExitRevert::Reverted,
					output: Into::<&str>::into(e).as_bytes().to_vec(),
					cost: target_gas.unwrap_or_default(),
				})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: vec![],
					logs: Default::default(),
				})
			}
		}
	}
}

pub struct Pricer<R>(PhantomData<R>);

impl<Runtime> Pricer<Runtime>
where
	Runtime:
		module_currencies::Config + module_evm::Config + module_prices::Config + module_transaction_payment::Config,
{
	pub const BASE_COST: u64 = 200;

	fn cost(
		input: &Input<
			Action,
			Runtime::AccountId,
			<Runtime as module_evm::Config>::AddressMapping,
			Runtime::Erc20InfoMapping,
		>,
		currency_id: CurrencyId,
	) -> Result<u64, PrecompileFailure> {
		let action = input.action()?;
		let cost = match action {
			Action::QueryName | Action::QuerySymbol | Action::QueryDecimals => {
				let cost = Self::read_cost(currency_id);
				cost
			}
			Action::QueryTotalIssuance => {
				let cost = Self::read_cost(currency_id);
				cost
			}
			Action::QueryBalance => {
				let cost = Self::read_cost(currency_id);
				cost
			}
			Action::Transfer => {
				let weight = if currency_id == <Runtime as module_transaction_payment::Config>::NativeCurrencyId::get()
				{
					<Runtime as module_currencies::Config>::WeightInfo::transfer_native_currency()
				} else {
					<Runtime as module_currencies::Config>::WeightInfo::transfer_non_native_currency()
				};

				let cost = WeightToGas::convert(weight);
				cost
			}
		};

		Ok(Self::BASE_COST.saturating_add(cost))
	}

	fn dex_share_read_cost(share: DexShare) -> u64 {
		match share {
			DexShare::Erc20(_) | DexShare::ForeignAsset(_) => WeightToGas::convert(Runtime::DbWeight::get().reads(1)),
			_ => Self::BASE_COST,
		}
	}

	fn read_cost(currency_id: CurrencyId) -> u64 {
		match currency_id {
			CurrencyId::Erc20(_) | CurrencyId::StableAssetPoolToken(_) | CurrencyId::ForeignAsset(_) => {
				WeightToGas::convert(Runtime::DbWeight::get().reads(1))
			}
			CurrencyId::DexShare(symbol_0, symbol_1) => {
				Self::dex_share_read_cost(symbol_0).saturating_add(Self::dex_share_read_cost(symbol_1))
			}
			_ => Self::BASE_COST,
		}
	}
}
