use super::*;

use crate::utils::{
    make_msm_args,
    make_pairing_args,
    make_scalar_args,
    make_scalar_args_projective,
    // serialize_argument,
};
#[allow(unused)]
use crate::Pallet as Template;
// use ark_ff::{Fp, MontBackend};
// use ark_serialize::CanonicalDeserialize;
// use ark_std::vec;
use codec::Encode;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

// Min number of elements for multi scalar multiplication
const MSM_LEN_MIN: u32 = 10;
// Max number of elements for multi scalar multiplication
const MSM_LEN_MAX: u32 = 100;

// Scalar min words for single scalar multiplication (1 = 64 bit)
const SCALAR_WORDS_MIN: u32 = 1;
// Scalar max words for single scalar multiplication (16 = 1024 bit)
const SCALAR_WORDS_MAX: u32 = 16;

pub static PROOF_SERIALIZED: &[u8] = &[
    160, 91, 229, 15, 171, 87, 149, 187, 135, 132, 57, 58, 80, 69, 249, 135, 71, 23, 58, 210, 135,
    245, 94, 33, 52, 113, 189, 85, 151, 69, 85, 20, 82, 69, 60, 76, 58, 57, 231, 200, 131, 16, 132,
    159, 60, 122, 31, 195, 173, 99, 72, 182, 183, 179, 76, 134, 191, 55, 167, 72, 205, 45, 130,
    162, 80, 223, 198, 72, 70, 117, 102, 136, 37, 161, 111, 125, 166, 160, 77, 52, 36, 17, 62, 50,
    92, 231, 52, 236, 68, 149, 96, 130, 192, 160, 110, 95, 24, 104, 225, 241, 166, 229, 89, 185,
    254, 129, 241, 169, 1, 248, 166, 52, 27, 48, 28, 69, 178, 93, 48, 128, 251, 197, 3, 147, 83,
    216, 247, 27, 85, 11, 39, 78, 196, 192, 124, 112, 205, 17, 83, 86, 44, 49, 76, 151, 181, 105,
    204, 73, 27, 77, 240, 53, 203, 244, 158, 149, 31, 212, 254, 48, 170, 130, 54, 176, 226, 175,
    104, 244, 193, 89, 44, 212, 13, 235, 235, 113, 138, 243, 54, 57, 219, 107, 193, 226, 218, 157,
    152, 229, 83, 229, 234, 237,
];

pub const VK_SERIALIZED: &[u8] = &[
    183, 29, 177, 250, 95, 65, 54, 46, 147, 2, 91, 53, 86, 215, 110, 173, 18, 37, 207, 89, 13, 28,
    219, 158, 56, 42, 31, 235, 183, 150, 61, 205, 36, 165, 30, 24, 223, 4, 171, 34, 27, 236, 175,
    41, 22, 159, 175, 37, 179, 162, 107, 11, 71, 18, 231, 141, 93, 113, 120, 109, 150, 19, 42, 124,
    88, 80, 35, 163, 102, 50, 202, 218, 68, 23, 26, 195, 244, 93, 181, 36, 195, 246, 87, 12, 138,
    63, 125, 236, 53, 174, 26, 195, 48, 155, 5, 221, 11, 48, 109, 180, 247, 79, 217, 236, 66, 28,
    167, 12, 84, 66, 93, 146, 46, 172, 76, 64, 59, 0, 219, 145, 111, 222, 223, 6, 91, 220, 224, 14,
    206, 23, 185, 122, 78, 151, 23, 62, 77, 89, 137, 129, 142, 223, 170, 76, 181, 172, 184, 0, 205,
    73, 237, 140, 189, 219, 244, 145, 161, 252, 248, 171, 252, 147, 240, 157, 56, 187, 178, 236,
    182, 176, 142, 35, 164, 100, 44, 229, 156, 155, 3, 134, 83, 154, 195, 206, 205, 251, 102, 169,
    240, 39, 252, 33, 15, 37, 149, 16, 117, 100, 68, 188, 94, 239, 101, 79, 77, 6, 18, 181, 214,
    55, 95, 149, 38, 177, 185, 102, 206, 83, 184, 241, 37, 148, 225, 179, 153, 208, 130, 49, 207,
    230, 194, 105, 164, 74, 168, 213, 135, 242, 54, 157, 179, 170, 121, 123, 175, 163, 154, 72,
    246, 248, 124, 36, 131, 200, 148, 194, 129, 200, 7, 130, 28, 71, 48, 31, 251, 117, 90, 207,
    207, 210, 44, 35, 35, 206, 223, 99, 73, 199, 254, 221, 50, 0, 164, 174, 85, 134, 49, 229, 1,
    210, 153, 235, 147, 19, 92, 7, 207, 105, 76, 161, 24, 209, 179, 134, 73, 5, 41, 198, 15, 87,
    147, 92, 239, 168, 159, 202, 250, 19, 168, 63, 132, 32, 123, 118, 254, 7, 141, 200, 89, 212, 2,
    116, 61, 70, 140, 21, 2, 0, 0, 0, 0, 0, 0, 0, 183, 246, 208, 109, 211, 229, 36, 110, 246, 181,
    27, 7, 92, 48, 182, 143, 212, 144, 251, 248, 94, 2, 5, 247, 159, 160, 77, 129, 19, 49, 146, 19,
    148, 99, 181, 232, 239, 178, 44, 57, 239, 61, 209, 197, 9, 32, 21, 184, 162, 230, 55, 219, 255,
    82, 161, 228, 168, 197, 217, 133, 179, 65, 31, 197, 253, 68, 175, 96, 126, 66, 146, 62, 171,
    180, 122, 216, 118, 225, 240, 43, 91, 224, 52, 173, 175, 115, 149, 42, 232, 175, 254, 229, 245,
    24, 65, 222,
];

