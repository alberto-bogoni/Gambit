use core::fmt;

fn main() {
    let board = build_chessboard();

    println!("board: \n{board}");
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct ChessBoard {
    white_team: BitBoard,
    black_team: BitBoard
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let complete_board = self.white_team.composed_board() | self.black_team.composed_board();

        for i in (0..64).rev() {
            let bit = (complete_board >> i) & 1;
            write!(f, "{}", bit)?;

            if i % 8 == 0 && i > 0 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct BitBoard {
    pawns: u64,
    rooks: u64,
    knights: u64,
    bishops: u64,
    queen: u64,
    king: u64
}

impl BitBoard {
   fn composed_board(&self) -> u64 {
        self.pawns | self.rooks | self.knights | self.bishops | self.queen | self.king
   }
}

fn build_chessboard() -> ChessBoard {

    ChessBoard { 
        white_team: BitBoard {
            pawns: 255 << 8,
            rooks: (1u64 << 7) | (1u64 << 0),
            knights: (1u64 << 6) | (1u64 << 1),
            bishops: (1u64 << 5) | (1u64 << 2),
            queen: 1u64 << 4,
            king: 1u64 << 3
        },
        black_team: BitBoard {
            pawns: 255 << 48,
            rooks: (1u64 << 63) | (1u64 << 56),
            knights: (1u64 << 62) | (1u64 << 57),
            bishops: (1u64 << 61) | (1u64 << 58),
            queen: 1u64 << 60,
            king: 1u64 << 59
        },
    }
}
