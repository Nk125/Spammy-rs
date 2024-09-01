use super::identifier;
use std::env;

const MUST_BE_PRESENT: &[&str] = &[
    identifier::COMMAND_NAME_STR_ENV,
    identifier::DISCORD_TOKEN_STR_ENV,
    identifier::LISTEN_PORT_STR_ENV,
    identifier::PUBLIC_KEY_STR_ENV,
    identifier::RESPONSE_STR_ENV,
];

pub fn all_vars_defined() -> bool {
    let mut all_defined = true;

    for key in MUST_BE_PRESENT {
        all_defined &= match env::var(key).is_ok() {
            true => true,
            false => {
                log::error!("Missing env variable: {}", key);
                false
            }
        };
    }

    all_defined
}
