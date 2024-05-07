// Copyright 2024, The Horizen Foundation

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

use nh_runtime::currency::{Balance, NZEN};
use nh_runtime::{currency, AccountId, RuntimeGenesisConfig, SessionKeys, Signature, WASM_BINARY};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::{ChainType, Properties};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

const ENDOWMENT: Balance = 1_000_000 * NZEN;
const STASH_BOND: Balance = ENDOWMENT / 100;
const DEFAULT_ENDOWED_SEEDS: [&str; 6] = ["Alice", "Bob", "Charlie", "Dave", "Eve", "Ferdie"];
const LOCAL_N_AUTH: usize = 2;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn from_ss58check<T: sp_core::crypto::Ss58Codec>(
    key: &str,
) -> Result<T, sp_core::crypto::PublicError> {
    <T as sp_core::crypto::Ss58Codec>::from_ss58check(key)
}

fn session_keys(babe: BabeId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
    SessionKeys {
        babe,
        grandpa,
        im_online,
    }
}

/// Generate a session authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, BabeId, GrandpaId, ImOnlineId) {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<BabeId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<ImOnlineId>(s),
    )
}

// Generate authority IDs from SS58 addresses.
pub fn authority_ids_from_ss58(
    sr25519_key: &str,
    ed25519_key: &str,
) -> Result<(AccountId, BabeId, GrandpaId, ImOnlineId), String> {
    Ok((
        from_ss58check(sr25519_key).map_err(|error| {
            format!(
                "An error occurred while converting SS58 to AccountId: {}",
                error
            )
        })?,
        from_ss58check(sr25519_key).map_err(|error| {
            format!(
                "An error occurred while converting SS58 to BabeId: {}",
                error
            )
        })?,
        from_ss58check(ed25519_key).map_err(|error| {
            format!(
                "An error occurred while converting SS58 to GrandpaId: {}",
                error
            )
        })?,
        from_ss58check(sr25519_key).map_err(|error| {
            format!(
                "An error occurred while converting SS58 to ImOnlineId: {}",
                error
            )
        })?,
    ))
}

pub fn development_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_properties({
        let mut props = Properties::new();
        props.insert("tokenSymbol".into(), "nZEN".into());
        props.insert("tokenDecimals".into(), 18.into());
        props
    })
    .with_genesis_config_patch(genesis(
        // Initial PoA authorities
        DEFAULT_ENDOWED_SEEDS
            .into_iter()
            .map(|seed| (authority_keys_from_seed(seed), STASH_BOND))
            .take(1)
            .collect::<Vec<_>>(),
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>(DEFAULT_ENDOWED_SEEDS[0]),
        // Pre-funded accounts
        DEFAULT_ENDOWED_SEEDS
            .into_iter()
            .map(|seed| (get_account_id_from_seed::<sr25519::Public>(seed), ENDOWMENT))
            .take(2)
            .collect::<Vec<_>>(),
        true,
    ))
    .build())
}

pub fn local_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("NH Local")
    .with_id("nh_local")
    .with_protocol_id("lzen")
    .with_chain_type(ChainType::Local)
    .with_properties({
        let mut props = Properties::new();
        props.insert("tokenSymbol".into(), "nZEN".into());
        props.insert("tokenDecimals".into(), 18.into());
        props
    })
    .with_genesis_config_patch(genesis(
        // Initial PoA authorities
        DEFAULT_ENDOWED_SEEDS
            .into_iter()
            .map(|seed| (authority_keys_from_seed(seed), STASH_BOND))
            .take(LOCAL_N_AUTH)
            .collect::<Vec<_>>(),
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>(DEFAULT_ENDOWED_SEEDS[0]),
        // Pre-funded accounts
        DEFAULT_ENDOWED_SEEDS
            .into_iter()
            .map(|seed| (get_account_id_from_seed::<sr25519::Public>(seed), ENDOWMENT))
            .collect::<Vec<_>>(),
        true,
    ))
    .build())
}

