use std::{collections::{BTreeSet, HashSet}, io::Write};

use evtx::SerializedEvtxRecord;
use serde_json::Value;

use super::{SessionEvent, SessionId, SessionAsJson};

pub struct Session {
    events: BTreeSet<SessionEvent>,
    session_id: SessionId,
    usernames: HashSet<String>,
}

impl Session{
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn add_event(&mut self, event: SessionEvent) {
        assert_eq!(event.session_id(), &self.session_id);
        if let Some(username) = Self::username_of(event.record()) {
            self.usernames.insert(username);
        }
        self.events.insert(event);
    }

    pub fn first_event(&self) -> &SessionEvent {
        debug_assert!(! self.events.is_empty());
        self.events.first().unwrap()
    }

    pub fn last_event(&self) -> &SessionEvent {
        debug_assert!(! self.events.is_empty());
        self.events.last().unwrap()
    }

    pub fn into_markdown(self) -> String {
        unimplemented!()
    }

    pub fn into_json<W>(self, writer: &mut W) -> serde_json::Result<()> where W: Write{
        serde_json::to_writer(writer, &Into::<SessionAsJson>::into(self))
    }

    pub fn into_latex(self) -> String {
        unimplemented!()
    }

    pub fn into_dot(self) -> String {
        unimplemented!()
    }

    fn username_of(record: &SerializedEvtxRecord<Value>) -> Option<String> {
        record.data["Event"]["EventData"]["TargetUserName"].as_str().map(|u| u.into())
    }
}

impl From<SessionEvent> for Session{
    fn from(value: SessionEvent) -> Self {
        log::trace!("creating new session, starting at {}", value.record().timestamp);

        let mut events = BTreeSet::<SessionEvent>::new();
        let session_id = (*value.session_id()).clone();

        let mut usernames = HashSet::new();
        if let Some(username) = Self::username_of(value.record()) {
            usernames.insert(username);
        }
        events.insert(value);
        Self {
            events,
            session_id,
            usernames
        }
    }
}

impl Ord for Session {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first_event().cmp(other.first_event())
    }
}

impl PartialOrd for Session {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Session {
    
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.first_event().eq(other.first_event())
    }
}

#[allow(clippy::from_over_into)]
impl Into<SessionAsJson> for Session {
    fn into(self) -> SessionAsJson {
        let begin = self.first_event().record().timestamp;
        let end = self.last_event().record().timestamp;
        let duration = end - begin;
        let session_id = self.session_id().clone();
        let events = self.events.len();
        SessionAsJson {
            begin,
            end,
            duration,
            session_id,
            usernames: self.usernames,
            events
        }
    }
}