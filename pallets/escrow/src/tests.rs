use frame_system::Origin as OtherOrigin;
use crate::{self as escrow, BalanceOf, Config, Contract};
use frame_support::{assert_noop, assert_ok, construct_runtime, parameter_types};
use frame_support::traits::ExistenceRequirement;
use frame_support::traits::ExistenceRequirement::AllowDeath;
use sp_core::H256;
use crate::Error;
use sp_io::TestExternalities;
use sp_runtime::{
    testing::Header,
    traits::{AtLeast32Bit, BlakeTwo256, IdentityLookup},
};


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		Escrow: escrow::{Module, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for TestRuntime {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Index = u64;
    type Call = Call;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for TestRuntime {
    type MaxLocks = ();
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

impl Config for TestRuntime {
    type Event = Event;
    type Currency = Balances;
}

struct ExternalityBuilder;

impl ExternalityBuilder {
    pub fn build() -> TestExternalities {
        let storage = frame_system::GenesisConfig::default()
            .build_storage::<TestRuntime>()
            .unwrap();
        let mut ext = TestExternalities::from(storage);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

#[test]
fn sign_contract_test() {
    ExternalityBuilder::build().execute_with(|| {
        {
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
			Error::<TestRuntime>::SameAddressError);
        }


        {
            const ALICE: u64 = 1;
            const BOB: u64 = 2;

            let origin = Origin::signed(ALICE);
            let to = BOB;
            let amount = 4000;
            let work_days = 2;
            let take_action_days = 3;

            assert_ok!(Escrow::sign_contract(
			origin.clone(),
			to,
			amount.clone(),
			work_days,
			take_action_days
		));
        }

        {
            const ALICE: u64 = 1;
            const BOB: u64 = 2;
            let origin = ALICE;
            let to = BOB;
            let amount = 4000;
            let current_block_number: u64 = 1;
            let work_days_in_block_number: u64 = 5761;
            let take_action_days_in_block: u64 = 14401;
            let contract_event = Event::escrow(crate::Event::ContractEvent(
                origin,
                to,
                amount,
                current_block_number,
                work_days_in_block_number,
                take_action_days_in_block,
            ));


            assert_eq!(System::events()[0].event, contract_event);



            let locked_event = Event::escrow(crate::Event::Locked(
                origin,
                amount,
            ));

            assert_eq!(System::events()[2].event, locked_event);

        }
    })
}
