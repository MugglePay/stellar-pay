//! This contract implements swap of one token pair between one offeror and
//! multiple acceptors.
//! It demonstrates one of the ways of how swap might be implemented.
#![no_std]

use crate::admin::{has_admin, read_admin, write_admin};
use crate::fee::{fee_get, fee_set};
use crate::order::{accept_order, create_order};
use crate::types::{
    FeeInfo, BALANCE_BUMP_AMOUNT, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD,
};
use soroban_sdk::{contract, contractimpl, token, Address, Env};

mod admin;
mod allow;
mod fee;
mod order;
mod test;
mod types;

#[contract]
pub struct MuggleSwap;

#[contractimpl]
impl MuggleSwap {
    pub fn initialization(env: Env, admin: Address) {
        if has_admin(&env) {
            panic!("Already Initialized")
        }

        write_admin(&env, &admin)
    }

    /// Admin Section
    pub fn set_admin(env: Env, new_admin: Address) {
        let admin = read_admin(&env);
        admin.require_auth();

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_admin(&env, &new_admin);
    }

    pub fn set_fee(e: Env, fee_rate: u32, fee_wallet: Address) {
        let fee_info: FeeInfo = FeeInfo {
            fee_rate,
            fee_wallet,
        };
        fee_set(&e, &fee_info);
    }

    pub fn get_fee(e: Env) -> (u32, Address) {
        let fee_info: FeeInfo = fee_get(&e);
        (fee_info.fee_rate, fee_info.fee_wallet)
    }

    // Order Section
    pub fn create_order(
        e: Env,
        sender: Address,
        send_token: Address,
        recv_token: Address,
        send_amount: u64,
        recv_amount: u64,
        min_recv_amount: u64,
    ) -> u32 {
        let ret: u32 = create_order(
            &e,
            &sender,
            &send_token,
            &recv_token,
            send_amount,
            recv_amount,
            min_recv_amount,
        );

        ret
    }

    pub fn accept_order(e: Env, receiver: Address, order_id: u32, amount: u64) -> u32 {
        let ret: u32 = accept_order(&e, &receiver, order_id, amount);

        ret
    }

    // Mint tokens to a specified address
    pub fn mint(env: Env, to: Address, amount: i128) {
        // Add minting logic
        let mut balance = env.storage().persistent().get(&to).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&to, &balance);
    }

    // Approve token spending
    pub fn approve(env: Env, token: Address, from: Address, spender: Address, amount: i128) {
        from.require_auth();

        // Create token client
        let token_client = token::Client::new(&env, &token);

        token_client.approve(
            &from.clone(),
            &spender.clone(),
            &amount,
            &(env.ledger().sequence() + BALANCE_BUMP_AMOUNT),
        );
    }
}
