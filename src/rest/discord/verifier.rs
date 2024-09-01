use crate::{crypto, env::identifier};
use ntex::web::HttpRequest;
use std::error::Error;

#[derive(Debug)]
pub struct VerifierError;

impl core::fmt::Display for VerifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request must be sent from discord")
    }
}

impl Error for VerifierError {}

pub struct Verifier;

impl Verifier {
    pub fn verify(req: HttpRequest, body: &str) -> Result<(), VerifierError> {
        let timestamp = req
            .headers()
            .get("X-Signature-Timestamp")
            .ok_or(VerifierError)?;

        let signature = req
            .headers()
            .get("X-Signature-Ed25519")
            .ok_or(VerifierError)?;

        log::info!("Timestamp: {:?}, Signature: {:?}", timestamp, signature);

        let public_key = crypto::load::public_key_from_string(
            &std::env::var(identifier::PUBLIC_KEY_STR_ENV).unwrap(),
        )
        .map_err(|e| {
            log::error!("Failed to load public_key: {:?}", e);
            VerifierError
        })?;

        let verifier = crypto::discord_verify::DefaultSigVerifier {
            verifying_key: public_key,
        };

        let signature = crypto::load::signature_from_string(signature.to_str().map_err(|e| {
            log::error!("Non-ascii string passed for signature header: {:?}", e);
            VerifierError
        })?)
        .map_err(|_| VerifierError)?;

        verifier
            .verify(timestamp.as_bytes(), body.as_bytes(), &signature)
            .map_err(|e| {
                log::error!("Failed to verify signature: {:?}", e);
                VerifierError
            })?;

        Ok(())
    }
}
