use std::fmt::Display;

use evtx::SerializedEvtxRecord;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum EventProvider {
    TerminalServicesRemoteConnectionManager,
    TerminalServicesLocalConnectionManager,
    SecurityAuditing,
    DesktopWindowManager,
    UnsupportedProvider,
}

impl Display for EventProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventProvider::TerminalServicesRemoteConnectionManager => {
                "Microsoft-Windows-Terminal-Services-RemoteConnectionManager"
            }
            EventProvider::TerminalServicesLocalConnectionManager => {
                "Microsoft-Windows-TerminalServices-LocalSessionManager"
            }
            EventProvider::SecurityAuditing => "Microsoft-Windows-Security-Auditing",
            EventProvider::DesktopWindowManager => "Desktop Window Manager",
            EventProvider::UnsupportedProvider => "UNSUPPORTED PROVIDER",
        }
        .fmt(f)
    }
}

impl TryFrom<&SerializedEvtxRecord<Value>> for EventProvider {
    type Error = anyhow::Error;

    fn try_from(record: &SerializedEvtxRecord<Value>) -> Result<Self, Self::Error> {
        let provider_name =
            &record.data["Event"]["System"]["Provider"]["#attributes"]["Name"].to_string()[..];
        Ok(match provider_name {
            "Microsoft-Windows-Terminal-Services-RemoteConnectionManager" => {
                EventProvider::TerminalServicesRemoteConnectionManager
            }
            "Microsoft-Windows-TerminalServices-LocalSessionManager" => {
                EventProvider::TerminalServicesLocalConnectionManager
            }
            "Microsoft-Windows-Security-Auditing" => EventProvider::SecurityAuditing,
            "Desktop Window Manager" => EventProvider::DesktopWindowManager,
            _ => Self::UnsupportedProvider,
        })
    }
}