pub fn testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("NH Testnet")
    .with_id("nh_testnet")
    .with_protocol_id("tzen")
    .with_chain_type(ChainType::Live)
    .with_properties({
        let mut props = Properties::new();
        props.insert("tokenSymbol".into(), "nZEN".into());
        props.insert("tokenDecimals".into(), 18.into());
        props
    })
    .with_genesis_config_patch(genesis(
        // Initial PoA authorities
        vec![
            // nh-validator-t1
            (
                authority_ids_from_ss58(
                    "5DkkLph1sMf13yZ2NJAQQAmW7v3gs7nq1Hq8VBzaz9qsnkTn",
                    "5GLr2vfut1KixwPcqzmqg57zZ3bXgV3EisUCxA8Ws3eCiSwH",
                )?,
                4 * currency::MILLIONS,
            ),
            // nh-validator-t2
            (
                authority_ids_from_ss58(
                    "5FRPTHzWMtLiZPyf2YLnzeesLMscet8BBb4Khz14ftPxwFUj",
                    "5H7fi3cWJkF4Mjm9cMm3umvB2QieZ2qwfZekJCb4LvaQVkN5",
                )?,
                4 * currency::MILLIONS,
            ),
            // nh-validator-t3
            (
                authority_ids_from_ss58(
                    "5F9A9ktpR7pf5d3LQtppANSFfAXXzpGcCzYM5jBwfDBUpWZ6",
                    "5Ep6w32bTWnPUMjyAVwEtrttmmjbM6Yo5vwP6gg79N74tPnw",
                )?,
                2 * currency::MILLIONS,
            ),
        ],
        // Sudo account [nh-sudo-t1]
        from_ss58check("5D9txxK9DTvgCznTjJo7q1cxAgmWa83CzHvcz8zhBtLgaLBV")
            .map_err(|error| error.to_string())?,
        // Initial balances
        vec![
            // nh-validator-t1
            (
                from_ss58check("5DkkLph1sMf13yZ2NJAQQAmW7v3gs7nq1Hq8VBzaz9qsnkTn")
                    .map_err(|error| error.to_string())?,
                4 * currency::MILLIONS + currency::NZEN,
            ),
            // nh-validator-t2
            (
                from_ss58check("5FRPTHzWMtLiZPyf2YLnzeesLMscet8BBb4Khz14ftPxwFUj")
                    .map_err(|error| error.to_string())?,
                4 * currency::MILLIONS + currency::NZEN,
            ),
            // nh-validator-t3
            (
                from_ss58check("5F9A9ktpR7pf5d3LQtppANSFfAXXzpGcCzYM5jBwfDBUpWZ6")
                    .map_err(|error| error.to_string())?,
                2 * currency::MILLIONS + currency::NZEN,
            ),
            // nh-sudo-t1
            (
                from_ss58check("5D9txxK9DTvgCznTjJo7q1cxAgmWa83CzHvcz8zhBtLgaLBV")
                    .map_err(|error| error.to_string())?,
                100 * currency::THOUSANDS,
            ),
            // nh-wallet-custody-t1
            (
                from_ss58check("5GKWyvfHyK2PsbZEyTdh5BiP8rhizDaEs7ph2W9YokNLpwpM")
                    .map_err(|error| error.to_string())?,
                currency::MILLIONS,
            ),
            // nh-wallet-custody-t2
            (
                from_ss58check("5DnUTtZRaAbpYnwrdr7zme6snYeWXfQWJ5Rq253zUjJhd2fv")
                    .map_err(|error| error.to_string())?,
                currency::MILLIONS,
            ),
            // nh-wallet-automated-t1
            (
                from_ss58check("5CkmKQbsvME3TZa6ULc7meNRRfrTNPU4aprMLymvFDMruJ9H")
                    .map_err(|error| error.to_string())?,
                500 * currency::THOUSANDS,
            ),
            // nh-wallet-user-t1
            (
                from_ss58check("5HQRNiMdkVrhtEcrDcYD6K6FATYU13p9RKbN6iyu7kbgRN4u")
                    .map_err(|error| error.to_string())?,
                100 * currency::THOUSANDS,
            ),
            // nh-wallet-faucet-t1
            (
                from_ss58check("5FCFo9uuY5iZmBc4rE7wFeZcKf3gR8KujVVCPnib6H8XDHTM")
                    .map_err(|error| error.to_string())?,
                currency::MILLIONS,
            ),
            // cdk-aggregator-nh-t1
            (
                from_ss58check("5GCM2e4WzGPBy12xNZVc6XF72gund3esdRZAfAtGqiYCd697")
                    .map_err(|error| error.to_string())?,
                currency::MILLIONS,
            ),
        ],
        true,
    ))
    .build())
}

/// Configure initial storage state for FRAME modules.
fn genesis(
    initial_authorities: Vec<((AccountId, BabeId, GrandpaId, ImOnlineId), Balance)>,
    root_key: AccountId,
    endowed_accounts: Vec<(AccountId, Balance)>,
    _enable_println: bool,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            // Configure endowed accounts with initial balance.
            "balances": endowed_accounts,
        },
        "babe": {
            "epochConfig": Some(nh_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        "session": {
            "keys": initial_authorities.iter()
                .cloned()
                .map(|((account, babe, grandpa, imonline), _staking)| { (account.clone(), account, session_keys(babe, grandpa, imonline)) })
                .collect::<Vec<_>>(),
        },
        "staking": {
            "minimumValidatorCount": 2,
            "validatorCount": 3,
            "stakers": initial_authorities.iter()
                .cloned()
                .map(|((account, _babe, _grandpa, _imonline), staking)| (account.clone(), account, staking, sp_staking::StakerStatus::Validator::<AccountId>))
                .collect::<Vec<_>>(),
        },
        "sudo": {
            // Assign network admin rights.
            "key": Some(root_key),
        },
    })
}

// This is a sample unit test
// Following Rust convention, unit tests are appended in the same file as the module they are
// testing. This is acceptable and should not create confusion, as long as the tests have a
// very narrow scope - i.e. for verifying the behavior of a single function of a module.
#[cfg(test)]
mod tests {
    use super::*;

    // The following test verifies whether we added session configuration in the genesis block
    // by checking that the json returned by testnet_genesis() contains the field "session"
    #[test]
    fn testnet_genesis_should_set_session_keys() {
        let initial_authorities = vec![(authority_keys_from_seed("Alice"), 7 * currency::NZEN)];
        let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

        let ret_val: serde_json::Value = genesis(initial_authorities, root_key, vec![], false);

        let session_config = &ret_val["session"];

        // Check that we have the field "session" in the genesis config
        assert!(!session_config.is_null());

        let auth_len = session_config
            .as_object()
            .map(|inner| inner["keys"].as_array().unwrap().len())
            .unwrap();
        // Check that we have one "keys" set
        assert_eq!(1, auth_len);
    }

    // This test checks whether the local testnet genesis configuration is generated correctly
    #[test]
    fn local_testnet_genesis_should_be_valid() {
        assert!(testnet_config().is_ok());
    }
}
