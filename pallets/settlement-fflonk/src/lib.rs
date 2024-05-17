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

/// This pallet provides FFlonk verification for CDK prover.
pub use pallet::*;

#[cfg(test)]
pub mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weight;

pub use weight::WeightInfo;
pub const FULL_PROOF_SIZE: usize = 25 * 32;
pub const PUBS_SIZE: usize = 32;
pub const PROOF_SIZE: usize = 24 * 32;
pub type Proof = [u8; FULL_PROOF_SIZE];

#[frame_support::pallet]
pub mod pallet {
    use super::{Proof, WeightInfo, FULL_PROOF_SIZE, PROOF_SIZE};
    use codec::{Decode, Encode};
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_system::pallet_prelude::*;
    use hp_poe::OnProofVerified;
    use scale_info::TypeInfo;
    use sp_core::{H256, U256};
    use sp_io::hashing::keccak_256;
    use sp_std::boxed::Box;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Proof verified call back
        type OnProofVerified: OnProofVerified;
        /// The weight definition for this pallet
        type WeightInfo: WeightInfo;
    }

    #[derive(Clone, Debug, Encode, Decode, PartialEq, TypeInfo)]
    struct Fr(U256);
    #[derive(Clone, Debug, Encode, Decode, PartialEq, TypeInfo)]
    struct Fq(U256);
    #[derive(Clone, Debug, Encode, Decode, PartialEq, TypeInfo)]
    struct Fq2(Fq, Fq);
    #[derive(Clone, Debug, Encode, Decode, PartialEq, TypeInfo)]
    struct G1(Fq, Fq, Fq);
    #[derive(Clone, Debug, Encode, Decode, PartialEq, TypeInfo)]
    struct G2(Fq2, Fq2, Fq2);

    #[derive(Clone, Debug, Encode, Decode, PartialEq, TypeInfo)]
    pub struct Vk {
        power: u8,
        k1: Fr,
        k2: Fr,
        w: Fr,
        w3: Fr,
        w4: Fr,
        w8: Fr,
        wr: Fr,
        x2: G2,
        c0: G1,
    }

    trait IntoBytes {
        fn into_bytes(self) -> [u8; 32];
    }

    impl IntoBytes for U256 {
        fn into_bytes(self) -> [u8; 32] {
            let mut out = [0; 32];
            self.to_big_endian(&mut out);
            out
        }
    }

    impl Into<substrate_bn::Fr> for Fr {
        fn into(self) -> substrate_bn::Fr {
            substrate_bn::Fr::from_slice(&self.0.into_bytes())
                .expect("BUG: should be hardcoded. qed")
        }
    }

    impl Into<substrate_bn::Fq> for Fq {
        fn into(self) -> substrate_bn::Fq {
            substrate_bn::Fq::from_slice(&self.0.into_bytes())
                .expect("BUG: should be hardcoded. qed")
        }
    }

    impl Into<substrate_bn::Fq2> for Fq2 {
        fn into(self) -> substrate_bn::Fq2 {
            substrate_bn::Fq2::new(self.0.into(), self.1.into())
        }
    }

    pub enum ConvertError {
        InvalidG1Point,
        InvalidG2Point,
    }

    impl TryInto<substrate_bn::G1> for G1 {
        type Error = ConvertError;

        fn try_into(self) -> Result<substrate_bn::G1, Self::Error> {
            let g1 = substrate_bn::G1::new(self.0.into(), self.1.into(), self.2.into());
            let mut check = g1.clone();
            use substrate_bn::Group;
            check.normalize();
            substrate_bn::AffineG1::new(check.x(), check.y())
                .map_err(|_e| ConvertError::InvalidG1Point)?;
            Ok(g1)
        }
    }

    impl TryInto<substrate_bn::G2> for G2 {
        type Error = ConvertError;

        fn try_into(self) -> Result<substrate_bn::G2, Self::Error> {
            let g2 = substrate_bn::G2::new(self.0.into(), self.1.into(), self.2.into());
            let mut check = g2.clone();
            use substrate_bn::Group;
            check.normalize();
            substrate_bn::AffineG2::new(check.x(), check.y())
                .map_err(|_e| ConvertError::InvalidG2Point)?;
            Ok(g2)
        }
    }

    impl TryInto<fflonk_verifier::VerificationKey> for Vk {
        type Error = ConvertError;

        fn try_into(self) -> Result<fflonk_verifier::VerificationKey, Self::Error> {
            Ok(fflonk_verifier::VerificationKey {
                power: self.power,
                k1: self.k1.into(),
                k2: self.k2.into(),
                w: self.w.into(),
                w3: self.w3.into(),
                w4: self.w4.into(),
                w8: self.w8.into(),
                wr: self.wr.into(),
                x2: self.x2.try_into()?,
                c0: self.c0.try_into()?,
            })
        }
    }

    pub fn verify_proof<T: Config>(
        vk: &fflonk_verifier::VerificationKey,
        full_proof: Proof,
    ) -> Result<(), Error<T>> {
        let pubs: fflonk_verifier::Public = (&full_proof[PROOF_SIZE..])
            .try_into()
            .map_err(|e| log::error!("Cannot extract public inputs: {:?}", e))
            .map_err(|_| Error::<T>::InvalidInput)?;
        let raw_proof = <[u8; PROOF_SIZE]>::try_from(&full_proof[..PROOF_SIZE])
            .map_err(|e| log::error!("Cannot get raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidProofData)?;
        let proof = fflonk_verifier::Proof::try_from(&raw_proof)
            .map_err(|e| log::debug!("Cannot extract raw proof data: {:?}", e))
            .map_err(|_| Error::<T>::InvalidProofData)?;
        log::trace!(
            "Extracted public inputs [{:?}...{:?}] and proof data [{:?}...{:?}]",
            &full_proof[PROOF_SIZE],
            &full_proof[FULL_PROOF_SIZE - 1],
            &full_proof[0],
            &full_proof[PROOF_SIZE - 1]
        );

        fflonk_verifier::verify(vk, &proof, &pubs)
            .map(|_x| T::OnProofVerified::on_proof_verified(compute_fflonk_hash(full_proof)))
            .map_err(|e| log::debug!("Cannot verify proof: {:?}", e))
            .map_err(|_| Error::<T>::VerifyError)
    }

    const PREFIX: &[u8; 7] = b"fflonk-";
    fn compute_fflonk_hash(full_proof: Proof) -> H256 {
        let mut data_to_hash = PREFIX.to_vec();
        data_to_hash.extend_from_slice(&full_proof[PROOF_SIZE..]);
        H256(keccak_256(data_to_hash.as_slice()))
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Provided data has not valid public inputs.
        InvalidInput,
        /// Provided data has not valid proof.
        InvalidProofData,
        /// Verify proof failed.
        VerifyError,
        /// Provided an invalid verification key.
        InvalidVerificationKey,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call(weight(<T as Config>::WeightInfo))]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        pub fn submit_proof(
            _origin: OriginFor<T>,
            raw_proof: Box<Proof>,
            vk: Option<Vk>,
        ) -> DispatchResultWithPostInfo {
            log::trace!("Submitting proof");
            let vk: fflonk_verifier::VerificationKey = match vk {
                Some(vk) => vk
                    .try_into()
                    .map_err(|_| Error::<T>::InvalidVerificationKey)?,
                None => fflonk_verifier::VerificationKey::default(),
            };
            verify_proof::<T>(&vk, *raw_proof)
                .map(Into::into)
                .map_err(Into::into)
        }

        // #[pallet::call_index(0)]
        // pub fn submit_proof_fid7(
        //     _origin: OriginFor<T>,
        //     raw_proof: Box<Proof>,
        // ) -> DispatchResultWithPostInfo {
        //     log::trace!("Submitting proof");
        //     verify_proof::<T>(*raw_proof)
        //         .map(Into::into)
        //         .map_err(Into::into)
        // }
    }

    #[test]
    fn fflonk_hash_as_expected() {
        let hash = compute_fflonk_hash(crate::tests::VALID_PROOF);

        assert_eq!(hash, H256(crate::tests::VALID_HASH));
    }
}
