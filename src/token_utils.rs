
use soroban_sdk::{Address, Env, log};

use crate::token;

pub fn balance(e: &Env, token: &Address, id: &Address) -> i128 {
    token::Client::new(e, token).balance(id)
}

pub fn transfer(e: &Env, token: &Address, from: &Address, to: &Address, amount: i128) {
    token::Client::new(e, token).transfer(from, to, &amount);
}

pub fn transfer_from(e: &Env, token: &Address, from: &Address, to: &Address, amount: i128) {
    let token_client = token::Client::new(e, token);
    let contract_address = e.current_contract_address();
    let approve_amount = amount * 2;

    log!(e, "{}, {}", balance(e, token, from), amount);
    token_client.increase_allowance(from, &contract_address, &approve_amount);
    token_client.transfer_from(&contract_address, from, to, &amount);
}

pub fn make_admin(e: &Env, token: &Address, to: &Address) {
    token::Client::new(e, token).set_admin(&to);
}

pub fn set_authorized(e: &Env, token: &Address, to: &Address) {
    token::Client::new(e, token).set_authorized(to, &true);
}

pub fn set_unauthorized(e: &Env, token: &Address, to: &Address) {
    token::Client::new(e, token).set_authorized(to, &false);
}

pub fn is_authorized(e: &Env, token: &Address, to: &Address) -> bool {
    token::Client::new(e, token).authorized(to)
}