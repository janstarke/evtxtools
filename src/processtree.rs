use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fmt::Display,
    path::PathBuf,
    rc::{Rc, Weak},
};

use anyhow::bail;
use chrono::{DateTime, Utc};
use clap::{Parser, ValueEnum};
use evtx::{EvtxParser, SerializedEvtxRecord};
use regex::Regex;
use serde_json::{json, Value};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

struct Process {
    timestamp: DateTime<Utc>,
    event_record_id: u64,
    subject_user_sid: String,
    subject_user_name: String,
    subject_domain_name: String,
    subject_logon_id: String,
    new_process_id: u64,
    new_process_name: String,
    token_elevation_type: String,
    process_id: u64,
    command_line: String,
    target_user_sid: String,
    target_user_name: String,
    target_domain_name: String,
    target_logon_id: String,
    parent_process_name: String,
    mandatory_label: String,
    children: BTreeMap<DateTime<Utc>, Weak<RefCell<Self>>>,
    is_root: bool,
}

impl From<&Process> for Value {
    fn from(process: &Process) -> Self {
        let children: BTreeMap<_, _> = process
            .children
            .values()
            .filter_map(|x| x.upgrade())
            .map(|p| {
                let p: &Process = &p.borrow();
                let v: Value = p.into();
                (p.timestamp, v)
            })
            .collect();
        let mut result: HashMap<_, _> = vec![
            ("timestamp".to_owned(), json!(process.timestamp)),
            ("event_record_id".to_owned(), json!(process.event_record_id)),
            ("SubjectUserSid".to_owned(), json!(process.subject_user_sid)),
            (
                "SubjectUserName".to_owned(),
                json!(process.subject_user_name),
            ),
            (
                "SubjectDomainName".to_owned(),
                json!(process.subject_domain_name),
            ),
            ("SubjectLogonId".to_owned(), json!(process.subject_logon_id)),
            ("NewProcessId".to_owned(), json!(process.new_process_id)),
            ("NewProcessName".to_owned(), json!(process.new_process_name)),
            (
                "TokenElevationType".to_owned(),
                json!(process.token_elevation_type),
            ),
            ("ProcessId".to_owned(), json!(process.process_id)),
            ("CommandLine".to_owned(), json!(process.command_line)),
            ("TargetUserSid".to_owned(), json!(process.target_user_sid)),
            ("TargetUserName".to_owned(), json!(process.target_user_name)),
            (
                "TargetDomainName".to_owned(),
                json!(process.target_domain_name),
            ),
            ("TargetLogonId".to_owned(), json!(process.target_logon_id)),
            (
                "ParentProcessName".to_owned(),
                json!(process.parent_process_name),
            ),
            ("MandatoryLabel".to_owned(), json!(process.mandatory_label)),
        ]
        .into_iter()
        .collect();

        result.extend(
            children
                .into_iter()
                .map(|(k, v)| (k.to_rfc3339_opts(chrono::SecondsFormat::Secs, true), v)),
        );

        json!(result)
    }
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "`{}` (`0x{:04x}`, created *`{}`*, user is `{}`)",
            self.new_process_name,
            self.new_process_id,
            self.timestamp
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            self.target_user_name
        )
    }
}

macro_rules! from_json {
    ($value: ident, $( $att:expr ),+ ) => {
        {
            let mut value = $value;
            $(
                value = value.get($att).ok_or(anyhow::anyhow!("missing '{}' key in {}", $att, value))?;
            )+
            value
        }
    };
}

fn u64_from_value(value: &Value) -> anyhow::Result<u64> {
    if let Some(v) = value.as_u64() {
        Ok(v)
    } else {
        bail!("Value '{value}' is no u64")
    }
}
fn u64_from_hex_value(value: &Value) -> anyhow::Result<u64> {
    if let Some(v) = value.as_str() {
        Ok(u64::from_str_radix(v.trim_start_matches("0x"), 16)?)
    } else {
        bail!("Value '{value}' is no string")
    }
}

impl TryFrom<SerializedEvtxRecord<Value>> for Process {
    type Error = anyhow::Error;

