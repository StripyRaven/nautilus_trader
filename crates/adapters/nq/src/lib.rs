// crates/adapters/nq/src/lib.rs

pub mod execution;
pub mod data;
pub mod gateway_client;

// Публичный экспорт для удобства
pub use execution::QuikExecutionClient;
pub use data::QuikDataClient;
