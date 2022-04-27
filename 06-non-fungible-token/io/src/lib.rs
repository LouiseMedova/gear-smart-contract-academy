#![no_std]

use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;
use scale_info::TypeInfo;
pub type TokenId = U256;

#[derive(Debug, Default, Decode, Encode, TypeInfo)]
pub struct Token {
    pub token_id: TokenId,
    pub owner_id: ActorId,
    pub metadata: Option<TokenMetadata>,
    pub approved_account_ids: Vec<ActorId>,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct TokenMetadata {
    // ex. "CryptoKitty #100"
    pub title: Option<String>,
    // free-form description
    pub description: Option<String>,
    // URL to associated media, preferably to decentralized, content-addressed storage
    pub media: Option<String>,
    // URL to an off-chain JSON file with more info.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
}

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct InitNFT {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum NFTAction {
    Mint {
        to: ActorId,
        token_id: TokenId,
        token_metadata: Option<TokenMetadata>,
    },
    Burn {
        token_id: TokenId,
    },
    Transfer {
        to: ActorId,
        token_id: TokenId,
    },
    Approve {
        to: ActorId,
        token_id: TokenId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum NFTEvent {
    Transfer {
        from: ActorId,
        to: ActorId,
        token_id: TokenId,
    },
    Approval {
        owner: ActorId,
        approved_account: ActorId,
        token_id: TokenId,
    },
}
