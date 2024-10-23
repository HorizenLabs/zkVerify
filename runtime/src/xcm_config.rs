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

#![cfg(feature = "relay")]

//! XCM configuration for Zkv.

use xcm::opaque::v2::MultiLocation;
use xcm_executor::traits::ConvertOrigin;
use core::marker::PhantomData;
use frame_support::traits::OriginTrait;

use super::{
    AccountId,
    AllPalletsWithSystem,
    Balances,
    Dmp,
    ParaId,
    Runtime,
    RuntimeCall,
    RuntimeEvent,
    RuntimeOrigin,
    TransactionByteFee,
    XcmPallet,
    //Treasury,
};
use crate::{
    governance::{FellowshipAdmin, GeneralAdmin, StakingAdmin, Treasurer},
    parachains::parachains_origin,
};
use frame_support::{
    parameter_types,
    traits::{Contains, Equals, Everything, Nothing},
    PalletId,
};
use frame_system::EnsureRoot;
use pallet_xcm::XcmPassthrough;
use polkadot_runtime_common::{
    xcm_sender::{ChildParachainRouter, ExponentialPrice},
    ToAuthor,
};
use sp_runtime::traits::AccountIdConversion;
use xcm::DoubleEncoded;
//use polkadot_runtime_constants::{
//	system_parachain::*,
//};
pub const FELLOWSHIP_ADMIN_INDEX: u32 = 1; // to be moved to some constants mod

use crate::currency::CENTS;
use sp_core::ConstU32;
use xcm::latest::prelude::*;
use xcm_builder::{
    AllowUnpaidExecutionFrom,
    AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
    AllowTopLevelPaidExecutionFrom, ChildParachainAsNative, ChildParachainConvertsVia,
    DescribeAllTerminal, DescribeFamily, FrameTransactionalProcessor, FungibleAdapter,
    HashedDescription, IsConcrete, MintLocation, OriginToPluralityVoice, SignedAccountId32AsNative,
    SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, TrailingSetTopicAsId,
    UsingComponents, WeightInfoBounds, WithComputedOrigin, WithUniqueTopic,
    XcmFeeManagerFromComponents, XcmFeeToAccount,
};

use crate::weights::pallet_xcm::ZKVWeight as XcmPalletZKVWeight;
use crate::weights::xcm::ZKVWeight as XcmZKVWeight;
//use xcm::v4::Junctions::X1;

parameter_types! {
    pub const RootLocation: Location = Here.into_location();
    /// The location of the ACME token, from the context of this chain. Since this token is native to this
    /// chain, we make it synonymous with it and thus it is the `Here` location, which means "equivalent to
    /// the context".
    pub const TokenLocation: Location = Here.into_location();
    /// The Polkadot network ID. This is named.
    pub const ThisNetwork: NetworkId = NetworkId::Polkadot;
    /// Our location in the universe of consensus systems.
    pub UniversalLocation: InteriorLocation = [GlobalConsensus(ThisNetwork::get())].into();
    /// The Checking Account, which holds any native assets that have been teleported out and not back in (yet).
    pub CheckAccount: AccountId = XcmPallet::check_account();
    /// The Checking Account along with the indication that the local chain is able to mint tokens.
    pub LocalCheckAccount: (AccountId, MintLocation) = (CheckAccount::get(), MintLocation::Local);
    /// Account of the treasury pallet.
    pub TreasuryAccount: AccountId = PalletId(*b"zk/trsry").into_account_truncating(); //Treasury::account_id();
}

/// The canonical means of converting a `Location` into an `AccountId`, used when we want to
/// determine the sovereign account controlled by a location.
pub type SovereignAccountOf = (
    // We can convert a child parachain using the standard `AccountId` conversion.
    ChildParachainConvertsVia<ParaId, AccountId>,
    // We can directly alias an `AccountId32` into a local account.
    AccountId32Aliases<ThisNetwork, AccountId>,
    // Foreign locations alias into accounts according to a hash of their standard description.
    HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
);

/// Our asset transactor. This is what allows us to interact with the runtime assets from the point
/// of view of XCM-only concepts like `Location` and `Asset`.
///
/// Ours is only aware of the Balances pallet, which is mapped to `TokenLocation`.
pub type LocalAssetTransactor = FungibleAdapter<
    // Use this currency:
    Balances,
    // Use this currency when it is a fungible asset matching the given location or name:
    IsConcrete<TokenLocation>,
    // We can convert the `Location`s with our converter above:
    SovereignAccountOf,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We track our teleports in/out to keep total issuance correct.
    LocalCheckAccount,
>;

pub struct ParaSignedAccountId32AsNative<Origin>(PhantomData<Origin>);
impl<Origin: OriginTrait> ConvertOrigin<Origin>
    for ParaSignedAccountId32AsNative<Origin>
