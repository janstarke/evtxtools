use std::{collections::HashMap, path::PathBuf};

use anyhow::bail;
use eventdata::SessionId;
use evtx::EvtxParser;

use super::{Session, SessionEvent};

pub struct SessionStore {
    sessions: HashMap<SessionId, Session>,
}

impl SessionStore {
    pub fn import(value: &Vec<PathBuf>, include_anonymous: bool) -> Result<Self, anyhow::Error> {
        let mut sessions = Self {
            sessions: HashMap::<SessionId, Session>::new(),
        };
        for path in value {
            if !(path.exists() && path.is_file()) {
                bail!("unable to read file {}", path.display());
            }
            log::info!("importing {} into session store", path.to_string_lossy());

            for event in EvtxParser::from_path(path)?
                .records_json_value()
                .map(|r| r.expect("error reading event"))
                .map(SessionEvent::try_from)
                .filter_map(|r| r.ok())
            {
                log::trace!("found session event at {}", event.record().timestamp);
                sessions.add_event(event);
            }
        }

        Ok(Self {
            sessions: sessions
                .sessions
                .into_iter()
                .filter(|s| {
                    if include_anonymous {
                        true
                    } else {
                        !s.1.is_anonymous()
                    }
                })
                .collect(),
        })
    }

    fn add_event(&mut self, event: SessionEvent) {
        if self.sessions.contains_key(event.session_id()) {
            self.sessions
                .entry(event.session_id().clone())
                .and_modify(|s| s.add_event(event));
        } else {
            self.sessions
                .insert(event.session_id().clone(), Session::from(event));
        }
    }
}

impl IntoIterator for SessionStore {
    type Item = Session;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut v = Vec::from_iter(self.sessions.into_values());
        v.sort();
        v.into_iter()
    }
}
