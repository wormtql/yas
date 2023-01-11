pub struct Buffer {
    pub data: Vec<Vec<f32>>
}

impl Buffer {
    fn new_zeroed(width: usize, height: usize) -> Buffer {
        Buffer {
            data: vec![vec![0.0; width]; height]
        }
    }
}