    fn try_from(record: SerializedEvtxRecord<Value>) -> Result<Self, Self::Error> {
        let value = &record.data;
        let event = from_json!(value, "Event");
        let system = from_json!(event, "System");
        let event_id = u64_from_value({
            let event_id = from_json!(system, "EventID");
            match event_id.get("#text") {
                Some(eid) => eid,
                None => event_id,
            }
        })?;

        if event_id != 4688 {
            bail!("event cannot be converted to process");
        }

        let event_data = from_json!(event, "EventData");

        let subject_user_sid = from_json!(event_data, "SubjectUserSid")
            .as_str()
            .unwrap()
            .into();
        let subject_user_name = from_json!(event_data, "SubjectUserName")
            .as_str()
            .unwrap()
            .into();
        let subject_domain_name = from_json!(event_data, "SubjectDomainName")
            .as_str()
            .unwrap()
            .into();
        let subject_logon_id = from_json!(event_data, "SubjectLogonId")
            .as_str()
            .unwrap()
            .into();
        let new_process_id = u64_from_hex_value(from_json!(event_data, "NewProcessId"))?;
        let new_process_name = from_json!(event_data, "NewProcessName")
            .as_str()
            .unwrap()
            .into();
        let token_elevation_type = from_json!(event_data, "TokenElevationType")
            .as_str()
            .unwrap()
            .into();
        let process_id = u64_from_hex_value(from_json!(event_data, "ProcessId"))?;
        let command_line = from_json!(event_data, "CommandLine")
            .as_str()
            .unwrap()
            .into();
        let target_user_sid = from_json!(event_data, "TargetUserSid")
            .as_str()
            .unwrap()
            .into();
        let target_user_name = from_json!(event_data, "TargetUserName")
            .as_str()
            .unwrap()
            .into();
        let target_domain_name = from_json!(event_data, "TargetDomainName")
            .as_str()
            .unwrap()
            .into();
        let target_logon_id = from_json!(event_data, "TargetLogonId")
            .as_str()
            .unwrap()
            .into();
        let parent_process_name = from_json!(event_data, "ParentProcessName")
            .as_str()
            .unwrap()
            .into();
        let mandatory_label = from_json!(event_data, "MandatoryLabel")
            .as_str()
            .unwrap()
            .into();

        Ok(Self {
            timestamp: record.timestamp,
            event_record_id: record.event_record_id,
            subject_user_sid,
            subject_user_name,
            subject_domain_name,
            subject_logon_id,
            new_process_id,
            new_process_name,
            token_elevation_type,
            process_id,
            command_line,
            target_user_sid,
            target_user_name,
            target_domain_name,
            target_logon_id,
            parent_process_name,
            mandatory_label,
            children: Default::default(),
            is_root: true,
        })
    }
}

#[derive(ValueEnum, Clone)]
enum Format {
    Json,
    Markdown,
}

/// reconstructs a process tree, based on Windows audit logs
#[derive(Parser)]
#[clap(author,version,name=env!("CARGO_BIN_NAME"))]
struct Cli {
    /// Name of the evtx file to parse
    evtx_file: String,

    /// display only processes of this user (case insensitive regex search)
    #[clap(short('U'), long("username"))]
    username: Option<String>,

    #[clap(short('F'), long("format"), value_enum, default_value_t=Format::Json)]
    format: Format,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[clap(skip)]
    username_regex: Option<Regex>,
}

impl Cli {
    pub fn has_username(&self, process: &Process) -> bool {
        match self.username_regex.as_ref() {
            None => true,
            Some(username) => {
                username.is_match(&process.subject_user_name)
                    || username.is_match(&process.target_user_name)
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut cli = Cli::parse();

    TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    if let Some(username) = cli.username.as_ref() {
        cli.username_regex = Some(Regex::new(&format!("(?i){}", username))?)
    }

    let fp = PathBuf::from(&cli.evtx_file);
    let mut parser = EvtxParser::from_path(fp)?;
    let events: HashMap<_, _> = parser
        .records_json_value()
        .filter_map(Result::ok)
        .map(Process::try_from)
        .filter_map(Result::ok)
        .filter(|p| cli.has_username(p))
        .map(|e| {
            let pid = e.new_process_id;
            (pid, Rc::new(RefCell::new(e)))
        })
        .collect();

    log::warn!("found {} process creations", events.len());

    for new_process in events.values() {
        let parent_pid = new_process.borrow().process_id;
        if let Some(parent) = events.get(&parent_pid) {
            new_process.borrow_mut().is_root = false;
            let child_ts = new_process.borrow().timestamp;
            let child_process = Rc::downgrade(new_process);
            parent.borrow_mut().children.insert(child_ts, child_process);
        } else {
            log::warn!("found no parent for {}", new_process.borrow().command_line);
        }
    }

    let root_processes: BTreeMap<_, _> = events
        .values()
        .filter(|e| e.borrow().is_root)
        .map(|e| {
            let timestamp = e.borrow().timestamp;
            let value = Value::from(&*e.borrow());
            (timestamp, value)
        })
        .collect();

    log::warn!("{} processes have no parent", root_processes.len());

    match cli.format {
        Format::Json => {
            let root_processes: BTreeMap<_, _> = events
                .values()
                .filter(|e| e.borrow().is_root)
                .map(|e| {
                    let timestamp = e.borrow().timestamp;
                    let value = Value::from(&*e.borrow());
                    (timestamp, value)
                })
                .collect();

            let procs_as_json = json!(root_processes);
            println!("{}", serde_json::to_string_pretty(&procs_as_json)?);
        }

        Format::Markdown => {
            let root_processes: BTreeMap<_, _> = events
                .values()
                .filter(|e| e.borrow().is_root)
                .map(|e| {
                    let timestamp = e.borrow().timestamp;
                    let proc = Rc::downgrade(e);
                    (timestamp, proc)
                })
                .collect();
            display_markdown(&root_processes, 0);
        }
    }

    Ok(())
}

fn display_markdown(procs: &BTreeMap<DateTime<Utc>, Weak<RefCell<Process>>>, indent: usize) {
    for proc in procs.values() {
        if let Some(proc) = proc.upgrade() {
            println!("{}- {}", " ".repeat(indent), proc.borrow());
            display_markdown(&proc.borrow().children, indent + 2);
        }
    }
}
