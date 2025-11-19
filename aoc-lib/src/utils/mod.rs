pub mod input;
pub mod output;
// pub mod legacy;

// Re-export commonly used items
pub use input::{
    download_input, ensure_input, get_input_path, load_input, load_input_lines,
    parse_lines, parse_lines_with_delimiter,
};
pub use output::SolutionOutput;

// // Re-export legacy functions for backward compatibility
// #[allow(deprecated)]
// pub use legacy::{
//     get_input_file_path, load_file_data, process_lines_without_delimiter,
//     read_input, split_lines_by_delimiter,
// };
