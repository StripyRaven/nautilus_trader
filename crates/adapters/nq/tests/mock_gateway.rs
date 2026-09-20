// crates/adapters/nq/tests/mock_gateway.rs

use nautilus_nq::gateway_client::{Request, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct MockGateway {
    addr: String,
    shutdown_tx: tokio::sync::oneshot::Sender<()>,
}

impl MockGateway {
    pub async fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();

        let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = listener.accept() => {
                        if let Ok((mut stream, _)) = result {
                            tokio::spawn(async move {
                                let mut buf = vec![0u8; 4096];
                                loop {
                                    match stream.read(&mut buf).await {
                                        Ok(0) => break,
                                        Ok(n) => {
                                            if let Ok(request) = rmp_serde::from_slice::<Request>(&buf[..n]) {
                                                let response = match request {
                                                    Request::Connect { .. } => Response::Ok,
                                                    Request::Disconnect => Response::Ok,
                                                    Request::SendAsyncTransaction { trans_id, .. } => {
                                                        Response::Receipt {
                                                            reply_code: 0,
                                                            trans_id,
                                                            order_num: 12345,
                                                            message: Some("Mock order accepted".to_string()),
                                                        }
                                                    }
                                                    Request::SendSyncTransaction { .. } => {
                                                        Response::Receipt {
                                                            reply_code: 0,
                                                            trans_id: 1,
                                                            order_num: 12345,
                                                            message: Some("Mock sync order accepted".to_string()),
                                                        }
                                                    }
                                                    _ => Response::Ok,
                                                };

                                                if let Ok(bytes) = rmp_serde::to_vec(&response) {
                                                    let _ = stream.write_all(&bytes).await;
                                                }
                                            }
                                        }
                                        Err(_) => break,
                                    }
                                }
                            });
                        }
                    }
                    _ = &mut shutdown_rx => {
                        break;
                    }
                }
            }
        });

        Self { addr, shutdown_tx }
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }

    pub fn shutdown(self) {
        let _ = self.shutdown_tx.send(());
    }
}
