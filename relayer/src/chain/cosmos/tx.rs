use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_proto::google::protobuf::Any;
use tendermint::abci::responses::Event;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response;
use tendermint_rpc::{Client, HttpClient, Url};

use crate::chain::cosmos::encode::sign_and_encode_tx;
use crate::chain::cosmos::estimate::estimate_tx_fees;
use crate::chain::cosmos::event::split_events_by_messages;
use crate::chain::cosmos::query::account::query_account;
use crate::chain::cosmos::types::account::Account;
use crate::chain::cosmos::types::config::TxConfig;
use crate::chain::cosmos::wait::wait_tx_succeed;
use crate::config::types::Memo;
use crate::error::Error;
use crate::keyring::KeyEntry;

pub async fn estimate_fee_and_send_tx(
    config: &TxConfig,
    key_entry: &KeyEntry,
    account: &Account,
    tx_memo: &Memo,
    messages: Vec<Any>,
) -> Result<Response, Error> {
    let fee = estimate_tx_fees(config, key_entry, account, tx_memo, messages.clone()).await?;

    send_tx_with_fee(config, key_entry, account, tx_memo, messages, &fee).await
}

async fn send_tx_with_fee(
    config: &TxConfig,
    key_entry: &KeyEntry,
    account: &Account,
    tx_memo: &Memo,
    messages: Vec<Any>,
    fee: &Fee,
) -> Result<Response, Error> {
    let tx_bytes = sign_and_encode_tx(config, key_entry, account, tx_memo, messages, fee)?;

    let response = broadcast_tx_sync(&config.rpc_client, &config.rpc_address, tx_bytes).await?;

    Ok(response)
}

/// Perform a `broadcast_tx_sync`, and return the corresponding deserialized response data.
async fn broadcast_tx_sync(
    rpc_client: &HttpClient,
    rpc_address: &Url,
    data: Vec<u8>,
) -> Result<Response, Error> {
    let response = rpc_client
        .broadcast_tx_sync(data.into())
        .await
        .map_err(|e| Error::rpc(rpc_address.clone(), e))?;

    Ok(response)
}

/**
 A simplified version of send_tx that does not depend on `ChainHandle`.

 This allows different wallet ([`KeyEntry`]) to be used for submitting
 transactions. The simple behavior as follows:

 - Query the account information on the fly. This may introduce more
   overhead in production, but does not matter in testing.
 - Do not split the provided messages into smaller batches.
 - Wait for TX sync result, and error if any result contains
   error event.
*/
pub async fn simple_send_tx(
    config: &TxConfig,
    key_entry: &KeyEntry,
    messages: Vec<Any>,
) -> Result<Vec<Vec<Event>>, Error> {
    let account = query_account(&config.grpc_address, &key_entry.account)
        .await?
        .into();

    let response =
        estimate_fee_and_send_tx(config, key_entry, &account, &Default::default(), messages)
            .await?;

    if response.code.is_err() {
        return Err(Error::check_tx(response));
    }

    let response = wait_tx_succeed(
        &config.rpc_client,
        &config.rpc_address,
        &config.rpc_timeout,
        &response.hash,
    )
    .await?;

    let events = split_events_by_messages(response.tx_result.events);

    Ok(events)
}