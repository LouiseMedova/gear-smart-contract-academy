#![no_std]

use codec::{Decode, Encode};
use gstd::{exec, msg, prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Encode, Decode)]
pub struct InitLoan {
    amount: u128,
    interest: u128,
    lender: ActorId,
    borrower: ActorId,
    duration: u64,
}

#[derive(PartialEq, Debug, Encode, Decode, TypeInfo, Clone)]
 pub enum LoanState {
    Pending,
    Active,
    Closed,
}

impl Default for LoanState {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Default)]
pub struct Loan {
    state: LoanState,
    borrower: ActorId,
    lender: ActorId,
    duration: u64,
    end: u64,
    amount: u128,
    interest: u128,
}
#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum LoanAction {
    Fund,
    Reimburse,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum LoanEvent {
    Funded,
    Reimbursed,
}

static mut LOAN: Option<Loan> = None;

impl Loan {
    fn fund(&mut self) {
        self.check_msg_source(self.lender);
        self.check_state(LoanState::Pending);
        self.check_attached_value(self.amount);
        self.end = exec::block_timestamp() + self.duration;
        self.state = LoanState::Active;
        msg::send(self.borrower, b"Lending is active", msg::value()).unwrap();
        msg::reply(LoanEvent::Funded, 0).unwrap();
    }

    fn reimburse(&mut self) {
        self.check_msg_source(self.borrower);
        self.check_state(LoanState::Active);
        self.check_attached_value(self.amount + self.interest);

        if exec::block_timestamp() < self.end {
            panic!("Too early for reimbursement");
        }
        self.state = LoanState::Closed;
        msg::send(self.borrower, b"Reimburse", msg::value()).unwrap();
        msg::reply(LoanEvent::Reimbursed, 0).unwrap();
    }

    fn check_msg_source(&self, account: ActorId) {
        if msg::source() != account {
            panic!("`msg::source` must be {:?} account", account)
        }
    }

    fn check_state(&self, state: LoanState) {
        if self.state != state {
            panic!("Loan must be in the {:?} state", state)
        }
    }
    fn check_attached_value(&self, value: u128) {
        if msg::value() != value {
            panic!("Must attached the {:?} value", value)
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: LoanAction = msg::load().expect("Could not load LoanAction");
    let loan: &mut Loan = LOAN.get_or_insert(Loan::default());
    match action {
        LoanAction::Fund => loan.fund(),
        LoanAction::Reimburse => loan.reimburse(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let loan_config: InitLoan = msg::load().expect("Unable to decode InitLoan");
    let loan = Loan {
        borrower: loan_config.borrower,
        lender: loan_config.lender,
        duration: loan_config.duration,
        amount: loan_config.amount,
        interest: loan_config.interest,
        ..Loan::default()
    };
    LOAN = Some(loan);
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum LoanMetaState {
    CurrentState,
    Details,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum LoanMetaStateReply {
    CurrentState(LoanState),
    Details{ 
        lender: ActorId,
        borrower: ActorId,
        amount: u128, 
        interest: u128,
        end: u64,
    },
}
#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let state: LoanMetaState = msg::load().expect("failed to decode LoanMetaState");
    let loan: &mut Loan = LOAN.get_or_insert(Loan::default());
    let encoded = match state {
        LoanMetaState::CurrentState =>  LoanMetaStateReply::CurrentState(loan.state.clone()).encode(),
        LoanMetaState::Details => LoanMetaStateReply::Details {
            lender: loan.lender,
            borrower: loan.borrower,
            amount: loan.amount,
            interest: loan.interest,
            end: loan.end,
        }.encode(),
    };
    gstd::util::to_leak_ptr(encoded)
}
#[cfg(test)]
mod tests {
    use crate::*;
    use gtest::{Program, RunResult, System};

    pub const LENDER: u64 = 2;
    pub const BORROWER: u64 = 3;
    pub const DURATION: u64 = 10 * 24 * 60 * 60 * 1000;

    fn init_loan(sys: &System) {
        sys.init_logger();
        let loan = Program::current(&sys);
        let res = loan.send(
            LENDER,
            InitLoan {
                amount: 1000,
                interest: 100,
                lender: LENDER.into(),
                borrower: BORROWER.into(),
                duration: DURATION,
            },
        );
        assert!(res.log().is_empty());
    }

    fn fund(loan: &Program, from: u64, amount: u128) -> RunResult {
        loan.send_with_value(from, LoanAction::Fund, amount)
    }
    fn reimburse(loan: &Program, from: u64, amount: u128) -> RunResult {
        loan.send_with_value(from, LoanAction::Reimburse, amount)
    }

    #[test]
    fn fund_success() {
        let sys = System::new();
        init_loan(&sys);
        let loan = sys.get_program(1);
        let res = fund(&loan, LENDER, 1000);
        assert!(res.contains(&(LENDER, LoanEvent::Funded.encode())));
    }

    #[test]
    fn reimburse_success() {
        let sys = System::new();
        init_loan(&sys);
        let loan = sys.get_program(1);
        assert!(!fund(&loan, LENDER, 1000).main_failed());
        sys.spend_blocks(DURATION as u32);
        let res = reimburse(&loan, BORROWER, 1100);
        assert!(res.contains(&(BORROWER, LoanEvent::Reimbursed.encode())));
    }

    #[test]
    fn fund_failures() {
        let sys = System::new();
        init_loan(&sys);
        let loan = sys.get_program(1);
        // must fail since the caller account is not a lender
        assert!(fund(&loan, BORROWER, 1000).main_failed());
        // must fail since attached value is not equat to the amount indicated in the contract
        assert!(fund(&loan, LENDER, 1001).main_failed());

        // funded
        assert!(!fund(&loan, LENDER, 1000).main_failed());
        sys.spend_blocks(DURATION as u32);
        // reimbursed
        assert!(!reimburse(&loan, BORROWER, 1100).main_failed());

        // must fail since loan is already closed
        assert!(fund(&loan, LENDER, 1000).main_failed());
    }

    #[test]
    fn reimburse_failures() {
        let sys = System::new();
        init_loan(&sys);
        let loan = sys.get_program(1);

        // must fail since the loan is in pending state
        assert!(reimburse(&loan, BORROWER, 1100).main_failed());

        // funded
        assert!(!fund(&loan, LENDER, 1000).main_failed());
        // must fail since the loan time is not finished
        assert!(reimburse(&loan, BORROWER, 1100).main_failed());

        sys.spend_blocks(DURATION as u32);

        // must fail since the caller is not a borrower
        assert!(reimburse(&loan, LENDER, 1100).main_failed());

        // must fail since the attached value is not equal to amount + interest
        assert!(reimburse(&loan, BORROWER, 1101).main_failed());

        // reimbursed
        assert!(!reimburse(&loan, BORROWER, 1100).main_failed());

        // must fail since loan is already closed
        assert!(reimburse(&loan, BORROWER, 1100).main_failed());
    }
}
