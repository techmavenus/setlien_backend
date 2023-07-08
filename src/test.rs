#![cfg(test)]
extern crate std;

use std::{print, println};

use crate::{contract::SetLien, contract::SetLienClient, storage_types::LeaseState, token};
use soroban_sdk::{testutils::Address as _, Address, Env, IntoVal, Symbol};

fn create_setlien<'a>(e: &Env, admin: &Address, payment_token: &Address) -> SetLienClient<'a> {
    let token = SetLienClient::new(e, &e.register_contract(None, SetLien {}));
    token.initialize(admin, payment_token);
    token
}

fn create_token<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let leaser = Address::random(&e);
    let renter = Address::random(&e);

    // create token and set owner as leaser
    let token_client: token::Client<'_> = create_token(&e, &leaser);
    let token: Address = token_client.address.clone();
    // mint 1 token to leaser
    token_client.mint(&leaser, &1);
    assert_eq!(1, token_client.balance(&leaser));

    let payment_client = create_token(&e, &admin);
    let payment: Address = payment_client.address.clone();
    payment_client.mint(&renter, &20);

    println!("{}, {:?}", payment_client.balance(&renter), payment);

    let set_lien: SetLienClient<'_> = create_setlien(&e, &admin, &payment);

    let price = 10;
    let max_duration = 30 * 24 * 60 * 60;
    let duration = 1 * 24 * 60 * 60;

    println!("{}", token_client.balance(&leaser));

    set_lien.lease(&leaser, &token, &price, &max_duration);
    // assert_eq!(
    //     e.auths(),
    //     [(
    //         leaser.clone(),
    //         set_lien.address.clone(),
    //         Symbol::short("lease"),
    //         (&leaser, &token, price, max_duration).into_val(&e),
    //     )]
    // );

    let lease = set_lien.get_lease(&token);
    assert_eq!(LeaseState::Listed, lease.state);
    assert_eq!(leaser, lease.leasing.leaser);
    assert_eq!(max_duration, lease.leasing.max_duration);
    assert_eq!(price, lease.leasing.price);

    println!("{}, {:?}", payment_client.balance(&renter), set_lien.get_payment_token());

    payment_client.increase_allowance(&renter, &set_lien.address, &1000);
    set_lien.rent(&renter, &token, &duration);
    // assert_eq!(
    //     e.auths(),
    //     [(
    //         renter.clone(),
    //         set_lien.address.clone(),
    //         Symbol::short("rent"),
    //         (&renter, &token, duration).into_val(&e),
    //     )]
    // );

    let lease = set_lien.get_lease(&token);
    assert_eq!(LeaseState::Rented, lease.state);
    assert_eq!(leaser, lease.leasing.leaser);
    assert_eq!(max_duration, lease.leasing.max_duration);
    assert_eq!(price, lease.leasing.price);
    assert_eq!(renter, lease.renting.renter);
    assert_eq!(duration, lease.renting.rent_duration);
}
