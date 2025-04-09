use crate::error::*;
use futures::{AsyncReadExt, AsyncWriteExt};

#[async_trait::async_trait]
pub trait Frame {
    async fn read_framed(reader: &mut (impl AsyncReadExt + Unpin + Sync + Send)) -> Result<Vec<u8>>;

    async fn write_framed(writer: &mut (impl AsyncWriteExt + Unpin + Sync + Send), data: &[u8]) -> Result<()>;
}
