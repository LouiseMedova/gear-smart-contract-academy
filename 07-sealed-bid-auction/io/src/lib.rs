#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use primitive_types::{H256, U256};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct SBAuction {
    pub seller: ActorId,
    pub ft_contract_id: Option<ActorId>,
    pub duration: u64,
    pub min_price: u128,
    pub ended_at: u64,
    pub bids: BTreeMap<ActorId, H256>,
    pub highest_bid: u128,
    pub highest_bidder: ActorId,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum SBAuctionAction {
    StartAuction {
        nft_contract_id: ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        duration: u64,
        min_price: u128,
    },
    Bid {
        nft_contract_id: ActorId,
        token_id: U256,
        hash: H256,
    },
    Reveal {
        nft_contract_id: ActorId,
        token_id: U256,
        amount: u128,
        nonce: u128,
    },
    GetNFT {
        nft_contract_id: ActorId,
        token_id: U256,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum SBAuctionEvent {
    AuctionCreated {
        nft_contract_id: ActorId,
        token_id: U256,
        ended_at: u64,
        min_price: u128,
    },
    BidMade {
        nft_contract_id: ActorId,
        token_id: U256,
        hash: H256,
    },
    BidRevealed {
        nft_contract_id: ActorId,
        token_id: U256,
        amount: u128,
    },
    AuctionClosed {
        nft_contract_id: ActorId,
        token_id: U256,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitSBAuctionsContract {
    pub minimum_duration: u64,
    pub revealing_period: u64,
}
