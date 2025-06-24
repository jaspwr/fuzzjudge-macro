#[macro_export]
/// ## Usage
/// 
/// ### main.rs
/// 
/// ```rs
/// fn fuzz(_seed: u64) -> String {
///     "Linus".to_string()
/// }
/// 
/// fn judge(_seed: u64, input: &str) -> Result<(), String> {
///     if input != "Hello, Linus!" {
///         return Err("Incorrect solution.".to_string())
///     }
///     Ok(())
/// }
/// 
/// fuzzjudge_macro::main!(fuzz, judge);
/// ```
///
/// `main!` Arguments: 
/// - fuzz: `fn(seed: u64) -> String`,
/// - judge: `fn(seed: u64, input: &str) -> Result<(), String>`,
/// - solution (optional): `fn(seed: u64) -> String`,
/// 
/// ### prob.md
/// 
/// ```md
/// ---toml
/// [fuzz]
/// exec = ["cargo", "run", "--release", "--", "fuzz"]
/// env = {}
/// 
/// [judge]
/// exec = ["cargo", "run", "--release", "--quiet", "--", "judge"]
/// 
/// [problem]
/// points = 1
/// difficulty = 1
/// ---
/// 
/// # 👋 Hello Programmers!
/// 
/// Say hello to your fellow programmers!
/// ```
macro_rules! main {
    ($fuzz:ident, $judge: ident) => {
        fn main() {
            $crate::problem($fuzz, $judge, None);
        }
    };
    ($fuzz:ident, $judge: ident, $solution:ident) => {
        fn main() {
            $crate::problem($fuzz, $judge, Some($solution));
        }
    };
}

pub fn problem(
    fuzz: fn(u64) -> String,
    judge: fn(u64, &str) -> Result<(), String>,
    solution: Option<fn(u64) -> String>,
) {
    let args: Vec<_> = std::env::args().collect();
    let fail = |message: &str| {
        eprintln!("{}", message);
        std::process::exit(1);
    };

    if args.len() != 3 {
        fail("Invalid Usage: Expected `cargo run --release -- <fuzz | judge | solution> <seed>`");
    }

    let mut default_hasher = std::hash::DefaultHasher::new();
    std::hash::Hash::hash(&args[2], &mut default_hasher);
    let seed = std::hash::Hasher::finish(&default_hasher);

    match args[1].as_str() {
        "fuzz" => print!("{}", fuzz(seed)),
        "solution" => print!("{}", (solution.expect("No solution method provided"))(seed)),
        "judge" => {
            let mut buffer = String::new();
            let _ = std::io::stdin().read_line(&mut buffer);
            if let Err(e) = judge(seed, &buffer) {
                fail(&e);
            }
        }
        _ => fail(&format!(
            "`{}` was not a valid method. Valid methods are `fuzz`, `judge` and `solution`.",
            args[1]
        )),
    }
}
