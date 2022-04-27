#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum ICOAction {
    AddInvestor { account: ActorId },
    Start {
        ft_contract_id: ActorId,
        duration: u64,
        price: u128,
        available_tokens: u128,
        min_purchase: u128,
        max_purchase: u128,
    },
    Buy,
    Withdraw {
        to: ActorId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ICOEvent {
    ICOStarted {
        price: u128,
        available_tokens: u128,
    },
    TokensTransferred {
        amount: u128,
    },
    TokensWithdrawn {amount: u128},
}
