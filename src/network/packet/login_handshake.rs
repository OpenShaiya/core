/// The opcode for a login handshake.
pub const LOGIN_HANDSHAKE_OPCODE: u16 = 0xA101;

/// The length of the public exponent array.
const EXPONENT_LENGTH: u8 = 64;

/// The length of the modulus array.
const MODULUS_LENGTH: u8 = 128;

/// Represents a request from the login server to a client, to initiate a handshake and configure
/// the AES encryption to use for further communication.
#[repr(C, packed)]
#[derive(Debug)]
pub struct LoginHandshakeRequest {
    opcode: u16,
    encrypted: bool,
    exponent_length: u8,
    modulus_length: u8,
    exponent: [u8; EXPONENT_LENGTH as usize],
    modulus: [u8; MODULUS_LENGTH as usize]
}

/// The method implementation for the login handshake request.
impl LoginHandshakeRequest {

    /// Initialises a new login request.
    pub fn new() -> LoginHandshakeRequest {
        LoginHandshakeRequest {
            opcode: LOGIN_HANDSHAKE_OPCODE,
            encrypted: true,
            exponent_length: EXPONENT_LENGTH,
            modulus_length: MODULUS_LENGTH,
            exponent: [0; EXPONENT_LENGTH as usize],
            modulus: [0; MODULUS_LENGTH as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network::packet::LoginHandshakeRequest;
    use core::mem;
    use crate::network::packet::login_handshake::{LOGIN_HANDSHAKE_OPCODE, EXPONENT_LENGTH, MODULUS_LENGTH};

    /// Test the length of the handshake packet.
    #[test]
    fn test_packet_length() {
        assert_eq!(mem::size_of::<LoginHandshakeRequest>(), 197);
    }

    /// Test that the opcode and public key sizes are correct in a default-initialised struct.
    #[test]
    #[allow(safe_packed_borrows)]
    fn test_default_values() {
        let request = LoginHandshakeRequest::new();
        assert_eq!(request.opcode, LOGIN_HANDSHAKE_OPCODE);
        assert_eq!(request.exponent_length, EXPONENT_LENGTH);
        assert_eq!(request.modulus_length, MODULUS_LENGTH);
    }

}