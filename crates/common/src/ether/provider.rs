//! Create a custom data transport to use with a Provider.
use alloy::{
    network::Ethereum,
    primitives::{Address, TxHash},
    providers::{ext::TraceApi, IpcConnect, Provider, ProviderBuilder, RootProvider, WsConnect},
    pubsub::PubSubFrontend,
    rpc::types::{
        trace::parity::{TraceResults, TraceResultsWithTransactionHash, TraceType},
        Filter, Log, Transaction,
    },
    transports::http::Http,
};
use eyre::Result;
use reqwest::{Client, Url};
use std::{fmt::Debug, str::FromStr};

/// [`MultiTransportProvider`] is a convenience wrapper around the different transport types
/// supported by the [`Provider`].
#[derive(Clone, Debug)]
pub enum MultiTransportProvider {
    /// WebSocket transport
    Ws(RootProvider<PubSubFrontend, Ethereum>),
    /// IPC transport
    Ipc(RootProvider<PubSubFrontend, Ethereum>),
    /// HTTP transport
    Http(RootProvider<Http<Client>, Ethereum>),
}

// We implement a convenience "constructor" method, to easily initialize the transport.
// This will connect to [`Http`] if the rpc_url contains 'http', to [`Ws`] if it contains 'ws',
// otherwise it'll default to [`Ipc`].
impl MultiTransportProvider {
    /// Connect to a provider using the given rpc_url.
    pub async fn connect(rpc_url: &str) -> Result<Self> {
        if rpc_url.is_empty() {
            return Err(eyre::eyre!("No RPC URL provided"));
        }

        let this = if rpc_url.to_lowercase().contains("http") {
            let url = Url::from_str(rpc_url)?;
            Self::Http(ProviderBuilder::new().on_http(url))
        } else if rpc_url.to_lowercase().contains("ws") {
            let ws = WsConnect::new(rpc_url);
            Self::Ws(ProviderBuilder::new().on_ws(ws).await?)
        } else {
            let ipc = IpcConnect::new(rpc_url.to_string());
            Self::Ipc(ProviderBuilder::new().on_ipc(ipc).await?)
        };
        Ok(this)
    }

    /// Get the chain id.
    pub async fn get_chainid(&self) -> Result<u64> {
        Ok(match self {
            Self::Ws(provider) => provider.get_chain_id().await?,
            Self::Ipc(provider) => provider.get_chain_id().await?,
            Self::Http(provider) => provider.get_chain_id().await?,
        })
    }

    /// Get the latest block number.
    pub async fn get_block_number(&self) -> Result<u64> {
        Ok(match self {
            Self::Ws(provider) => provider.get_block_number().await?,
            Self::Ipc(provider) => provider.get_block_number().await?,
            Self::Http(provider) => provider.get_block_number().await?,
        })
    }

    /// Get the bytecode at the given address.
    pub async fn get_code_at(&self, address: Address) -> Result<Vec<u8>> {
        Ok(match self {
            Self::Ws(provider) => provider.get_code_at(address).await?,
            Self::Ipc(provider) => provider.get_code_at(address).await?,
            Self::Http(provider) => provider.get_code_at(address).await?,
        }
        .to_vec())
    }

    /// Get the transaction by hash.
    pub async fn get_transaction_by_hash(&self, tx_hash: TxHash) -> Result<Option<Transaction>> {
        Ok(match self {
            Self::Ws(provider) => provider.get_transaction_by_hash(tx_hash).await?,
            Self::Ipc(provider) => provider.get_transaction_by_hash(tx_hash).await?,
            Self::Http(provider) => provider.get_transaction_by_hash(tx_hash).await?,
        })
    }

    /// Replays the transaction at the given hash.
    /// The `trace_type` parameter is a list of the types of traces to return.
    pub async fn trace_replay_transaction(
        &self,
        tx_hash: &str,
        trace_type: &[TraceType],
    ) -> Result<TraceResults> {
        let tx_hash = tx_hash.parse()?;

        Ok(match self {
            Self::Ws(provider) => provider.trace_replay_transaction(tx_hash, trace_type).await?,
            Self::Ipc(provider) => provider.trace_replay_transaction(tx_hash, trace_type).await?,
            Self::Http(provider) => provider.trace_replay_transaction(tx_hash, trace_type).await?,
        })
    }

    /// Replays the block at the given number.
    /// The `trace_type` parameter is a list of the types of traces to return.
    pub async fn trace_replay_block_transactions(
        &self,
        block_number: u64,
        trace_type: &[TraceType],
    ) -> Result<Vec<TraceResultsWithTransactionHash>> {
        let block_number = block_number.into();

        Ok(match self {
            Self::Ws(provider) => {
                provider.trace_replay_block_transactions(block_number, trace_type).await?
            }
            Self::Ipc(provider) => {
                provider.trace_replay_block_transactions(block_number, trace_type).await?
            }
            Self::Http(provider) => {
                provider.trace_replay_block_transactions(block_number, trace_type).await?
            }
        })
    }

    /// Get the logs that match the given filter.
    pub async fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>> {
        Ok(match self {
            Self::Ws(provider) => provider.get_logs(filter).await?,
            Self::Ipc(provider) => provider.get_logs(filter).await?,
            Self::Http(provider) => provider.get_logs(filter).await?,
        })
    }
}
