use obadh_engine::{ObadhEngine, VerbosityLevel, OutputFormat};
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, stdin, stdout};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "obadh",
    about = "A linguistically accurate Roman to Bengali transliteration engine",
    version,
    author
)]
struct Cli {
    /// Input text to transliterate
    #[arg(index = 1)]
    text: Option<String>,

    /// Input file to read from (use - for stdin)
    #[arg(short, long, value_name = "FILE")]
    input_file: Option<PathBuf>,

    /// Output file to write to (defaults to stdout)
    #[arg(short, long, value_name = "FILE")]
    output_file: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = CliOutputFormat::Text)]
    format: CliOutputFormat,

    /// Verbosity level
    #[arg(short, long, value_enum, default_value_t = CliVerbosityLevel::Normal)]
    verbosity: CliVerbosityLevel,
    
    /// Enable debug mode with token information and performance metrics (outputs JSON)
    #[arg(short = 'd', long)]
    debug: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum CliOutputFormat {
    /// Plain text output
    Text,
    /// JSON formatted output
    Json,
    /// XML formatted output
    Xml,
    /// HTML formatted output
    Html,
    /// Markdown formatted output
    Markdown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum CliVerbosityLevel {
    /// Only show the final output
    Quiet,
    /// Show basic info (default)
    Normal,
    /// Show detailed analysis
    Detailed,
    /// Show full debugging information
    Debug,
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(format: CliOutputFormat) -> Self {
        match format {
            CliOutputFormat::Text => OutputFormat::Text,
            CliOutputFormat::Json => OutputFormat::Json,
            CliOutputFormat::Xml => OutputFormat::Xml,
            CliOutputFormat::Html => OutputFormat::Html,
            CliOutputFormat::Markdown => OutputFormat::Markdown,
        }
    }
}

impl From<CliVerbosityLevel> for VerbosityLevel {
    fn from(level: CliVerbosityLevel) -> Self {
        match level {
            CliVerbosityLevel::Quiet => VerbosityLevel::Quiet,
            CliVerbosityLevel::Normal => VerbosityLevel::Normal,
            CliVerbosityLevel::Detailed => VerbosityLevel::Detailed,
            CliVerbosityLevel::Debug => VerbosityLevel::Debug,
        }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    // Configure the engine
    let mut engine = ObadhEngine::new()
        .with_verbosity(cli.verbosity.into());
    
    // If debug mode is enabled, force JSON output format
    let output_format = if cli.debug {
        OutputFormat::Json
    } else {
        cli.format.into()
    };
    
    engine = engine.with_output_format(output_format);
    
    // Set up output
    let mut output: Box<dyn Write> = match cli.output_file {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(stdout()),
    };
    
    // Set up input
    let input: Box<dyn BufRead> = match cli.input_file {
        Some(path) if path.to_string_lossy() == "-" => Box::new(BufReader::new(stdin())),
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => {
            if let Some(text) = cli.text {
                // Process text directly without streaming
                let result = if cli.debug {
                    engine.transliterate_with_performance(&text)
                } else {
                    engine.transliterate_as(&text)
                };
                writeln!(output, "{}", result)?;
                return Ok(());
            } else {
                // No input specified, use stdin
                Box::new(BufReader::new(stdin()))
            }
        }
    };
    
    // Process input line by line
    for line_result in input.lines() {
        let line = line_result?;
        if line.trim().is_empty() {
            writeln!(output, "")?;
            continue;
        }
        
        let result = if cli.debug {
            engine.transliterate_with_performance(&line)
        } else {
            engine.transliterate_as(&line)
        };
        writeln!(output, "{}", result)?;
    }
    
    Ok(())
} 