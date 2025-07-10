# Claude Code Schedule

Schedule Claude Code to run at a specific time.

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
# Run at 6:00 AM (default)
ccschedule

# Run at a specific time
ccschedule --time 05:30
```

The tool will show a countdown and run Claude Code when the time arrives. Press Ctrl+C to cancel.

## About

Created by **Ian Macalinao** - [ianm.com](https://ianm.com) | [@simplyianm](https://twitter.com/simplyianm)

Apache 2.0 License