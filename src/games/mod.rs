use crate::cards::{Card, Rank, Suit};

mod shenzen;

pub enum Move {
    Individual { from: usize, to: usize },
    Stack { row: usize, start: usize, to: usize },
}

pub enum MoveResult {}

pub struct Board<R: Rank, S: Suit> {
    stacks: Vec<Vec<Card<R, S>>>,
}

pub enum BoardConfig {
    Even,
    Descending,
}

impl<R: Rank, S: Suit> Board<R, S> {
    pub fn apply_move(&mut self, mv: Move) {
        match mv {
            Move::Individual { from, to } => {
                let card = self.stacks.get_mut(from).unwrap().pop().unwrap(); // TODO: do safely
                self.stacks.get_mut(to).unwrap().push(card);
            }
            Move::Stack { row, start, to } => {
                let tail = self.stacks.get_mut(row).unwrap().split_off(start);
                self.stacks.get_mut(to).unwrap().extend_from_slice(&tail);
            }
        }
    }

    pub fn next_board(&mut self, mv: Move) -> Board<R, S> {
        let new_stacks = self.stacks.clone();

        let mut new_board = Board { stacks: new_stacks };

        new_board.apply_move(mv);

        new_board
    }

    // pub fn new(cards: Vec<Card<R, S>>, board_config: BoardConfig) -> Board<R, S> {}

    pub fn board(&self) -> &Vec<Vec<Card<R, S>>> {
        &self.stacks
    }

    pub fn columns(&self) -> usize {
        self.stacks.len()
    }

    pub fn column_length(&self, column: usize) -> usize {
        self.stacks.get(column).unwrap().len()
    }

    pub fn card_at(&self, column_row: (usize, usize)) -> &Card<R, S> {
        self.stacks
            .get(column_row.0)
            .unwrap()
            .get(column_row.1)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    enum TestRank {
        One,
        Two,
        Three,
    }

    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    enum TestSuit {
        Black,
        White,
    }

    impl Rank for TestRank {
        fn ordinal(&self) -> usize {
            match self {
                Self::One => 1,
                Self::Two => 2,
                Self::Three => 3,
            }
        }
    }

    impl Suit for TestSuit {}

    #[test]
    fn individual_movmement_works() {
        let initial_stacks = vec![
            vec![
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::Black,
                },
            ],
            vec![
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::White,
                },
            ],
            vec![
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::White,
                },
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::White,
                },
            ],
        ];

        let expected_stacks = vec![
            vec![
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::White,
                },
            ],
            vec![Card {
                rank: TestRank::Three,
                suit: TestSuit::Black,
            }],
            vec![
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::White,
                },
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::White,
                },
            ],
        ];

        let mut board: Board<TestRank, TestSuit> = Board {
            stacks: initial_stacks,
        };

        board.apply_move(Move::Individual { from: 1, to: 0 });

        assert_eq!(board.stacks, expected_stacks);
    }

    #[test]
    fn stack_movmement_works() {
        let initial_stacks = vec![
            vec![
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::Black,
                },
            ],
            vec![
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::White,
                },
            ],
            vec![
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::White,
                },
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::White,
                },
            ],
        ];

        let expected_stacks = vec![
            vec![
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::White,
                },
            ],
            vec![],
            vec![
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::White,
                },
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::White,
                },
            ],
        ];

        let mut board: Board<TestRank, TestSuit> = Board {
            stacks: initial_stacks,
        };

        board.apply_move(Move::Stack {
            row: 1,
            start: 0,
            to: 0,
        });

        assert_eq!(board.stacks, expected_stacks);
    }

    #[test]
    fn derive_new_board() {
        let initial_stacks = vec![
            vec![
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::Black,
                },
            ],
            vec![
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::White,
                },
            ],
            vec![
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::White,
                },
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::White,
                },
            ],
        ];

        let expected_stacks = vec![
            vec![
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::Black,
                },
                Card {
                    rank: TestRank::One,
                    suit: TestSuit::White,
                },
            ],
            vec![Card {
                rank: TestRank::Three,
                suit: TestSuit::Black,
            }],
            vec![
                Card {
                    rank: TestRank::Two,
                    suit: TestSuit::White,
                },
                Card {
                    rank: TestRank::Three,
                    suit: TestSuit::White,
                },
            ],
        ];

        let mut board: Board<TestRank, TestSuit> = Board {
            stacks: initial_stacks.clone(),
        };

        let new_board = board.next_board(Move::Individual { from: 1, to: 0 });

        assert_eq!(board.stacks, initial_stacks);
        assert_eq!(new_board.stacks, expected_stacks);
    }
}
