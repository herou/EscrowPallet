
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::traits::{Currency, LockIdentifier};
pub use pallet::*;

const EXAMPLE_ID: LockIdentifier = *b"example ";
const ID: u8 = 100;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use core::convert::TryInto;
	use sp_std::vec::Vec;

	use frame_support::{
		traits::{Currency, ExistenceRequirement::AllowDeath},
	};
	use frame_support::traits::{LockableCurrency, WithdrawReasons};
	use frame_support::sp_runtime::SaturatedConversion;
	use crate::{BalanceOf, EXAMPLE_ID, ID};

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

	#[derive(Encode, Decode, RuntimeDebug, PartialEq, Eq, TypeInfo)]
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
	pub(super) type InvoiceSender<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Invoice<T::AccountId, T::AccountId, BalanceOf<T>>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn invoice_receiver)]
	pub(super) type InvoiceReceiver<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Invoice<T::AccountId, T::AccountId, BalanceOf<T>>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn simple_map)]
	pub(super) type SimpleMap<T: Config> =
	StorageMap<_, Blake2_128Concat, u8, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Create invoice
		InvoiceEvent(T::AccountId, T::AccountId, BalanceOf<T>, Vec<u8>, bool, u64),

		//InvoiceListEvent(Vec<Invoice<T::AccountId, T::AccountId, BalanceOf<T>>>),

		/// Transfer from sender to receiver
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

			let mut id:u64 = 0;
			if <SimpleMap<T>>::contains_key(ID) {
				let invoice_id = <SimpleMap<T>>::get(ID);
				id = invoice_id + 1;
			}

			// Save in storage the sender and the invoices
			<InvoiceSender<T>>::insert(from.clone(), &invoice_vec);
			// Save in storage the receiver and the invoices
			<InvoiceReceiver<T>>::insert(to.clone(), &invoice_vec);
			<SimpleMap<T>>::insert(ID, id);
			//Throw Contract event
			Self::deposit_event(Event::InvoiceEvent(from.clone(), to, amount, msg, false, id));

			Ok(().into())
		}


		/// Create invoice between two addresses
		#[pallet::weight(10_000)]
		pub fn show_all_invoices(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
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

		// pub fn  decrease_balance(
		// 	id:T::FungibleTokenId,
		// 	from: &T::AccountId,
		// 	amount:Balance,
		// )->DispatchResult{
		// 	Balances::<T>::try_mutate(id,from, |balance| ->DispatchResult{
		// 		*balance = balance.checked_sub(amount).ok_or(Error::<T>::NumOverflow)?;
		// 		Ok(())
		//
		// 	})?;
		//
		// 	Ok(())
		//
		// }

		/// Create invoice between two addresses
		#[pallet::weight(10_000)]
		pub fn pay_invoices(
			origin: OriginFor<T>,
			to: T::AccountId,
			id: u64,
		) -> DispatchResultWithPostInfo {
			// Check if Tx is signed
			let from = ensure_signed(origin)?;

			ensure!(from != to, Error::<T>::SameAddressError);
			// Check if the sender and receiver have not the same address

			if <InvoiceReceiver<T>>::contains_key(&from) {
				let maybe_invoice_recevier = <InvoiceReceiver<T>>::get(&from);
				if let Some(mut invoices_recevier) = maybe_invoice_recevier {

						if <InvoiceSender<T>>::contains_key(&to) {
								let maybe_invoice_sender = <InvoiceSender<T>>::get(&to);
								if let Some(mut invoices_sender) = maybe_invoice_sender {



									let mut amount_copy: BalanceOf<T> =  0u64.saturated_into();
									InvoiceReceiver::<T>::mutate(&from, |invoice_recevier| {
										invoices_recevier.iter_mut().filter(|i| i.id == id && !i.status).for_each(|i| {
											i.status = true;
											amount_copy = i.amount
										})
									});

									for z in invoices_recevier {
										let mut aa = z.id;
										let mut bb = z.status;
										let mut bb = z.status;
										let mut bb = z.status;
									}

									InvoiceSender::<T>::mutate(&from, |invoice_sender| {
										invoices_sender.iter_mut().filter(|i| i.id == id && !i.status).for_each(|i| {
											i.status = true;
										})
									});

									for z in invoices_sender {
										let mut aa = z.id;
										let mut bb = z.status;
										let mut bb = z.status;
										let mut bb = z.status;
									}

									T::Currency::transfer(&from, &to, amount_copy, AllowDeath)?;
									Self::deposit_event(Event::Transfer(from, to, amount_copy));
								}
						}
				}
			}

			Ok(().into())
		}
	}
}
