enum Square {
    EMPTY,
    QUEEN,
    OUT
}

struct Board {
    board:Vec<Square>,
    n:i64
}

impl Board {
    fn new(n:i64) -> Self {
        let mut v:Vec<Square> = Vec::new();
        for i in 0..n*n {
            v.push(Square::EMPTY);
        }
        Board {
            board:v,
            n:n
        }
    }
    
    fn print(&self) -> () {
        let n = self.n as usize;
        println!();
        for i in 0..n {
            for j in 0..n {
                match self.board[(i * n) + j] {
                    Square::OUT => { print!{" O "} },
                    Square::QUEEN => { print!{" Q "} },
                    Square::EMPTY => { print!{" - "} }
                }
            }
            println!();
        }
        println!();
    }
    
    fn at(&self, r:i64, c:i64) -> Square {
        let n = self.n;
        if r >= n || c >= n || r <= -1 || c <= -1 {
            return Square::OUT;
        }
        let r = r as usize;
        let c = c as usize;
        let n_u = n as usize;
        match self.board[(r * n_u) + c] {
            Square::QUEEN => Square::QUEEN,
            Square::EMPTY => Square::EMPTY,
            Square::OUT => Square::OUT
        }
    }
    
    fn place(&mut self, r:usize, c:usize) -> () {
        let n = self.n as usize;
        match self.at(r as i64, c as i64) {
            Square::QUEEN => (),
            Square::EMPTY => self.board[(r * n) + c] = Square::QUEEN,
            Square::OUT => ()
        }
    }
    
    fn remove(&mut self, r:usize, c:usize) -> () {
        let n = self.n as usize;
        match self.at(r as i64, c as i64) {
            Square::QUEEN => self.board[(r * n) + c] = Square::EMPTY,
            Square::EMPTY => (),
            Square::OUT => ()
        }
    }
    
    fn valid(&self, r:i64, c:i64) -> bool {
        match self.at(r,c) {
            Square::OUT => false,
            Square::QUEEN => false,
            Square::EMPTY => {
                for i in 0..c {
                    match self.at(r,i) {
                        Square::OUT => {},
                        Square::EMPTY => {},
                        Square::QUEEN => return false
                    }
                    match self.at(r-(c-i),i) {
                        Square::OUT => {},
                        Square::EMPTY => {},
                        Square::QUEEN => return false
                    }
                    match self.at(r+(c-i),i) {
                        Square::OUT => {},
                        Square::EMPTY => {},
                        Square::QUEEN => return false
                    }
                }
                true
            }
        }
    }
    
    fn queen(&mut self, r:usize, c:usize) -> bool {
//        println!("{} {}", r, c);
        let n = self.n as usize;
        if c >= n { return true; }
        
        let row = r as i64;
        let col = c as i64;
//        println!("{} {} {}", self.valid(row, col), row, col);
        if self.valid(row,col) { 
            self.place(r,c);
//            self.print();
        }
        else { return false; }
        
        for i in 0..n {
            if self.queen(i,c+1) { return true; }
        }
        self.remove(r,c);
        false
    }
    
    fn n_queens(&mut self) -> bool {
        let n = self.n as usize;
        for i in 0..n {
            if self.queen(i,0) { return true; }
        }
        false
    }
}

fn main() {
    let mut b:Board = Board::new(8);
    println!("{}", b.n_queens());
    b.print();
}