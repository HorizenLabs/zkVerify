// This is a sample integration test. Almost. Except it formally isn't.
// The following test verifies that the pallet "balances" correctly interact with the runtime, hence
// it's more than a unit test, as we are required to build a mock runtime, increasing the complexity
// of the test.
// In this example, we create a mock runtime with the two modules (system and balances),
// then we add a test that exercise the balances module by:
// - creating a test externality
// - running the test itself
//
// Rust convention dictates that integration tests like this should be contained in their own "tests"
// directory, but doing that now would be quite messy: ideally, this can be correctly achieved if
// we had a newly developed pallet in its own directory.
// The first poc shows how it should be implemented, as we had our utxo_pallet.
//
// The end result is that this test is formally in integration test, but disguised as unit one...
//
// A test fixture is included in the 'tests/testfixtures.rs' file as example

use super::*;

use frame_support::traits::{fungible::Inspect, Currency, ExistenceRequirement, WithdrawReasons};
pub use frame_support::{
    assert_err, assert_ok, construct_runtime, pallet_prelude::*, parameter_types, weights::Weight,
};
use sp_runtime::traits::IdentityLookup;

mod testsfixtures;

// Assemble and configure a minimal runtime for the test
construct_runtime!(
    pub enum Test {
        System: frame_system,
        Balances: pallet_balances,
    }
);

impl frame_system::Config for Test {
    type AccountData = pallet_balances::AccountData<Balance>;
    type AccountId = AccountId;
    type BaseCallFilter = frame_support::traits::Everything;
    type Block = Block;
    type BlockHashCount = frame_support::traits::ConstU32<250>;
    type BlockLength = ();
    type BlockWeights = ();
    type DbWeight = RocksDbWeight;
    type Hash = sp_core::H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type Nonce = Nonce;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type PalletInfo = PalletInfo;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type SS58Prefix = ();
    type SystemWeightInfo = ();
    type Version = ();
    type OnSetCode = ();
}

impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<{ testsfixtures::EXISTENTIAL_DEPOSIT }>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type MaxHolds = ();
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeEvent = RuntimeEvent;
    type RuntimeFreezeReason = ();
    type RuntimeHoldReason = ();
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

// Function used for creating the environment for the test.
// It must return a sp_io::TestExternalities, and the actual test will execute this one before running.
fn new_test_ext() -> sp_io::TestExternalities {
    // This builds the initial genesis storage for this test
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    // Create four users by calling the fixture function
    let sample_users = testsfixtures::get_sample_users();

    // Incorporate the pairs account / starting balance into the Genesis config
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (
                sample_users[0].raw_account.into(),
                sample_users[0].starting_balance,
            ),
            (
                sample_users[1].raw_account.into(),
                sample_users[1].starting_balance,
            ),
            (
                sample_users[2].raw_account.into(),
                sample_users[2].starting_balance,
            ),
            (
                sample_users[3].raw_account.into(),
                sample_users[3].starting_balance,
            ),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::from(t);

    ext.execute_with(|| System::set_block_number(1));

    // Return the test externalities
    ext
}

// Test definition and execution. Test body must be written in the execute_with closure.
#[test]
fn test_check_starting_balances_and_existential_limit() {
    new_test_ext().execute_with(|| {
        // This creates a few public keys used to be converted to AccountId
        let sample_users = testsfixtures::get_sample_users();

        // For each account, let's check the hardcoded starting balance embedded in the genesis block
        assert_eq!(
            Balances::balance(&sample_users[0].raw_account.into()),
            sample_users[0].starting_balance
        );
        assert_eq!(
            Balances::balance(&sample_users[1].raw_account.into()),
            sample_users[1].starting_balance
        );
        assert_eq!(
            Balances::balance(&sample_users[2].raw_account.into()),
            sample_users[2].starting_balance
        );
        assert_eq!(
            Balances::balance(&sample_users[3].raw_account.into()),
            sample_users[3].starting_balance
        );

        // Now perform a withdraw on the fourth account, leaving its balance under the EXISTENTIAL_DEPOSIT limit
        // This should kill the account, when executed with the ExistenceRequirement::AllowDeath option
        let id_3_withdraw = Balances::withdraw(
            &sample_users[3].raw_account.into(),
            12,
            WithdrawReasons::TIP,
            ExistenceRequirement::AllowDeath,
        );

        // Verify that the fourth account balance is now 0
        assert_eq!(Balances::balance(&sample_users[3].raw_account.into()), 0);
    });
}
