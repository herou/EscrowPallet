
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
		/// Create invoice
		InvoiceEvent(T::AccountId, T::AccountId, BalanceOf<T>),

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
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Sign contract between two addresses
		#[pallet::weight(10_000)]
		pub fn create_invoice(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			// Check if Tx is signed
			let from = ensure_signed(origin)?;
			// Check if the sender and receiver have not the same address
			ensure!(from != to, Error::<T>::SameAddressError);


			//Creating a Contract object
			let contract = Contract {
				origin: from.clone(),
				to: to.clone(),
				amount,
			};

			// Save in storage the sender and the contract
			<ContractSender<T>>::insert(from.clone(), &contract);
			// Save in storage the reciever and the contract
			<ContractReceiver<T>>::insert(to.clone(), contract);
			//Throw Contract event
			Self::deposit_event(Event::InvoiceEvent(from.clone(), to, amount));

			Ok(().into())
		}
	}
}