pub const C_SERIALIZED: &[u8] = &[
    24, 246, 200, 56, 227, 0, 59, 95, 49, 157, 206, 57, 13, 141, 238, 168, 24, 78, 144, 62, 155,
    209, 70, 78, 67, 71, 89, 204, 203, 208, 132, 24,
];

#[benchmarks]
mod benchmarks {
    use super::*;

    // ---------------------------------------------
    // Benchmarks for bn254
    // ---------------------------------------------

    #[benchmark]
    fn bn254_pairing_opt() {
        let (a, b) = make_pairing_args::<native::G1Affine, native::G2Affine>();

        #[extrinsic_call]
        bn254_pairing_opt(RawOrigin::None, a, b);
    }

    #[benchmark]
    fn bn254_msm_g1_opt(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) = make_msm_args::<native::G1Projective>(x);

        #[extrinsic_call]
        bn254_msm_g1_opt(RawOrigin::None, bases.encode(), scalars.encode());
    }

    #[benchmark]
    fn bn254_msm_g2_opt(x: Linear<MSM_LEN_MIN, MSM_LEN_MAX>) {
        let (bases, scalars) = make_msm_args::<native::G2Projective>(x);

        #[extrinsic_call]
        bn254_msm_g2_opt(RawOrigin::None, bases.encode(), scalars.encode());
    }

    #[benchmark]
    fn bn254_mul_projective_g1_opt(x: Linear<SCALAR_WORDS_MIN, SCALAR_WORDS_MAX>) {
        let (base, scalar) = make_scalar_args_projective::<native::G1Projective>(x);

        #[extrinsic_call]
        bn254_mul_projective_g1_opt(RawOrigin::None, base.encode(), scalar.encode());
    }

    #[benchmark]
    fn bn254_mul_affine_g1_opt(x: Linear<SCALAR_WORDS_MIN, SCALAR_WORDS_MAX>) {
        let (base, scalar) = make_scalar_args::<native::G1Affine>(x);

        #[extrinsic_call]
        bn254_mul_affine_g1_opt(RawOrigin::None, base.encode(), scalar.encode());
    }

    #[benchmark]
    fn bn254_mul_projective_g2_opt(x: Linear<SCALAR_WORDS_MIN, SCALAR_WORDS_MAX>) {
        let (base, scalar) = make_scalar_args_projective::<native::G2Projective>(x);

        #[extrinsic_call]
        bn254_mul_projective_g2_opt(RawOrigin::None, base.encode(), scalar.encode());
    }

    #[benchmark]
    fn bn254_mul_affine_g2_opt(x: Linear<SCALAR_WORDS_MIN, SCALAR_WORDS_MAX>) {
        let (base, scalar) = make_scalar_args::<native::G2Affine>(x);

        #[extrinsic_call]
        bn254_mul_affine_g2_opt(RawOrigin::None, base.encode(), scalar.encode());
    }
}
