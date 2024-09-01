use core::str;
use hex::decode;
use super::identifier::RESPONSE_STR_ENV;

pub fn load_message() -> String {
    let msg_content = decode(&std::env::var(RESPONSE_STR_ENV).unwrap())
        .expect("Your message must be decodifiable with hex");

    str::from_utf8(&msg_content).expect("Your message must contain valid UTF-8 chars").into()
}
