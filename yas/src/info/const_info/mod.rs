use super::*;

mod genshin;
mod starrail;

pub enum WindowSize {
    Windows43x18,
    WIndows7x3,
    Windows16x9,
    Windows8x5,
    Windows4x3,
    MacOS8x5,
}

type R = RectBound<WindowInfoType>;
type P = Pos<WindowInfoType>;
type S = Size<WindowInfoType>;
