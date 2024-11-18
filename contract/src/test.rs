#![cfg(test)]
extern crate std;

use crate::types::BALANCE_BUMP_AMOUNT;
use crate::{MuggleSwap, MuggleSwapClient};
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::{
    testutils::{Address as _}, Address, Env
};

pub(crate) const DEF_FEE_RATE: u32 = 30; // default fee_rate is 0.3%
pub(crate) const TOKEN_DECIMALS: u32 = 4;

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (Address, TokenClient<'a>, StellarAssetClient<'a>) {
    let contract_address = e
        .register_stellar_asset_contract_v2(admin.clone())
        .address();
    (
        contract_address.clone(),
        TokenClient::new(e, &contract_address),
        StellarAssetClient::new(e, &contract_address),
    )
}

fn create_token_swap_contract<'a>(e: &Env) -> MuggleSwapClient<'a> {
    let contract_id = e.register(MuggleSwap, {});
    let contract = MuggleSwapClient::new(&e, &contract_id);

    contract
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let token_admin = Address::generate(&e);
    let sender = Address::generate(&e);
    let receiver = Address::generate(&e);
    const MUL_VAL: u64 = u64::pow(10, TOKEN_DECIMALS);

    // create contract
    let token_swap = create_token_swap_contract(&e);

    token_swap.initialization(&token_admin.clone());

    // Input Token
    let (send_token_id, send_token_client, send_token_admin_client) =
        create_token_contract(&e, &token_admin);
    send_token_admin_client.mint(&sender.clone(), &(1000_i128 * MUL_VAL as i128));

    // Output Token
    let (recv_token_id, recv_token_client, recv_token_admin_client) =
        create_token_contract(&e, &token_admin);
    recv_token_admin_client.mint(&receiver.clone(), &(100_i128 * MUL_VAL as i128));

    // Setup fee
    let fee_rate = DEF_FEE_RATE;
    let fee_wallet = Address::generate(&e);

    token_swap.set_fee(&fee_rate, &fee_wallet);

    send_token_client.approve(
        &sender.clone(),
        &token_swap.address.clone(),
        &((1000 * MUL_VAL) as i128),
        &(e.ledger().sequence() + BALANCE_BUMP_AMOUNT),
    );
    recv_token_client.approve(
        &receiver.clone(),
        &token_swap.address.clone(),
        &((1000 * MUL_VAL) as i128),
        &(e.ledger().sequence() + BALANCE_BUMP_AMOUNT),
    );

    // Initial transaction - create order
    // 500 send_tokens : 50 recv_tokens (10 min_recv_tokens)
    let order_id: u32 = token_swap.create_order(
        &sender,
        &send_token_id,
        &recv_token_id,
        &(500 * MUL_VAL),
        &(50 * MUL_VAL),
        &(10 * MUL_VAL),
    );

    // Verify that authorization is required for the sender.
    /*assert_eq!(
        e.auths(),
        std::vec![(
            sender.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token_swap.address.clone(),
                    Symbol::new(&e, "create"),
                    (
                        sender.clone(),
                        &send_token_id,
                        &recv_token_id,
                        timestamp,
                        500 * MUL_VAL,
                        50 * MUL_VAL,
                        10 * MUL_VAL
                    )
                        .into_val(&e)
                )),
                sub_invocations: std::vec![
                    AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            send_token_id.clone(),
                            symbol_short!("transfer"),
                            (
                                sender.clone(),
                                token_swap.address.clone(),
                                (500 * MUL_VAL) as i128,
                            )
                                .into_val(&e)
                        )),
                        sub_invocations: std::vec![]
                    },
                    AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            send_token_id.clone(),
                            symbol_short!("transfer"),
                            (sender.clone(), fee_wallet.clone(), 12500_i128,).into_val(&e)
                        )),
                        sub_invocations: std::vec![],
                    }
                ]
            }
        )]
    );*/

    // trying to create an offer with same params - fails due to insufficient balance
    let _ = token_swap.try_create_order(
        &sender,
        &send_token_id,
        &recv_token_id,
        &(500 * MUL_VAL),
        &(50 * MUL_VAL),
        &(10 * MUL_VAL),
    );

    // Try accepting 9 recv_token for at least 10 recv_token - that wouldn't
    // succeed because minimum recv amount is 10 recv_token.
    assert!(token_swap
        .try_accept_order(&receiver, &order_id, &(9 * MUL_VAL))
        .is_err());

    // acceptor accepts 10 recv_tokens.
    token_swap.accept_order(&receiver, &order_id, &(10 * MUL_VAL));

    // 0.3% fee * amount = 15000
    assert_eq!(
        send_token_client.balance(&sender),
        (500 * MUL_VAL) as i128 - 15000
    );
    assert_eq!(
        send_token_client.balance(&token_swap.address),
        (400 * MUL_VAL) as i128
    );
    assert_eq!(
        send_token_client.balance(&receiver),
        (100 * MUL_VAL) as i128
    );
    assert_eq!(send_token_client.balance(&fee_wallet), 15000);

    assert_eq!(recv_token_client.balance(&sender), (10 * MUL_VAL) as i128);
    assert_eq!(recv_token_client.balance(&token_swap.address), 0);
    assert_eq!(
        recv_token_client.balance(&receiver),
        (90 * MUL_VAL) as i128 - 300
    );
    assert_eq!(recv_token_client.balance(&fee_wallet), 300);
}
