use crate::impls::one_for_all::relay::OfaRelayContext;
use crate::impls::packet_relayers::base_ack_packet::BaseAckPacketRelayer;
use crate::impls::packet_relayers::base_receive_packet::BaseReceivePacketRelayer;
use crate::impls::packet_relayers::full_relay::FullRelayer;
use crate::impls::packet_relayers::retry::RetryRelayer;
use crate::impls::packet_relayers::skip_received_packet::SkipReceivedPacketRelayer;
use crate::traits::one_for_all::relay::OfaRelay;
use crate::traits::packet_relayer::PacketRelayer;
use crate::traits::packet_relayers::ack_packet::AckPacketRelayer;
use crate::traits::packet_relayers::receive_packet::ReceivePacketRelayer;

pub fn full_packet_relayer<Relay: OfaRelay>(
    max_retry: usize,
) -> impl PacketRelayer<OfaRelayContext<Relay>> {
    let relayer1 = FullRelayer {
        receive_relayer: receive_packet_relayer(),
        ack_relayer: ack_packet_relayer(),
    };

    RetryRelayer::new(max_retry, relayer1)
}

pub fn receive_packet_relayer<Relay: OfaRelay>() -> impl ReceivePacketRelayer<OfaRelayContext<Relay>>
{
    SkipReceivedPacketRelayer::new(BaseReceivePacketRelayer)
}

pub fn ack_packet_relayer<Relay: OfaRelay>() -> impl AckPacketRelayer<OfaRelayContext<Relay>> {
    BaseAckPacketRelayer
}
