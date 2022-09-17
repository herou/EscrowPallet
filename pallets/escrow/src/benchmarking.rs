//! Benchmarking setup for escrow
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Escrow;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use frame_system::{EventRecord, RawOrigin};
use crate::Event;
use frame_support::sp_runtime::traits::Bounded;

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

	impl_benchmark_test_suite!(Escrow, crate::mock::new_test_ext(), crate::mock::Test);
}

