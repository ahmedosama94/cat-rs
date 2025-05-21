use std::thread::JoinHandle;

use clap::Parser;

const PARALLELISM_FACTOR: usize = 4;
const ABOUT: &str = "Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.";

#[derive(Parser, Debug)]
#[command(version = "0.0.0", about = ABOUT, long_about = ABOUT)]
pub struct CatArgs {
    #[arg(short = 'A', help = "equivalent to -vET")]
    show_all: bool,

    #[arg(short = 'b')]
    number_nonblank: bool,

    #[arg(value_name = "FILE", value_delimiter = ' ', num_args=1..)]
    value: Vec<String>,
}

impl CatArgs {
    pub fn exec(self) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        let mut handlers = Vec::new();
        let max_threads: usize = std::thread::available_parallelism()?.into();
        let max_threads = max_threads * PARALLELISM_FACTOR;

        let max_threads = if max_threads == 1 {
            max_threads
        } else {
            max_threads - 1
        };

        let mut nonblank_count = 0;

        for path in self.value {
            let handler = std::thread::spawn(move || {
                let bytes = std::fs::read(&path).expect("Unable to read file");

                unsafe { String::from_utf8_unchecked(bytes) }
            });

            handlers.push(handler);

            if handlers.len() >= max_threads {
                drain_handlers(
                    handlers,
                    &mut output,
                    &mut nonblank_count,
                    self.number_nonblank,
                );

                handlers = Vec::new();
            }
        }

        if !handlers.is_empty() {
            drain_handlers(
                handlers,
                &mut output,
                &mut nonblank_count,
                self.number_nonblank,
            );
        }

        Ok(output)
    }
}

fn drain_handlers(
    handlers: Vec<JoinHandle<String>>,
    output: &mut String,
    nonblank_count: &mut u32,
    number_nonblank: bool,
) {
    for handler in handlers {
        let file_content = handler.join().expect("Unable to join threads");

        if !number_nonblank {
            output.push_str(&file_content)
        } else {
            for line in file_content.lines() {
                if !line.is_empty() {
                    *nonblank_count += 1;
                    output.push_str(&format!("{:6}\t{}\n", nonblank_count, line));
                } else {
                    output.push_str(line);
                    output.push('\n');
                }
            }
        }
    }
}
