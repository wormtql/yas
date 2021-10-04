pub struct Buffer {
    pub data: Vec<Vec<f32>>
}

impl Buffer {
    fn new(width: usize, height: usize) -> Buffer {
        Buffer {
            data: vec![vec![0.0; width]; height]
        }
    }
}