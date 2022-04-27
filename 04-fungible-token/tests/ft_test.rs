use codec::Encode;
use gtest::{Program, System};
use ft_io::*;
mod utils;
use utils::*;

#[test]
fn mint_success() {
    let sys = System::new();
    init_fungible_token(&sys);
    let ft = sys.get_program(1);
    let res = mint(&ft, USERS[0], 10000);
    assert!(res.contains(&(
        USERS[0],
        FTEvent::Transfer {
            from: ZERO_ID.into(),
            to: USERS[0].into(),
            amount: 10000
        }
        .encode()
    )));
}

#[test]
fn burn_success() {
    let sys = System::new();
    init_fungible_token(&sys);
    let ft = sys.get_program(1);
    assert!(!mint(&ft, USERS[0], 10000).main_failed());
    let res = burn(&ft, USERS[0], 8000);
    assert!(res.contains(&(
        USERS[0],
        FTEvent::Transfer {
            from: USERS[0].into(),
            to: ZERO_ID.into(),
            amount: 8000
        }
        .encode()
    )));
}

#[test]
fn transfer_success() {
    let sys = System::new();
    init_fungible_token(&sys);
    let ft = sys.get_program(1);
    assert!(!mint(&ft, USERS[0], 10000).main_failed());
    let res = transfer(&ft, USERS[0], USERS[0], USERS[1], 8000);
    assert!(res.contains(&(
        USERS[0],
        FTEvent::Transfer {
            from: USERS[0].into(),
            to: USERS[1].into(),
            amount: 8000
        }
        .encode()
    )));
}

#[test]
fn approve_success() {
    let sys = System::new();
    init_fungible_token(&sys);
    let ft = sys.get_program(1);
    assert!(!mint(&ft, USERS[0], 10000).main_failed());
    let res = approve(&ft, USERS[0], USERS[1], 8000);
    assert!(res.contains(&(
        USERS[0],
        FTEvent::Approve {
            from: USERS[0].into(),
            to: USERS[1].into(),
            amount: 8000
        }
        .encode()
    )));
}