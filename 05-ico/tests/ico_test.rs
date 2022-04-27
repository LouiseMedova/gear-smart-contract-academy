use codec::Encode;
use gtest::{Program, System};
mod utils;
use utils::*;
use ico_io::*;
pub const DURATION: u64 = 1_000_000;
pub const PRICE: u128 = 100;
pub const AVAILABLE_TOKENS: u128 = 1_000_000;
pub const MIN_PURCHASE: u128 = 1_000;
pub const MAX_PURCHASE: u128 = 50_000_000;

#[test]
fn start_success() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_ico(&sys);
    let ico = sys.get_program(2);
    let res = start(&ico, ADMIN, DURATION, PRICE, AVAILABLE_TOKENS, MIN_PURCHASE, MAX_PURCHASE);
    assert!(res.contains(&(
        ADMIN,
        ICOEvent::ICOStarted {
            price: PRICE,
            available_tokens: AVAILABLE_TOKENS,
        }
        .encode()
    )));
}

#[test]
fn buy_success() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_ico(&sys);
    let ico = sys.get_program(2);
    assert!(!start(&ico, ADMIN, DURATION, PRICE, AVAILABLE_TOKENS, MIN_PURCHASE, MAX_PURCHASE).main_failed());
    assert!(!add_investor(&ico, ADMIN, INVESTORS[0]).main_failed());
    let res = buy(&ico, INVESTORS[0], 10_002);
    assert!(res.contains(&(
        INVESTORS[0],
        ICOEvent::TokensTransferred {
            amount: 100,
        }
        .encode()
    )));
}

#[test]
fn sold_out() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_ico(&sys);
    let ico = sys.get_program(2);
    assert!(!start(&ico, ADMIN, DURATION, PRICE, AVAILABLE_TOKENS, MIN_PURCHASE, MAX_PURCHASE).main_failed());
    INVESTORS.iter().for_each(|investor| {
        assert!(!add_investor(&ico, ADMIN, *investor).main_failed());
    });
    assert!(!buy(&ico, INVESTORS[0], 10_000_000).main_failed());
    assert!(!buy(&ico, INVESTORS[1], 20_000_000).main_failed());
    assert!(!buy(&ico, INVESTORS[2], 30_000_000).main_failed());
    assert!(!buy(&ico, INVESTORS[3], 40_000_000).main_failed());

    //assert!(!buy(&ico, INVESTORS[3], 50_000_000).main_failed());
}