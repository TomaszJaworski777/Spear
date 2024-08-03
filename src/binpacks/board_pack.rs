use std::u16;

use crate::{base_structures::Side, Bitboard, ChessBoard};

pub struct ChessBoardPacked {
    board: [Bitboard; 4],
    side_to_move: Side,
    score: u16,
    result: i8
}

impl ChessBoardPacked {
    pub fn from_board(board: &ChessBoard, score: f32) -> Self {
        Self {
            board: board_to_compressed(board),
            side_to_move: board.side_to_move(),
            score: (score * u16::MAX as f32) as u16,
            result: 0
        }
    } 

    #[inline] 
    pub fn get_board(&self) -> &[Bitboard; 4] {
        &self.board
    }

    #[inline] 
    pub fn get_side_to_move(&self) -> Side {
        self.side_to_move
    }

    #[inline] 
    pub fn get_result(&self) -> i8 {
        self.result
    }

    #[inline] 
    pub fn get_white_perspective_score(&self) -> f32 {
        let stm_score = self.score as f32 / u16::MAX as f32;
        if self.side_to_move == Side::WHITE { stm_score } else { 1.0 - stm_score }
    }

    #[inline] 
    pub fn apply_result(&mut self, winner: Side) {
        self.result = if winner == Side::WHITE { 1 } else { -1 }
    }
}

fn board_to_compressed(board: &ChessBoard) -> [Bitboard; 4] {
    let mut result = [Bitboard::EMPTY; 4];

    board.get_occupancy().map(|square|{
        let piece = board.get_piece_on_square(square);
        let color = board.get_piece_color_on_square(square);
        for bit_index in 0..3usize {
            if (piece.get_raw() & 1 << bit_index) > 0 {
                result[bit_index].set_bit(square);
            }
        }
        if color == Side::BLACK {
            result[3].set_bit(square);
        }
    });

    result
}