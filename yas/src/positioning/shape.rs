pub struct Shape3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Shape3D<T> {
    pub fn new(x: T, y: T, z: T) -> Shape3D<T> {
        Shape3D {
            x, y, z
        }
    }
}
