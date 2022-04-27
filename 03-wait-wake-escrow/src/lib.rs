#![no_std]

use codec::{Decode, Encode};
use gstd::{exec, msg, MessageId, prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Encode, Decode)]
pub struct InitEscrow {
    seller: ActorId,
    price: u128,
}

#[derive(Default)]
pub struct Escrow {
    buyer: ActorId,
    seller: ActorId,
    state: EscrowState,
    price: u128,
}
#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum EscrowAction {
    SendPayment,
    ApproveSale(bool),
    ConfirmDelivery,
}

#[derive(Debug, PartialEq, Encode, Decode, TypeInfo, Clone)]
 pub enum EscrowState {
    Uninitialized,
    PaymentSent { payment_id: MessageId },
    ApprovedBySeller,
    AwaitingDelivery,
    ConfirmDelivery,
    Completed,
}

impl Default for EscrowState {
    fn default() -> Self {
        Self::Uninitialized
    }
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum EscrowEvent {
    PaymentSentToContract,
    PaymentSentBack,
    EscrowCompleted,
}

static mut ESCROW: Option<Escrow> = None;

impl Escrow {
    fn send_payment(&mut self) {
        if msg::value() != self.price {
            panic!("must attach exactly {:?}", self.price)
        }
        self.buyer = msg::source();
        match self.state {
            EscrowState::Uninitialized =>  {
                self.state = EscrowState::PaymentSent{payment_id: msg::id()};
                exec::wait();
            },
            EscrowState::ApprovedBySeller => {
                self.state = EscrowState::AwaitingDelivery;
                msg::reply(EscrowEvent::PaymentSentToContract, 0).unwrap()
            },
            _ => msg::reply(EscrowEvent::PaymentSentBack, msg::value()).unwrap(),
        };
    }

    fn approve_sale(&mut self, approve: bool) {
        self.check_msg_source(self.seller);
        if let EscrowState::PaymentSent{ payment_id } = self.state  {
            if approve {
                self.state = EscrowState::ApprovedBySeller;
            }
            exec::wake(payment_id);
        } else {
            panic!("Escrow must be in PaymentSent state");
        }
    }

    fn confirm_delivery(&mut self) {
        self.check_msg_source(self.buyer);
        if self.state != EscrowState::AwaitingDelivery {
            panic!("Escrow must be in AwaitingDelivery state");
        }
        self.check_msg_source(self.buyer);
        msg::send(self.seller, b"Payment", msg::value()).unwrap();
        self.state = EscrowState::Completed;   
        msg::reply(EscrowEvent::EscrowCompleted, 0).unwrap();
    }

    fn check_msg_source(&self, account: ActorId) {
        if msg::source() != account {
            panic!("`msg::source` must be {:?} account", account)
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: EscrowAction = msg::load().expect("Could not load EscrowAction");
    let escrow: &mut Escrow = ESCROW.get_or_insert(Escrow::default());
    match action {
        EscrowAction::SendPayment => escrow.send_payment(),
        EscrowAction::ApproveSale(approve) => escrow.approve_sale(approve),
        EscrowAction::ConfirmDelivery => escrow.confirm_delivery(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let price: u128 = msg::load().expect("Unable to decode price");
    let escrow = Escrow {
        seller: msg::source(),
        price,
        ..Escrow::default()
    };
    ESCROW = Some(escrow);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use gtest::{Program, RunResult, System};
    extern crate std;
    use std::println;
    pub const BUYER: u64 = 3;
    pub const SELLER: u64 = 4;

    fn init_escrow(sys: &System) {
        sys.init_logger();
        let escrow = Program::current(&sys);
        let res = escrow.send(SELLER, 1000 as u128);
        assert!(res.log().is_empty());
    }

    fn send_payment(escrow: &Program, from: u64, amount: u128) -> RunResult {
        escrow.send_with_value(from, EscrowAction::SendPayment, amount)
    }

    fn approve_sale(escrow: &Program, from: u64, approve: bool) -> RunResult {
        escrow.send(from, EscrowAction::ApproveSale(approve))
    }

    fn confirm_delivery(escrow: &Program, from: u64) -> RunResult {
        escrow.send(from, EscrowAction::ConfirmDelivery)
    }

    #[test]
    fn send_payment_success() {
        let sys = System::new();
        init_escrow(&sys);
        let escrow = sys.get_program(1);
        assert!(!send_payment(&escrow, BUYER, 1000 as u128).main_failed());
    }

    #[test]
    fn approve_sale_success() {
        let sys = System::new();
        init_escrow(&sys);
        let escrow = sys.get_program(1);
        assert!(!send_payment(&escrow, BUYER, 1000 as u128).main_failed());
        let res = approve_sale(&escrow, SELLER, true);
        assert!(res.contains(&(BUYER, EscrowEvent::PaymentSentToContract.encode())));

        init_escrow(&sys);
        let escrow = sys.get_program(2);
        assert!(!send_payment(&escrow, BUYER, 1000 as u128).main_failed());
        let res = approve_sale(&escrow, SELLER, false);
        assert!(res.contains(&(BUYER, EscrowEvent::PaymentSentBack.encode())));
    }

    #[test]
    fn confirm_delivery_success() {
        let sys = System::new();
        init_escrow(&sys);
        let escrow = sys.get_program(1);
        assert!(!send_payment(&escrow, BUYER, 1000 as u128).main_failed());
        assert!(!approve_sale(&escrow, SELLER, true).main_failed());
    
        let res = confirm_delivery(&escrow, BUYER);
        assert!(res.contains(&(BUYER, EscrowEvent::PaymentSentBack.encode())));
    }

    #[test]
    fn send_payment_failures() {
        let sys = System::new();
        init_escrow(&sys);
        let escrow = sys.get_program(1);
        assert!(!send_payment(&escrow, BUYER, 1000 as u128).main_failed());
    }
}
