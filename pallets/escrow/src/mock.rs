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

use core::convert::*;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;


construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
        Balances: pallet_balances,
		Escrow: escrow,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
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
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
}

impl Config for Test {
    type Event = Event;
    type Currency = Balances;
}

pub fn new_test_ext() -> TestExternalities {
    frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
