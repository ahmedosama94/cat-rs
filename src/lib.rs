use clap::Parser;

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

        for path in self.value {
            let handler = std::thread::spawn(move || std::fs::read_to_string(&path));

            handlers.push(handler);
        }

        let mut nonblank_count = 0;
        for handler in handlers {
            let file_content = handler.join().expect("Unable to read file")?;

            if !self.number_nonblank {
                output.push_str(&file_content)
            } else {
                for line in file_content.lines() {
                    if !line.is_empty() {
                        nonblank_count += 1;
                        output.push_str(&format!("{:6}  {}\n", nonblank_count, line));
                    } else {
                        output.push_str(line);
                        output.push('\n');
                    }
                }
            }
        }

        Ok(output)
    }
}
