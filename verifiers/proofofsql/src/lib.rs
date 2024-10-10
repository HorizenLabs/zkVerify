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

use codec::{Decode, Encode, MaxEncodedLen};
use derivative::Derivative;
use errors::ErrorWrapper;
use frame_support::weights::Weight;
use hp_verifiers::{Cow, Verifier, VerifyError};
use proof_of_sql_verifier::VerificationKey;
use scale_info::TypeInfo;
use sp_core::Get;
use sp_std::{marker::PhantomData, vec::Vec};

pub mod benchmarking;
mod errors;
mod verifier_should;
mod weight;
pub use weight::WeightInfo;

#[derive(Derivative, Encode, Decode, TypeInfo)]
#[derivative(Clone, Debug, PartialEq)]
#[scale_info(skip_type_params(T))]
pub struct Vk<T>(Vec<u8>, PhantomData<T>);

impl<T: Config> Vk<T> {
    pub fn validate_size(&self) -> Result<(), VerifyError> {
        let max_nu = T::largest_max_nu();
        let max_vk_size = VerificationKey::serialized_size(max_nu as usize);
        if self.0.len() > max_vk_size {
            Err(VerifyError::InvalidVerificationKey)
        } else {
            Ok(())
        }
    }
}

impl<T> From<Vec<u8>> for Vk<T> {
    fn from(value: Vec<u8>) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: Config> MaxEncodedLen for Vk<T> {
    fn max_encoded_len() -> usize {
        let max_nu = T::largest_max_nu();
        let len = VerificationKey::serialized_size(max_nu as usize);
        codec::Compact(len as u32).encoded_size() + len
    }
}

pub trait Config: 'static {
    /// Maximum value allowed for `max_nu` in the verification key
    type LargestMaxNu: Get<u32>;
}

trait LargestMaxNu {
    fn largest_max_nu() -> u32;
}

impl<T: Config> LargestMaxNu for T {
    fn largest_max_nu() -> u32 {
        <Self as Config>::LargestMaxNu::get()
    }
}

#[pallet_verifiers::verifier]
pub struct ProofOfSql<T>;

impl<T: Config> Verifier for ProofOfSql<T> {
    type Proof = Vec<u8>;

    type Pubs = Vec<u8>;

    type Vk = Vk<T>;

    fn hash_context_data() -> &'static [u8] {
        b"proofofsql-v0.28"
    }

    fn verify_proof(
        vk: &Self::Vk,
        proof: &Self::Proof,
        pubs: &Self::Pubs,
    ) -> Result<(), VerifyError> {
        vk.validate_size()?;
        let proof = proof_of_sql_verifier::Proof::try_from(&proof[..])
            .map_err(Into::<ErrorWrapper>::into)?;
        let pubs = proof_of_sql_verifier::PublicInput::try_from(&pubs[..])
            .map_err(Into::<ErrorWrapper>::into)?;
        let vk = proof_of_sql_verifier::VerificationKey::try_from(&vk.0[..])
            .map_err(Into::<ErrorWrapper>::into)?;
        proof_of_sql_verifier::verify_proof(&proof, &pubs, &vk)
            .map_err(Into::<ErrorWrapper>::into)?;
        Ok(())
    }

    fn validate_vk(vk: &Self::Vk) -> Result<(), VerifyError> {
        vk.validate_size()
            .inspect_err(|_| log::debug!("Verification key is too big"))?;
        VerificationKey::try_from(vk.0.as_slice())
            .inspect_err(|e| log::debug!("Cannot parse verification key: {:?}", e))
            .map_err(|_| VerifyError::InvalidVerificationKey)?;
        Ok(())
    }

    fn pubs_bytes(pubs: &Self::Pubs) -> Cow<[u8]> {
        let data = pubs.to_vec();
        Cow::Owned(data)
    }
}

pub struct ProofOfSqlWeight<W: weight::WeightInfo>(PhantomData<W>);

impl<T: Config, W: weight::WeightInfo> pallet_verifiers::WeightInfo<ProofOfSql<T>>
    for ProofOfSqlWeight<W>
{
    fn submit_proof(
        _proof: &<ProofOfSql<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<ProofOfSql<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof()
    }

    fn submit_proof_with_vk_hash(
        _proof: &<ProofOfSql<T> as hp_verifiers::Verifier>::Proof,
        _pubs: &<ProofOfSql<T> as hp_verifiers::Verifier>::Pubs,
    ) -> Weight {
        W::submit_proof_with_vk_hash()
    }

    fn register_vk(_vk: &<ProofOfSql<T> as hp_verifiers::Verifier>::Vk) -> Weight {
        W::register_vk()
    }
}

#[cfg(test)]
mod vk {
    use super::*;

    use frame_support::{assert_err, assert_ok};
    use sp_core::ConstU32;

    pub struct ConfigWithMaxNuEqualTo5;

    impl Config for ConfigWithMaxNuEqualTo5 {
        type LargestMaxNu = ConstU32<5>;
    }

    pub struct ConfigWithMaxNuEqualTo4;

    impl Config for ConfigWithMaxNuEqualTo4 {
        type LargestMaxNu = ConstU32<4>;
    }

    pub struct ConfigWithMaxNuEqualTo3;

    impl Config for ConfigWithMaxNuEqualTo3 {
        type LargestMaxNu = ConstU32<3>;
    }

    #[test]
    fn length_should_match_max_encoded_len() {
        let vk_with_nu_equal_to_4: <ProofOfSql<ConfigWithMaxNuEqualTo4> as Verifier>::Vk =
            include_bytes!("resources/VALID_VK.bin").to_vec().into();
        let encoded_len = vk_with_nu_equal_to_4.encode().len();
        let expected_encoded_len =
            <ProofOfSql<ConfigWithMaxNuEqualTo4> as Verifier>::Vk::max_encoded_len();
        assert_eq!(encoded_len, expected_encoded_len);
    }

    mod max_encoded_len {
        use super::*;

        #[test]
        fn should_reject_too_big_vk() {
            let vk_with_nu_equal_to_4: <ProofOfSql<ConfigWithMaxNuEqualTo3> as Verifier>::Vk =
                include_bytes!("resources/VALID_VK.bin").to_vec().into();
            assert_err!(
                vk_with_nu_equal_to_4.validate_size(),
                VerifyError::InvalidVerificationKey
            );
        }

        #[test]
        fn should_accept_maximum_size_vk() {
            let vk_with_nu_equal_to_4: <ProofOfSql<ConfigWithMaxNuEqualTo4> as Verifier>::Vk =
                include_bytes!("resources/VALID_VK.bin").to_vec().into();
            assert_ok!(vk_with_nu_equal_to_4.validate_size());
        }

        #[test]
        fn should_accept_small_vk() {
            let vk_with_nu_equal_to_4: <ProofOfSql<ConfigWithMaxNuEqualTo5> as Verifier>::Vk =
                include_bytes!("resources/VALID_VK.bin").to_vec().into();
            assert_ok!(vk_with_nu_equal_to_4.validate_size());
        }
    }
}
