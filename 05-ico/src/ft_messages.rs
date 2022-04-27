use ft_io::*;
use gstd::{msg, ActorId};

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

pub async fn total_supply(ft_contract_id: &ActorId) -> u128 {
    let total_supply: FTEvent =
        msg::send_and_wait_for_reply(*ft_contract_id, FTAction::TotalSupply, 0)
            .unwrap()
            .await
            .expect("Error in total_supply message");
    if let FTEvent::TotalSupply(total_supply) = total_supply {
        total_supply
    } else {
        0
    }
}