use std::ptr::null;
use crate::{mock::*, Error, Event, Invoice};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_invoice_same_address_error() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 2;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_noop!(Escrow::create_invoice(
                origin,
                to,
                amount,
                msg,
            ),
                Error::<Test>::SameAddressError);
    })
}

#[test]
fn create_invoice_ok() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 1;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_ok!(Escrow::create_invoice(
                origin.clone(),
                to,
                amount.clone(),
                msg.clone()));
    })
}


#[test]
fn create_invoice_ok_2() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 1;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        // assert_ok!(Escrow::create_invoice(
        // 	origin.clone(),
        // 	to,
        // 	amount.clone(),
        // 	msg.clone()));
        //
        // let contract = Invoice {
        //     origin: from.clone(),
        //     to: to.clone(),
        //     amount,
        //     status: false,
        //     id: 0,
        //     msg: msg.clone(),
        // };
        //
        // let mut invoice_vec: Vec<Invoice<T::AccountId, T::AccountId, BalanceOf<T>>> = Vec::new();
        // invoice_vec.push(contract);
    })
}

#[test]
fn pay_invoices_same_address_error() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 2;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_noop!(Escrow::create_invoice(
                origin,
                to,
                amount,
                msg,
            ),Error::<Test>::SameAddressError);
    })
}

#[test]
fn pay_invoices_error_any_1() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 1;
        const BOB: u64 = 2;

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_ok!(Escrow::create_invoice(
                origin.clone(),
                to,
                amount.clone(),
                msg.clone()));

        assert_noop!(Escrow::pay_invoices(
                origin.clone(),
                to,
                0),  Error::<Test>::AnyError);
    })
}


#[test]
fn pay_invoices_error_any_2() {
    new_test_ext().execute_with(|| {
        const ALICE: u64 = 1;
        const BOB: u64 = 2;

        let origin = Origin::signed(BOB);
        let to = ALICE;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_ok!(Escrow::create_invoice(
                origin.clone(),
                to,
                amount.clone(),
                msg.clone()));

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let id = 20;

        assert_noop!(Escrow::pay_invoices(
                origin.clone(),
                to,
                id),  Error::<Test>::AnyError);
    })
}

#[test]
fn pay_invoices_success() {
    new_test_ext().execute_with(|| {
        type AccountId = u64;
        type BalanceOf = u64;
      //  let mut invoice_vec: Vec<Invoice<T::AccountId, T::AccountId, BalanceOf<T>>> = Vec::new();

        const ALICE: u64 = 1;
        const BOB: u64 = 2;

        let origin = Origin::signed(BOB);
        let to = ALICE;
        let amount = 4000;
        let msg: Vec<u8> = vec![2, 23, 34, 45];

        assert_ok!(Escrow::create_invoice(
                origin.clone(),
                to,
                amount.clone(),
                msg.clone()));

        let origin = Origin::signed(ALICE);
        let to = BOB;
        let id = 0;
        assert_ok!(Escrow::pay_invoices(
                origin.clone(),
                 to,
            id));

        let invoice = Invoice {
            origin: BOB,
            to: to.clone(),
            amount,
            status: false,
            id: 0,
            msg: msg.clone(),
        };

        let mut invoice_vec: Vec<Invoice<AccountId, AccountId, BalanceOf>> = Vec::new();
        invoice_vec.push(invoice.clone());


       let rripi =  Escrow::invoice_receiver(ALICE);

        let rripi2 =  invoice_vec;


       // assert_eq!(null(), Some(null()));
        assert_eq!(Escrow::invoice_receiver(ALICE), Some(invoice_vec.clone()));

    })
}


