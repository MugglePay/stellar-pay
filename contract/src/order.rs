use soroban_sdk::{log, symbol_short, token, Address, Env, Symbol};
use soroban_sdk::unwrap::UnwrapOptimized;

const ORDER: Symbol = symbol_short!("ORDER");

use crate::fee::{calculate_fee, fee_check, fee_get};
use crate::types::{
    OrderInfo, OrderStatus, StorageKey, BALANCE_BUMP_AMOUNT, INSTANCE_BUMP_AMOUNT,
    INSTANCE_LIFETIME_THRESHOLD,
};

/*
How this contract should be used:

1. Call `create_order` once to create an order and register its sender.
2. sender transfers send_amount of the `send_token` to the
   contract address for swap. He may also update the recv_amount and/or min_recv_amount.
3. Receiver may call `accept` to accept the order. The contract will
   immediately perform the swap and send the respective amounts of `recv_token`
   and `send_token` to the sender and receiver respectively.
4. Sender may call `close_order` to claim any remaining `send_token` balance.
*/

pub fn order_error(e: &Env) -> u32 {
    if !e.storage().instance().has(&StorageKey::ErrorCode) {
        return 1000;
    }

    let err_code: u32 = e
        .storage()
        .instance()
        .get(&StorageKey::ErrorCode)
        .unwrap_or(0);
    err_code

    // 1001
}

pub fn order_count(e: &Env) -> u32 {
    let count: u32 = e
        .storage()
        .instance()
        .get(&StorageKey::OrderCount)
        .unwrap_or(0);
    count
}

// Creates the order for sender for the given token pair and initial amounts.
// See comment above the `Order` struct for information on swap.
pub fn create_order(
    env: &Env,
    sender: &Address,
    send_token: &Address,
    recv_token: &Address,
    send_amount: u64,
    recv_amount: u64,
    min_recv_amount: u64,
) -> u32 {
    if !fee_check(env) {
        // panic!("fee wasn't set");
        return 101;
    }

    let order_count: u32 = env
        .storage()
        .instance()
        .get(&StorageKey::OrderCount)
        .unwrap_or(0);
    let order_id: u32 = order_count;
    log!(env, "order_id = {}", order_id);

    if send_amount == 0 || recv_amount == 0 {
        // panic!("zero amount is not allowed");
        return 104;
    }

    if min_recv_amount > recv_amount {
        // panic!("minimum receive amount can't be greater than receive amount");
        return 105;
    }

    // Authorization for create order to verify their identity
    sender.require_auth();

    let fee = fee_get(env);
    let fee_amount = calculate_fee(env, &fee.clone(), send_amount);
    let transfer_amount = send_amount + fee_amount;

    let contract = env.current_contract_address();
    let send_token_client = token::Client::new(env, &send_token.clone());

    if send_token_client.balance(&sender) < (transfer_amount as i128) {
        // panic!("insufficient balance");
        return 106;
    }

    if send_token_client.allowance(&sender, &contract) < (transfer_amount as i128) {
        // panic!(e, "insufficient creator's allowance");
        send_token_client.approve(
            &sender,
            &contract,
            &(transfer_amount as i128),
            &(env.ledger().sequence() + BALANCE_BUMP_AMOUNT),
        );
        // return 107;
    }

    send_token_client.transfer(&sender, &contract, &(send_amount as i128));
    send_token_client.transfer(&sender, &fee.fee_wallet, &(fee_amount as i128));

    write_order(
        &env,
        order_id,
        &OrderInfo {
            sender: sender.clone(),
            send_token: send_token.clone(),
            recv_token: recv_token.clone(),
            send_amount,
            recv_amount,
            min_output_amount: min_recv_amount,
            status: OrderStatus::ACTIVE,
        },
    );

    let new_order_count: u32 = order_count + 1;
    env.storage()
        .instance()
        .set(&StorageKey::OrderCount, &new_order_count);
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

    order_id
}

