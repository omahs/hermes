//! These functions bind the `one_for_all` trait to the `all_for_one`. Specifically,
//! this is where we 'prove' that the types that we want to implement the `all_for_one`
//! functionality implement the required trait bounds, i.e. `OfaRelayWrapper` or
//! `OfaChainWrapper`, etc.

use crate::base::all_for_one::chain::AfoBaseChain;
use crate::base::all_for_one::error::AfoError;
use crate::base::all_for_one::relay::AfoBaseRelay;
use crate::base::one_for_all::traits::chain::{OfaBaseChain, OfaChainWrapper, OfaIbcChain};
use crate::base::one_for_all::traits::components::chain::OfaIbcChainComponents;
use crate::base::one_for_all::traits::components::relay::OfaBaseRelayComponents;
use crate::base::one_for_all::traits::error::OfaError;
use crate::base::one_for_all::traits::error::OfaErrorContext;
use crate::base::one_for_all::traits::relay::{OfaBaseRelay, OfaRelayWrapper};

/// Given a relay context `Relay` that implements the `OfaBaseRelay` trait, returns a type
/// that implements the `AfoBaseRelay`, meaning that this type exposes concrete APIs
/// that are used to construct custom relayer instances (i.e. relayer-cosmos).
pub fn afo_relay_context<Relay>(relay: OfaRelayWrapper<Relay>) -> impl AfoBaseRelay
where
    Relay: OfaBaseRelay,
    Relay::Components: OfaBaseRelayComponents<Relay>,
{
    relay
}

/// Given a chain context `Chain` that implements the `OfaIbcChain` trait, returns a type
/// that implements the `AfoBaseChain`, which is necessary for a relay context that
/// wants to relay between this IBC-enabled chain context and an IBC-enabled counterparty
/// chain context can do so.
pub fn afo_chain_context<Chain, Counterparty, Components>(
    chain: OfaChainWrapper<Chain>,
) -> impl AfoBaseChain<OfaChainWrapper<Counterparty>>
where
    Chain: OfaBaseChain<Components = Components>,
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaIbcChain<Chain>,
    Components: OfaIbcChainComponents<Chain, Counterparty>,
{
    chain
}

pub fn afo_error<Error>(error: OfaErrorContext<Error>) -> impl AfoError
where
    Error: OfaError,
{
    error
}
