#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "1024"]
// #![deny(missing_docs)]

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod benchmarking;
#[cfg(test)]
mod tests;

#[cfg(feature = "bn254")]
mod bn254;

#[cfg(feature = "bn254")]
mod utils;

pub mod weights;
pub use weights::*;

use ark_scale::hazmat::ArkScaleProjective;

const USAGE: ark_scale::Usage = ark_scale::WIRE;

type ArkScale<T> = ark_scale::ArkScale<T, USAGE>;

// pub use frame_system::pallet::*;
#[cfg(not(doc))]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::{bn254, utils::ScalarFieldFor, ArkScale, ArkScaleProjective, WeightInfo};
    // use ark_bn254::G1Affine;
    use ark_std::{vec, vec::Vec};
    use codec::Decode;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type WeightInfo: WeightInfo;
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // ---------------------------------------------
        // Calls for bn254
        // ---------------------------------------------

        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn bn254_pairing_opt(
            _: OriginFor<T>,
            a: ArkScale<native::G1Affine>,
            b: ArkScale<native::G2Affine>,
        ) -> DispatchResult {
            bn254::pairing_opt(a.0, b.0);
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn bn254_msm_g1_opt(
            _: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
        ) -> DispatchResult {
            let bases = ArkScale::<Vec<native::G1Affine>>::decode(&mut bases.as_slice()).unwrap();
            let scalars =
                ArkScale::<Vec<ScalarFieldFor<native::G1Affine>>>::decode(&mut scalars.as_slice())
                    .unwrap();

            bn254::msm_g1_opt(&bases.0, &scalars.0);
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn bn254_msm_g2_opt(
            _: OriginFor<T>,
            bases: Vec<u8>,
            scalars: Vec<u8>,
        ) -> DispatchResult {
            let bases = ArkScale::<Vec<native::G2Affine>>::decode(&mut bases.as_slice()).unwrap();
            let scalars =
                ArkScale::<Vec<ScalarFieldFor<native::G2Affine>>>::decode(&mut scalars.as_slice())
                    .unwrap();

            bn254::msm_g2_opt(&bases.0, &scalars.0);
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn bn254_mul_projective_g1_opt(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base =
                ArkScaleProjective::<native::G1Projective>::decode(&mut base.as_slice()).unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

            bn254::mul_projective_g1_opt(&base.0, &scalar.0);
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn bn254_mul_affine_g1_opt(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base = ArkScale::<native::G1Affine>::decode(&mut base.as_slice()).unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

            bn254::mul_affine_g1_opt(&base.0, &scalar.0);
            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn bn254_mul_projective_g2_opt(
            _origin: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base =
                ArkScaleProjective::<native::G2Projective>::decode(&mut base.as_slice()).unwrap();
            let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).unwrap();

            bn254::mul_projective_g2_opt(&base.0, &scalar.0);
            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn bn254_mul_affine_g2_opt(
            _: OriginFor<T>,
            base: Vec<u8>,
            scalar: Vec<u8>,
        ) -> DispatchResult {
            let base = ArkScale::<native::G2Affine>::decode(&mut base.as_slice()).unwrap();
            let scalar = ArkScale::<Vec<u64>>::decode(&mut scalar.as_slice()).unwrap();

            bn254::mul_affine_g2_opt(&base.0, &scalar.0);
            Ok(())
        }
    }
}
