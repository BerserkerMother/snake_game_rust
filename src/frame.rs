mod hand;

pub use hand::HandFrame;

pub trait Frame {
    fn render_frame(&self);
    fn set_pixel(&mut self, i: usize, j: usize, ch: char);
    fn add_snake(&mut self, shape: &Vec<(usize, usize)>);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn clear(&mut self);
}