// Swaps `amount` of recv_token from sender for `send_token` amount calculated by the amount.
// receiver needs to authorize the `swap` call and internal `transfer` call to the contract address.

pub fn accept_order(env: &Env, receiver: &Address, order_id: u32, amount: u64) -> u32 {
    if !env
        .storage()
        .instance()
        .has(&StorageKey::RegOrders(order_id))
    {
        // panic!("can't find order");
        return 110;
    }

    let mut order = load_order(env, order_id);

    if !fee_check(env) {
        // panic!("fee isn't set");
        return 111;
    }

    if order.status != OrderStatus::ACTIVE {
        // panic!("order not available");
        return 112;
    }

    if order.recv_amount < amount {
        panic!("amount is greater than max_recv_amount");
        // return 113;
    }

    if amount < order.min_output_amount {
        panic!("amount must be more than min_output_amount");
        // return 114;
    }

    receiver.require_auth();

    let send_token_client = token::Client::new(env, &order.send_token);
    let recv_token_client = token::Client::new(env, &order.recv_token);

    let fee = fee_get(env);
    let fee_amount = calculate_fee(env, &fee.clone(), amount);
    let contract = env.current_contract_address();

    if recv_token_client.balance(&receiver) < (amount + fee_amount) as i128 {
        // panic!("insufficient balance");
        return 115;
    }

    if recv_token_client.allowance(&receiver, &contract.clone()) < (amount + fee_amount) as i128 {
        // panic!("insufficient allowance");
        recv_token_client.approve(
            &receiver,
            &contract,
            &((amount + fee_amount) as i128),
            &(env.ledger().sequence() + BALANCE_BUMP_AMOUNT),
        );
        // return 116;
    }

    // Compute the amount of send_token that acceptor can receive.
    let prop_send_amount = amount.checked_mul(order.send_amount).unwrap_optimized() / order.recv_amount;

    // Perform the trade in 3 `transfer` steps.
    // Note, that we don't need to verify any balances - the contract would
    // just trap and roll back in case if any of the transfers fails for
    // any reason, including insufficient balance.

    // Transfer the `recv_token` from receiver to this contract.
    // This `transfer` call should be authorized by receiver.
    // This could as well be a direct transfer to the sender, but sending to
    // the contract address allows building more transparent signature
    // payload where the receiver doesn't need to worry about sending token to
    // some 'unknown' third party.
    recv_token_client.transfer(&receiver, &fee.fee_wallet, &(fee_amount as i128));
    // Transfer the `recv_token` to the offeror immediately.
    recv_token_client.transfer(&receiver, &order.sender, &(amount as i128));
    // Transfer the `send_token` from contract to acceptor.
    send_token_client.transfer(&contract, &receiver, &(prop_send_amount as i128));

    // Update Oredr
    order.send_amount -= prop_send_amount;
    order.recv_amount -= amount;

    if order.recv_amount == 0 {
        order.status = OrderStatus::COMPLETE;
    } else if order.recv_amount < order.min_output_amount  {
        order.min_output_amount = order.recv_amount;
    }

    write_order(env, order_id, &order);

    0
}

/// Utilities
// Check balances
pub fn order_balances(
    e: &Env,
    account: &Address,
    send_token: &Address,
    recv_token: &Address,
) -> (u64, u64) {
    let send_token_client = token::Client::new(e, send_token);
    let recv_token_client = token::Client::new(e, recv_token);

    (
        send_token_client.balance(account) as u64,
        recv_token_client.balance(account) as u64,
    )
}

pub fn load_order(e: &Env, key: u32) -> OrderInfo {
    e.storage()
        .instance()
        .get(&StorageKey::RegOrders(key))
        .unwrap()
}

fn write_order(e: &Env, key: u32, order: &OrderInfo) {
    e.storage()
        .instance()
        .set(&StorageKey::RegOrders(key), order);
}