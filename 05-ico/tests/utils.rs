use ft_io::*;
use ico_io::*;
use gtest::{Program, RunResult, System};
pub const INVESTORS: &'static [u64] = &[3, 4, 5, 6, 7];
pub const ADMIN: u64 = 10;

pub fn init_fungible_token(sys: &System) {
    sys.init_logger();
    let ft = Program::from_file(
        &sys,
        "../04-fungible-token/target/wasm32-unknown-unknown/release/fungible_token.wasm",
    );

    assert!(ft.send(
        ADMIN,
        InitFToken {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
        },
    ).log().is_empty());

    assert!(!ft.send(ADMIN, FTAction::Mint {amount: 1000000}).main_failed());
}

pub fn init_ico(sys: &System) {
    sys.init_logger();
    let ico = Program::current(&sys);
    assert!(ico.send_bytes(ADMIN, b"INIT").log().is_empty());
    let ft = sys.get_program(1);
    assert!(!ft.send(ADMIN, FTAction::Transfer {from: ADMIN.into(), to: 2.into(), amount: 1000000}).main_failed());
}

pub fn start(ico: &Program, from: u64, duration: u64, price: u128, available_tokens: u128, min_purchase: u128, max_purchase: u128) -> RunResult {
    ico.send(from, ICOAction::Start { ft_contract_id: 1.into(), duration, price, available_tokens, min_purchase, max_purchase })
}

pub fn add_investor(ico: &Program, from: u64, investor: u64) -> RunResult {
    ico.send(from, ICOAction::AddInvestor { account: investor.into() })
}
pub fn buy(ico: &Program, from: u64, amount: u128) -> RunResult {
    ico.send_with_value(from, ICOAction::Buy, amount)
}

