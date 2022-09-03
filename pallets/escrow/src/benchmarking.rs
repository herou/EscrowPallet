//! Benchmarking setup for escrow

use super::*;

#[allow(unused)]
use crate::Pallet as Escrow;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	sign_contract {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();

		sign_contract::<ContractSender<T>>::insert(from.clone(), &contract);
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(Escrow, crate::mock::new_test_ext(), crate::mock::Test);
}
