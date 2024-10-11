use rand::Rng;

use crate::frame::{Frame, HandFrame};

#[macro_export]
macro_rules! snake {
    ($row:expr, $col:expr, $len:expr) => {{
        let mut s = vec![];
        for i in 0..$len {
            s.push(($row, i + $col));
        }
        s
    }};
}

#[derive(PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UNKNOWN,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'a' => Direction::LEFT,
            'w' => Direction::UP,
            'd' => Direction::RIGHT,
            's' => Direction::DOWN,
            _ => Direction::UNKNOWN,
        }
    }
}

pub struct Game<T: Frame> {
    frame: T,
    snake: Vec<(usize, usize)>,
    direction: Direction,
    food: (usize, usize),
}

impl<T: Frame + Default> Default for Game<T> {
    fn default() -> Self {
        Game {
            frame: T::default(),
            food: (24, 8),
            snake: vec![],
            direction: Direction::RIGHT,
        }
    }
}

impl<T: Frame> Game<T> {
    pub fn new(length: usize, frame: T) -> Self {
        Game {
            frame,
            food: (8, 8),
            snake: snake!(4, 4, length),
            direction: Direction::RIGHT,
        }
    }

    fn step_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0..self.frame.height());
        let y: usize = rng.gen_range(0..self.frame.width());
        self.food = (x, y);
    }

    pub fn score(&self) -> usize {
        self.snake.len()
    }

    fn next_head_location(&self) -> (usize, usize) {
        let mut head = self.snake.last().unwrap().clone();
        match self.direction {
            Direction::UP => {
                if head.0 == 0 {
                    head.0 = self.frame.height();
                }
                head.0 -= 1
            }
            Direction::DOWN => head.0 += 1,
            Direction::LEFT => {
                if head.1 == 0 {
                    head.1 = self.frame.width();
                }
                head.1 -= 1
            }
            Direction::RIGHT => head.1 += 1,
            Direction::UNKNOWN => (),
        }
        head.0 %= self.frame.height();
        head.1 %= self.frame.width();
        head
    }

    fn head(&self) -> (usize, usize) {
        return self.snake.last().unwrap().clone();
    }

    pub fn do_move(&mut self) {
        let next_head = self.next_head_location();
        if next_head == self.food {
            self.snake.push(next_head);
            self.step_food();
            return;
        }
        for i in 0..self.snake.len() - 1 {
            self.snake[i] = self.snake[i + 1];
        }
        *self.snake.last_mut().unwrap() = next_head;
    }

    pub fn render(&mut self) {
        self.frame.clear();
        self.frame.add_snake(&self.snake);
        self.frame.set_pixel(self.food.0, self.food.1, 'F');
        self.frame.render_frame();
    }

    fn lost(&self) -> bool {
        let head = self.head();
        for part in self.snake.iter().take(self.snake.len() - 1) {
            if *part == head {
                return true;
            }
        }
        return false;
    }

    pub fn step(&mut self, input: Direction) -> bool {
        if input != Direction::UNKNOWN {
            self.direction = input;
        }
        self.do_move();
        return self.lost();
    }
}

mod tests {
    use crate::frame::HandFrame;

    use super::{Direction, Game};

    #[test]
    fn do_move_right_test() {
        let mut game = Game::<HandFrame>::default();
        game.snake = snake!(4, 5, 4);
        game.do_move();
        assert!(game.snake == snake!(4, 6, 4));
    }

    #[test]
    fn do_move_left_test() {
        let mut game = Game::<HandFrame>::default();
        game.direction = Direction::LEFT;
        game.snake = snake!(4, 5, 4);
        game.do_move();
        let mut new_snake = snake!(4, 6, 3);
        new_snake.push((4, 7));
        assert!(game.snake == new_snake)
    }

    #[test]
    fn do_move_up_test() {
        let mut game = Game::<HandFrame>::default();
        game.direction = Direction::UP;
        game.snake = snake!(4, 5, 4);
        game.do_move();
        let mut new_snake = snake!(4, 6, 3);
        new_snake.push((3, 8));
        assert!(game.snake == new_snake)
    }

    #[test]
    fn do_move_down_test() {
        let mut game = Game::<HandFrame>::default();
        game.direction = Direction::DOWN;
        game.snake = snake!(4, 5, 4);
        game.do_move();
        let mut new_snake = snake!(4, 6, 3);
        new_snake.push((5, 8));
        assert!(game.snake == new_snake)
    }
}
