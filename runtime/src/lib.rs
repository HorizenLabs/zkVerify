// Copyright 2024, Horizen Labs, Inc.

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

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use pallet_babe::AuthorityId as BabeId;
use pallet_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use proof_of_existence_rpc_runtime_api::MerkleProof;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One, OpaqueKeys, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_election_provider_support::{
    bounds::{ElectionBounds, ElectionBoundsBuilder},
    onchain,
    onchain::OnChainExecution,
    SequentialPhragmen,
};
use frame_support::genesis_builder_helper::{build_config, create_default_config};

// A few exports that help ease life for downstream crates.
use frame_support::traits::EqualPrivilegeOnly;

pub use frame_support::{
    construct_runtime, derive_impl,
    dispatch::DispatchClass,
    parameter_types,
    traits::{
        ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness,
        StorageInfo,
    },
    weights::{
        constants::{RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use frame_system::Call as SystemCall;
use frame_system::EnsureRoot;
pub use pallet_balances::Call as BalancesCall;
use pallet_session::historical as pallet_session_historical;
pub use pallet_timestamp::Call as TimestampCall;
use static_assertions::const_assert;
use weights::block_weights::BlockExecutionWeight;
use weights::extrinsic_weights::ExtrinsicBaseWeight;

use pallet_transaction_payment::{ConstFeeMultiplier, CurrencyAdapter, Multiplier};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

pub mod governance;
use governance::pallet_custom_origins;

#[cfg(test)]
mod tests;
mod weights;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub mod currency {
    pub type Balance = u128;
    pub const ACME: Balance = 1_000_000_000_000_000_000;
    pub const CENTS: Balance = ACME / 100;
    pub const THOUSANDS: Balance = 1_000 * ACME;
    pub const MILLIONS: Balance = 1_000 * THOUSANDS;
    pub const MILLICENTS: Balance = CENTS / 1_000;
    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 200 * CENTS + (bytes as Balance) * 100 * MILLICENTS
    }
}

use currency::*;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
}

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("nh-node"), // TODO: change to zkv-node when ok with regenesis
    impl_name: create_runtime_str!("zkv-node"),
    authoring_version: 1,
    // The version of the runtime specification. A full node will not attempt to use its native
    //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    //   `spec_version`, and `authoring_version` are the same between Wasm and native.
    spec_version: 4_000,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_babe` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// 1 in 4 blocks will be primary babe blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time, with maximum proof size.
const MAXIMUM_BLOCK_WEIGHT: Weight =
    Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), u64::MAX);

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = VERSION;

    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();

    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);

    // ASCII for 'Z'+'K'+'V'
    pub const SS58Prefix: u8 = 251;
}

/// The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
/// [`SoloChainDefaultConfig`](`struct@frame_system::config_preludes::SolochainDefaultConfig`),
/// but overridden as needed.
#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
    /// The block type for the runtime.
    type Block = Block;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = BlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = BlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The type for storing how many extrinsics an account has signed.
    type Nonce = Nonce;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type SystemWeightInfo = weights::frame_system::ZKVWeight<Runtime>;
}

parameter_types! {
    pub const ExpectedBlockTime: u64 = MILLISECS_PER_BLOCK; // Should use primitives::Moment
    pub const EpochDurationInBlocks: BlockNumber = HOURS; // TODO: use prod_or_fast!

    /// How long (in blocks) an equivocation report is valid for
    pub ReportLongevity: u64 = EpochDurationInBlocks::get() as u64 * 10;
    /// How many authorities BABE and GRANDPA have storage for
    pub const MaxAuthorities: u32 = MaxActiveValidators::get();
}

impl pallet_babe::Config for Runtime {
    type EpochDuration = EpochDurationInBlocks;
    type ExpectedBlockTime = ExpectedBlockTime;
    // session module is the trigger
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    type DisabledValidators = Session;
    type WeightInfo = weights::pallet_babe::ZKVWeight<Runtime>;
    type MaxAuthorities = MaxAuthorities;
    type MaxNominators = ConstU32<MAX_VOTERS>;
    type KeyOwnerProof = sp_session::MembershipProof;
    type EquivocationReportSystem =
        pallet_babe::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    type WeightInfo = weights::pallet_grandpa::ZKVWeight<Runtime>;
    type MaxAuthorities = MaxAuthorities;
    type MaxNominators = ConstU32<MAX_VOTERS>;
    type MaxSetIdSessionEntries = ConstU64<0>;

    type KeyOwnerProof = sp_session::MembershipProof;
    type EquivocationReportSystem =
        pallet_grandpa::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Babe;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>; // this is a Babe assumption
    type WeightInfo = weights::pallet_timestamp::ZKVWeight<Runtime>;
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = MILLICENTS;

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = weights::pallet_balances::ZKVWeight<Runtime>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = ();
}

parameter_types! {
    pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub babe: Babe,
        pub grandpa: Grandpa,
        pub im_online: ImOnline,
    }
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = weights::pallet_sudo::ZKVWeight<Runtime>;
}

parameter_types! {
    // One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
    pub const MultisigDepositBase: Balance = currency::deposit(1, 88);
    // Additional storage item size of 32 bytes.
    pub const MultisigDepositFactor: Balance = currency::deposit(0, 32);
    pub const MaxSignatories: u32 = 100;
}
impl pallet_multisig::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type Currency = Balances;
    type DepositBase = MultisigDepositBase;
    type DepositFactor = MultisigDepositFactor;
    type MaxSignatories = MaxSignatories;
    type WeightInfo = weights::pallet_multisig::ZKVWeight<Runtime>;
}

parameter_types! {
    pub const PreimageBaseDeposit: Balance = deposit(2, 64);
    pub const PreimageByteDeposit: Balance = deposit(0, 1);
    pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
}

impl pallet_preimage::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_preimage::ZKVWeight<Runtime>;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type Consideration = frame_support::traits::fungible::HoldConsideration<
        AccountId,
        Balances,
        PreimageHoldReason,
        frame_support::traits::LinearStoragePrice<
            PreimageBaseDeposit,
            PreimageByteDeposit,
            Balance,
        >,
    >;
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
    pub MaxScheduledPerBlock: u32 = 50;
}

impl pallet_scheduler::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;

    type OriginPrivilegeCmp = EqualPrivilegeOnly;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = weights::pallet_scheduler::ZKVWeight<Runtime>;
    type Preimages = Preimage;
}

pub const MILLISECS_PER_PROOF_ROOT_PUBLISHING: u64 = MILLISECS_PER_BLOCK * 10;
pub const MIN_PROOFS_FOR_ROOT_PUBLISHING: u32 = 5;
// We should avoid publishing attestations for empty trees
static_assertions::const_assert!(MIN_PROOFS_FOR_ROOT_PUBLISHING > 0);

impl pallet_poe::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MinProofsForPublishing = ConstU32<MIN_PROOFS_FOR_ROOT_PUBLISHING>;
    type MaxElapsedTimeMs = ConstU64<MILLISECS_PER_PROOF_ROOT_PUBLISHING>;
    type WeightInfo = weights::pallet_poe::ZKVWeight<Runtime>;
}

pub struct ValidatorIdOf;
impl sp_runtime::traits::Convert<AccountId, Option<AccountId>> for ValidatorIdOf {
    fn convert(a: AccountId) -> Option<AccountId> {
        Some(a)
    }
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ValidatorIdOf;
    type ShouldEndSession = Babe;
    type NextSessionRotation = Babe;
    type SessionManager = Staking;
    type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type WeightInfo = weights::pallet_session::ZKVWeight<Runtime>;
}

//TODO: Set these parameters appropriately.
pallet_staking_reward_curve::build! {
    const REWARD_CURVE: sp_runtime::curve::PiecewiseLinear<'static> = curve!(
        min_inflation: 0_025_000,
        max_inflation: 0_100_000,
        ideal_stake: 0_500_000,
        falloff: 0_050_000,
        max_piece_count: 40,
        test_precision: 0_005_000,
    );
}

parameter_types! {
    pub const SessionsPerEra: sp_staking::SessionIndex = 6 * HOURS / EpochDurationInBlocks::get(); // number of sessions in 1 era, 6h

    pub const RewardCurve: &'static sp_runtime::curve::PiecewiseLinear<'static> = &REWARD_CURVE;

    pub const BondingDuration: sp_staking::EraIndex = 1; // number of sessions for which staking
                                                         // remains locked
    pub const SlashDeferDuration: sp_staking::EraIndex = 0; // eras to wait before slashing is
                                                            // applied
    pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
    pub HistoryDepth: u32 = 30; // Number of eras to keep in history. Older eras cannot be claimed.
}

/// Maximum number of election targets (eligible authorities) to account for
pub const MAX_TARGETS: u32 = 32;
/// Maximum number of election voters to account for
pub const MAX_VOTERS: u32 = 32;

parameter_types! {
    pub ElectionBoundsOnChain: ElectionBounds = ElectionBoundsBuilder::default().voters_count((MAX_TARGETS + MAX_VOTERS).into()).targets_count(MAX_TARGETS.into()).build();
    pub const MaxActiveValidators: u32 = MAX_TARGETS;
}

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
    type System = Runtime;
    type Solver = SequentialPhragmen<AccountId, sp_runtime::Perbill>;
    type DataProvider = Staking;
    type WeightInfo = weights::frame_election_provider_support::ZKVWeight<Runtime>;
    type MaxWinners = MaxActiveValidators;
    type Bounds = ElectionBoundsOnChain;
}

/// The numbers configured here could always be more than the the maximum limits of staking pallet
/// to ensure election snapshot will not run out of memory. For now, we set them to smaller values
/// since the staking is bounded and the weight pipeline takes hours for this single pallet.
pub struct ElectionProviderBenchmarkConfig;
impl pallet_staking::BenchmarkingConfig for ElectionProviderBenchmarkConfig {
    type MaxNominators = ConstU32<MAX_VOTERS>;
    type MaxValidators = ConstU32<MAX_TARGETS>;
}

impl pallet_staking::Config for Runtime {
    type Currency = Balances;
    type CurrencyBalance = Balance;
    type UnixTime = Timestamp;
    type CurrencyToVote = sp_staking::currency_to_vote::U128CurrencyToVote;
    type RewardRemainder = (); // burn the remainder, should be Treasury
    type RuntimeEvent = RuntimeEvent;
    type Slash = (); // burn the slashed funds, should be Treasury.
    type Reward = (); // rewards are minted from the void
    type SessionsPerEra = SessionsPerEra;
    type BondingDuration = BondingDuration;
    type SlashDeferDuration = SlashDeferDuration;
    /// A super-majority of the council can cancel the slash.
    type AdminOrigin = EnsureRoot<AccountId>;
    type SessionInterface = Self;
    type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
    type NextNewSession = Session;
    type OffendingValidatorsThreshold = OffendingValidatorsThreshold; // Exceeding this threshold would force a new era
    type ElectionProvider = OnChainExecution<OnChainSeqPhragmen>;
    type GenesisElectionProvider = OnChainExecution<OnChainSeqPhragmen>;
    // TODO: consider switching to bags-list
    type VoterList = pallet_staking::UseNominatorsAndValidatorsMap<Self>;
    type NominationsQuota = pallet_staking::FixedNominationsQuota<10>;
    // TODO: consider switching to bags-list
    type TargetList = pallet_staking::UseValidatorsMap<Self>;
    type MaxUnlockingChunks = ConstU32<32>;
    type HistoryDepth = HistoryDepth; // Number of eras to keep in history
    type EventListeners = (); // NominationPools;
    type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
    type BenchmarkingConfig = ElectionProviderBenchmarkConfig;
    type MaxExposurePageSize = ConstU32<64>;
    type MaxControllersInDeprecationBatch = ConstU32<0>; // We do not have any controller accounts
}

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
    type EventHandler = (Staking, ImOnline);
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    RuntimeCall: From<C>,
{
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = RuntimeCall;
}

impl pallet_session::historical::Config for Runtime {
    type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
    type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

parameter_types! {
    pub const MaxKeys: u32 = 10_000; // We need them for benchmarking
    pub const MaxPeerInHeartbeats: u32 = 10_000;
}

impl pallet_im_online::Config for Runtime {
    type AuthorityId = ImOnlineId;
    type RuntimeEvent = RuntimeEvent;
    type NextSessionRotation = Babe;
    type ValidatorSet = Historical;
    type ReportUnresponsiveness = Offences;
    type UnsignedPriority = ();
    type WeightInfo = weights::pallet_im_online::ZKVWeight<Runtime>;
    type MaxKeys = MaxKeys;
    type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
}

impl pallet_offences::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = Staking;
}

impl pallet_verifiers::Config<pallet_fflonk_verifier::Fflonk> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnProofVerified = Poe;
    type WeightInfo =
        pallet_fflonk_verifier::FflonkWeight<weights::pallet_fflonk_verifier::ZKVWeight<Runtime>>;
}

impl pallet_verifiers::Config<pallet_zksync_verifier::Zksync> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnProofVerified = Poe;
    type WeightInfo =
        pallet_zksync_verifier::ZksyncWeight<weights::pallet_zksync_verifier::ZKVWeight<Runtime>>;
}

pub const GROTH16_MAX_NUM_INPUTS: u32 = 16;
parameter_types! {
    pub const Groth16MaxNumInputs: u32 = GROTH16_MAX_NUM_INPUTS;
}

impl pallet_groth16_verifier::Config for Runtime {
    const MAX_NUM_INPUTS: u32 = Groth16MaxNumInputs::get();
}

// We should be sure that the max number of inputs does not exceed the max number of inputs in the verifier crate.
const_assert!(
    <Runtime as pallet_groth16_verifier::Config>::MAX_NUM_INPUTS
        <= pallet_groth16_verifier::MAX_NUM_INPUTS
);

impl pallet_verifiers::Config<pallet_groth16_verifier::Groth16<Runtime>> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnProofVerified = Poe;
    type WeightInfo = pallet_groth16_verifier::Groth16Weight<
        weights::pallet_groth16_verifier::ZKVWeight<Runtime>,
    >;
}

parameter_types! {
    pub const Risc0MaxProofSize: u32 = 2455714; // 2455714: risc0 proof size for a 2^24 cycle-count run
    pub const Risc0MaxPubsSize: u32 = 8 + 4 + 32 * 64; // 8: for bincode::serialize,
                                                       // 4: bytes for payload length,
                                                       // 32 * 64: sufficient multiple of 32 bytes
}

impl pallet_risc0_verifier::Config for Runtime {
    type MaxProofSize = Risc0MaxProofSize;
    type MaxPubsSize = Risc0MaxPubsSize;
}

impl pallet_verifiers::Config<pallet_risc0_verifier::Risc0<Runtime>> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnProofVerified = Poe;
    type WeightInfo =
        pallet_risc0_verifier::Risc0Weight<weights::pallet_risc0_verifier::ZKVWeight<Runtime>>;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        Authorship: pallet_authorship,
        Staking: pallet_staking,
        Session: pallet_session,
        Babe: pallet_babe,
        Grandpa: pallet_grandpa,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        Multisig: pallet_multisig,
        Scheduler: pallet_scheduler,
        Preimage: pallet_preimage,
        ConvictionVoting: pallet_conviction_voting,
        Origins: pallet_custom_origins,
        Whitelist: pallet_whitelist,
        Referenda: pallet_referenda,
        Offences: pallet_offences,
        Historical: pallet_session_historical::{Pallet},
        ImOnline: pallet_im_online,
        SettlementFFlonkPallet: pallet_fflonk_verifier,
        Poe: pallet_poe,
        SettlementZksyncPallet: pallet_zksync_verifier,
        SettlementGroth16Pallet: pallet_groth16_verifier,
        SettlementRisc0Pallet: pallet_risc0_verifier,
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// All migrations of the runtime, aside from the ones declared in the pallets.
///
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = ();

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    define_benchmarks!(
        [frame_benchmarking, BaselineBench::<Runtime>]
        [frame_system, SystemBench::<Runtime>]
        [pallet_balances, Balances]
        [pallet_babe, crate::Babe]
        [pallet_grandpa, crate::Grandpa]
        [pallet_timestamp, Timestamp]
        [pallet_sudo, Sudo]
        [pallet_multisig, Multisig]
        [pallet_scheduler, Scheduler]
        [pallet_preimage, Preimage]
        [pallet_session, SessionBench::<Runtime>]
        [pallet_staking, Staking]
        [pallet_im_online, ImOnline]
        [frame_election_provider_support, ElectionProviderBench::<Runtime>]
        [pallet_poe, Poe]
        [pallet_zksync_verifier, ZksyncVerifierBench::<Runtime>]
        [pallet_fflonk_verifier, FflonkVerifierBench::<Runtime>]
        [pallet_groth16_verifier, Groth16VerifierBench::<Runtime>]
        [pallet_risc0_verifier, Risc0VerifierBench::<Runtime>]
    );
}

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
    sp_consensus_babe::BabeEpochConfiguration {
        c: PRIMARY_PROBABILITY,
        allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
    };

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_babe::BabeApi<Block> for Runtime {
        fn configuration() -> sp_consensus_babe::BabeConfiguration {
            let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
            sp_consensus_babe::BabeConfiguration {
                slot_duration: Babe::slot_duration(),
                epoch_length: EpochDurationInBlocks::get().into(),
                c: epoch_config.c,
                authorities: Babe::authorities().to_vec(),
                randomness: Babe::randomness(),
                allowed_slots: epoch_config.allowed_slots,
            }
        }

        fn current_epoch_start() -> sp_consensus_babe::Slot {
            Babe::current_epoch_start()
        }

        fn current_epoch() -> sp_consensus_babe::Epoch {
            Babe::current_epoch()
        }

        fn next_epoch() -> sp_consensus_babe::Epoch {
            Babe::next_epoch()
        }

        fn generate_key_ownership_proof(
            _slot: sp_consensus_babe::Slot,
            authority_id: BabeId,
        ) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
            use codec::Encode;

            Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
                .map(|p| p.encode())
                .map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
            key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Babe::submit_unsigned_equivocation_report(
                equivocation_proof,
                key_owner_proof,
            )
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Grandpa::submit_unsigned_equivocation_report(
                equivocation_proof,
                key_owner_proof,
            )
        }

        fn generate_key_ownership_proof(
            _set_id: sp_consensus_grandpa::SetId,
            _authority_id: GrandpaId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
        for Runtime
    {
        fn query_call_info(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_call_info(call, len)
        }
        fn query_call_fee_details(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_call_fee_details(call, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl proof_of_existence_rpc_runtime_api::PoEApi<Block> for Runtime {
        fn get_proof_path(
            attestation_id: u64,
            proof_hash: sp_core::H256
        ) -> Result<MerkleProof, proof_of_existence_rpc_runtime_api::AttestationPathRequestError> {
            Poe::get_proof_path_from_pallet(attestation_id, proof_hash).map(|c| c.into())
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;
            use pallet_election_provider_support_benchmarking::Pallet as ElectionProviderBench;
            use pallet_session_benchmarking::Pallet as SessionBench;
            use pallet_fflonk_verifier::benchmarking::Pallet as FflonkVerifierBench;
            use pallet_zksync_verifier::benchmarking::Pallet as ZksyncVerifierBench;
            use pallet_groth16_verifier::benchmarking::Pallet as Groth16VerifierBench;
            use pallet_risc0_verifier::benchmarking::Pallet as Risc0VerifierBench;

            let mut list = Vec::<BenchmarkList>::new();

            list_benchmarks!(list, extra);
            let storage_info = AllPalletsWithSystem::storage_info();

            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};
            use sp_storage::TrackedStorageKey;
            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;
            use pallet_election_provider_support_benchmarking::Pallet as ElectionProviderBench;
            use pallet_session_benchmarking::Pallet as SessionBench;
            use pallet_fflonk_verifier::benchmarking::Pallet as FflonkVerifierBench;
            use pallet_zksync_verifier::benchmarking::Pallet as ZksyncVerifierBench;
            use pallet_groth16_verifier::benchmarking::Pallet as Groth16VerifierBench;
            use pallet_risc0_verifier::benchmarking::Pallet as Risc0VerifierBench;

            impl frame_system_benchmarking::Config for Runtime {}
            impl baseline::Config for Runtime {}
            impl pallet_election_provider_support_benchmarking::Config for Runtime {}

            impl pallet_session_benchmarking::Config for Runtime {}

            use frame_support::traits::WhitelistedStorageKeys;
            let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            Ok(batches)
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here. If any of the pre/post migration checks fail, we shall stop
            // right here and right now.
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, BlockWeights::get().max_block)
        }

        fn execute_block(
            block: Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect
        ) -> Weight {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here.
            Executive::try_execute_block(block, state_root_check, signature_check, select).expect("execute-block failed")
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn create_default_config() -> Vec<u8> {
            create_default_config::<RuntimeGenesisConfig>()
        }

        fn build_config(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_config::<RuntimeGenesisConfig>(config)
        }
    }
}
