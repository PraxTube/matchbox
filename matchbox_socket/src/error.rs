use crate::webrtc_socket::error::{MessageSendError, SignalingError};

/// Errors that can happen when using Matchbox sockets.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurring during the signaling loop.
    #[error("An error in the signaling loop")]
    Signaling(#[from] SignalingError),

    /// An error occurring during the messaging loop.
    #[error("An error in the message loop")]
    Messaging(#[from] MessageSendError),
}
