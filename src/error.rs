use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Message has no payload")]
    NoPayload,
    #[error("Message payload is not a string")]
    PayloadIsNotString,
    #[error("Payload deserialized error")]
    PayloadDeserializedError(#[from] serde_json::error::Error),
}
