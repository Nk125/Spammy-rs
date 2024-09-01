use ed25519::signature::Verifier;

pub struct DiscordSignatureVerifier<V> {
    pub verifying_key: V,
}

impl<V> DiscordSignatureVerifier<V>
where
    V: Verifier<ed25519::Signature>,
{
    pub fn verify(
        &self,
        timestamp: &[u8],
        req_body: &[u8],
        signature: &ed25519::Signature,
    ) -> Result<(), ed25519::Error> {
        self.verifying_key
            .verify(&[timestamp, req_body].concat(), signature)
    }
}

pub type DefaultSigVerifier = DiscordSignatureVerifier<ed25519_dalek::VerifyingKey>;
