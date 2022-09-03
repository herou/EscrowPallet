use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn sign_contract_test() {
    new_test_ext().execute_with(|| {

            const ALICE: u64 = 2;
            const BOB: u64 = 2;

            let origin = Origin::signed(ALICE);
            let to = BOB;
            let amount = 4000;
            let work_days = 2;
            let take_action_days = 3;

            assert_noop!(Escrow::sign_contract(
			origin,
			to,
			amount,
			work_days,
			take_action_days
		),
			Error::<Test>::SameAddressError);

    })
}
