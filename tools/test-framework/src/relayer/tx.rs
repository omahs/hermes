use core::str::FromStr;
use core::time::Duration;
use http::uri::Uri;
use ibc::core::ics24_host::identifier::ChainId;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::gas::calculate_fee;
use ibc_relayer::chain::cosmos::types::config::TxConfig;
use ibc_relayer::chain::cosmos::types::gas::GasConfig;
use ibc_relayer::config::GasPrice;
use tendermint_rpc::{HttpClient, Url};

use crate::error::{handle_generic_error, Error};

pub fn gas_config_for_test() -> GasConfig {
    let max_gas = 3000000;
    let gas_multiplier = 1.1;
    let gas_price = GasPrice::new(0.001, "stake".to_string());

    let default_gas = max_gas;
    let fee_granter = "".to_string();

    let max_fee = Fee {
        amount: vec![calculate_fee(max_gas, &gas_price)],
        gas_limit: max_gas,
        payer: "".to_string(),
        granter: fee_granter.clone(),
    };

    GasConfig {
        default_gas,
        max_gas,
        gas_multiplier,
        gas_price,
        max_fee,
        fee_granter,
    }
}

pub fn new_tx_config_for_test(
    chain_id: ChainId,
    raw_rpc_address: String,
    raw_grpc_address: String,
) -> Result<TxConfig, Error> {
    let rpc_address = Url::from_str(&raw_rpc_address).map_err(handle_generic_error)?;

    let rpc_client = HttpClient::new(rpc_address.clone()).map_err(handle_generic_error)?;

    let grpc_address = Uri::from_str(&raw_grpc_address).map_err(handle_generic_error)?;

    let gas_config = gas_config_for_test();

    let rpc_timeout = Duration::from_secs(30);

    let address_type = Default::default();

    Ok(TxConfig {
        chain_id,
        gas_config,
        rpc_client,
        rpc_address,
        grpc_address,
        rpc_timeout,
        address_type,
    })
}