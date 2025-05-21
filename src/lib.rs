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

    #[arg(short = 'n', help = "number all output lines")]
    number: bool,

    #[arg(short = 'b', help = "number nonempty output lines, overrides -n")]
    number_nonblank: bool,

    #[arg(value_name = "FILE", value_delimiter = ' ', num_args=1..)]
    value: Vec<String>,
}

impl CatArgs {
    pub fn exec(mut self) -> Result<String, Box<dyn std::error::Error>> {
        if self.number_nonblank {
            self.number = false;
        }

        let mut output = String::new();
        let mut handlers = Vec::new();
        let max_threads: usize = std::thread::available_parallelism()?.into();
        let max_threads = max_threads * PARALLELISM_FACTOR;

        let max_threads = if max_threads == 1 {
            max_threads
        } else {
            max_threads - 1
        };

        let mut line_count = 0;
        let paths = self.value;

        // Bypass partial move problem in for loop
        self.value = Vec::new();

        for path in paths {
            let handler = std::thread::spawn(move || {
                let bytes = std::fs::read(&path).expect("Unable to read file");

                unsafe { String::from_utf8_unchecked(bytes) }
            });

            handlers.push(handler);

            if handlers.len() >= max_threads {
                self.drain_handlers(handlers, &mut output, &mut line_count);

                handlers = Vec::new();
            }
        }

        self.value = Vec::new();
        if !handlers.is_empty() {
            self.drain_handlers(handlers, &mut output, &mut line_count);
        }

        Ok(output)
    }

    fn drain_handlers(
        &self,
        handlers: Vec<JoinHandle<String>>,
        output: &mut String,
        line_count: &mut u32,
    ) {
        for handler in handlers {
            let file_content = handler.join().expect("Unable to join threads");

            if !self.number && !self.number_nonblank {
                output.push_str(&file_content)
            } else {
                for line in file_content.lines() {
                    if self.number || !line.is_empty() {
                        *line_count += 1;
                        output.push_str(&format!("{:6}\t{}\n", line_count, line));
                    } else {
                        output.push_str(line);
                        output.push('\n');
                    }
                }
            }
        }
    }
}
