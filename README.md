# Claude Code Schedule by Ian Macalinao

Schedule Claude Code to run overnight and refresh your rate limits before work.

## What it does

This tool runs Claude Code at 6:00 AM (or any time you choose) so your rate limits are refreshed when you start working. Just leave it running in a terminal overnight.

## Usage

```bash
# Run at 6:00 AM (default)
ccschedule

# Run at a specific time
ccschedule --time 05:30

# Run 3 hours before work starts
ccschedule --work-time 09:00

# Custom message
ccschedule --message "Good morning, let's code!"

# See what would happen without running
ccschedule --dry-run
```

## Example

```bash
$ ccschedule
Claude Code Schedule by Ian Macalinao
Scheduled to run at: 2024-01-15 06:00:00
Command: claude --dangerously-skip-permissions "Good morning!"
Press Ctrl+C to cancel...

Time remaining: 08:32:15
```

The tool will count down and automatically run Claude Code at the scheduled time.

## Installation

```bash
cargo install claude-code-schedule
```

## About

Created by **Ian Macalinao** - [ianm.com](https://ianm.com) | [@simplyianm](https://twitter.com/simplyianm)

Apache 2.0 License