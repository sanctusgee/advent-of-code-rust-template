pub mod input;
pub mod output;

// Re-export commonly used items
pub use input::{
    download_input, ensure_input, get_input_path, load_input, load_input_lines,
    parse_lines, parse_lines_with_delimiter,
};
pub use output::SolutionOutput;

