use std::io::Error;
use std::process::Stdio;
use async_process::Command;
use async_std::prelude::*;
use actix_rt::System;
use crate::secrets::WHITELIST_SCRIPT_PATH;

pub async fn add_to_whitelist(args: &str) -> Result<String, Error> {
    let mut child = Command::new("sh")
        .arg(WHITELIST_SCRIPT_PATH)
        .arg(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let output = child.output().await?;

    let success = output.status.success();
    let output_text = String::from_utf8_lossy(&output.stdout).to_string();
    let error_text = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(format!("==stdout==\n{}\n\n==stderr==\n{}", output_text, error_text))
}