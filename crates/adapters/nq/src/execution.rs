// crates/adapters/nq/src/execution.rs

use crate::gateway_client::GatewayClient;
use anyhow::Result;
use async_trait::async_trait;
use nautilus_common::{
    clients::ExecutionClient,
    messages::execution::{CancelOrder, ModifyOrder, SubmitOrder},
};
use nautilus_core::{UnixNanos, params::Params};
use nautilus_model::{
    accounts::AccountAny,
    enums::OmsType,
    identifiers::{AccountId, ClientId, TraderId, Venue},
    types::{AccountBalance, MarginBalance},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

#[allow(dead_code)]
pub struct QuikExecutionClient {
    trader_id: TraderId,
    venue: Venue,
    account_id: AccountId,
    client_id: ClientId,
    gateway: Arc<Mutex<GatewayClient>>,
    connected: bool,
}

impl QuikExecutionClient {
    pub async fn create(
        trader_id: TraderId,
        venue: Venue,
        account_id: AccountId,
        client_id: ClientId,
        gateway_addr: String,
    ) -> Result<Self> {
        let gateway = GatewayClient::connect(&gateway_addr).await?;
        Ok(Self {
            trader_id,
            venue,
            account_id,
            client_id,
            gateway: Arc::new(Mutex::new(gateway)),
            connected: true,
        })
    }
}

#[async_trait(?Send)]
impl ExecutionClient for QuikExecutionClient {
    fn is_connected(&self) -> bool {
        self.connected
    }

    fn client_id(&self) -> ClientId {
        self.client_id
    }

    fn account_id(&self) -> AccountId {
        self.account_id
    }

    fn venue(&self) -> Venue {
        self.venue
    }

    fn oms_type(&self) -> OmsType {
        OmsType::Netting
    }

    fn get_account(&self) -> Option<AccountAny> {
        None
    }

    fn generate_account_state(
        &self,
        _balances: Vec<AccountBalance>,
        _margins: Vec<MarginBalance>,
        _reported: bool,
        _ts_event: UnixNanos,
        _info: Option<Params>,
    ) -> Result<()> {
        warn!("generate_account_state not yet implemented for QUIK");
        Ok(())
    }

    fn start(&mut self) -> Result<()> {
        info!("Starting QUIK execution client");
        self.connected = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        info!("Stopping QUIK execution client");
        self.connected = false;
        Ok(())
    }

    async fn connect(&mut self) -> Result<()> {
        info!("Connecting QUIK execution client");
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        info!("Disconnecting QUIK execution client");
        self.connected = false;
        Ok(())
    }

    fn submit_order(&self, command: SubmitOrder) -> Result<()> {
        info!("Submitting order to QUIK: {:?}", command.client_order_id);
        warn!("submit_order implementation pending - requires Order to QUIK TX mapping");
        Ok(())
    }

    fn modify_order(&self, _command: ModifyOrder) -> Result<()> {
        warn!("modify_order not yet implemented for QUIK");
        Ok(())
    }

    fn cancel_order(&self, _command: CancelOrder) -> Result<()> {
        warn!("cancel_order not yet implemented for QUIK");
        Ok(())
    }
}
