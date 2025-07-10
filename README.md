# Claude Code Schedule by Ian Macalinao

[![Crates.io](https://img.shields.io/crates/v/claude-code-schedule.svg)](https://crates.io/crates/claude-code-schedule)

Schedule Claude Code to run overnight so your rate limits refresh before work.

## Why Use This?

Run this tool before going to bed to ensure Claude Code gets used overnight. This refreshes your rate limits so you start each workday with a clean slate. Perfect for developers who want to maximize their Claude Code usage.

## Installation

```bash
# Install from crates.io
cargo install claude-code-schedule

# Or clone and build locally
git clone https://github.com/macalinao/claude-code-schedule
cd claude-code-schedule
cargo install --path .
```

## Usage

```bash
# Run at 6:00 AM (default) - perfect for overnight scheduling
ccschedule

# Run at a specific time
ccschedule --time 05:30

# Custom message for Claude
ccschedule --message "Start the day by reviewing code quality"
```

The tool will show a countdown and automatically run `claude --dangerously-skip-permissions` when the time arrives. Press Ctrl+C to cancel.

## Typical Workflow

1. **Before bed**: Run `ccschedule` in a terminal
2. **Go to sleep**: Let it run overnight
3. **Wake up**: Claude has already run and your rate limits are refreshed
4. **Start coding**: Full rate limits available for your workday

## About

Created by **Ian Macalinao** - [ianm.com](https://ianm.com) | [@simplyianm](https://twitter.com/simplyianm)

Apache 2.0 License
