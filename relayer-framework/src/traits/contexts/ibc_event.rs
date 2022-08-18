use crate::traits::contexts::chain::{ChainContext, IbcChainContext};
use crate::traits::core::Async;

pub trait HasIbcEvents<Counterparty>: IbcChainContext<Counterparty>
where
    Counterparty: ChainContext,
{
    type WriteAcknowledgementEvent: Async;

    fn try_extract_write_acknowledgement_event(
        event: Self::IbcEvent,
    ) -> Option<Self::WriteAcknowledgementEvent>;
}
