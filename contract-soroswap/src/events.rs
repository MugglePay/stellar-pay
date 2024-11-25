use soroban_sdk::{Address, Env};

pub struct Events;

impl Events {
    pub fn swap_completed(
        env: &Env,
        customer: &Address,
        merchant: &Address,
        amount_in: i128,
        amount_out: i128,
    ) {
        env.events().publish(
            ("swap", customer, merchant),
            (amount_in, amount_out),
        );
    }

    pub fn contract_initialized(
        env: &Env,
        router: &Address,
    ) {
        env.events().publish(
            ("contract_initialized", router),
            (),
        );
    }
}
