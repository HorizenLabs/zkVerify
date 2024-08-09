use frame_system::EnsureRoot;
use polkadot_primitives::ValidatorId;

pub use polkadot_runtime_parachains::{
    assigner_parachains as parachains_assigner_parachains,
    configuration as parachains_configuration, disputes as parachains_disputes,
    disputes::slashing as parachains_slashing,
    dmp as parachains_dmp, hrmp as parachains_hrmp, inclusion as parachains_inclusion,
    initializer as parachains_initializer, origin as parachains_origin, paras as parachains_paras,
    paras_inherent as parachains_paras_inherent, reward_points as parachains_reward_points,
    runtime_api_impl::{
        v10 as parachains_runtime_api_impl, vstaging as parachains_staging_runtime_api_impl,
    },
    scheduler as parachains_scheduler, session_info as parachains_session_info,
    shared as parachains_shared,
};

//use polkadot_runtime_common::{paras_registrar, paras_sudo_wrapper, prod_or_fast, slots};
pub use polkadot_runtime_common::paras_sudo_wrapper;

use super::{
    AccountId, Babe, Balances, Historical, KeyOwnerProofSystem, KeyTypeId, MaxAuthorities,
    Offences, ParaInclusion, ParachainsAssignmentProvider, ParasDisputes, ParasSlashing,
    ReportLongevity, Runtime, RuntimeEvent, RuntimeOrigin, Session, Weight,
};
use sp_runtime::transaction_validity::TransactionPriority;

use sp_core::parameter_types;
use sp_runtime::FixedU128;

parameter_types! {
    pub const OnDemandTrafficDefaultValue: FixedU128 = FixedU128::from_u32(1);
}

impl parachains_assigner_parachains::Config for Runtime {}

impl parachains_initializer::Config for Runtime {
    type Randomness = pallet_babe::RandomnessFromOneEpochAgo<Runtime>;
    type ForceOrigin = EnsureRoot<AccountId>;
    // type WeightInfo = weights::runtime_parachains_initializer::WeightInfo<Runtime>;
    type WeightInfo = ();

    type CoretimeOnNewSession = ();
}

impl parachains_disputes::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RewardValidators = parachains_reward_points::RewardValidatorsWithEraPoints<Runtime>;
    type SlashingHandler = parachains_slashing::SlashValidatorsForDisputes<ParasSlashing>;
    // type WeightInfo = weights::runtime_parachains_disputes::WeightInfo<Runtime>;
    type WeightInfo = parachains_disputes::TestWeightInfo;
}

impl parachains_slashing::Config for Runtime {
    type KeyOwnerProofSystem = Historical;
    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, ValidatorId)>>::Proof;
    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        ValidatorId,
    )>>::IdentificationTuple;
    type HandleReports = parachains_slashing::SlashingReportHandler<
        Self::KeyOwnerIdentification,
        Offences,
        ReportLongevity,
    >;
    type WeightInfo = parachains_slashing::TestWeightInfo;
    type BenchmarkingConfig = parachains_slashing::BenchConfig<200>;
}

impl parachains_dmp::Config for Runtime {}

impl parachains_hrmp::Config for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeEvent = RuntimeEvent;
    type ChannelManager = EnsureRoot<AccountId>;
    type Currency = Balances;
    // type WeightInfo = weights::runtime_parachains_hrmp::WeightInfo<Runtime>;
    type WeightInfo = parachains_hrmp::TestWeightInfo;
}

impl parachains_paras_inherent::Config for Runtime {
    // type WeightInfo = weights::runtime_parachains_paras_inherent::WeightInfo<Runtime>;
    type WeightInfo = parachains_paras_inherent::TestWeightInfo;
}

impl parachains_scheduler::Config for Runtime {
    type AssignmentProvider = ParachainsAssignmentProvider;
}

impl parachains_origin::Config for Runtime {}

pub struct FakeParachainConfigWeight;
impl parachains_configuration::WeightInfo for FakeParachainConfigWeight {
    fn set_config_with_block_number() -> Weight {
        Default::default()
    }
    fn set_config_with_u32() -> Weight {
        Default::default()
    }
    fn set_config_with_option_u32() -> Weight {
        Default::default()
    }
    fn set_config_with_balance() -> Weight {
        Default::default()
    }
    fn set_hrmp_open_request_ttl() -> Weight {
        Default::default()
    }
    fn set_config_with_executor_params() -> Weight {
        Default::default()
    }
    fn set_config_with_perbill() -> Weight {
        Default::default()
    }
    fn set_node_feature() -> Weight {
        Default::default()
    }

