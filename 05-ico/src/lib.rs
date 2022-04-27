#![no_std]

use gstd::{exec, msg, debug, prelude::*, ActorId};
pub mod ft_messages;
pub use ft_messages::*;
use ico_io::*;

#[derive(Debug, Default)]
struct Ico {
    admin: ActorId,
    investors: Vec<ActorId>,
    price: u128,
    min_purchase: u128,
    max_purchase: u128,
    ft_contract_id: ActorId,
    end: u64,
    available_tokens: u128,
}

static mut ICO: Option<Ico> = None;

impl Ico {
    fn add_investor(&mut self, account: &ActorId) {
        self.assert_admin();
        self.investors.push(*account);
    }
    async fn start(&mut self, ft_contract_id: &ActorId, duration: u64, price: u128, available_tokens: u128, min_purchase: u128, max_purchase: u128) {
        self.assert_admin();
        if self.end > 0 {
            panic!("ICO should not be active")
        }
        debug!("{:?}", self.ft_contract_id);
        let total_supply = total_supply(&ft_contract_id).await;
        debug!("a");
        if available_tokens == 0 || available_tokens > total_supply {
            panic!("Indicated amount of available tokens must be > 0 and < total supply")
        }

        if duration == 0 || price == 0 || min_purchase == 0 || max_purchase == 0 {
            panic!("Duration, price, min_purchase and max_purchase must be > 0")
        }

        if max_purchase >= available_tokens * price {
            panic!("Max_purchase must be < available tokens * price")
        } 
        self.ft_contract_id = *ft_contract_id;
        self.price = price;
        self.available_tokens = available_tokens;
        self.min_purchase = min_purchase;
        self.max_purchase = max_purchase;
        self.end = exec::block_timestamp() + duration;
        msg::reply(ICOEvent::ICOStarted {
            price: self.price,
            available_tokens: self.available_tokens ,
        }, 0).unwrap();
    }

    async fn buy(&mut self) {
        if !self.investors.contains(&msg::source()) {
            panic!("`msg::source()` is not an investor")
        }
        if self.available_tokens == 0 || exec::block_timestamp() > self.end {
            panic!("ICO is not active")
        }
        if msg::value() < self.min_purchase || msg::value() > self.max_purchase {
            panic!("Must attach between min_purchase and max_purchase")
        }
        let amount = msg::value() / self.price;
        if amount > self.available_tokens {
            panic!("Not enough tokens for sale")
        }
        self.available_tokens -= amount;
        transfer_tokens(&self.ft_contract_id, &exec::program_id(), &msg::source(), amount).await;
        msg::reply(ICOEvent::TokensTransferred {amount}, 0).unwrap();
    }

    async fn withdraw(&mut self, to: &ActorId) {
        self.assert_admin();
        if exec::block_timestamp() <= self.end {
            panic!("ICO must have ended")
        }
        if self.available_tokens == 0 {
            panic!("No tokens to withdraw")
        }
        transfer_tokens(&self.ft_contract_id, &exec::program_id(), to, self.available_tokens).await;
        msg::reply(ICOEvent::TokensWithdrawn {amount: self.available_tokens}, 0).unwrap();
        self.available_tokens = 0;
    }

    fn assert_admin(&self) {
        if msg::source() != self.admin {
            panic!("Only admin can call that function")
        }
    }
}
  

#[gstd::async_main]
async unsafe fn main(){
    let action: ICOAction = msg::load().expect("Could not load Action");
    let ico: &mut Ico = unsafe {ICO.get_or_insert(Ico::default())};
    match action {
        ICOAction::AddInvestor { account } => ico.add_investor(&account),
        ICOAction::Start { ft_contract_id, duration, price, available_tokens, min_purchase, max_purchase } => 
            ico.start(&ft_contract_id, duration, price, available_tokens, min_purchase, max_purchase).await,
        ICOAction::Buy => 
            ico.buy().await,
        ICOAction::Withdraw { to } => 
            ico.withdraw(&to).await,
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let ico = Ico {
        admin: msg::source(),
        ..Ico::default()
    };
    ICO = Some(ico);
}

