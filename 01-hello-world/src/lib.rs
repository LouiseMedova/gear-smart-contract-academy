#![no_std]

use gstd::{msg, prelude::*};

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let new_msg = String::from_utf8(msg::load_bytes()).expect("Invalid message");
    if new_msg == "Hello" {
        msg::reply_push(b"Hel").unwrap();
        msg::reply_push(b"lo!").unwrap();
        msg::reply_commit(0).unwrap();
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {}

#[cfg(test)]
mod tests {
    use gtest::{Program, System};

    #[test]
    fn test_hello() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(10, "INIT");
        assert!(res.log().is_empty());

        let res = program.send_bytes(10, "Hello");
        let reply = "Hello!".as_bytes();
        assert!(res.contains(&(10, reply)));
    } 
}