    fn set_config_with_scheduler_params() -> Weight {
        Default::default()
    }
}

impl parachains_configuration::Config for Runtime {
    // type WeightInfo = weights::runtime_parachains_configuration::WeightInfo<Runtime>;
    type WeightInfo = FakeParachainConfigWeight;
}

impl parachains_shared::Config for Runtime {
    type DisabledValidators = Session;
}

impl parachains_session_info::Config for Runtime {
    type ValidatorSet = Historical;
}

impl parachains_inclusion::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type DisputesHandler = ParasDisputes;
    type RewardValidators = parachains_reward_points::RewardValidatorsWithEraPoints<Runtime>;
    // type MessageQueue = MessageQueue;
    type MessageQueue = ();
    // type WeightInfo = weights::runtime_parachains_inclusion::WeightInfo<Runtime>;
    type WeightInfo = ();
}

parameter_types! {
    pub const ParasUnsignedPriority: TransactionPriority = TransactionPriority::MAX;
}

pub struct FakeParasWeightInfo;

impl parachains_paras::WeightInfo for FakeParasWeightInfo {
    fn force_set_current_code(_c: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn force_set_current_head(_s: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn force_set_most_recent_context() -> Weight {
        Weight::from_parts(0, 0)
    }
    fn force_schedule_code_upgrade(_c: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn force_note_new_head(_s: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn force_queue_action() -> Weight {
        Weight::from_parts(0, 0)
    }
    fn add_trusted_validation_code(_c: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn poke_unused_validation_code() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn include_pvf_check_statement_finalize_upgrade_accept() -> Weight {
        Weight::from_parts(0, 0)
    }
    fn include_pvf_check_statement_finalize_upgrade_reject() -> Weight {
        Weight::from_parts(0, 0)
    }
    fn include_pvf_check_statement_finalize_onboarding_accept() -> Weight {
        Weight::from_parts(0, 0)
    }
    fn include_pvf_check_statement_finalize_onboarding_reject() -> Weight {
        Weight::from_parts(0, 0)
    }
    fn include_pvf_check_statement() -> Weight {
        Weight::from_parts(0, 0)
    }
}

impl parachains_paras::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = FakeParasWeightInfo;
    // type WeightInfo = weights::runtime_parachains_paras::WeightInfo<Runtime>;
    type UnsignedPriority = ParasUnsignedPriority;
    type QueueFootprinter = ParaInclusion;
    type NextSessionRotation = Babe;
    // type OnNewHead = Registrar;
    type OnNewHead = ();
    type AssignCoretime = ();
}

// parameter_types! {
//     /// Amount of weight that can be spent per block to service messages.
//     ///
//     /// # WARNING
//     ///
//     /// This is not a good value for para-chains since the `Scheduler` already uses up to 80% block weight.
//     pub MessageQueueServiceWeight: Weight = Perbill::from_percent(20) * BlockWeights::get().max_block;
//     pub const MessageQueueHeapSize: u32 = 32 * 1024;
//     pub const MessageQueueMaxStale: u32 = 96;
// }

// /// Message processor to handle any messages that were enqueued into the `MessageQueue` pallet.
// pub struct MessageProcessor;

// impl ProcessMessage for MessageProcessor {
//     type Origin = AggregateMessageOrigin;

//     fn process_message(
//         message: &[u8],
//         origin: Self::Origin,
//         meter: &mut WeightMeter,
//         id: &mut [u8; 32],
//     ) -> Result<bool, ProcessMessageError> {
//         let para = match origin {
//             AggregateMessageOrigin::Ump(UmpQueueId::Para(para)) => para,
//         };
//         xcm_builder::ProcessXcmMessage::<
//             Junction,
//             xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
//             RuntimeCall,
//         >::process_message(message, Junction::Parachain(para.into()), meter, id)
//     }
// }

// impl pallet_message_queue::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Size = u32;
//     type HeapSize = MessageQueueHeapSize;
//     type MaxStale = MessageQueueMaxStale;
//     type ServiceWeight = MessageQueueServiceWeight;
//     // type MessageProcessor = MessageProcessor;
//     // #[cfg(feature = "runtime-benchmarks")]
//     type MessageProcessor =
//         pallet_message_queue::mock_helpers::NoopMessageProcessor<AggregateMessageOrigin>;
//     type QueueChangeHandler = ParaInclusion;
//     type QueuePausedQuery = ();
//     type WeightInfo = ();
// }

impl pallet_authority_discovery::Config for Runtime {
    type MaxAuthorities = MaxAuthorities;
}

impl paras_sudo_wrapper::Config for Runtime {}

// parameter_types! {
//     pub const ParaDeposit: Balance = 40 * ACME;
//     pub const DataDepositPerByte: Balance = 1 * CENTS;
// }

// impl paras_registrar::Config for Runtime {
//     type RuntimeOrigin = RuntimeOrigin;
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances;
//     // type OnSwap = (Crowdloan, Slots);
//     type OnSwap = Slots;
//     type ParaDeposit = ParaDeposit;
//     type DataDepositPerByte = DataDepositPerByte;
//     type WeightInfo = paras_registrar::TestWeightInfo;
// }

// parameter_types! {
//     pub LeasePeriod: BlockNumber = prod_or_fast!(1 * DAYS, 1 * DAYS, "ZKV_LEASE_PERIOD");
// }

// impl slots::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances;
//     type Registrar = Registrar;
//     type LeasePeriod = LeasePeriod;
//     type LeaseOffset = ();
//     //type ForceOrigin = EitherOf<EnsureRoot<Self::AccountId>, LeaseAdmin>;
//     type ForceOrigin = EnsureRoot<Self::AccountId>;
//     type WeightInfo = slots::TestWeightInfo;
// }

// /// System Parachains.
// pub mod system_parachain {
//     use xcm::latest::prelude::*;

//     // /// Network's Asset Hub parachain ID.
//     // pub const ASSET_HUB_ID: u32 = 1000;
//     // /// Contracts parachain ID.
//     // pub const CONTRACTS_ID: u32 = 1002;
//     // /// Encointer parachain ID.
//     // pub const ENCOINTER_ID: u32 = 1003;
//     // /// BridgeHub parachain ID.
//     // pub const BRIDGE_HUB_ID: u32 = 1013;

//     frame_support::match_types! {
//         pub type SystemParachains: impl Contains<MultiLocation> = {
//             MultiLocation { parents: 0, interior: X1(Parachain(1000)) }
//             // MultiLocation { parents: 0, interior: X1(Parachain(ASSET_HUB_ID | CONTRACTS_ID | ENCOINTER_ID | BRIDGE_HUB_ID)) }
//         };
//     }
// }

/// All migrations that will run on the next runtime upgrade.
///
/// This contains the combined migrations of the last 10 releases. It allows to skip runtime
/// upgrades in case governance decides to do so. THE ORDER IS IMPORTANT.
pub type Migrations = migrations::Unreleased;

pub mod migrations {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "add-parachain-upgrade")]
    pub mod add_parachain_upgrade {
        use super::*;
        pub struct AddParachainUpgrade;
        const ADD_PARACHAIN_VERSION: u32 = 4_000;
        const PARACHAIN_PARATEST_ID: u32 = 1_599;

        impl frame_support::traits::OnRuntimeUpgrade for AddParachainUpgrade {
            fn on_runtime_upgrade() -> Weight {
                if crate::System::last_runtime_upgrade_spec_version() > ADD_PARACHAIN_VERSION {
                    log::info!("Skipping add paratest parachain upgrade: already applied");
                    return <Runtime as frame_system::Config>::DbWeight::get().reads(1);
                }
                log::info!("Inject paratest parachain");
                let genesis = include_bytes!("paratest_genesis").to_vec();
                let wasm = include_bytes!("paratest_wasm").to_vec();

                let genesis = parachains_paras::GenesisConfig::<Runtime> {
                    _config: core::marker::PhantomData,
                    paras: sp_std::vec![(
                        PARACHAIN_PARATEST_ID.into(),
                        parachains_paras::ParaGenesisArgs {
                            genesis_head: genesis.into(),
                            validation_code: wasm.into(),
                            para_kind: parachains_paras::ParaKind::Parachain,
                        }
                    )],
                };
                use frame_support::traits::BuildGenesisConfig;
                genesis.build();
                sp_runtime::Perbill::from_percent(50) * crate::BlockWeights::get().max_block
            }
        }
    }

    #[cfg(feature = "add-parachain-upgrade")]
    pub type AddParachainUpgrade = add_parachain_upgrade::AddParachainUpgrade;

    #[cfg(not(feature = "add-parachain-upgrade"))]
    pub type AddParachainUpgrade = ();

    /// Unreleased migrations. Add new ones here:
    pub type Unreleased = (AddParachainUpgrade,);
}
