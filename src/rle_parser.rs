use super::units::Cell;
use std::convert::TryFrom;

pub fn read(rle: String) -> Vec<Cell> {
    /* Following symbols can appear in file: #,b,o,$,!,[0-9], all in ASCII.*/
    /* File format description: https://conwaylife.com/wiki/Run_Length_Encoded */
    let mut cur_number: i64 = 0;
    let mut is_comment_line = false;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut alive_cells: Vec<Cell> = vec![];

    let mut chars = rle.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, is_comment_line) {
            ('!', _) => { break; },
            ('\n', _) => { 
                let next = chars.peek();
               
                match next {
                    Some('#') | Some('x') => {is_comment_line = true;}
                    _ => {}
                }
            },
            (_, true) => {},
            ('$', false) => {
                let repeat : i64 = if cur_number == 0 {1} else {cur_number};
                
                y += repeat;
                x = 0;
                cur_number = 0;
            },
            ('b', false) => {
                let repeat : i64 = if cur_number == 0 {1} else {cur_number};

                x += repeat;
                cur_number = 0;
            },
            ('o', false) => {
                let repeat: i64 = if cur_number == 0 {1} else {cur_number};

                for _ in 0..repeat { alive_cells.push(Cell{x, y}); }
            }
            ('0'..='9', false) => {
                cur_number *= 10;
                let digit: u32 = c.to_digit(10).unwrap(); // c is in [0..9]
                cur_number += i64::try_from(digit).unwrap();    // digit is in [0..9]
            },
            _ => {}
        }
    }

    alive_cells
}
