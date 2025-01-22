use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

fn grep_naive(target: &str) -> io::Result<()> {
    let stdin = io::stdin();
    // why need to lock this handle to stdin?
    // to lock Stdin for current thread exclusive use as rust std protects it with a mutex.
    // Without a mutex, two threads trying to read from stdin at the same time would cause undefined behavior.
    //
    // C has the same issue and solves it the same way: all the C standard input and output functions obtain a lock behind the scenes.
    // The only difference is that in Rust, the lock is part of the API.
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

/// Taking grep further and add support for searching files on disk.
fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn grep_main() -> Result<(), Box<dyn Error>> {
    // Get the command-line arguments. The first argument is the // string to search for; the rest are filenames.
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE...")?
    };
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();
    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }
    Ok(())
}

fn main() {
    let result = grep_main();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use super::*;

    #[test]
    fn test_grep() -> io::Result<()> {
        let target = "rian";

        // not running here as it blocks test waiting for input
        // let stdin = io::stdin();
        // grep(&target, stdin.lock())?;

        // file also works
        let mut file = tempfile::tempfile()?;
        writeln!(file, "Brian was here. Briefly.")?;
        // in most languages, files are buffered by default, not Rust.
        grep(&target, BufReader::new(file))?;

        Ok(())
    }
}