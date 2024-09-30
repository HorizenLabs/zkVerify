use crate::RuntimeCall;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::InstanceFilter;
use sp_runtime::RuntimeDebug;

/// The type used to represent the kinds of proxying allowed.
#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Encode,
    Decode,
    RuntimeDebug,
    MaxEncodedLen,
    scale_info::TypeInfo,
)]
pub enum ProxyType {
    Any = 0,
    NonTransfer = 1,
    Governance = 2,
    Staking = 3,
    CancelProxy = 4,
}

impl Default for ProxyType {
    fn default() -> Self {
        Self::Any
    }
}
impl InstanceFilter<RuntimeCall> for ProxyType {
    fn filter(&self, c: &RuntimeCall) -> bool {
        match self {
            ProxyType::Any => true,
            ProxyType::NonTransfer => matches!(
                c,
                RuntimeCall::System(..) |
				RuntimeCall::Scheduler(..) |
				RuntimeCall::Babe(..) |
				RuntimeCall::Timestamp(..) |
				// Specifically omitting Indices `transfer`, `force_transfer`
				// Specifically omitting the entire Balances pallet
				RuntimeCall::Staking(..) |
				RuntimeCall::Session(..) |
				RuntimeCall::Grandpa(..) |
				RuntimeCall::Treasury(..) |
				RuntimeCall::Bounties(..) |
				RuntimeCall::ChildBounties(..) |
				RuntimeCall::ConvictionVoting(..) |
				RuntimeCall::Referenda(..) |
				RuntimeCall::Whitelist(..) |
				RuntimeCall::Vesting(pallet_vesting::Call::vest{..}) |
				RuntimeCall::Vesting(pallet_vesting::Call::vest_other{..}) |
				// Specifically omitting Vesting `vested_transfer`, and `force_vested_transfer`
				RuntimeCall::Utility(..) |
				RuntimeCall::Proxy(..) |
				RuntimeCall::Multisig(..) |
				RuntimeCall::VoterList(..) |
                 // RuntimeCall::Indices(pallet_indices::Call::claim{..}) |
                                                            // RuntimeCall::Indices(pallet_indices::Call::free{..}) |
                                                            // RuntimeCall::Indices(pallet_indices::Call::freeze{..}) |
                                                            // RuntimeCall::Registrar(paras_registrar::Call::register {..}) |
                                                            // RuntimeCall::Registrar(paras_registrar::Call::deregister {..}) |
                                                            // // Specifically omitting Registrar `swap`
                                                            // RuntimeCall::Registrar(paras_registrar::Call::reserve {..}) |
                                                            // RuntimeCall::Crowdloan(..) |
                                                            // RuntimeCall::Slots(..) |
                                                            // RuntimeCall::Auctions(..) | // Specifically omitting the entire XCM Pallet
                                                            // RuntimeCall::NominationPools(..) |
                                                            // RuntimeCall::FastUnstake(..) |
                                                            // RuntimeCall::Claims(..)

                // zkVerify specifics
                RuntimeCall::Poe(..) |
                RuntimeCall::SettlementFFlonkPallet(..) |
                RuntimeCall::SettlementZksyncPallet(..) |
                RuntimeCall::SettlementGroth16Pallet(..) |
                RuntimeCall::SettlementRisc0Pallet(..) |
                RuntimeCall::SettlementUltraplonkPallet(..)
            ),
            ProxyType::Governance => matches!(
                c,
                RuntimeCall::Treasury(..)
                    | RuntimeCall::Bounties(..)
                    | RuntimeCall::Utility(..)
                    | RuntimeCall::ChildBounties(..)
                    | RuntimeCall::ConvictionVoting(..)
                    | RuntimeCall::Referenda(..)
                    | RuntimeCall::Whitelist(..)
            ),
            ProxyType::Staking => {
                matches!(
                    c,
                    RuntimeCall::Staking(..)
                        | RuntimeCall::Session(..)
                        | RuntimeCall::Utility(..)
                        | RuntimeCall::VoterList(..) // RuntimeCall::FastUnstake(..) |
                                                     // RuntimeCall::NominationPools(..)
                )
            }
            ProxyType::CancelProxy => {
                matches!(
                    c,
                    RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })
                )
            }
        }
    }
    fn is_superset(&self, o: &Self) -> bool {
        match (self, o) {
            (x, y) if x == y => true,
            (ProxyType::Any, _) => true,
            (_, ProxyType::Any) => false,
            (ProxyType::NonTransfer, _) => true,
            _ => false,
        }
    }
}
