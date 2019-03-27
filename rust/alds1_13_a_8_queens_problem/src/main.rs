// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_13_A

macro_rules! scan {
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_| scan!($t ;;)).collect::<Vec<_>>()
    };
}

struct Board {
    width: usize,
    height: usize,
    queens: Vec<(usize, usize)>,
}

impl Board {
    fn new(board: &Board) -> Board {
        return Board {
            width: board.width,
            height: board.height,
            queens: board.queens.to_vec()
        }
    }

    fn display(&self) {
        let mut squares = vec![vec![false; self.height]; self.width];
        
        for &queen in &self.queens {
            let (qi, qj) = queen;
            squares[qi][qj] = true;
        }
        
        for row in squares {
            let mut line = String::new();
            for square in row {
                match square {
                    true => line += "Q",
                    false => line += ".",
                }
            }
            println!("{}", line);
        }
    }

    fn incorrect(&self) -> bool {
        for &queen in &self.queens {
            let (qi, qj) = queen;
            for &other_queen in &self.queens {
                let (oi, oj) = other_queen;
                let h_diff = std::cmp::max(oi, qi) - std::cmp::min(oi, qi);
                let v_diff = std::cmp::max(oj, qj) - std::cmp::min(oj, qj);

                if oi == qi && oj == qj { 
                    continue;
                } else if oi == qi || oj == qj || h_diff == v_diff {
                    return true;
                }
            }
        }
        return false;
    }

    fn correct(&self, n: usize) -> bool {
        if self.queens.len() == n {
            true
        } else {
            false
        }
    }

    fn backtrack(self, queen_requirement: usize) -> Option<Board> {
        if self.incorrect() {
            return None;
        }
        if self.correct(queen_requirement) {
            return Some(self);
        }

        let mut generator = BoardGenerator::from(self);
        while let Some(next_board) = generator.next() {
            if let Some(solution) = next_board.backtrack(queen_requirement) {
                return Some(solution);
            }
        }
        return None;
    }
}

struct BoardGenerator {
    initial_board: Board,
    i: usize,
    j: usize,
}

impl BoardGenerator {
    fn from(board: Board) -> BoardGenerator {
        return BoardGenerator {
            initial_board: board,
            i: 0,
            j: 0
        }
    }
    fn next(&mut self) -> Option<Board> {
        self.j += 1;
        
        if self.j >= self.initial_board.height {
            self.i += 1;
            self.j = 0;
        }

        if self.i >= self.initial_board.width {
            return None
        }

        match self.initial_board.queens.binary_search(&(self.i, self.j)) {
            Ok(_) => return self.next(),
            Err(idx) => {
                let mut next_board = Board::new(&self.initial_board);
                next_board.queens.insert(idx, (self.i, self.j));
                return Some(next_board);
            }
        }
    }
} 


fn main() {
    let mut b = Board { width: 8, height: 8, queens: Vec::new() };

    let k = scan!(usize);
    for _ in 0..k {
        let (r, c) = scan!(usize, usize);
        b.queens.push((r, c));
    }
    if let Some(solution) = b.backtrack(8) {
        solution.display();
    } else {
        println!("No Solution");
    }
}
