use frame_support::derive_impl;
use frame_system as system;
use sp_core::{ConstU32, ConstU64};
use sp_runtime::{traits::IdentityLookup, BuildStorage};

// Timestamp
impl pallet_timestamp::Config for Test {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<5>;
    type WeightInfo = ();
}

// Poe
pub const MILLISECS_PER_PROOF_ROOT_PUBLISHING: u64 = 6000;
pub const MIN_PROOFS_FOR_ROOT_PUBLISHING: u32 = 2;

impl crate::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type MinProofsForPublishing = ConstU32<MIN_PROOFS_FOR_ROOT_PUBLISHING>;
    type MaxElapsedTimeMs = ConstU64<MILLISECS_PER_PROOF_ROOT_PUBLISHING>;
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Poe: crate::{Pallet, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call},
    }
);

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl system::Config for Test {
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = frame_system::mocking::MockBlockU32<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut ext = sp_io::TestExternalities::from(
        frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap(),
    );
    ext.execute_with(|| System::set_block_number(1));
    ext
}
