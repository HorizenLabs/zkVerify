// Existential deposit used in pallet_balances
pub const EXISTENTIAL_DEPOSIT: u128 = 500;
pub const EXISTENTIAL_DEPOSIT_REMINDER: u128 = 1;

pub struct SampleAccount {
    pub raw_account: [u8; 32],
    pub starting_balance: u128,
}

// Build a vector containing a few sample user accounts, along with their starting balances
pub fn get_sample_users() -> Vec<SampleAccount> {
    vec![
        SampleAccount {
            raw_account: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            starting_balance: 1000001,
        },
        SampleAccount {
            raw_account: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 2,
            ],
            starting_balance: 12345432,
        },
        SampleAccount {
            raw_account: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 3,
            ],
            starting_balance: 9955223,
        },
        SampleAccount {
            raw_account: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 4,
            ],
            starting_balance: EXISTENTIAL_DEPOSIT,
        },
    ]
}
