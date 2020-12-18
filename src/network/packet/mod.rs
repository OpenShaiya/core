mod login_handshake;
pub use login_handshake::LoginHandshakeRequest;
use bytes::BytesMut;

/// A trait that a structure must implement if it wishes to be treated as a serializable packet.
pub trait SerializablePacket {

    /// Serializes the structure into a slice of bytes.
    fn serialize(&self, writer: &mut BytesMut);
}