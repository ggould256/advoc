#![allow(dead_code)]

use std::fmt::Debug;

use nalgebra::Vector2;

pub type Scalar = i64;
pub type Xy = Vector2<Scalar>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub fn to_char(self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }

    pub fn to_offset(self) -> Xy {
        match self {
            Direction::North => Xy::new(0, -1),
            Direction::East => Xy::new(1, 0),
            Direction::South => Xy::new(0, 1),
            Direction::West => Xy::new(-1, 0),
        }
    }

    pub fn from_char(c: char) -> Direction {
        match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        }
        .unwrap()
    }

    pub fn cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board<BoardContent> {
    pub(crate) board: Vec<Vec<BoardContent>>,
}

impl<BoardContent> Board<BoardContent>
where
    BoardContent: Copy + Debug + ToString,
    BoardContent: TryFrom<char, Error: Debug>,
{
    pub fn height(&self) -> usize {
        self.board.len()
    }
    pub fn width(&self) -> usize {
        self.board[0].len()
    }
    pub fn dimensions(&self) -> Xy {
        Xy::new(self.width() as Scalar, self.height() as Scalar)
    }

    pub fn all_coords(&self) -> Vec<Xy> {
        let mut result = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                result.push(Xy::new(x as Scalar, y as Scalar));
            }
        }
        result
    }

    pub fn at(&self, xy: Xy) -> BoardContent {
        self.board[xy[1] as usize][xy[0] as usize]
    }

    pub fn maybe_at(&self, xy: Xy) -> Option<BoardContent> {
        if xy[0] < 0
            || xy[0] >= self.width() as Scalar
            || xy[1] < 0
            || xy[1] >= self.height() as Scalar
        {
            None
        } else {
            Some(self.board[xy[1] as usize][xy[0] as usize])
        }
    }

    pub fn set_at(&mut self, xy: Xy, c: BoardContent) {
        self.board[xy[1] as usize][xy[0] as usize] = c;
    }

    pub fn fill_rect(&mut self, ul: Xy, br: Xy, c: BoardContent) {
        for y in ul[1]..br[1] {
            self.board[y as usize][ul[0] as usize..br[0] as usize].fill(c);
        }
    }

    pub fn update_rect(&mut self, ul: Xy, br: Xy, f: impl Fn(BoardContent) -> BoardContent) {
        for y in ul[1]..br[1] {
            for x in ul[0]..br[0] {
                let xy = Xy::new(x, y);
                let old_value = self.at(xy);
                let new_value = f(old_value);
                self.set_at(xy, new_value);
            }
        }
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut result = Vec::new();
        for row in &self.board {
            result.push(String::from_iter(row.iter().map(|c| c.to_string())));
        }
        result
    }

    pub fn from_strings(strings: &Vec<String>) -> Board<BoardContent> {
        let mut result = Board { board: Vec::new() };
        for row_string in strings {
            result.board.push(
                row_string
                    .chars()
                    .map(|c| BoardContent::try_from(c).unwrap())
                    .collect(),
            );
        }
        result
    }

    pub fn neighbors(&self) -> Vec<BoardContent> {
        let mut result = Vec::new();
        for dir in Direction::ALL.iter() {
            let offset = dir.to_offset();
            if let Some(c) = self.maybe_at(offset) {
                result.push(c);
            }
        }
        result
    }

    pub fn neighbors8(&self, loc: Xy) -> Vec<BoardContent> {
        let mut result = Vec::new();
        let dirs8: Vec<Xy> = vec![
            Xy::new(1, 0),
            Xy::new(1, 1),
            Xy::new(0, 1),
            Xy::new(-1, 1),
            Xy::new(-1, 0),
            Xy::new(-1, -1),
            Xy::new(0, -1),
            Xy::new(1, -1),
        ];
        for &offset in dirs8.iter() {
            if let Some(c) = self.maybe_at(loc + offset) {
                result.push(c);
            }
        }
        result
    }

    pub fn iter(&self) -> BoardIterator<BoardContent> {
        BoardIterator {
            board: self,
            current_x: 0,
            current_y: 0,
        }
    }
}

pub struct BoardIterator<'a, BoardContent> {
    board: &'a Board<BoardContent>,
    current_x: usize,
    current_y: usize,
}
impl<BoardContent> Iterator for BoardIterator<'_, BoardContent>
where
    BoardContent: Copy + Debug + ToString,
    BoardContent: TryFrom<char, Error: Debug>,
{
    type Item = (Xy, BoardContent);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y >= self.board.height() {
            return None;
        }
        let xy = Xy::new(self.current_x as Scalar, self.current_y as Scalar);
        let content = self.board.at(xy);
        self.current_x += 1;
        if self.current_x >= self.board.width() {
            self.current_x = 0;
            self.current_y += 1;
        }
        Some((xy, content))
    }
}
