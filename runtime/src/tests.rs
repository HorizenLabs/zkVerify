// Copyright 2024, The Horizen Foundation

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::*;

use frame_support::traits::{fungible::Inspect, Currency, ExistenceRequirement, WithdrawReasons};
use pallet_settlement_fflonk::{Proof, FULL_PROOF_SIZE};

mod testsfixtures;

// Function used for creating the environment for the test.
// It must return a sp_io::TestExternalities, and the actual test will execute this one before running.
fn new_test_ext() -> sp_io::TestExternalities {
    // This builds the initial genesis storage for this test
    let mut t = frame_system::GenesisConfig::<super::Runtime>::default()
        .build_storage()
        .unwrap();

    // Create four users by calling the fixture function
    let sample_users = testsfixtures::get_sample_users();

    // Incorporate the pairs account / starting balance into the Genesis config
    pallet_balances::GenesisConfig::<super::Runtime> {
        balances: sample_users
            .into_iter()
            .map(|user| (user.raw_account.into(), user.starting_balance))
            .collect(),
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
fn check_starting_balances_and_existential_limit() {
    new_test_ext().execute_with(|| {
        // This creates a few public keys used to be converted to AccountId
        let sample_users = testsfixtures::get_sample_users();

        for sample_user in &sample_users {
            assert_eq!(
                Balances::balance(&sample_user.raw_account.into()),
                sample_user.starting_balance
            );
        }

        // Now perform a withdraw on the fourth account, leaving its balance under the EXISTENTIAL_DEPOSIT limit
        // This should kill the account, when executed with the ExistenceRequirement::AllowDeath option
        let _id_3_withdraw = Balances::withdraw(
            &sample_users[3].raw_account.into(),
            testsfixtures::EXISTENTIAL_DEPOSIT_REMINDER, // Withdrawing more th
            WithdrawReasons::TIP,
            ExistenceRequirement::AllowDeath,
        );

        // Verify that the fourth account balance is now 0
        assert_eq!(Balances::balance(&sample_users[3].raw_account.into()), 0);
    });
}

// Test definition and execution. Test body must be written in the execute_with closure.
#[test]
fn pallet_fflonk_availability() {
    new_test_ext().execute_with(|| {
        let dummy_origin = sp_runtime::AccountId32::new([0; 32]);
        let dummy_raw_proof: Proof = [0; FULL_PROOF_SIZE];
        assert!(SettlementFFlonkPallet::submit_proof(
            RuntimeOrigin::signed(dummy_origin),
            dummy_raw_proof.into()
        )
        .is_err());
        // just checking code builds, hence the pallet is available to the runtime
    });
}

// Test definition and execution. Test body must be written in the execute_with closure.
#[test]
fn pallet_poe_availability() {
    new_test_ext().execute_with(|| {
        assert!(Poe::publish_attestation(RuntimeOrigin::root()).is_ok());
        // just checking code builds, hence the pallet is available to the runtime
    });
}
