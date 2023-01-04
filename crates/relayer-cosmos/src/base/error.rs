use eyre::Report;
use flex_error::{define_error, DisplayOnly, ErrorMessageTracer, TraceError};
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::foreign_client::ForeignClientError;
use ibc_relayer_runtime::tokio::error::Error as TokioError;
use ibc_relayer_types::core::ics04_channel::error::Error as ChannelError;
use prost::EncodeError;
use tendermint::Hash as TxHash;

define_error! {
    #[derive(Clone, Debug)]
    Error {
        Generic
            [ TraceError<Report> ]
            | _ | { "generic error" },

        Tokio
            [ TokioError ]
            | _ | { "tokio runtime error" },

        Channel
            [ DisplayOnly<ChannelError> ]
            | _ | { "channel error" },

        Relayer
            [ DisplayOnly<RelayerError> ]
            | _ | { "ibc-relayer error" },

        ForeignClient
            [ DisplayOnly<ForeignClientError> ]
            | _ | { "foreign client error" },

        Encode
            [ TraceError<EncodeError> ]
            | _ | { "protobuf encode error" },

        MismatchConsensusState
            | _ | { "consensus state of a cosmos chain on the counterparty chain must be a tendermint consensus state" },

        MismatchEventType
            { expected: String, actual: String }
            | e | { format_args!("mismatch event type, expected: {}, actual: {}", e.expected, e.actual) },

        TxNoResponse
            { tx_hash: TxHash }
            | e | { format_args!("failed to receive tx response for tx hash: {}", e.tx_hash) },

        MissingSimulateGasInfo
            | _ | { "missing gas info returned from send_tx_simulate" },
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        Error(self.detail().clone(), ErrorMessageTracer::new_message(self))
    }
}