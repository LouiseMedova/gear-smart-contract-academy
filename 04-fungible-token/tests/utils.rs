use ft_io::*;
use gtest::{Program, RunResult, System};
pub const USERS: &'static [u64] = &[2, 3, 4];
pub const ZERO_ID: u64 = 0;

pub fn init_fungible_token(sys: &System) {
    sys.init_logger();
    let ft = Program::current(&sys);

    let res = ft.send(
        USERS[0],
        InitFToken {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
        },
    );

    assert!(res.log().is_empty());
}

pub fn mint(ft: &Program, from: u64, amount: u128) -> RunResult {
    ft.send(from, FTAction::Mint { amount })
}

pub fn burn(ft: &Program, from: u64, amount: u128) -> RunResult {
    ft.send(from, FTAction::Burn { amount })
}

pub fn transfer(ft: &Program, caller: u64, from: u64, to: u64, amount: u128) -> RunResult {
    ft.send(
        caller,
        FTAction::Transfer {
            from: from.into(),
            to: to.into(),
            amount,
        },
    )
}

pub fn approve(ft: &Program, caller: u64, to: u64, amount: u128) -> RunResult {
    ft.send(
        caller,
        FTAction::Approve {
            to: to.into(),
            amount,
        },
    )
}
