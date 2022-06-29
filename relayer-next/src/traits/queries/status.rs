use async_trait::async_trait;

use crate::traits::chain_context::ChainContext;
use crate::types::aliases::{Height, Timestamp};

pub trait ChainStatus<Chain: ChainContext> {
    fn height(&self) -> Height<Chain>;

    fn timestamp(&self) -> Timestamp<Chain>;
}

#[async_trait]
pub trait ChainStatusQuerier<Chain>
where
    Chain: ChainContext,
{
    type ChainStatus: ChainStatus<Chain>;

    async fn query_chain_status(&self) -> Result<Self::ChainStatus, Chain::Error>;
}