#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct LuminousContract;

#[contractimpl]
impl LuminousContract {
    pub fn greet(env: Env, name: String) -> String {
        let greeting = String::from_slice(&env, "Hello, ");
        greeting.concat(&name)
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

        let name = String::from_slice(&env, "Luminous");
        let result = client.greet(&name);
        assert_eq!(result.to_string(), "Hello, Luminous");
    }
}
