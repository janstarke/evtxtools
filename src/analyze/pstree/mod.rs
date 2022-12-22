pub mod process;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    rc::{Rc, Weak},
};

use chrono::{DateTime, Utc};
use evtx::EvtxParser;
pub(crate) use process::*;
use regex::Regex;
use serde_json::{json, Value};

use crate::analyze::Format;

use super::Cli;

pub(crate) fn display_pstree(cli: &Cli, username: &Option<String>) -> anyhow::Result<()> {
    let username_regex =
        username.as_ref().map(|s| Regex::new(&format!("(?i){}", s)).expect("invalid username regex"));

    let has_username = |p: &Process| match username_regex.as_ref() {
        None => true,
        Some(username) => {
            username.is_match(&p.subject_user_name)
                || username.is_match(&p.target_user_name)
        }
    };

    let fp = PathBuf::from(&cli.evtx_file);
    let mut parser = EvtxParser::from_path(fp)?;
    let events: HashMap<_, _> = parser
        .records_json_value()
        .filter_map(Result::ok)
        .map(Process::try_from)
        .filter_map(Result::ok)
        .filter(|p| has_username(p))
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
