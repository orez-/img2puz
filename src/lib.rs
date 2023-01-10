mod generate_puz;
mod parse_grid;

use std::fmt;
use wasm_bindgen::prelude::*;
use crate::parse_grid::CrosswordGrid;

#[derive(Debug)]
pub enum CrosswordCell {
    Char(char),
    Rebus(String),
    Wall,
}

impl CrosswordCell {
    pub fn empty() -> Self {
        Self::Char('A')
    }
}

pub struct Crossword {
    width: u8,
    height: u8,
    cells: Vec<CrosswordCell>,
    across_clues: Vec<(u16, String)>,
    down_clues: Vec<(u16, String)>,
    title: String,
    author: String,
    copyright: String,
    notes: String,
}

impl fmt::Debug for Crossword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "    title: {}", self.title)?;
        writeln!(f, "   author: {}", self.author)?;
        writeln!(f, "copyright: {}", self.copyright)?;
        writeln!(f, "    notes: {}", self.notes)?;
        let mut it = self.cells.iter();
        for _ in 0..self.height {
            for _ in 0..self.width {
                match it.next().unwrap() {
                    CrosswordCell::Char(c) => write!(f, "{}", c)?,
                    CrosswordCell::Rebus(_) => todo!(),
                    CrosswordCell::Wall => write!(f, "░")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

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
    Ok(clues)
}

#[wasm_bindgen]
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
pub fn generate_puz_file(input: CrosswordInput) -> Vec<u8> {
    let CrosswordInput { image, across_clues, down_clues, title, author, copyright, notes } = input;
    let across_clues = parse_clue_block(&across_clues).unwrap();
    let down_clues = parse_clue_block(&down_clues).unwrap();
    let CrosswordGrid { width, height, cells } = parse_grid::load_crossword("puz.png").unwrap();
    let xword = Crossword {
        width, height, cells,
        title, author, copyright, notes,
        across_clues, down_clues,
    };
    xword.as_puz()
}
