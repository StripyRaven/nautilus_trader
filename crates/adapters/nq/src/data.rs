// crates/adapters/nq/src/data.rs

use anyhow::Result;
use async_trait::async_trait;
use nautilus_common::{
    clients::DataClient,
    messages::data::{
        SubscribeInstruments, SubscribeQuotes, SubscribeTrades, UnsubscribeQuotes, UnsubscribeTrades,
    },
};
use nautilus_model::identifiers::{ClientId, Venue};
use tracing::{info, warn};

#[allow(dead_code)]
pub struct QuikDataClient {
    client_id: ClientId,
    venue: Venue,
    connected: bool,
}

impl QuikDataClient {
    pub fn new(client_id: ClientId, venue: Venue) -> Self {
        Self {
            client_id,
            venue,
            connected: false,
        }
    }
}

#[async_trait(?Send)]
impl DataClient for QuikDataClient {
    fn client_id(&self) -> ClientId {
        self.client_id
    }

    fn venue(&self) -> Option<Venue> {
        Some(self.venue)
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn is_disconnected(&self) -> bool {
        !self.connected
    }

    fn start(&mut self) -> Result<()> {
        info!("Starting QUIK data client");
        self.connected = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        info!("Stopping QUIK data client");
        self.connected = false;
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        info!("Resetting QUIK data client");
        Ok(())
    }

    fn dispose(&mut self) -> Result<()> {
        info!("Disposing QUIK data client");
        self.connected = false;
        Ok(())
    }

    fn subscribe_quotes(&mut self, _cmd: SubscribeQuotes) -> Result<()> {
        warn!("subscribe_quotes not yet implemented for QUIK");
        Ok(())
    }

    fn subscribe_trades(&mut self, _cmd: SubscribeTrades) -> Result<()> {
        warn!("subscribe_trades not yet implemented for QUIK");
        Ok(())
    }

    fn subscribe_instruments(&mut self, _cmd: SubscribeInstruments) -> Result<()> {
        warn!("subscribe_instruments not yet implemented for QUIK");
        Ok(())
    }

    fn unsubscribe_quotes(&mut self, _cmd: &UnsubscribeQuotes) -> Result<()> {
        Ok(())
    }

    fn unsubscribe_trades(&mut self, _cmd: &UnsubscribeTrades) -> Result<()> {
        Ok(())
    }
}
