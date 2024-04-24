use enigo::{Enigo, MouseControllable};

use crate::system_control::system_control::SystemControl;
use crate::utils;

pub struct MacOSControl {
    enigo: Enigo
}

impl MacOSControl {
    pub fn new() -> MacOSControl {
        MacOSControl {
            enigo: Enigo::new()
        }
    }

    pub fn mouse_move_to(&mut self, x: i32, y: i32) -> anyhow::Result<()> {
        self.enigo.mouse_move_to(x, y);

        anyhow::Ok(())
    }

    pub fn mouse_click(&mut self) -> anyhow::Result<()> {
        self.enigo.mouse_click(MouseButton::Left);

        anyhow::Ok(())
    }

    pub fn mouse_scroll(&mut self, amount: i32) -> anyhow::Result<()> {
        self.enigo.mouse_scroll_y(-amount);

        anyhow::Ok(())
    }

    pub fn mac_scroll(&mut self, length: i32, delta: i32, times: i32) {
        let enigo = &mut self.enigo;

        for _j in 0..length {
            enigo.mouse_down(MouseButton::Left);
            for _i in 0..times {
                enigo.mouse_move_relative(0, -delta);
                utils::sleep(10);
            }
    
            enigo.mouse_up(MouseButton::Left);
            utils::sleep(10);
    
            enigo.mouse_down(MouseButton::Left);
            utils::sleep(5);
            enigo.mouse_up(MouseButton::Left);
            utils::sleep(5);
    
            enigo.mouse_move_relative(0, times * delta);
            utils::sleep(20);
        }
    }
    
    pub fn mac_scroll_fast(length: i32) {
        mac_scroll(length, 4, 30);
    }
    
    pub fn mac_scroll_slow(length: i32) {
        mac_scroll(length, 4, 5);
    }
}