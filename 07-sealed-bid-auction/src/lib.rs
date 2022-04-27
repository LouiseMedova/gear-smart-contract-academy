#![no_std]

use codec::{Decode, Encode};
use gstd::{exec, msg, prelude::*, ActorId};
use primitive_types::{H256, U256};
use scale_info::TypeInfo;
use sealed_bid_auction_io::*;

pub mod ft_nft_messages;
use ft_nft_messages::*;

pub type ContractAndTokenId = String;

fn get_hash(price: u128, nonce: u128) -> H256 {
    let price_vec: Vec<u8> = price.to_be_bytes().into();
    let nonce_vec: Vec<u8> = nonce.to_be_bytes().into();
    sp_core_hashing::blake2_256(&[price_vec, nonce_vec].concat()).into()
}

#[derive(Default, Debug, Decode, Encode, TypeInfo)]
pub struct SBAuctionsContract {
    auctions: BTreeMap<ContractAndTokenId, SBAuction>,
    minimum_duration: u64,
    revealing_period: u64,
}

static mut CONTRACT: Option<SBAuctionsContract> = None;

impl SBAuctionsContract {
    async fn start_auction(
        &mut self,
        nft_contract_id: &ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        duration: u64,
        min_price: u128,
    ) {
        let contract_and_token_id =
            format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);
        self.assert_auction_exists(&contract_and_token_id);
        if duration < self.minimum_duration {
            panic!("auction duration can't be minimum bid period");
        }
        if min_price == 0 {
            panic!("price can't be equal to zero");
        }

        nft_approve(nft_contract_id, &exec::program_id(), token_id).await;

        self.auctions.insert(
            contract_and_token_id,
            SBAuction {
                seller: msg::source(),
                ft_contract_id,
                duration,
                min_price,
                ended_at: exec::block_timestamp() + duration,
                bids: BTreeMap::new(),
                highest_bid: 0,
                highest_bidder: ActorId::new([0u8; 32]),
            },
        );
        msg::reply(
            SBAuctionEvent::AuctionCreated {
                nft_contract_id: *nft_contract_id,
                token_id,
                ended_at: exec::block_timestamp() + duration,
                min_price,
            },
            0,
        )
        .unwrap();
    }

    fn bid(&mut self, nft_contract_id: &ActorId, token_id: U256, hash: H256) {
        let contract_and_token_id =
            format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);
        let auction = self
            .auctions
            .get_mut(&contract_and_token_id)
            .expect("Auction doesn not exist");
        if auction.ended_at < exec::block_timestamp() {
            panic!("Auction has already ended");
        }
        auction.bids.insert(msg::source(), hash);
        msg::reply(
            SBAuctionEvent::BidMade {
                nft_contract_id: *nft_contract_id,
                token_id,
                hash,
            },
            0,
        )
        .unwrap();
    }

    fn reveal(&mut self, nft_contract_id: &ActorId, token_id: U256, amount: u128, nonce: u128) {
        let contract_and_token_id =
            format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);
        let auction = self
            .auctions
            .get_mut(&contract_and_token_id)
            .expect("Auction doesn not exist");

        if auction.ended_at < exec::block_timestamp()
            && exec::block_timestamp() < auction.ended_at + self.revealing_period
        {
            panic!("Auction is not over or revealing_period has alredy finished");
        }
        let hash = auction
            .bids
            .get(&msg::source())
            .expect("That bid does not exist");
        if hash != &get_hash(amount, nonce) {
            panic!("Wrong indecated nonce or amount")
        }
        if auction.highest_bid < amount {
            auction.highest_bid = amount;
            auction.highest_bidder = msg::source();
        }
        msg::reply(
            SBAuctionEvent::BidRevealed {
                nft_contract_id: *nft_contract_id,
                token_id,
                amount,
            },
            0,
        )
        .unwrap();
    }

    async fn get_nft(&mut self, nft_contract_id: &ActorId, token_id: U256) {
        let contract_and_token_id =
            format!("{}{}", H256::from_slice(nft_contract_id.as_ref()), token_id);
        let auction = self
            .auctions
            .get_mut(&contract_and_token_id)
            .expect("Auction doesn not exist");
        if exec::block_timestamp() > auction.ended_at + self.revealing_period {
            panic!("Auction revealing_period must be over");
        }
        if msg::source() != auction.highest_bidder {
            panic!("Only auction winner can call that action");
        }
        if auction.ft_contract_id.is_none() && msg::value() != auction.highest_bid {
            panic!("attached value is not equal the indicated bid");
        } else {
            transfer_tokens(
                &auction.ft_contract_id.unwrap(),
                &msg::source(),
                &auction.seller,
                auction.highest_bid,
            )
            .await;
        }
        nft_transfer(nft_contract_id, &auction.highest_bidder, token_id).await;

        msg::reply(
            SBAuctionEvent::AuctionClosed {
                nft_contract_id: *nft_contract_id,
                token_id,
            },
            0,
        )
        .unwrap();
    }

    fn assert_auction_exists(&self, contract_and_token_id: &String) {
        if self.auctions.contains_key(contract_and_token_id) {
            panic!("Auction on that NFT already exist")
        }
    }
}

#[gstd::async_main]
async unsafe fn main() {
    let action: SBAuctionAction = msg::load().expect("Could not load Action");
    let contract: &mut SBAuctionsContract =
        unsafe { CONTRACT.get_or_insert(SBAuctionsContract::default()) };
    match action {
        SBAuctionAction::StartAuction {
            nft_contract_id,
            ft_contract_id,
            token_id,
            duration,
            min_price,
        } => {
            contract
                .start_auction(
                    &nft_contract_id,
                    ft_contract_id,
                    token_id,
                    duration,
                    min_price,
                )
                .await
        }
        SBAuctionAction::Bid {
            nft_contract_id,
            token_id,
            hash,
        } => contract.bid(&nft_contract_id, token_id, hash),
        SBAuctionAction::Reveal {
            nft_contract_id,
            token_id,
            amount,
            nonce,
        } => contract.reveal(&nft_contract_id, token_id, amount, nonce),
        SBAuctionAction::GetNFT {
            nft_contract_id,
            token_id,
        } => contract.get_nft(&nft_contract_id, token_id).await,
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitSBAuctionsContract = msg::load().expect("Unable to decode Init");

    let contract = SBAuctionsContract {
        auctions: BTreeMap::new(),
        minimum_duration: config.minimum_duration,
        revealing_period: config.revealing_period,
    };
    CONTRACT = Some(contract);
}
