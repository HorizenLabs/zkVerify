// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use ismp::module::IsmpModule;
use ismp::router::{PostRequest, Response, Timeout};
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use frame_support::{pallet_prelude::*, PalletId};
    use frame_system::pallet_prelude::*;
    use ismp::dispatcher::{DispatchPost, DispatchRequest, FeeMetadata, IsmpDispatcher};
    use ismp::host::StateMachine;
    use pallet_ismp::ModuleId;

    pub const ZKV_MODULE_ID: ModuleId = ModuleId::Pallet(PalletId(*b"ZKVE-MOD"));

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_ismp::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// [`IsmpDispatcher`] implementation
        type IsmpDispatcher: IsmpDispatcher<Account = Self::AccountId, Balance = Self::Balance>
            + Default;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MessageReceived,
        TimeoutProcessed,
    }

    #[pallet::error]
    pub enum Error<T> {
        MessageDispatchFailed,
    }

    // Hack for implementing the [`Default`] bound needed for
    // [`IsmpDispatcher`](ismp::dispatcher::IsmpDispatcher) and
    // [`IsmpModule`](ismp::module::IsmpModule)
    impl<T> Default for Pallet<T> {
        fn default() -> Self {
            Self(PhantomData)
        }
    }

    /// Extrisnic params for evm dispatch
    #[derive(
        Clone, codec::Encode, codec::Decode, scale_info::TypeInfo, PartialEq, Eq, RuntimeDebug,
    )]
    pub struct Params<Balance> {
        /// Attestation id
        pub aggregation_id: u64,

        /// Attestation of Merkle tree
        pub aggregation: sp_core::H256,

        /// Destination contract
        pub module: sp_core::H160,

        /// Destination State Machine
        pub destination: StateMachine,

        /// Timeout timestamp on destination chain in seconds
        pub timeout: u64,

        /// A relayer fee for message delivery
        pub fee: Balance,
    }

    #[derive(Encode, Decode)]
    struct AggregationData {
        aggregation_id: u64,
        aggregation: sp_core::H256,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Dispatch aggregation to given EVM chain
        #[pallet::weight(Weight::from_parts(1_000_000, 0))]
        #[pallet::call_index(0)]
        pub fn dispatch_aggregation(
            origin: OriginFor<T>,
            params: Params<T::Balance>,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;

            // Create and encode the aggregation data
            let data = AggregationData {
                aggregation_id: params.aggregation_id,
                aggregation: params.aggregation,
            };
            let body = data.encode();

            let post = DispatchPost {
                dest: params.destination,
                from: ZKV_MODULE_ID.to_bytes(),
                to: params.module.0.to_vec(),
                timeout: params.timeout,
                body,
            };

            let dispatcher = T::IsmpDispatcher::default();

            // dispatch the request
            // This call will attempt to collect the protocol fee and relayer fee from the user's account
            dispatcher
                .dispatch_request(
                    DispatchRequest::Post(post),
                    FeeMetadata {
                        payer: origin,
                        fee: params.fee,
                    },
                )
                .map_err(|_| Error::<T>::MessageDispatchFailed)?;

            Ok(())
        }
    }
}

impl<T: Config> IsmpModule for Pallet<T> {
    fn on_accept(&self, _request: PostRequest) -> Result<(), anyhow::Error> {
        // Here you would perform validations on the post request data
        // Ensure it can be executed successfully before making any state changes
        // You can also dispatch a post response after execution
        Self::deposit_event(Event::<T>::MessageReceived);
        Ok(())
    }

    fn on_response(&self, _response: Response) -> Result<(), anyhow::Error> {
        // Here you would perform validations on the post request data
        // Ensure it can be executed successfully before making any state changes
        Self::deposit_event(Event::<T>::MessageReceived);
        Ok(())
    }

    fn on_timeout(&self, _request: Timeout) -> Result<(), anyhow::Error> {
        // Here you would revert all the state changes that were made when the
        // request was initially dispatched
        Self::deposit_event(Event::<T>::TimeoutProcessed);
        Ok(())
    }
}
