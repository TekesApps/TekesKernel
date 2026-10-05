use std::sync::Arc;

use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccessLogRecord {
    pub v: u64,
    pub request_id: String,
    pub operation: String,
    pub path: String,
    pub status: u16,
    pub elapsed_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl AccessLogRecord {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, String> {
        serde_json_canonicalizer::to_vec(self).map_err(|error| error.to_string())
    }
}

pub trait AccessLogSink: Send + Sync + 'static {
    fn record(&self, record: &AccessLogRecord);
}

#[derive(Default)]
pub struct NoopAccessLog;

impl AccessLogSink for NoopAccessLog {
    fn record(&self, _record: &AccessLogRecord) {}
}

pub(crate) fn noop() -> Arc<dyn AccessLogSink> {
    Arc::new(NoopAccessLog)
}
