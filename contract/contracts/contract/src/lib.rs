#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Map
};

#[contract]
pub struct FaucetContract;

#[contractimpl]
impl FaucetContract {

    // 🎁 Claim reward (chỉ 1 lần)
    pub fn claim(env: Env, user: Address) {
        user.require_auth();

        let key_claimed = symbol_short!("CLAIMED");
        let key_balance = symbol_short!("BAL");

        let mut claimed: Map<Address, bool> =
            env.storage().instance().get(&key_claimed).unwrap_or(Map::new(&env));

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&key_balance).unwrap_or(Map::new(&env));

        // ❌ nếu đã claim rồi
        if claimed.get(user.clone()).unwrap_or(false) {
            panic!("Already claimed");
        }

        // 🎁 reward fixed
        let reward: i128 = 100;

        let current = balances.get(user.clone()).unwrap_or(0);

        balances.set(user.clone(), current + reward);
        claimed.set(user.clone(), true);

        env.storage().instance().set(&key_balance, &balances);
        env.storage().instance().set(&key_claimed, &claimed);
    }

    // 📊 Check balance
    pub fn get_balance(env: Env, user: Address) -> i128 {
        let key_balance = symbol_short!("BAL");

        let balances: Map<Address, i128> =
            env.storage().instance().get(&key_balance).unwrap_or(Map::new(&env));

        balances.get(user).unwrap_or(0)
    }

    // 🔍 Check claimed
    pub fn has_claimed(env: Env, user: Address) -> bool {
        let key_claimed = symbol_short!("CLAIMED");

        let claimed: Map<Address, bool> =
            env.storage().instance().get(&key_claimed).unwrap_or(Map::new(&env));

        claimed.get(user).unwrap_or(false)
    }
}