use super::Frame;

pub struct HandFrame {
    pixel: Vec<char>,
    width: usize,
    height: usize,
}

impl Default for HandFrame {
    fn default() -> Self {
        Self::new(120, 25)
    }
}

impl HandFrame {
    pub fn new(width: usize, height: usize) -> Self {
        let pixel = vec![' '; width * height];
        HandFrame {
            pixel,
            width,
            height,
        }
    }

    fn index(&self, i: usize, j: usize) -> usize {
        i * self.width + j
    }
}

impl Frame for HandFrame {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn clear(&mut self) {
        self.pixel.fill(' ');
    }

    fn set_pixel(&mut self, i: usize, j: usize, ch: char) {
        let index = self.index(i, j);
        self.pixel[index] = ch;
    }
    fn render_frame(&self) {
        print!("{}\n\r", "=".repeat(self.width));
        for i in 0..self.height {
            print!("|");
            for j in 0..self.width {
                print!("{}", self.pixel[self.index(i, j)]);
            }
            print!("|");
            print!("\n\r");
        }
        print!("{}\n\r", "=".repeat(self.width));
    }
    fn add_snake(&mut self, shape: &Vec<(usize, usize)>) {
        for &(i, j) in shape {
            self.set_pixel(i, j, '+');
        }
        let &(i_last, j_last) = shape.last().unwrap();
        self.set_pixel(i_last, j_last, '*');
    }
}