where
    Origin::AccountId: From<[u8; 32]>,
{
    fn convert_origin(
        origin: impl Into<Location>,
        kind: OriginKind,
    ) -> Result<Origin, Location> {
        let origin = origin.into();
        log::trace!(
            target: "xcm::origin_conversion",
            "FOOOOOOOOOOOOOO origin: {:?}, kind: {:?}",
            origin, kind,
        );
        match (kind, origin.unpack()) {
            (
                OriginKind::Native,
                (0, [Parachain(1599), Junction::AccountId32{ id, .. }])
                //Location { parents: 0, interior: [Parachain(1599)].into() },
            ) =>
                Ok(Origin::signed((*id).into())),
            _ => Err(origin),
        }
    }
}


/// The means that we convert an XCM origin `Location` into the runtime's `Origin` type for
/// local dispatch. This is a conversion function from an `OriginKind` type along with the
/// `Location` value and returns an `Origin` value or an error.
type LocalOriginConverter = (
    // If the origin kind is `Sovereign`, then return a `Signed` origin with the account determined
    // by the `SovereignAccountOf` converter.
    SovereignSignedViaLocation<SovereignAccountOf, RuntimeOrigin>,
    // If the origin kind is `Native` and the XCM origin is a child parachain, then we can express
    // it with the special `parachains_origin::Origin` origin variant.
    ChildParachainAsNative<parachains_origin::Origin, RuntimeOrigin>,
    // If the origin kind is `Native` and the XCM origin is the `AccountId32` location, then it can
    // be expressed using the `Signed` origin variant.
    SignedAccountId32AsNative<ThisNetwork, RuntimeOrigin>,
    ParaSignedAccountId32AsNative<RuntimeOrigin>,
    // Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
    XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
    /// The amount of weight an XCM operation takes. This is a safe overestimate.
    pub const BaseXcmWeight: Weight = Weight::from_parts(1_000_000_000, 1024);
    /// Maximum number of instructions in a single XCM fragment. A sanity check against weight
    /// calculations getting too crazy.
    pub const MaxInstructions: u32 = 100;
    /// The asset ID for the asset that we use to pay for message delivery fees.
    pub FeeAssetId: AssetId = AssetId(TokenLocation::get());
    /// The base fee for the message delivery fees.
    pub const BaseDeliveryFee: u128 = CENTS.saturating_mul(3);
}

pub type PriceForChildParachainDelivery =
    ExponentialPrice<FeeAssetId, BaseDeliveryFee, TransactionByteFee, Dmp>;

/// The XCM router. When we want to send an XCM message, we use this type. It amalgamates all of our
/// individual routers.
pub type XcmRouter = WithUniqueTopic<(
    // Only one router so far - use DMP to communicate with child parachains.
    ChildParachainRouter<Runtime, XcmPallet, PriceForChildParachainDelivery>,
)>;

parameter_types! {
    pub const Acme: AssetFilter = Wild(AllOf { fun: WildFungible, id: AssetId(TokenLocation::get()) });
    pub TestParaLocation: Location = Parachain(1599).into_location();
    pub AcmeForTest: (AssetFilter, Location) = (Acme::get(), TestParaLocation::get());
    pub const MaxAssetsIntoHolding: u32 = 64;
}

/// Polkadot Relay recognizes/respects AssetHub, Collectives, and BridgeHub chains as teleporters.
pub type TrustedTeleporters = xcm_builder::Case<AcmeForTest>;

pub struct OnlyParachains;
impl Contains<Location> for OnlyParachains {
    fn contains(loc: &Location) -> bool {
        matches!(loc.unpack(), (0, [Parachain(_)]))
    }
}

pub struct LocalPlurality;
impl Contains<Location> for LocalPlurality {
    fn contains(loc: &Location) -> bool {
        matches!(loc.unpack(), (0, [Plurality { .. }]))
    }
}

//pub struct TestPara;
//impl Contains<Location> for TestPara {
//    fn contains(loc: &Location) -> bool {
//        *loc == Location { parents: 0, interior: X1(TestParaLocation::get()) }
//    }
//}



/// The barriers one of which must be passed for an XCM message to be executed.
pub type Barrier = TrailingSetTopicAsId<(
    // Weight that is paid for may be consumed.
    TakeWeightCredit,
    // Expected responses are OK.
    AllowKnownQueryResponses<XcmPallet>,
    //AllowUnpaidExecutionFrom<OnlyParachains>,
    WithComputedOrigin<
        (
            // If the message is one that immediately attempts to pay for execution, then allow it.
            AllowTopLevelPaidExecutionFrom<Everything>,
            // Subscriptions for version tracking are OK.
            AllowSubscriptionsFrom<OnlyParachains>,
            // Collectives and Fellows plurality get free execution.
            //AllowExplicitUnpaidExecutionFrom<CollectivesOrFellows>,
        ),
        UniversalLocation,
        ConstU32<8>,
    >,
)>;

