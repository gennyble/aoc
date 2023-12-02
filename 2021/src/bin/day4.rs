#![feature(stmt_expr_attributes)]

use std::str::FromStr;

use aoc2021::day_parse;

fn main() {
    let mut game: BingoGame = day_parse!();
    let (winner, final_drawn) = game.clone().draw_until_winner();
    let unmarked: usize = winner.unmarked().iter().map(|v| *v as usize).sum();

    println!(
        "Unmarked sum {}, last drawn {}. Product {}",
        unmarked,
        final_drawn,
        unmarked * final_drawn as usize
    );

    let (winner, final_drawn) = game.find_last_win();
    let unmarked: usize = winner.unmarked().iter().map(|v| *v as usize).sum();

    println!(
        "LAST WIN\nUnmarked sum {}, last drawn {}. Product {}",
        unmarked,
        final_drawn,
        unmarked * final_drawn as usize
    );
}

#[derive(Clone)]
struct BingoGame {
    drawing: Vec<u8>,

    boards: Vec<Board>,
}

impl BingoGame {
    pub fn draw_until_winner(&mut self) -> (Board, u8) {
        for draw in &self.drawing {
            for board in &mut self.boards {
                board.drawn(*draw);

                if board.check_won() {
                    return (board.clone(), *draw);
                }
            }
        }

        unreachable!()
    }

    pub fn find_last_win(&mut self) -> (Board, u8) {
        let drawing = self.drawing.clone();

        for draw in drawing {
            println!("ahh {}", draw);
            for board in &mut self.boards {
                board.drawn(draw);
            }

            // gen 2023-12-02: back in 2021 there was a drain_filter feature.
            // this code was making r-a mad and I had to fix it, so I did this...
            let mut lost: Vec<Board> = vec![];
            let mut won: Vec<Board> = vec![];
            for board in self.boards.drain(..) {
                if board.check_won() {
                    won.push(board);
                } else {
                    lost.push(board);
                }
            }
            self.boards = lost;

            println!("Board count {}", self.boards.len());
            if self.boards.len() == 0 {
                return (won[0].clone(), draw);
            }
        }

        unreachable!()
    }
}

impl FromStr for BingoGame {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let drawing = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| u8::from_str_radix(s, 10).unwrap())
            .collect();

        let mut boards = vec![];
        let lines: Vec<&str> = lines.collect();
        for board in lines.chunks(6) {
            let board: Vec<String> = board[1..].iter().map(|&s| s.to_owned()).collect();
            boards.push(Board::from_line_vec(board));
        }

        Ok(Self { drawing, boards })
    }
}

#[derive(Clone)]
struct Board {
    data: Vec<u8>,
    marks: Vec<bool>,
}

impl Board {
    pub fn drawn(&mut self, num: u8) {
        for (idx, v) in self.data.iter().enumerate() {
            if *v == num {
                self.marks[idx] = true;
            }
        }
    }

    pub fn check_won(&self) -> bool {
        // Check rows
        'row: for row in 0..5 {
            for col in 0..5 {
                let idx = row * 5 + col;

                if self.marks[idx] == false {
                    continue 'row;
                }
            }

            return true;
        }

        // Check cols
        'col: for col in 0..5 {
            for row in 0..5 {
                let idx = row * 5 + col;

                if self.marks[idx] == false {
                    continue 'col;
                }
            }

            return true;
        }

        false
    }

    /*
    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    */
    pub fn from_line_vec(lines: Vec<String>) -> Board {
        if lines.len() != 5 {
            panic!("board drom_line is not 5 data")
        }

        let mut data = vec![];
        for line in lines {
            data.extend(Self::do_line(&line));
        }

        Self {
            data,
            marks: vec![false; 25],
        }
    }

    pub fn unmarked(&self) -> Vec<u8> {
        self.data
            .iter()
            .zip(self.marks.iter())
            .filter_map(|(num, marked)| if !marked { Some(*num) } else { None })
            .collect()
    }

    fn do_line(raw: &str) -> Vec<u8> {
        let mut nums = vec![];

        let mut curr = String::new();
        for (idx, ch) in raw.chars().enumerate() {
            match idx % 3 {
                0 => curr.push(ch),
                1 => {
                    curr.push(ch);
                    nums.push(u8::from_str_radix(curr.trim(), 10).unwrap());
                    curr.clear();
                }
                _ => (),
            }
        }

        nums
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    fn test_board() -> String {
        [
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .join("\n")
    }

    fn day_test() -> BingoGame {
        let data = read_to_string("input/day4_test").unwrap();
        data.parse().unwrap()
    }

    #[test]
    fn board_fails_to_parse_line() {
        let line = test_board().lines().next().unwrap().to_owned();
        let expected = vec![22, 13, 17, 11, 0];

        assert_eq!(expected, Board::do_line(&line))
    }

    #[test]
    fn board_fails_to_find_win() {
        let lines = test_board().lines().map(|l| l.to_owned()).collect();
        let board = Board::from_line_vec(lines);

        let mut row = board.clone();
        row.drawn(22);
        row.drawn(13);
        row.drawn(17);
        row.drawn(11);
        row.drawn(0);
        assert!(row.check_won());

        let mut col = board.clone();
        col.drawn(13);
        col.drawn(2);
        col.drawn(9);
        col.drawn(10);
        col.drawn(12);
        assert!(col.check_won());
    }

    #[test]
    fn test_first_win_fails() {
        let mut game = day_test();
        let (board, last) = game.draw_until_winner();

        assert_eq!(board.data[0], 14);
        assert_eq!(last, 24);
    }

    #[test]
    fn test_last_win_fails() {
        let mut game = day_test();
        let (board, last) = game.find_last_win();

        assert_eq!(board.data[0], 3);
        assert_eq!(last, 13);
    }
}
