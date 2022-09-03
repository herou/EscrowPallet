#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

//! Struct Storage
//! This pallet demonstrates how to declare and store `strcuts` that contain types
//! that come from the pallet's configuration trait.

use frame_support::{Parameter, sp_runtime};
use frame_support::traits::{Currency, LockIdentifier, Time};
use sp_runtime::traits::*;
pub use pallet::*;
use codec::{Decode, Encode, FullCodec, MaxEncodedLen};

const EXAMPLE_ID: LockIdentifier = *b"example ";

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::Origin;
	use frame_system::pallet_prelude::*;
	use core::convert::TryInto;

	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement::AllowDeath, ReservableCurrency},
	};
	use frame_support::storage::child::get;
	use frame_support::traits::{LockableCurrency, Time, WithdrawReasons};
	use crate::{BalanceOf, EXAMPLE_ID};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The lockable currency type
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}


	#[derive(Clone, Copy, Encode, Decode, RuntimeDebug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
	pub struct Contract<Origin, AccountId, Amount> {
		pub origin: Origin,
		pub to: AccountId,
		pub amount: Amount,
		pub current_block_number: u64,
		pub work_days_in_block_number: u64,
		pub take_action_days_in_block: u64,
	}

	impl<Origin, AccountId, Amount> Default for Contract<Origin, AccountId, Amount> {
		fn default() -> Self {
			Self::default()
		}
	}


	#[pallet::storage]
	#[pallet::getter(fn contract_sender)]
	pub(super) type ContractSender<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Contract<T::AccountId, T::AccountId, BalanceOf<T>>, ValueQuery>;


	#[pallet::storage]
	#[pallet::getter(fn contract_receiver)]
	pub(super) type ContractReceiver<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Contract<T::AccountId, T::AccountId, BalanceOf<T>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Sign Contract
		ContractEvent(T::AccountId, T::AccountId, BalanceOf<T>, u64, u64, u64),
		/// Lock funds
		Locked(T::AccountId, BalanceOf<T>),

		/// Unlock funds
		UnLock(T::AccountId, BalanceOf<T>),

		/// Transfer
		Transfer(T::AccountId, T::AccountId, BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The requested user has not stored a value yet
		NoValueStored,
		/// Expiring Date was wrong/older than current date
		WrongExpiringDate,
		/// Means the contract is signed by the same people
		SameAddressError
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Sign contract between two addresses
		#[pallet::weight(10_000)]
		pub fn sign_contract(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			work_days: u64,
			take_action_days: u64,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			ensure!(from != to, Error::<T>::SameAddressError);

			let prod_block_per_sec = 30;
			let day_per_second = 86400;
			let prod_block_per_day  = day_per_second/prod_block_per_sec;

			let current_block_number: u64 = frame_system::Pallet::<T>::block_number().try_into().unwrap_or(0);
			let work_days_in_block_number = current_block_number + (work_days * prod_block_per_day);
			let take_action_days_in_block = work_days_in_block_number + (take_action_days * prod_block_per_day);

			let contract = Contract {
				origin: from.clone(),
				to: to.clone(),
				amount: amount.clone(),
				 current_block_number: current_block_number.clone(),
				 work_days_in_block_number: work_days_in_block_number.clone(),
				 take_action_days_in_block: take_action_days_in_block.clone(),
			};

			 <ContractSender<T>>::insert(from.clone(), &contract);
			 <ContractReceiver<T>>::insert(to.clone(), contract);
			Self::deposit_event(Event::ContractEvent(from.clone(), to, amount.clone(), current_block_number, work_days_in_block_number, take_action_days_in_block));

			T::Currency::set_lock(EXAMPLE_ID, &from, amount.clone(), WithdrawReasons::all());
			Self::deposit_event(Event::Locked(from, amount));

			Ok(().into())
		}


		/// Withdraw funds
		#[pallet::weight(10_000)]
		pub fn withdraw_funds(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;

			 ensure!(
				<ContractSender<T>>::contains_key(&from) || <ContractReceiver<T>>::contains_key(&from),
				Error::<T>::NoValueStored
			);

			// When is period of take action day, sender can unlock their funds
			if <ContractSender<T>>::contains_key(&from) {
				let current_block_number: u64 = frame_system::Pallet::<T>::block_number().try_into().unwrap_or(0);
				let work_days_in_block_number = <ContractSender<T>>::get(&from).work_days_in_block_number;
				let take_action_days_in_block = <ContractSender<T>>::get(&from).take_action_days_in_block;
				let amount = <ContractSender<T>>::get(from.clone()).amount;

				if current_block_number >= work_days_in_block_number && current_block_number <= take_action_days_in_block {
					T::Currency::remove_lock(EXAMPLE_ID, &from);
					Self::deposit_event(Event::UnLock(from.clone(), amount));
				}
			}

			// When take action day is expired, receiver can withdraw funds by himself
			if <ContractReceiver<T>>::contains_key(&from) {
				let current_block_number: u64 = frame_system::Pallet::<T>::block_number().try_into().unwrap_or(0);
				let work_days_in_block_number = <ContractReceiver<T>>::get(&from).work_days_in_block_number;
				let take_action_days_in_block = <ContractReceiver<T>>::get(&from).take_action_days_in_block;

				if current_block_number > work_days_in_block_number + take_action_days_in_block {
					let to = <ContractReceiver<T>>::get(&from).origin;
					let from = <ContractReceiver<T>>::get(&from).to;
					let amount = <ContractReceiver<T>>::get(from.clone()).amount;

					T::Currency::remove_lock(EXAMPLE_ID, &from);
					Self::deposit_event(Event::UnLock(from.clone(), amount.clone()));

					T::Currency::transfer(&from, &to, amount, AllowDeath)?;
					Self::deposit_event(Event::Transfer(from, to,amount));
				}
			}

			Ok(().into())
		}

		/// Send funds
		#[pallet::weight(10_000)]
		pub fn send_funds(
			origin: OriginFor<T>
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;

				ensure!(
                    <ContractSender<T>>::contains_key(&from),
                    Error::<T>::NoValueStored
                );

                // When is take action day/ take action day is expired, only sender can send funds to the receiver
                if <ContractSender<T>>::contains_key(&from) {
                    let current_block_number: u64 = frame_system::Pallet::<T>::block_number().try_into().unwrap_or(0);
                    let work_days_in_block_number = <ContractSender<T>>::get(&from).work_days_in_block_number;
                    let take_action_days_in_block = <ContractSender<T>>::get(&from).take_action_days_in_block;

                    if current_block_number >= work_days_in_block_number {
                        let entry = <ContractSender<T>>::get(from.clone());
                        let to = entry.to;
                        let amount = entry.amount;

                        T::Currency::remove_lock(EXAMPLE_ID, &from);
                        Self::deposit_event(Event::UnLock(from.clone(), amount.clone()));

                        T::Currency::transfer(&from, &to, amount, AllowDeath)?;
                        Self::deposit_event(Event::Transfer(from, to,amount));
                    }
                }

			Ok(().into())
		}
	}
}
