pub trait IJWTHandler<Payload> {
    fn verify(&self, jwt: &str) -> Result<Payload, biscuit::errors::Error>;
}
