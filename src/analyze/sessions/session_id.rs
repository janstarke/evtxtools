use crate::data::ActivityId;
use evtx::SerializedEvtxRecord;
use serde::Serialize;
use serde_json::Value;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug, Serialize)]
pub enum SessionId {
    ActivityId(String),
    SessionName(String),
    LogonId(String)
}

pub trait SessionIdGenerator {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId;
}

pub struct SessionNameInEventData {}
impl SessionIdGenerator for SessionNameInEventData {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        SessionId::SessionName(
            record.data["Event"]["EventData"]["SessionName"]
                .as_str()
                .expect("missing SessionName in event")
                .into(),
        )
    }
}

pub struct SessionNameInActivityId {}
impl SessionIdGenerator for SessionNameInActivityId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        let activity_id = ActivityId::try_from(record).expect("missing activity id in event");

        match activity_id.value().as_str() {
            None => {
                SessionId::ActivityId("".into())
            }
            Some(activity_id) => {
                SessionId::ActivityId(activity_id.into())
            }
        }
    }
}

pub struct SessionNameInTargetLogonId {}
impl SessionIdGenerator for SessionNameInTargetLogonId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        SessionId::LogonId(
            record.data["Event"]["EventData"]["TargetLogonId"]
                .as_str()
                .expect("missing TargetLogonId in event")
                .into(),
        )
    }
}

pub struct SessionNameInSubjectLogonId {}
impl SessionIdGenerator for SessionNameInSubjectLogonId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        SessionId::LogonId(
            record.data["Event"]["EventData"]["SubjectLogonId"]
                .as_str()
                .expect("missing SubjectLogonId in event")
                .into(),
        )
    }
}

pub struct SessionNameInLogonId {}
impl SessionIdGenerator for SessionNameInLogonId {
    fn session_id_of(record: &SerializedEvtxRecord<Value>) -> SessionId {
        SessionId::LogonId(
            record.data["Event"]["EventData"]["LogonId"]
                .as_str()
                .expect("missing LogonId in event")
                .into(),
        )
    }
}
