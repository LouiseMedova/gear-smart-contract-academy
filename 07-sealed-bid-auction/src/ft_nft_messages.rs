use ft_io::*;
use gstd::{msg, ActorId};
use nft_io::*;

pub async fn transfer_tokens(ft_contract_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128) {
    let _transfer_response: FTEvent = msg::send_and_wait_for_reply(
        *ft_contract_id,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("Error in transfer");
}

pub async fn nft_transfer(nft_program_id: &ActorId, to: &ActorId, token_id: TokenId) {
    let _transfer_response: NFTEvent = msg::send_and_wait_for_reply(
        *nft_program_id,
        NFTAction::Transfer { to: *to, token_id },
        0,
    )
    .unwrap()
    .await
    .expect("error in transfer");
}

pub async fn nft_approve(nft_program_id: &ActorId, to: &ActorId, token_id: TokenId) {
    let _approve_response: NFTEvent =
        msg::send_and_wait_for_reply(*nft_program_id, NFTAction::Approve { to: *to, token_id }, 0)
            .unwrap()
            .await
            .expect("error in transfer");
}
