# Release Notes - v0.3.0

## Breaking Changes

### New Required Environment Variable: AOC_USER_AGENT

Users must now set the `AOC_USER_AGENT` environment variable to download puzzle inputs.

**Before (v0.2.0):**
```bash
export AOC_SESSION="your_session_cookie"
cargo run --bin aoc download 2024 1
```

**Now (v0.3.0):**
```bash
export AOC_SESSION="your_session_cookie"
export AOC_USER_AGENT="github.com/yourname/your-repo (your@email.com)"
cargo run --bin aoc download 2024 1
```

### Why This Change?

The previous version used a hardcoded User-Agent that referenced the template author's GitHub repository. This caused:

1. **Attribution issues**: All users' requests appeared to come from the same source
2. **Rate limiting concerns**: Shared User-Agent could lead to collective rate limits
3. **Poor etiquette**: Not following best practices for automated web requests

### Migration Guide

Add this to your shell profile (`.bashrc`, `.zshrc`, etc.):

```bash
export AOC_USER_AGENT="github.com/yourname/your-repo (your@email.com)"
```

Or set it temporarily in your current session:

```bash
export AOC_USER_AGENT="github.com/yourname/your-repo (your@email.com)"
```

### Error Message

If you forget to set `AOC_USER_AGENT`, you will see:

```
Error: AOC_USER_AGENT environment variable not set.
Please set it to identify yourself, e.g.:
    export AOC_USER_AGENT="github.com/yourname/your-repo (contact@email.com)"

This helps website admins contact you if there are issues with your requests.
```

## Files Changed

- `aoc-lib/src/utils/input.rs` - Now requires `AOC_USER_AGENT` environment variable
- `README.md` - Updated documentation for both required environment variables
- `.gitignore` - Updated comment to mention both env vars

## Upgrade Steps

1. Pull the latest changes
2. Set the `AOC_USER_AGENT` environment variable
3. Continue using the template as before

## Version Bump Rationale

This is a MINOR version bump (0.2.0 to 0.3.0) rather than a PATCH because:

- It introduces a breaking change (new required environment variable)
- Existing users must take action to continue using the download feature
- The API contract has changed

## Justification

This change is about proper attribution and internet etiquette, not security. User-Agent headers are public information and do not expose credentials.

As noted by the Advent of Code creator in [this Reddit post](https://www.reddit.com/r/adventofcode/comments/1pa472d/reminder_please_throttle_your_aoc_traffic/), automated tools should identify themselves properly to help with debugging and prevent issues.

**Why we're doing this:**
- Proper attribution - Your requests should identify you, not the template author
- Internet etiquette - Following community guidelines for automated requests
- Avoiding shared rate limits - If many users share one User-Agent, they share rate limits

## Additional Resources

For more information about being a good citizen when making automated requests to Advent of Code:
- https://www.reddit.com/r/adventofcode/comments/1pa472d/reminder_please_throttle_your_aoc_traffic/
