use super::error::Error;
use ed25519_dalek::{Signature, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};

fn bytes_from_hex_string<const LEN_CONSTRAINT: usize>(hex_str: &str) -> Result<Vec<u8>, Error<'_>> {
    if (hex_str.len() / 2) != LEN_CONSTRAINT {
        return Err(Error::InvalidLength);
    }

    hex::decode(hex_str).map_err(|e| {
        log::debug!("Invalid hex str: {}, e: {:?}", hex_str, e);
        Error::ConversionError("hex")
    })
}

fn truncate_some<'a, const TRUNCATE: usize>(bytes: Vec<u8>) -> Result<[u8; TRUNCATE], Error<'a>> {
    bytes
        .first_chunk::<TRUNCATE>()
        .ok_or(Error::ConversionError("truncate chunk"))
        .cloned()
}

pub fn public_key_from_string(hex_public_key: &str) -> Result<VerifyingKey, Error<'_>> {
    let bytes = bytes_from_hex_string::<PUBLIC_KEY_LENGTH>(hex_public_key)?;

    VerifyingKey::from_bytes(&truncate_some::<PUBLIC_KEY_LENGTH>(bytes)?).map_err(|e| {
        log::info!("Signature error: {:?}", e);
        Error::SignatureError
    })
}

pub fn signature_from_string(hex_signature: &str) -> Result<Signature, Error<'_>> {
    let bytes = bytes_from_hex_string::<SIGNATURE_LENGTH>(hex_signature)?;

    Ok(Signature::from(&truncate_some::<SIGNATURE_LENGTH>(bytes)?))
}
