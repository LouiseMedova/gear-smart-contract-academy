#![no_std]

use gstd::{ exec, msg, prelude::*, ActorId};
use nft_io::*;
const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

#[derive(Debug, Default)]
pub struct NonFungibleToken {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub owner_by_id: BTreeMap<TokenId, ActorId>,
    pub token_approvals: BTreeMap<TokenId, Vec<ActorId>>,
    pub token_metadata_by_id: BTreeMap<TokenId, Option<TokenMetadata>>,
    pub tokens_for_owner: BTreeMap<ActorId, Vec<TokenId>>,
}

gstd::metadata! {
    title: "NFT",
        init:
            input: InitNFT,
        handle:
            input: NFTAction,
            output: NFTEvent,
}

static mut NON_FUNGIBLE_TOKEN: Option<NonFungibleToken> = None;

impl NonFungibleToken {
    fn mint(&mut self, to: &ActorId, token_id: TokenId, token_metadata: Option<TokenMetadata>) {
        self.assert_token_exists(token_id);
        self.owner_by_id.insert(token_id, *to);
        self.tokens_for_owner
            .entry(*to)
            .and_modify(|tokens| tokens.push(token_id));
        self.token_metadata_by_id.insert(token_id, token_metadata);
        msg::reply(
            NFTEvent::Transfer {
                from: ZERO_ID,
                to: *to,
                token_id,
            },
            0,
        )
        .unwrap();
    }

    fn burn(&mut self, token_id: TokenId) {
        self.assert_owner(token_id);
        let owner = *self
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");
        self.owner_by_id.remove(&token_id);
        self.tokens_for_owner
            .entry(owner)
            .and_modify(|tokens| tokens.retain(|&token| token != token_id));
        msg::reply(
            NFTEvent::Transfer {
                from: owner,
                to: ZERO_ID,
                token_id,
            },
            0,
        )
        .unwrap();
    }

    fn transfer(&mut self, to: &ActorId, token_id: TokenId) {
        self.assert_can_transfer(token_id);
        self.assert_zero_address(to);
        let owner = *self
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");

        // assign new owner
        self.owner_by_id
            .entry(token_id)
            .and_modify(|owner| *owner = *to);
        // push token to new owner
        self.tokens_for_owner
            .entry(*to)
            .and_modify(|tokens| tokens.push(token_id));
        // remove token from old owner
        self.tokens_for_owner
            .entry(owner)
            .and_modify(|tokens| tokens.retain(|&token| token != token_id));
        // remove approvals if any
        self.token_approvals.remove(&token_id);

        msg::reply(
            NFTEvent::Transfer {
                from: owner,
                to: *to,
                token_id,
            },
            0,
        )
        .unwrap();
    }

    fn approve(&mut self, to: &ActorId, token_id: TokenId) {
        self.assert_owner(token_id);
        self.assert_zero_address(to);
        let owner = *self
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");
        self.token_approvals
            .entry(token_id)
            .and_modify(|approvals| approvals.push(*to))
            .or_insert_with(|| vec![*to]);
        msg::reply(
            NFTEvent::Approval {
                owner,
                approved_account: *to,
                token_id,
            },
            0,
        )
        .unwrap();
    }

    fn assert_token_exists(&self, token_id: TokenId) {
        if self.owner_by_id.contains_key(&token_id) {
            panic!("NonFungibleToken: Token already exists");
        }
    }

    fn assert_zero_address(&self, account: &ActorId) {
        if account == &ZERO_ID {
            panic!("NonFungibleToken: Zero address");
        }
    }

    fn assert_can_transfer(&self, token_id: TokenId) {
        if let Some(approved_accounts) = self.token_approvals.get(&token_id) {
            if approved_accounts.contains(&msg::source()) {
                return;
            }
        }
        self.assert_owner(token_id);
    }

    fn assert_owner(&self, token_id: TokenId) {
        let owner = self
            .owner_by_id
            .get(&token_id)
            .expect("NonFungibleToken: token does not exist");
        if !(owner == &msg::source() || owner == &exec::origin()) {
            panic!("Not allowed to transfer");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: NFTAction = msg::load().expect("Could not load Action");
    let nft: &mut NonFungibleToken = NON_FUNGIBLE_TOKEN.get_or_insert(NonFungibleToken::default());
    match action {
        NFTAction::Mint {
            to,
            token_id,
            token_metadata,
        } => nft.mint(&to, token_id, token_metadata),
        NFTAction::Burn { token_id } => nft.burn(token_id),

        NFTAction::Transfer { to, token_id } => nft.transfer(&to, token_id),

        NFTAction::Approve { to, token_id } => nft.approve(&to, token_id),
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitNFT = msg::load().expect("Unable to decode init message");
    let nft = NonFungibleToken {
        name: config.name,
        symbol: config.symbol,
        base_uri: config.base_uri,
        ..NonFungibleToken::default()
    };
    NON_FUNGIBLE_TOKEN = Some(nft);
}
