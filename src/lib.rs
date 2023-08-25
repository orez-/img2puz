mod multi_error;
mod parse_grid;

use serde::Deserialize;
use wasm_bindgen::prelude::*;
use xword_puz::{Crossword, CrosswordArgs};
use crate::multi_error::MultiError;
use crate::parse_grid::CrosswordGrid;

fn parse_clue_prefix(line: &str) -> Option<(u16, String)> {
    if let Some((num, clue)) = line.trim_start().split_once('.') {
        if let Ok(num) = num.parse() {
            return Some((num, clue.to_string()));
        }
    }
    None
}

fn parse_clue_block(clue_block: &str) -> Result<Vec<(u16, String)>, &'static str> {
    let mut clues = Vec::new();
    let mut lines = clue_block.lines();
    let first_line = lines.next()
        .ok_or("no clues provided")?;
    let (mut cur_num, mut cur_clue) = parse_clue_prefix(first_line)
        .ok_or("clues must include line numbers: ` 1. Clue`")?;
    for line in lines {
        if let Some((num, clue)) = parse_clue_prefix(line) {
            clues.push((cur_num, cur_clue));
            cur_num = num;
            cur_clue = clue;
        } else {
            cur_clue.push('\n');
            cur_clue.push_str(line);
        }
    }
    clues.push((cur_num, cur_clue));
    Ok(clues)
}

#[wasm_bindgen]
#[derive(Deserialize)]
pub struct CrosswordInput {
    image: Vec<u8>,
    across_clues: String,
    down_clues: String,
    title: String,
    author: String,
    copyright: String,
    notes: String,
}

#[wasm_bindgen]
impl CrosswordInput {
    #[wasm_bindgen(constructor)]
    pub fn new(blob: JsValue) -> CrosswordInput {
        serde_wasm_bindgen::from_value(blob)
            .expect("crossword input should match required shape")
    }
}

#[wasm_bindgen]
pub fn generate_puz_file(input: CrosswordInput) -> Result<Vec<u8>, MultiError> {
    set_panic_hook();

    let mut errors = MultiError::new();
    let CrosswordInput { image, across_clues, down_clues, title, author, copyright, notes } = input;
    let across_clues = parse_clue_block(&across_clues);
    if let Err(msg) = across_clues {
        errors.push("across_clues", msg.into());
    }
    let down_clues = parse_clue_block(&down_clues);
    if let Err(msg) = down_clues {
        errors.push("down_clues", msg.into());
    }
    let img = image::load_from_memory(&image);
    if let Err(ref msg) = img {
        errors.push("image", format!("could not load image: {msg}"));
    }
    let (Ok(across_clues), Ok(down_clues), Ok(img)) = (across_clues, down_clues, img)
        else { return Err(errors) };
    let CrosswordGrid { width, height, cells } = parse_grid::parse_crossword(img);
    let xword: Crossword = CrosswordArgs {
        width, height, grid: cells,
        title, author, copyright, notes,
        across_clues, down_clues,
    }.into();
    xword.validate()?;
    Ok(xword.as_puz())
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
