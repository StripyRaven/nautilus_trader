// crates/adapters/nq/src/gateway_client.rs

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::info;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Gateway error: {0}")]
    GatewayError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Connect {
        quik_path: String,
    },
    Disconnect,
    SendSyncTransaction {
        tx_string: String,
    },
    SendAsyncTransaction {
        tx_string: String,
        trans_id: u32,
    },
    SubscribeOrders {
        class_code: String,
        sec_code: String,
    },
    Shutdown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Ok,
    Error {
        message: String,
    },
    Receipt {
        reply_code: i32,
        trans_id: u32,
        order_num: u64,
        message: Option<String>,
    },
    TransactionId {
        trans_id: u32,
    },
    OrderEvent {
        trans_id: u32,
        order_num: u64,
        class_code: String,
        sec_code: String,
        price: f64,
        balance: i64,
        is_sell: bool,
        status: i32,
    },
}

pub struct GatewayClient {
    stream: TcpStream,
}

impl GatewayClient {
    pub async fn connect(addr: &str) -> Result<Self, GatewayError> {
        info!("Connecting to nq-gateway at {}", addr);
        let stream = TcpStream::connect(addr)
            .await
            .map_err(|e| GatewayError::ConnectionFailed(e.to_string()))?;

        // В Tokio таймауты реализуются через tokio::time::timeout на уровне вызовов read/write,
        // а не через свойства стрима. Для скелета оставляем как есть.

        Ok(Self { stream })
    }

    pub async fn send_request(&mut self, request: &Request) -> Result<Response, GatewayError> {
        let req_bytes = rmp_serde::to_vec(request)
            .map_err(|e| GatewayError::SerializationError(e.to_string()))?;

        self.stream
            .write_all(&req_bytes)
            .await
            .map_err(|e| GatewayError::ConnectionFailed(e.to_string()))?;

        let mut resp_bytes = vec![0u8; 4096];
        let n = self
            .stream
            .read(&mut resp_bytes)
            .await
            .map_err(|e| GatewayError::ConnectionFailed(e.to_string()))?;

        let response: Response = rmp_serde::from_slice(&resp_bytes[..n])
            .map_err(|e| GatewayError::SerializationError(e.to_string()))?;

        if let Response::Error { message } = &response {
            return Err(GatewayError::GatewayError(message.clone()));
        }

        Ok(response)
    }
}
