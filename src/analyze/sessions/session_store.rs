use std::{path::PathBuf, collections::HashSet};

use anyhow::bail;
use evtx::{EvtxParser, SerializedEvtxRecord};

use super::{SessionEvent, SessionId, Session};

pub struct SessionStore {
    sessions: HashSet<SessionId, Session>,
}

impl TryFrom<Vec<PathBuf>> for SessionStore {
    type Error = anyhow::Error;

    fn try_from(value: Vec<PathBuf>) -> Result<Self, Self::Error> {
        let mut sessions = Self {
            sessions: Default::default()
        };
        for path in value {
            if !(path.exists() && path.is_file()) {
                bail!("unable to read file {}", path.display());
            }

            for event in EvtxParser::from_path(path)?
                .records_json_value()
                .map(|r| r.expect("error reading event"))
                .map(SessionEvent::try_from)
                .filter_map(|r| r.ok())
            {
                sessions.add_event(event);
            }
        }
        Ok(sessions)
    }
}

impl SessionStore {
    fn add_event(&mut self, event: SessionEvent) {}
}
