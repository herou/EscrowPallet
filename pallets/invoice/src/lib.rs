//! # Escrow Pallet
//!
//! We aim to build a Escrow pallet which will help communities to release funds easier to devs/teams.
//! I would like to explain the purpose of the project with a real example.
//! A developer wants to fix/develop a bug/feature. He requested some funds and his proposal got approved.
//! Both of the parties (for ex: web3 community and contributor) sign an Escrow contract by defining the Web3 community address, the contributor address, the amount, the expiry date, and “take action days”.
//! “Take action days” would be some supplement time after the contract has ended for the Web3 to test/check if the bug/feature has been delivered correctly.
//! Once the contract is signed the Web3 funds will be locked into another address.
//! During the take action days Web3 can decide to:
//! 1-Send the funds manually to the contributor 2-Withdraw the funds because nothing was delivered 3-Do not take any action during the “take action days” and when these additive days are expired the contributor can withdraw the funds by himself.
//! (This option would reduce the releasing fund steps by giving additional work to the contributor)
//!
//! ### Dispatchable Functions
//! #### Role setting
//!
//! * `sign_contract` -It is used to sign an Escrow contract between two users.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::traits::{Currency, LockIdentifier};
pub use pallet::*;

const EXAMPLE_ID: LockIdentifier = *b"example ";

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use core::convert::TryInto;

	use frame_support::{
		traits::{Currency, ExistenceRequirement::AllowDeath},
	};
	use frame_support::traits::{LockableCurrency, WithdrawReasons};
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


	#[pallet::storage]
	#[pallet::getter(fn contract_sender)]
	pub(super) type ContractSender<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Contract<T::AccountId, T::AccountId, BalanceOf<T>>, OptionQuery>;


	#[pallet::storage]
	#[pallet::getter(fn contract_receiver)]
	pub(super) type ContractReceiver<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Contract<T::AccountId, T::AccountId, BalanceOf<T>>, OptionQuery>;

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
		/// Wrong address
		WrongAddress,
		/// Expiring Date was wrong/older than current date
		WrongExpiringDate,
		/// Contract is signed by the same addresses
		SameAddressError,
		/// Working days have not expired
		WorkingDaysNotExpired,
		/// Take action days not expired
		TakeActionDaysNotExpired
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
			// Check if Tx is signed
			let from = ensure_signed(origin)?;
			// Check if the sender and receiver have not the same address
			ensure!(from != to, Error::<T>::SameAddressError);

			// calculate how many blocks per day gets generated
			let prod_block_per_sec = 6;
			let day_per_second = 86400;
			let prod_block_per_day = day_per_second / prod_block_per_sec;

			let current_block_number: u64 = frame_system::Pallet::<T>::block_number().try_into().unwrap_or(0);
			let work_days_in_block_number = current_block_number + (work_days * prod_block_per_day);
			let take_action_days_in_block = work_days_in_block_number + (take_action_days * prod_block_per_day);

			//Creating a Contract object
			let contract = Contract {
				origin: from.clone(),
				to: to.clone(),
				amount,
				current_block_number,
				work_days_in_block_number,
				take_action_days_in_block,
			};

			// Save in storage the sender and the contract
			<ContractSender<T>>::insert(from.clone(), &contract);
			// Save in storage the reciever and the contract
			<ContractReceiver<T>>::insert(to.clone(), contract);
			//Throw Contract event
			Self::deposit_event(Event::ContractEvent(from.clone(), to, amount, current_block_number, work_days_in_block_number, take_action_days_in_block));

			//Lock the funds
			T::Currency::set_lock(EXAMPLE_ID, &from, amount, WithdrawReasons::all());

			//Thrown Lock event
			Self::deposit_event(Event::Locked(from, amount));

			Ok(().into())
		}

	}
}
