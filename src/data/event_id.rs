use std::fmt::Display;

use anyhow::bail;
use evtx::SerializedEvtxRecord;
use serde_json::Value;

#[derive(PartialEq, Eq, Clone)]
pub struct EventId {
    id: u16,
}

impl TryFrom<&SerializedEvtxRecord<Value>> for EventId {
    type Error = anyhow::Error;

    fn try_from(record: &SerializedEvtxRecord<Value>) -> Result<Self, Self::Error> {
        let event_id = &record.data["Event"]["System"]["EventID"];

        let event_id = match event_id.get("#text") {
            Some(eid) => eid,
            None => event_id,
        };

        if let Some(event_id) = event_id.as_u64() {
            let id: u16 = event_id.try_into()?;
            Ok(Self{id})
        } else {
            bail!("event id cannot be converted to u16: {event_id}")
        }
    }
}

impl From<EventId> for u16 {
    fn from(me: EventId) -> Self {
        me.id
    }
}

impl From<u16> for EventId {
    fn from(id: u16) -> Self {
        Self {
            id
        }
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}