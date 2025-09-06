use anyhow::{Context, Result};
use chrono::{DateTime, Local, Timelike};
use clap::Parser;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser, Debug)]
#[command(
    author = "Ian Macalinao <ian@macalinao.com>",
    version,
    about = "Schedule Claude Code to run at a specific time - by Ian Macalinao",
    long_about = "A CLI tool by Ian Macalinao that runs Claude Code at a scheduled time. \
                  The tool will stay running in your terminal and execute the command when the time is reached.\
                  \n\nCreated by Ian Macalinao - https://ianm.com"
)]
struct Args {
    /// Run Claude Code at a specific time (format: HH:MM, default: 06:00)
    #[arg(short, long, value_name = "HH:MM")]
    time: Option<String>,

    /// Message to pass to Claude Code (default: "Continue working on what you were working on previously. If you weren't working on something previously, then come up with a list of tasks to work on based on what is left in the codebase.")
    #[arg(
        short,
        long,
        default_value = "Continue working on what you were working on previously. If you weren't working on something previously, then come up with a list of tasks to work on based on what is left in the codebase."
    )]
    message: String,

    /// Dry run - print what would happen without scheduling
    #[arg(short, long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let target_time = if let Some(time_str) = args.time {
        parse_time(&time_str)?
    } else {
        // Default to 6:00 AM
        parse_time("06:00")?
    };

    let target_time = if target_time <= Local::now() {
        target_time + chrono::Duration::days(1)
    } else {
        target_time
    };

    if args.dry_run {
        println!("Would run at: {}", target_time.format("%Y-%m-%d %H:%M:%S"));
        println!("Command: {}", build_claude_command(&args.message));
        return Ok(());
    }

    println!("Claude Code Schedule by Ian Macalinao");
    println!(
        "Scheduled to run at: {}",
        target_time.format("%Y-%m-%d %H:%M:%S")
    );
    println!("Command: {}", build_claude_command(&args.message));
    println!("Press Ctrl+C to cancel...\n");

    // Set up Ctrl+C handler
    tokio::spawn(async {
        tokio::signal::ctrl_c().await.unwrap();
        println!("\nCancelled by user");
        std::process::exit(0);
    });

    // Wait until the target time
    loop {
        let now = Local::now();
        if now >= target_time {
            println!("\nRunning scheduled command...");
            run_claude_command(&args.message)?;
            println!("Command completed successfully!");
            println!("Claude Code Schedule by Ian Macalinao - https://ianm.com");
            break;
        }

        let duration_until = target_time.signed_duration_since(now);
        let hours = duration_until.num_hours();
        let minutes = duration_until.num_minutes() % 60;
        let seconds = duration_until.num_seconds() % 60;

        print!(
            "\rTime remaining: {hours:02}:{minutes:02}:{seconds:02}"
        );
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        // Sleep for 1 second
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

fn parse_time(time_str: &str) -> Result<DateTime<Local>> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid time format. Expected HH:MM");
    }

    let hour: u32 = parts[0].parse().context("Invalid hour")?;
    let minute: u32 = parts[1].parse().context("Invalid minute")?;

    if hour >= 24 || minute >= 60 {
        anyhow::bail!("Invalid time. Hour must be 0-23, minute must be 0-59");
    }

    let now = Local::now();
    now.with_hour(hour)
        .and_then(|t| t.with_minute(minute))
        .and_then(|t| t.with_second(0))
        .and_then(|t| t.with_nanosecond(0))
        .context("Failed to create target time")
}

fn build_claude_command(message: &str) -> String {
    format!(
        "claude -c --dangerously-skip-permissions \"{}\"",
        message.replace("\"", "\\\"")
    )
}

fn run_claude_command(message: &str) -> Result<()> {
    let output = Command::new("claude")
        .args(["-c", "--dangerously-skip-permissions", message])
        .spawn()
        .context("Failed to start claude command")?
        .wait()
        .context("Failed to wait for claude command")?;

    if !output.success() {
        anyhow::bail!("Claude command failed with exit code: {:?}", output.code());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_claude_command() {
        assert_eq!(
            build_claude_command("Hello, world!"),
            "claude --dangerously-skip-permissions \"Hello, world!\""
        );
        assert_eq!(
            build_claude_command("Hello \"world\""),
            "claude --dangerously-skip-permissions \"Hello \\\"world\\\"\""
        );
    }

    #[test]
    fn test_parse_time() {
        let time = parse_time("14:30").unwrap();
        assert_eq!(time.hour(), 14);
        assert_eq!(time.minute(), 30);
    }

    #[test]
    fn test_parse_invalid_time() {
        assert!(parse_time("25:00").is_err());
        assert!(parse_time("12:60").is_err());
        assert!(parse_time("12").is_err());
        assert!(parse_time("12:30:45").is_err());
    }
}
