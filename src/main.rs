use clap::Parser;
use mini_grep::{
    GrepErrors, GrepResult,
    cmd::Args,
    search::{Matcher, search_file},
    walk::collect_files,
};

fn main() -> anyhow::Result<()> {
    println!("A Mini Grep Write in Rust");
    let args = Args::parse();
    println!("parsed args: {:?}", args);

    run(&args)?;
    Ok(())
}

fn run(args: &Args) -> GrepResult<()> {
    let matcher = Matcher::new(args)?;

    let paths = collect_files(&args.all_paths())?;

    for path in paths {
        let result = search_file(&path, &matcher)?;
        if result.matched.is_empty() {
            continue;
        }
        println!("{}", path.display());
        for line in result.matched {
            let line_content = String::from_utf8_lossy(&result.lines[line - 1]);
            println!("\t{}: {}", line, line_content);
        }
        println!()
    }

    Ok(())
}
