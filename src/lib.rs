#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct LuminousContract;

#[contractimpl]
impl LuminousContract {
    pub fn greet() -> Symbol {
        symbol_short!("hello")
    }

    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&symbol_short!("count")).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&symbol_short!("count"), &count);
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_greet() {
        let env = Env::default();
        let contract_id = env.register_contract(None, LuminousContract);
        let client = LuminousContractClient::new(&env, &contract_id);
        assert_eq!(client.greet(), symbol_short!("hello"));
    }

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, LuminousContract);
        let client = LuminousContractClient::new(&env, &contract_id);
        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
    }
}
