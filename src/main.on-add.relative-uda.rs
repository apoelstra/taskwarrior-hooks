// SPDX-License-Identifier: GPL-3.0-or-later

use std::{io, process};
use std::io::BufRead as _;

use anyhow::Context as _;
use tasklib::prelude::UdaValue;
use tasklib::chrono;

const HOOK_NAME: &str = "on-add.relative-uda";

fn process_task_line(s: &str) -> Result<tasklib::Task, anyhow::Error> {
    let mut task = s.parse::<tasklib::Task>()
        .with_context(|| format!("parsing JSON \"{s}\""))?;

    // Skip the recurring parent: status=recurring and has "recur" set
    if *task.status() == tasklib::Status::Recurring && task.recur().is_some() {
        return Ok(task);
    }

    // Attempt to parse UDAs.
    let udas = task.udas();
    let untilrel = udas.get("untilrel")
        .map(UdaValue::as_uda_duration)
        .transpose()
        .map_err(anyhow::Error::from_boxed)
        .with_context(|| format!("parsing 'untilrel' {:?}", udas.get("untilrel")))?;
    let waitrel = udas.get("waitrel")
        .map(UdaValue::as_uda_duration)
        .transpose()
        .map_err(anyhow::Error::from_boxed)
        .with_context(|| format!("parsing 'waitrel' {:?}", udas.get("waitrel")))?;
    
    // Skip tasks without due dates
    let due = match task.due() {
        Some(due) => *due,
        None => {
            if untilrel.is_some() {
                eprintln!("Warning [task {}]: task has 'untilrel' set but no 'due' date. Ignoring 'untilrel'.", task.uuid());
            }
            if waitrel.is_some() {
                eprintln!("Warning [task {}]: task has 'waitrel' set but no 'due' date. Ignoring 'waitrel'.", task.uuid());
            }
            return Ok(task);
        }
    };

    // Annoyingly all UdaValue::as_uda_duration does is parse a string
    // into a duration, leaving the type as a UdaValue even though it's
    // guaranteed that it will be a datetime.

    // Compute until = due + untilrel
    if let Some(UdaValue::Duration(offset)) = untilrel {
        let offset = chrono::Duration::seconds(i64::from(offset.num_seconds()));
        if let Some(until) = task.until() {
            eprintln!("Warning [task {}]: clobbering existing 'until' value {until} with {}", task.uuid(), due + offset);
        }
        *task.until_mut() = Some(due + offset);
    }

    // Compute wait = due - waitrel
    if let Some(UdaValue::Duration(offset)) = waitrel {
        let offset = chrono::Duration::seconds(i64::from(offset.num_seconds()));
        if let Some(wait) = task.wait() {
            eprintln!("Warning [task {}]: clobbering existing 'wait' value {wait} with {}", task.uuid(), due - offset);
        }
        *task.wait_mut() = Some(due - offset);
    }

    Ok(task)
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line_result in handle.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                eprintln!("relative-until hook: read error: {e}");
                process::exit(1);
            }
        };

        match process_task_line(&line) {
            Ok(task) => task.to_stdout()?,
            Err(e) => {
                eprintln!("Hook {HOOK_NAME} failed. Passing unmodified task through.");
                eprintln!("Error:");
                eprintln!("{e}");
                println!("{line}");
            }
        }
    }

    Ok(())
}