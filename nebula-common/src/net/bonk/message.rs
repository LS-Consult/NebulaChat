use futures::{AsyncReadExt, AsyncWriteExt};
use serde::Serializer;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Message {
    /// A simple keep-alive message
    Bonk,

    /// Execute a x25519 key exchange
    Handshake([u8; 32]),

    /// Publish peer information
    PublishPeer(PeerInformation),

    /// Request peer information
    RequestPeers,

    /// Broadcast a signal into the network
    Broadcast(Signal),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerInformation {
    pub public_key: [u8; 32],
    pub address: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Signal {
    pub signal_id: [u8; 16],
    pub sender: [u8; 32],
    pub receiver: [u8; 32],
    pub encrypted_data: EncryptedData,
    pub signature: Signature,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptedData {
    pub nonce: [u8; 32],
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature(pub [u8; 64]);

impl serde::Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = <Vec<u8>>::deserialize(deserializer)?;
        if bytes.len() != 64 {
            return Err(serde::de::Error::invalid_length(
                bytes.len(),
                &"expected an array of length 64",
            ));
        }
        let mut array = [0u8; 64];
        array.copy_from_slice(&bytes);
        Ok(Self(array))
    }
}

#[async_trait::async_trait]
impl super::frame::Frame for Message {
    async fn read_framed(
        reader: &mut (impl AsyncReadExt + Unpin + Sync + Send),
    ) -> crate::error::Result<Vec<u8>> {
        let mut length_bytes = [0u8; 4];
        reader
            .read_exact(&mut length_bytes)
            .await
            .map_err(|_| crate::error::BonkError::MalformedFrame)?;

        let length = u32::from_be_bytes(length_bytes);

        let mut data = vec![0u8; length as usize];
        reader
            .read_exact(&mut data)
            .await
            .map_err(|_| crate::error::BonkError::MalformedFrame)?;

        Ok(data)
    }

    async fn write_framed(
        writer: &mut (impl AsyncWriteExt + Unpin + Sync + Send),
        data: &[u8],
    ) -> crate::error::Result<()> {
        let length = data.len() as u32;
        let length_bytes = length.to_be_bytes();

        writer
            .write(&length_bytes)
            .await
            .map_err(|_| crate::error::BonkError::WriterFailure)?;
        
        writer
            .write(data)
            .await
            .map_err(|_| crate::error::BonkError::WriterFailure)?;

        Ok(())
    }
}
