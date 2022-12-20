#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::traits::{Currency, LockIdentifier};
pub use pallet::*;

const EXAMPLE_ID: LockIdentifier = *b"example ";
const ID: u8 = 100;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;


#[cfg(test)]
mod tests;
#[cfg(test)]
mod mock;

#[frame_support::pallet]
pub mod pallet {
	use core::convert::TryInto;
	use frame_support::{dispatch::*, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	use crate::{BalanceOf, EXAMPLE_ID, ID};
	use frame_support::sp_runtime::SaturatedConversion;
	use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath};
	use frame_support::traits::{LockableCurrency, WithdrawReasons};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The lockable currency type
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct Invoice<Origin, AccountId, Amount> {
		pub origin: Origin,
		pub to: AccountId,
		pub amount: Amount,
		pub status: bool,
		pub id: u64,
		pub msg: Vec<u8>,
	}

	#[pallet::storage]
	#[pallet::getter(fn invoice_sender)]
	#[pallet::unbounded]
	pub(super) type InvoiceSender<T: Config> = StorageMap<_,
		Blake2_128Concat,
		T::AccountId,
		Vec<Invoice<T::AccountId, T::AccountId, BalanceOf<T>>>,
		OptionQuery, >;

	#[pallet::storage]
	#[pallet::getter(fn invoice_receiver)]
	pub(super) type InvoiceReceiver<T: Config> = StorageMap<_,
		Blake2_128Concat,
		T::AccountId,
		Vec<Invoice<T::AccountId, T::AccountId, BalanceOf<T>>>,
		OptionQuery, >;

	#[pallet::storage]
	#[pallet::getter(fn simple_map)]
	pub(super) type SimpleMap<T: Config> = StorageMap<_, Blake2_128Concat, u8, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Create invoice
		InvoiceEvent(T::AccountId, T::AccountId, BalanceOf<T>, Vec<u8>, bool, u64),

		/// Transfer from sender to receiver
		Transfer(T::AccountId, T::AccountId, BalanceOf<T>, bool),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Wrong address
		WrongAddress,

		/// Contract is signed by the same addresses
		SameAddressError,

		/// AnyError
		AnyError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create invoice between two addresses
		#[pallet::weight(10_000)]
		pub fn create_invoice(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			msg: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			// Check if Tx is signed
			let from = ensure_signed(origin)?;
			// Check if the sender and receiver have not the same address
			ensure!(from != to, Error::<T>::SameAddressError);

			//Creating a Contract object
			let contract = Invoice {
				origin: from.clone(),
				to: to.clone(),
				amount,
				status: false,
				id: 0,
				msg: msg.clone(),
			};

			let mut invoice_vec: Vec<Invoice<T::AccountId, T::AccountId, BalanceOf<T>>> = Vec::new();
			invoice_vec.push(contract);

			let mut invoice_id: u64 = 0;
			if <SimpleMap<T>>::contains_key(ID) {
				let id = <SimpleMap<T>>::get(ID);
				invoice_id = id + 1;
			}

			// Save in storage the sender and the invoices
			<InvoiceSender<T>>::insert(from.clone(), invoice_vec.clone());
			// Save in storage the receiver and the invoices
			<InvoiceReceiver<T>>::insert(to.clone(), invoice_vec);
			// Save in storage the id of last invoice
			<SimpleMap<T>>::insert(ID, invoice_id);
			//Throw Contract event
			Self::deposit_event(Event::InvoiceEvent(from.clone(), to, amount, msg, false, invoice_id));

			Ok(().into())
		}

		/// Create invoice between two addresses
		#[pallet::weight(10_000)]
		pub fn show_all_invoices(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// Check if Tx is signed
			let from = ensure_signed(origin)?;
			// Check if the sender and receiver have not the same address

			if <InvoiceSender<T>>::contains_key(&from) {
				let maybe_invoice_sender = <InvoiceSender<T>>::get(&from);
				if let Some(invoice_sender) = maybe_invoice_sender {
					//Self::deposit_event(Event::InvoiceListEvent(invoice_sender.get(index)));
					//for i in invoice_sender {
					//Self::deposit_event(Event::InvoiceListEvent(invoice_sender));
					//Self::deposit_event(Event::InvoiceEvent(i.origin, i.to, i.amount, i.msg, i.status, i.id));
					//}
				}
			}

			/*			if <InvoiceReceiver<T>>::contains_key(&from) {
				let maybe_invoice_receiver = <InvoiceReceiver<T>>::get(&from);
				if let Some(invoice_receiver) = maybe_invoice_receiver {
					//Self::deposit_event(Event::InvoiceListEvent(invoice_receiver));
				}
			}*/

			Ok(().into())
		}

		/// Create invoice between two addresses
		#[pallet::weight(10_000)]
		pub fn pay_invoices(sender: OriginFor<T>, receiver: T::AccountId, id: u64) -> DispatchResult {
			// Check if Tx is signed
			let from = ensure_signed(sender)?;

			ensure!(from != receiver, Error::<T>::SameAddressError);
			// Check if the sender and receiver have not the same address

			let mut is_unpaid_invoice = false;
			if let Some(mut invoices_recevier) = Self::invoice_receiver(&from) {
				for invoice in &mut invoices_recevier {
					if invoice.id == id  && invoice.status == false {
						invoice.status = true;
						is_unpaid_invoice = true
					}
				}
				if is_unpaid_invoice {
					InvoiceReceiver::<T>::insert(from.clone(), invoices_recevier);

					let amount_copy: BalanceOf<T> = 0u64.saturated_into();

					T::Currency::transfer(&receiver, &from, amount_copy, AllowDeath)?;
					Self::deposit_event(Event::Transfer(receiver, from, amount_copy, true));

					return Ok(());
				}
			}
			Err(<Error<T>>::AnyError.into())
		}
	}
}