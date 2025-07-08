mod file;


mod code_gen;
mod previewScannerUtils;
mod utils;
use std::{fs, os};
use std::sync::{LazyLock, Mutex};

mod constants;

mod parser;
mod precedence_order;
mod project_basic_utils;

mod libs;

use project_basic_utils::token::*;
use project_basic_utils::tokenizer::*;

use parser::function_parser::*;
use parser::var_parser::Var;


use parser::class_parser::Class;
use parser::expression::Expression;

use crate::file::{CompilationStage, File};
use crate::parser::code_block::{self, ValidInCodeBlock};
use crate::parser::type_parser::Type_;




static FILES_TO_COMPILE: LazyLock<Vec<Mutex<File>>> = LazyLock::new(|| {
    let mut files = Vec::new();

    let input_dir = "input";
    let entries = fs::read_dir(input_dir).expect("Failed to read input directory");
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        let file_name  = (&path).to_str().expect("Failed to get file name").to_string();
        let file_name_without_input_dir_name = file_name.split("/").last().expect("Failed to get file name without dir name").to_string();
        let file_contents = fs::read_to_string(&path).expect("Failed to read file");
        if (&path).is_file() {
            let tokenizer = Tokenizer {
                mutex: Mutex::new(()),
                file_name: Box::leak(file_name_without_input_dir_name.into_boxed_str()),
                start_line: 1,
                code: Box::leak(file_contents.into_boxed_str()),
                parse_index: 0,
            };
            files.push(Mutex::new(File::new(tokenizer)));
        }
    }
    files
});

fn main() {
    color_backtrace::install();

    let mut running_threads = vec![];
    
    for FILE in FILES_TO_COMPILE.iter() {
        running_threads.push(std::thread::spawn(move || {
            let mut file = FILE.lock().unwrap();
            //having different stages is so that when files can import from each other, they will be able to check to make sure that the file is done parsing before trying to use it
            file.stage = CompilationStage::Parsing;
            file.generate_syntax_tree_from_source_code();
            file.stage = CompilationStage::TypeChecking;
            file.type_check();
            file.stage = CompilationStage::CodeGeneration;
            file.output_code_from_syntax_tree();
            file.stage = CompilationStage::Done;
        }));
    }
    for thread in running_threads {
        thread.join().unwrap();
    }
}
