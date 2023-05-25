use clap::ValueEnum;
use eventdata::{EvtxFieldView, EventId, EventRecordId, ActivityId, RelatedActivityId};
use evtx::SerializedEvtxRecord;
use serde_json::Value;

#[derive(ValueEnum, Clone)]
#[allow(clippy::enum_variant_names)]
pub (crate) enum SystemField {
    /// The identifier that the provider used to identify the event
    EventId,

    /// The record number assigned to the event when it was logged
    EventRecordId,

    /// A globally unique identifier that identifies the current activity. The events that are published with this identifier are part of the same activity.
    ActivityId,

    /// A globally unique identifier that identifies the activity to which control was transferred to. The related events would then have this identifier as their ActivityID identifier.
    RelatedActivityId,
}

pub (crate) trait FilterBySystemField {
    fn filter_fields<'a>(record: &'a Self, fields: &[SystemField], ) -> anyhow::Result<Vec<Box<dyn EvtxFieldView + 'a>>>;
}

impl FilterBySystemField for SerializedEvtxRecord<Value> {
    fn filter_fields<'a>(record: &'a Self, fields: &[SystemField], ) -> anyhow::Result<Vec<Box<dyn EvtxFieldView + 'a>>> {
        let mut result: Vec<Box<dyn EvtxFieldView>> = Vec::with_capacity(fields.len());
        for field in fields {
            match field {
                SystemField::EventId => result.push(Box::new(EventId::try_from(record)?)),
                SystemField::EventRecordId => result.push(Box::new(EventRecordId::from(record))),
                SystemField::ActivityId => result.push(Box::new(ActivityId::try_from(record)?)),
                SystemField::RelatedActivityId => result.push(Box::new(RelatedActivityId::try_from(record)?)),
            }
        }

        Ok(result)
    }
}