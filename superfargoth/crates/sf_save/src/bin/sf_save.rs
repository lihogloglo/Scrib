//! `sf_save info|verify|dump` (save.md "Interfaces"). Typed dumps: see `sf_save::cli`.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = sf_save::cli::run(&args, &mut std::io::stdout().lock(), &[]);
    std::process::exit(code);
}
