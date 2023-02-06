use crate::data::ActivityId;
use evtx::SerializedEvtxRecord;
use serde_json::Value;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SessionId(String);

impl From<&str> for SessionId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for SessionId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

pub trait SessionIdGenerator {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId;
}

pub struct SessionNameInEventData {}
impl SessionIdGenerator for SessionNameInEventData {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        record.data["Event"]["EventData"]["SessionName"]
            .as_str()
            .expect("missing SessionName in event")
            .into()
    }
}
pub struct SessionNameInActivityId {}
impl SessionIdGenerator for SessionNameInActivityId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        let activity_id = ActivityId::try_from(record)
            .expect("missing activity id in event")
            .value()
            .to_string();
        SessionId(activity_id)
    }
}

pub struct SessionNameInTargetLogonId {}
impl SessionIdGenerator for SessionNameInTargetLogonId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        record.data["Event"]["EventData"]["TargetLogonId"]
            .as_str()
            .expect("missing TargetLogonId in event")
            .into()
    }
}

pub struct SessionNameInLogonId {}
impl SessionIdGenerator for SessionNameInLogonId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        record.data["Event"]["EventData"]["LogonId"]
            .as_str()
            .expect("missing LogonId in event")
            .into()
    }
}