/// Locations that will not be charged fees in the executor, neither for execution nor delivery.
/// We only waive fees for system functions, which these locations represent.
pub type WaivedLocations = (Equals<RootLocation>, LocalPlurality);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    //type XcmRecorder = ();
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = LocalOriginConverter;
    // Polkadot Relay recognises no chains which act as reserves.
    type IsReserve = ();
    type IsTeleporter = TrustedTeleporters;
    type UniversalLocation = UniversalLocation;
    type Barrier = Barrier;
    type Weigher = WeightInfoBounds<XcmZKVWeight<RuntimeCall>, RuntimeCall, MaxInstructions>;
    // The weight trader piggybacks on the existing transaction-fee conversion logic.
    type Trader = UsingComponents<
        crate::IdentityFee<crate::Balance>,
        TokenLocation,
        AccountId,
        Balances,
        ToAuthor<Runtime>,
    >;
    type ResponseHandler = XcmPallet;
    type AssetTrap = XcmPallet;
    type AssetLocker = ();
    type AssetExchanger = ();
    type AssetClaims = XcmPallet;
    type SubscriptionService = XcmPallet;
    type PalletInstancesInfo = AllPalletsWithSystem;
    type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
    type FeeManager = XcmFeeManagerFromComponents<
        WaivedLocations,
        XcmFeeToAccount<Self::AssetTransactor, AccountId, TreasuryAccount>,
    >;
    // No bridges on the Relay Chain
    type MessageExporter = ();
    type UniversalAliases = Nothing;
    type CallDispatcher = RuntimeCall;
    type SafeCallFilter = Everything;
    type Aliasers = Nothing;
    type TransactionalProcessor = FrameTransactionalProcessor;
    type HrmpNewChannelOpenRequestHandler = ();
    type HrmpChannelAcceptedHandler = ();
    type HrmpChannelClosingHandler = ();
}

parameter_types! {
    // `GeneralAdmin` pluralistic body.
    pub const GeneralAdminBodyId: BodyId = BodyId::Administration;
    // StakingAdmin pluralistic body.
    pub const StakingAdminBodyId: BodyId = BodyId::Defense;
    // FellowshipAdmin pluralistic body.
    pub const FellowshipAdminBodyId: BodyId = BodyId::Index(FELLOWSHIP_ADMIN_INDEX);
    // `Treasurer` pluralistic body.
    pub const TreasurerBodyId: BodyId = BodyId::Treasury;
}

/// Type to convert the `GeneralAdmin` origin to a Plurality `Location` value.
pub type GeneralAdminToPlurality =
    OriginToPluralityVoice<RuntimeOrigin, GeneralAdmin, GeneralAdminBodyId>;

/// Type to convert an `Origin` type value into a `Location` value which represents an interior
/// location of this chain.
pub type LocalOriginToLocation = (
    GeneralAdminToPlurality,
    // And a usual Signed origin to be used in XCM as a corresponding AccountId32
    SignedToAccountId32<RuntimeOrigin, AccountId, ThisNetwork>,
);

/// Type to convert the `StakingAdmin` origin to a Plurality `Location` value.
pub type StakingAdminToPlurality =
    OriginToPluralityVoice<RuntimeOrigin, StakingAdmin, StakingAdminBodyId>;

/// Type to convert the `FellowshipAdmin` origin to a Plurality `Location` value.
pub type FellowshipAdminToPlurality =
    OriginToPluralityVoice<RuntimeOrigin, FellowshipAdmin, FellowshipAdminBodyId>;

/// Type to convert the `Treasurer` origin to a Plurality `Location` value.
pub type TreasurerToPlurality = OriginToPluralityVoice<RuntimeOrigin, Treasurer, TreasurerBodyId>;

/// Type to convert a pallet `Origin` type value into a `Location` value which represents an
/// interior location of this chain for a destination chain.
pub type LocalPalletOriginToLocation = (
    // GeneralAdmin origin to be used in XCM as a corresponding Plurality `Location` value.
    GeneralAdminToPlurality,
    // StakingAdmin origin to be used in XCM as a corresponding Plurality `Location` value.
    StakingAdminToPlurality,
    // FellowshipAdmin origin to be used in XCM as a corresponding Plurality `Location` value.
    FellowshipAdminToPlurality,
    // `Treasurer` origin to be used in XCM as a corresponding Plurality `Location` value.
    TreasurerToPlurality,
);

impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // We only allow the root, the general admin, the fellowship admin and the staking admin to send
    // messages.
    type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalPalletOriginToLocation>;
    type XcmRouter = XcmRouter;
    // Anyone can execute XCM messages locally.
    type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmExecuteFilter = Everything;
    type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
    type XcmTeleportFilter = Everything; // == Allow All
    type XcmReserveTransferFilter = Everything; // == Allow All
    type Weigher = WeightInfoBounds<XcmZKVWeight<RuntimeCall>, RuntimeCall, MaxInstructions>;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = ();
    type TrustedLockers = ();
    type SovereignAccountOf = SovereignAccountOf;
    type MaxLockers = ConstU32<8>;
    type MaxRemoteLockConsumers = ConstU32<0>;
    type RemoteLockConsumerIdentifier = ();
    type WeightInfo = XcmPalletZKVWeight<Runtime>; //crate::weights::pallet_xcm::WeightInfo<Runtime>;
    type AdminOrigin = EnsureRoot<AccountId>;
}
