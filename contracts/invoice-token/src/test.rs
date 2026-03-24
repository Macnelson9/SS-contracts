//! Unit tests for the invoice token contract.

use super::{InvoiceToken, InvoiceTokenClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String as SorobanString, Symbol};

fn setup_token(env: &Env) -> (InvoiceTokenClient<'_>, Address, Address) {
    let contract_id = env.register(InvoiceToken, ());
    let client = InvoiceTokenClient::new(&env, &contract_id);
    let admin = Address::generate(env);
    let minter = Address::generate(env);
    let name = SorobanString::from_str(env, "Invoice INV-001");
    let symbol = SorobanString::from_str(env, "INV001");
    let invoice_id = Symbol::new(env, "inv_001");
    client.initialize(&admin, &name, &symbol, &7u32, &invoice_id, &minter);
    (client, admin, minter)
}

#[test]
fn test_initialize_and_metadata() {
    let env = Env::default();
    let (client, admin, _minter) = setup_token(&env);

    assert_eq!(
        client.name(),
        SorobanString::from_str(&env, "Invoice INV-001")
    );
    assert_eq!(client.symbol(), SorobanString::from_str(&env, "INV001"));
    assert_eq!(client.decimals(), 7);
    assert_eq!(client.total_supply(), 0);
    assert_eq!(client.balance(&admin), 0);
    assert_eq!(client.invoice_id(), Symbol::new(&env, "inv_001"));
    assert!(client.transfer_locked());

    let other = Address::generate(&env);
    assert_eq!(client.balance(&other), 0);
    assert_eq!(client.allowance(&admin, &other), 0);
}

#[test]
fn test_transfer_locked_non_admin_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin, minter) = setup_token(&env);

    // Mint tokens to a non-admin user
    let user = Address::generate(&env);
    client.mint(&user, &1000, &minter);
    assert_eq!(client.balance(&user), 1000);

    // Transfer should be locked by default (transfer_locked = true)
    assert!(client.transfer_locked());

    // Non-admin transfer should fail with TransferLocked
    let recipient = Address::generate(&env);
    let result = client.try_transfer(&user, &recipient, &100);
    assert_eq!(result, Err(Ok(crate::errors::Error::TransferLocked)));
}

#[test]
fn test_transfer_locked_admin_succeeds() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, minter) = setup_token(&env);

    // Mint tokens to admin
    client.mint(&admin, &1000, &minter);
    assert_eq!(client.balance(&admin), 1000);

    // Transfer should be locked by default
    assert!(client.transfer_locked());

    // Admin transfer should succeed even when locked
    let recipient = Address::generate(&env);
    client.transfer(&admin, &recipient, &100);
    assert_eq!(client.balance(&admin), 900);
    assert_eq!(client.balance(&recipient), 100);
}

#[test]
fn test_transfer_from_locked_non_admin_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin, minter) = setup_token(&env);

    // Mint tokens to a non-admin user
    let user = Address::generate(&env);
    client.mint(&user, &1000, &minter);

    // User approves spender
    let spender = Address::generate(&env);
    let expiration = env.ledger().sequence() + 100;
    client.approve(&user, &spender, &500, &expiration);

    // Transfer should be locked by default
    assert!(client.transfer_locked());

    // transfer_from should fail when from is non-admin
    let recipient = Address::generate(&env);
    let result = client.try_transfer_from(&spender, &user, &recipient, &100);
    assert_eq!(result, Err(Ok(crate::errors::Error::TransferLocked)));
}

#[test]
fn test_transfer_from_locked_admin_succeeds() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, minter) = setup_token(&env);

    // Mint tokens to admin
    client.mint(&admin, &1000, &minter);

    // Admin approves spender
    let spender = Address::generate(&env);
    let expiration = env.ledger().sequence() + 100;
    client.approve(&admin, &spender, &500, &expiration);

    // Transfer should be locked by default
    assert!(client.transfer_locked());

    // transfer_from should succeed when from is admin
    let recipient = Address::generate(&env);
    client.transfer_from(&spender, &admin, &recipient, &100);
    assert_eq!(client.balance(&admin), 900);
    assert_eq!(client.balance(&recipient), 100);
    assert_eq!(client.allowance(&admin, &spender), 400);
}

#[test]
fn test_transfer_unlocked_all_succeed() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin, minter) = setup_token(&env);

    // Mint tokens to users
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    client.mint(&user1, &1000, &minter);
    client.mint(&user2, &1000, &minter);

    // Unlock transfers
    client.set_transfer_locked(&false);
    assert!(!client.transfer_locked());

    // Non-admin transfer should now succeed
    let recipient = Address::generate(&env);
    client.transfer(&user1, &recipient, &100);
    assert_eq!(client.balance(&user1), 900);
    assert_eq!(client.balance(&recipient), 100);

    // Non-admin transfer_from should also succeed
    let spender = Address::generate(&env);
    let expiration = env.ledger().sequence() + 100;
    client.approve(&user2, &spender, &500, &expiration);

    let recipient2 = Address::generate(&env);
    client.transfer_from(&spender, &user2, &recipient2, &200);
    assert_eq!(client.balance(&user2), 800);
    assert_eq!(client.balance(&recipient2), 200);
    assert_eq!(client.allowance(&user2, &spender), 300);
}

#[test]
fn test_set_transfer_locked_toggle() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin, minter) = setup_token(&env);

    // Mint tokens to a non-admin user
    let user = Address::generate(&env);
    client.mint(&user, &1000, &minter);

    // Initially locked
    assert!(client.transfer_locked());
    let recipient = Address::generate(&env);
    let result = client.try_transfer(&user, &recipient, &100);
    assert_eq!(result, Err(Ok(crate::errors::Error::TransferLocked)));

    // Unlock transfers
    client.set_transfer_locked(&false);
    assert!(!client.transfer_locked());
    client.transfer(&user, &recipient, &100);
    assert_eq!(client.balance(&user), 900);
    assert_eq!(client.balance(&recipient), 100);

    // Lock again
    client.set_transfer_locked(&true);
    assert!(client.transfer_locked());
    let result = client.try_transfer(&user, &recipient, &100);
    assert_eq!(result, Err(Ok(crate::errors::Error::TransferLocked)));
}

#[test]
fn test_transfer_locked_with_sufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin, minter) = setup_token(&env);

    // Mint tokens to a non-admin user with sufficient balance
    let user = Address::generate(&env);
    client.mint(&user, &10000, &minter);
    assert_eq!(client.balance(&user), 10000);

    // Transfer should still fail even with sufficient balance when locked
    assert!(client.transfer_locked());
    let recipient = Address::generate(&env);
    let result = client.try_transfer(&user, &recipient, &100);
    assert_eq!(result, Err(Ok(crate::errors::Error::TransferLocked)));

    // Balance should remain unchanged
    assert_eq!(client.balance(&user), 10000);
    assert_eq!(client.balance(&recipient), 0);
}
