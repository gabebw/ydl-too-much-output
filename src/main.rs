use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use std::process::{Command, Stdio};

// If the output is too large (>64kb?), the program will hang forever because
// internal kernel buffers are filled.
// This is an example program that demonstrates that.
// https://github.com/rust-lang/rust/issues/27152
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use something that will generate a lot of JSON.
    // This will print out 464kb of output.
    const PLAYLIST: &str =
        "https://www.youtube.com/watch?v=ML3n1c0FHDI&list=PLbpi6ZahtOH7CkYdkWCsAVMcX1hoYy3TP";

    let mut child = Command::new(Path::new("youtube-dl"))
        // The program does not hang if this is `Stdio::null()` instead, which
        // suggests that the issue is that the STDOUT buffer is filling up.
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(vec!["-J", PLAYLIST])
        .spawn()?;

    // Stream into stdout. Without these 3 lines, the program will hang for a
    // very long time (forever?).
    let mut stdout = Vec::new();
    let s = child.stdout.take();
    std::io::copy(&mut s.expect("Couldn't grab stdout"), &mut stdout)?;

    child.wait()?;
    Ok(())
}
