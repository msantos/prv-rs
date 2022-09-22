use clap::Parser;
use std::io;
use std::io::Write;
use std::time::Instant;

/// pressure relief valve for Unix process pipelines
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// message rate limit
    #[clap(short, long, default_value_t = 0)]
    limit: usize,

    /// message rate window
    #[clap(short, long, default_value_t = 1)]
    window: u64,

    /// behaviour if write buffer is full
    #[clap(short = 'W', long = "write-buffer", default_value = "block")]
    write_buffer: String,

    /// verbose mode
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    event_loop(&args)
}

fn event_loop(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let mut t0 = Instant::now();

    let mut count = 0;

    let mut buf = String::new();

    loop {
        buf.clear();
        let _len = match stdin.read_line(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(n) => n,
            Err(err) => return Err(Box::new(err)),
        };

        let t1 = Instant::now();

        if t1.duration_since(t0).as_secs() >= args.window {
            count = 0;
            t0 = t1;
        }

        if args.limit > 0 && count >= args.limit {
            if args.verbose {
                eprint!("DISCARD:{}/{}:{}", count, args.limit, buf);
            }
            continue;
        }

        count += 1;

        stdout.write_all(buf.as_bytes())?;
        stdout.flush()?;
    }
}
