use super::units::Cell;
use std::convert::TryFrom;

pub fn read(rle: String) -> Vec<Cell> {
    /* Following symbols can appear in file: #,b,o,$,!,[0-9], all in ASCII.*/
    /* File format description: https://conwaylife.com/wiki/Run_Length_Encoded */
    let mut cur_number: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut alive_cells: Vec<Cell> = vec![];
    let mut is_comment_line;

    let repeat = |x : i64| -> i64 {
        if x == 0 {1} else {x}
    };

    let is_comment = |c: Option<&char>| -> bool {
        return match c {Some('#') | Some('x') => true, _ => false};
    };

    let mut chars = rle.chars().peekable();

    is_comment_line = is_comment(chars.peek());    
    while let Some(c) = chars.next() {
        match (c, is_comment_line) {
            ('!', _) => { break; },
            ('\n', _) => { 
                let next = chars.peek();
               
                is_comment_line = is_comment(next);
            },
            (_, true) => {},
            ('$', false) => {
                y += repeat(cur_number);
                x = 0;
                cur_number = 0;
            },
            ('b', false) => {
                x += repeat(cur_number);
                cur_number = 0;
            },
            ('o', false) => {
                for _ in 0..repeat(cur_number) { 
                    alive_cells.push(Cell{x, y});
                    x += 1;
                }
                cur_number = 0;
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

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn glider() {
        let alive_cells = super::read(GLIDER_TEXT.to_string());
        assert_eq!(alive_cells, vec![
            Cell{x: 1, y: 0},
            Cell{x: 2, y: 1},
            Cell{x: 0, y: 2},
            Cell{x: 1, y: 2},
            Cell{x: 2, y: 2}
        ]);
    }

    #[test]
    fn two_squares() {
        let alive_cells = super::read(TWO_SQUARES_TEXT.to_string());
        assert_eq!(alive_cells, vec![
            Cell{x: 0, y: 0},
            Cell{x: 1, y: 0},
            Cell{x: 0, y: 1},
            Cell{x: 1, y: 1},
            Cell{x: 2, y: 4},
            Cell{x: 3, y: 4},
            Cell{x: 2, y: 5},
            Cell{x: 3, y: 5}
        ]);
    }

const GLIDER_TEXT: &str = 
"# comment1
# comment2
x = 10, y = 10
# comment3
bo$2bo$3o!

# is the same as

# bob
# bbo
# ooo
# (1,0), (2,1), (0,2), (1,2), (2,2)";

const TWO_SQUARES_TEXT: &str =
"# comment1
# comment2
x = 1024, y = 2048
# comment3
2o2b$2o2b3$2b2o$2b2o!

# is the same as

# oobb
# oobb
# bbbb
# bbbb
# bboo
# bboo
# (0,0), (1,0), (0,1), (1,1), (2,4), (3,4), (2,5), (3,5)"; 
}
