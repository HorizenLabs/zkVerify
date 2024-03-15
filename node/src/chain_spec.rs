use nh_runtime::{AccountId, RuntimeGenesisConfig, SessionKeys, Signature, WASM_BINARY};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

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

fn session_keys(aura: AuraId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
    SessionKeys {
        aura,
        grandpa,
        im_online,
    }
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AuraId, GrandpaId, ImOnlineId) {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<ImOnlineId>(s),
    )
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
    .with_genesis_config_patch(testnet_genesis(
        // Initial PoA authorities
        vec![authority_keys_from_seed("Alice")],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        ],
        true,
    ))
    .build())
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Local Testnet")
    .with_id("local_testnet")
    .with_chain_type(ChainType::Local)
    .with_properties({
        let mut props = Properties::new();
        props.insert("tokenSymbol".into(), "nZEN".into());
        props.insert("tokenDecimals".into(), 18.into());
        props
    })
    .with_genesis_config_patch(testnet_genesis(
        // Initial PoA authorities
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        true,
    ))
    .build())
}

const INIT_BALANCE: u128 = 1u128 << 60;
const INIT_STASH: u128 = 1u128 << 59; // Use half the balance as stake

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AuraId, GrandpaId, ImOnlineId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            // Configure endowed accounts with initial balance of 1 << 60.
            "balances": endowed_accounts.iter().cloned().map(|k| (k, INIT_BALANCE)).collect::<Vec<_>>(),
        },
        "session": {
            "keys": initial_authorities.iter().map(|x| { (x.0.clone(), x.0.clone(), session_keys(x.1.clone(), x.2.clone(), x.3.clone())) }).collect::<Vec<_>>(),
        },
        "staking": {
            "minimumValidatorCount": 2,
            "validatorCount": 3,
            "stakers": initial_authorities.iter().map(|x| (x.1.clone(), x.1.clone(), INIT_STASH, sp_staking::StakerStatus::Validator::<AccountId>)).collect::<Vec<_>>(),
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

    // The following test verifies whether we added aura configuration in the genesis block
    // by checking that the json returned by testnet_genesis() contains the field "aura"
    #[test]
    fn testnet_genesis_should_set_session_keys() {
        let initial_authorities = vec![authority_keys_from_seed("Alice")];
        let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

        let ret_val: serde_json::Value =
            testnet_genesis(initial_authorities, root_key, vec![], false);

        let session_config = &ret_val["session"];

        // Check that we have the field "aura" in the genesis config
        assert!(!session_config.is_null());

        let auth_len = session_config
            .as_object()
            .map(|inner| inner["keys"].as_array().unwrap().len())
            .unwrap();
        // Check that we have one "keys" set
        assert_eq!(1, auth_len);
    }
}
