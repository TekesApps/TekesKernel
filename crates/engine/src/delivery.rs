use std::collections::HashMap;

use schema::OriginTuple;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct OriginIdentity {
    principal: String,
    client: String,
    target: String,
    op: String,
    key: String,
}

impl From<&OriginTuple> for OriginIdentity {
    fn from(value: &OriginTuple) -> Self {
        Self {
            principal: value.principal.clone(),
            client: value.client.clone(),
            target: value.target.clone(),
            op: value.op.clone(),
            key: value.key.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryResolution {
    AppendAt(u64),
    Reack { original_seq: u64 },
}

#[derive(Clone, Debug, Default)]
pub struct DeliveryIndex {
    committed: HashMap<OriginIdentity, u64>,
}

impl DeliveryIndex {
    #[must_use]
    pub fn resolve(&self, origin: &OriginTuple, next_seq: u64) -> DeliveryResolution {
        self.committed
            .get(&OriginIdentity::from(origin))
            .map_or(DeliveryResolution::AppendAt(next_seq), |seq| {
                DeliveryResolution::Reack { original_seq: *seq }
            })
    }

    pub fn commit(&mut self, origin: &OriginTuple, seq: u64) {
        self.committed
            .entry(OriginIdentity::from(origin))
            .or_insert(seq);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CreateResolution {
    Create { thread_id: String },
    Existing { thread_id: String },
}

#[derive(Clone, Debug, Default)]
pub struct CreateIndex {
    created: HashMap<OriginIdentity, String>,
}

impl CreateIndex {
    pub fn resolve(
        &mut self,
        origin: &OriginTuple,
        proposed: impl Into<String>,
    ) -> CreateResolution {
        let identity = OriginIdentity::from(origin);
        if let Some(thread_id) = self.created.get(&identity) {
            return CreateResolution::Existing {
                thread_id: thread_id.clone(),
            };
        }
        let thread_id = proposed.into();
        self.created.insert(identity, thread_id.clone());
        CreateResolution::Create { thread_id }
    }
}
