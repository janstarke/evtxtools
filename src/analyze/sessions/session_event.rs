use crate::data::EventId;
use evtx::SerializedEvtxRecord;
use serde_json::Value;

use super::{
    EventProvider, SessionEventError, SessionId, SessionIdGenerator, SessionNameInActivityId,
    SessionNameInLogonId, SessionNameInTargetLogonId,
};

pub trait SessionEventInfo {
    fn event_id(&self) -> EventId;
    fn description(&self) -> &'static str;
    fn provider(&self) -> EventProvider;
    fn generate_id(&self, record: &SerializedEvtxRecord<Value>) -> SessionId;
}

macro_rules! session_event {
    ($name: ident, $provider: expr, $event_id: expr, $description: expr, $generator: ident) => {
        pub struct $name();

        impl Default for $name {
            fn default() -> Self {
                Self()
            }
        }
        impl SessionEventInfo for $name {
            fn event_id(&self) -> EventId {
                $event_id.into()
            }
            fn description(&self) -> &'static str {
                $description
            }
            fn provider(&self) -> EventProvider {
                $provider
            }
            fn generate_id(&self, record: &SerializedEvtxRecord<Value>) -> SessionId {
                $generator::session_id_of(record)
            }
        }
    };
}

session_event!(
    TSRCMUserAuthenticationSucceeded,
    EventProvider::TerminalServicesRemoteConnectionManager,
    1149,
    "User authentication succeeded",
    SessionNameInActivityId
);

session_event!(
    TSLCMSessionLogonSucceeded,
    EventProvider::TerminalServicesLocalConnectionManager,
    21,
    "Remote Desktop Services: Session logon succeeded",
    SessionNameInActivityId
);

session_event!(
    TSLCMShellStartNotificationReceived,
    EventProvider::TerminalServicesLocalConnectionManager,
    22,
    "Remote Desktop Services: Shell start notification received",
    SessionNameInActivityId
);

session_event!(
    TSLCMSessionLogoffSucceeded,
    EventProvider::TerminalServicesLocalConnectionManager,
    23,
    "Remote Desktop Services: Session logoff succeeded",
    SessionNameInActivityId
);

session_event!(
    TSLCMSessionHasBeenDisconnected,
    EventProvider::TerminalServicesLocalConnectionManager,
    24,
    "Remote Desktop Services: Session has been disconnected",
    SessionNameInActivityId
);

session_event!(
    TSLCMSessionReconnectionSucceeded,
    EventProvider::TerminalServicesLocalConnectionManager,
    25,
    "Remote Desktop Services: Session reconnection succeeded",
    SessionNameInActivityId
);

session_event!(
    TSLCMSessionXHasBeenDisconnectedBySessionY,
    EventProvider::TerminalServicesLocalConnectionManager,
    39,
    "Session <X> has been disconnected by session <Y>",
    SessionNameInActivityId
);
session_event!(
    TSLCMSessionXHasBeenDisconnectedReasonCodeZ,
    EventProvider::TerminalServicesLocalConnectionManager,
    40,
    "Session <X> has been disconnected, reason code <Z>",
    SessionNameInActivityId
);

session_event!(
    SecuritySuccessfulLogin,
    EventProvider::SecurityAuditing,
    4624,
    "An account was successfully logged on",
    SessionNameInTargetLogonId
);
session_event!(
    SecurityFailedLogin,
    EventProvider::SecurityAuditing,
    4625,
    "An account failed to log on",
    SessionNameInActivityId
);

session_event!(
    SecuritySuccessfulLogoff,
    EventProvider::SecurityAuditing,
    4634,
    "An account was successfully logged off",
    SessionNameInTargetLogonId
);

session_event!(
    SecurityUserInitiatedLogoff,
    EventProvider::SecurityAuditing,
    4647,
    "User initiated logoff",
    SessionNameInTargetLogonId
);

session_event!(
    SecuritySessionWasReconnected,
    EventProvider::SecurityAuditing,
    4778,
    "A session was reconnected to a Window Station",
    SessionNameInLogonId
);
session_event!(
    SecuritySessionWasDisconnected,
    EventProvider::SecurityAuditing,
    4779,
    "A session was disconnected from a Window Station.",
    SessionNameInLogonId
);

pub struct SessionEvent {
    event_type: Box<dyn SessionEventInfo>,
    record: SerializedEvtxRecord<serde_json::Value>,
    session_id: SessionId,
}

impl SessionEvent {
    fn new<I>(record: SerializedEvtxRecord<serde_json::Value>) -> Self
    where
        I: SessionEventInfo + Default + 'static,
    {
        let event_type = Box::<I>::default();
        let session_id = event_type.generate_id(&record);
        Self {
            event_type,
            record,
            session_id,
        }
    }

    pub fn record(&self) -> &SerializedEvtxRecord<Value> {
        &self.record
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }
}

impl TryFrom<SerializedEvtxRecord<serde_json::Value>> for SessionEvent {
    type Error = SessionEventError;

    fn try_from(record: SerializedEvtxRecord<serde_json::Value>) -> Result<Self, Self::Error> {
        let event_id = EventId::try_from(&record)?;
        let provider = EventProvider::try_from(&record)?;
        let event = match provider {
            EventProvider::TerminalServicesRemoteConnectionManager => match event_id.value() {
                1149 => Self::new::<TSRCMUserAuthenticationSucceeded>(record),
                _ => return Err(SessionEventError::NoSessionEvent),
            },
            EventProvider::TerminalServicesLocalConnectionManager => match event_id.value() {
                21 => Self::new::<TSLCMSessionLogonSucceeded>(record),
                22 => Self::new::<TSLCMShellStartNotificationReceived>(record),
                23 => Self::new::<TSLCMSessionLogoffSucceeded>(record),
                24 => Self::new::<TSLCMSessionHasBeenDisconnected>(record),
                25 => Self::new::<TSLCMSessionReconnectionSucceeded>(record),
                39 => Self::new::<TSLCMSessionXHasBeenDisconnectedBySessionY>(record),
                40 => Self::new::<TSLCMSessionXHasBeenDisconnectedReasonCodeZ>(record),
                _ => return Err(SessionEventError::NoSessionEvent),
            },
            EventProvider::SecurityAuditing => match event_id.value() {
                4624 => Self::new::<SecuritySuccessfulLogin>(record),
                4625 => Self::new::<SecurityFailedLogin>(record),
                4634 => Self::new::<SecuritySuccessfulLogoff>(record),
                4647 => Self::new::<SecurityUserInitiatedLogoff>(record),
                4778 => Self::new::<SecuritySessionWasReconnected>(record),
                4779 => Self::new::<SecuritySessionWasDisconnected>(record),
                _ => return Err(SessionEventError::NoSessionEvent),
            },
            _ => {
                log::error!("unknown event provider: {provider}");
                return Err(SessionEventError::NoSessionEvent)
            }
        };

        assert_eq!(&event_id, &event.event_type.event_id());
        assert_eq!(&provider, &event.event_type.provider());

        Ok(event)
    }
}

impl Ord for SessionEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.record.timestamp.cmp(&other.record.timestamp)
    }
}

impl PartialOrd for SessionEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SessionEvent {}

impl PartialEq for SessionEvent {
    fn eq(&self, other: &Self) -> bool {
        self.record.timestamp.eq(&other.record.timestamp)
    }
}
