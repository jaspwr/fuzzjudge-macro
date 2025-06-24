# FuzzJudge Problem Macro
Simple macro for implementing problems in Rust for [FuzzJudge](https://github.com/progsoc/fuzzjudge).

## Usage:

### main.rs

```rs
fn fuzz(_seed: u64) -> String {
    "Linus".to_string()
}

fn judge(_seed: u64, input: &str) -> Result<(), String> {
    if input != "Hello, Linus!" {
        return Err("Incorrect solution.".to_string())
    }
    Ok(())
}

fuzzjudge_macro::main!(fuzz, judge);
```

`main!` Arguments: 
- fuzz: fn(u64) -> String,
- judge: fn(u64, &str) -> Result<(), String>,
- solution (optional): fn(u64) -> String,

### prob.md

```md
---toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "fuzz"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "judge"]

[problem]
points = 1
difficulty = 1
---

# 👋 Hello Programmers!

Say hello to your fellow programmers!
```
