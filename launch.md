Title:

I made a Rust CLI tool to auto-run Claude Code at a specific time — useful for getting around rate limits

Post:

If you’ve ever hit Claude Code rate limits right when you’re trying to get stuff done, you’ll get why I made this.

It’s a tiny CLI tool called claude-code-schedule — it lets you schedule Claude Code to run at a specific time (like 6 AM), so your usage resets before you even start coding for the day.

Basically:

```bash
# Install from crates.io

cargo install claude-code-schedule

# Default run time is 6 AM

ccschedule

# Or customize it

ccschedule --time 05:30
```

Once it’s running, it just waits and launches Claude Code at the right time. I leave it running overnight and wake up with fresh limits.

GitHub: https://github.com/macalinao/claude-code-schedule
Crates: https://crates.io/crates/claude-code-schedule

Built it because I rely on Claude Code a lot during the day and was tired of hitting limits first thing in the morning. Feedback welcome — curious if others are doing anything similar.
