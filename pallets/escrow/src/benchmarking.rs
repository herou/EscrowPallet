//! Benchmarking setup for escrow
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Escrow;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use frame_system::{EventRecord, RawOrigin};
use crate::Event;
use frame_support::{assert_ok, sp_runtime::traits::Bounded};
use frame_support::sp_runtime::traits::Saturating;


fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}


benchmarks! {
	sign_contract {
		let to = account("receiver", 0, 0);
		let amount = BalanceOf::<T>::max_value();
		let work_days = 10u64.into();
		let take_action_days = 10u64.into();
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller.clone()), to, amount.clone(), work_days, take_action_days)
	verify {

	 assert_last_event::<T>(Event::Locked(caller, amount).into());
	}

	withdraw_funds {
		let to: T::AccountId = account("receiver", 0, 0);
		let amount = BalanceOf::<T>::max_value();
		let caller: T::AccountId = whitelisted_caller();
	  	let work_days: u64 = 5761;
		let take_action_days: u64 = 14401;

		let val: BalanceOf<T> = 1_000_000_000u32.into();
		T::Currency::make_free_balance_be(&caller, val.saturating_mul(4_000_000_000u32.into()));
		T::Currency::make_free_balance_be(&to, val.saturating_mul(4_000_000_000u32.into()));

          assert_ok!(
			Escrow::<T>::sign_contract(RawOrigin::Signed(caller.clone()).into(), to.clone(), amount, work_days, take_action_days)
		);


        let contract = Contract {
            origin: caller.clone(),
            to: to.clone(),
            amount,
            current_block_number: 0,
			work_days_in_block_number: 82958400,
            take_action_days_in_block: 290332800,
        };



	}: _(RawOrigin::Signed(caller.clone()))
	verify {

		assert_eq!(Escrow::<T>::contract_sender(caller), Some(contract.clone()));
		assert_eq!(Escrow::<T>::contract_receiver(to), Some(contract.clone()));
	}


    send_funds {
		let to: T::AccountId = account("receiver", 0, 0);
		let amount: BalanceOf<T> = 4000u32.into();
		let caller: T::AccountId = whitelisted_caller();
	  	let work_days: u64 = 5761;
		let take_action_days: u64 = 14401;

		let val: BalanceOf<T> = 1_000_000_000u32.into();
		T::Currency::make_free_balance_be(&caller, val.saturating_mul(4_000_000_000u32.into()));
		T::Currency::make_free_balance_be(&to, val.saturating_mul(4_000_000_000u32.into()));

          assert_ok!(
			Escrow::<T>::sign_contract(RawOrigin::Signed(caller.clone()).into(), to.clone(), amount, work_days, take_action_days)
		);


        let contract = Contract {
            origin: caller.clone(),
            to: to.clone(),
            amount,
            current_block_number: 0,
			work_days_in_block_number: 82958400,
            take_action_days_in_block: 290332800,
        };

		let block_number: u32 = 82958400;
		frame_system::Pallet::<T>::set_block_number(block_number.into());

	}: _(RawOrigin::Signed(caller.clone()))
	verify {

       	assert_eq!(Escrow::<T>::contract_sender(caller), Some(contract.clone()));
	}


	impl_benchmark_test_suite!(Escrow, crate::mock::new_test_ext(), crate::mock::Test);
}