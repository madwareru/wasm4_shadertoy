use std::ops::{Add, Div, Mul};
use std::panic::PanicInfo;

#[derive(Copy, Clone)]
struct Vec2 {
    x: f32,
    y: f32
}

impl Vec2 {
    fn yx(self) -> Self {
        Self {
            x: self.y,
            y: self.x
        }
    }

    fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    fn weird_dot(self, rhs: Self) -> f32 {
        self.dot(Self{x: -rhs.y, y: rhs.x})
    }

    fn magnitude_sqr(self) -> f32 {
        self.dot(self)
    }

    fn magnitude(self) -> f32 {
        self.magnitude_sqr().sqrt()
    }

    fn cos(self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos()
        }
    }

    fn sin(self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin()
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.dot(Self{ x: rhs.x, y: -rhs.y }),
            y: self.dot(self.yx())
        }
    }
}

fn pattern_foo(time: f32, uv: Vec2) -> Vec2 {
    let k = Vec2 {
        x: (time + uv.x * 5.0).cos(),
        y: (time * 0.917 + uv.y * 3.0).sin()
    };
    (
        (uv * k) + Vec2 { x: (time * 0.13).cos(), y: (time * 0.47).sin() }
    ) * (uv / 3.14)
}

fn pattern(time: f32, uv: Vec2) -> f32 {
    let uv2 = pattern_foo(time, uv);
    let uv3 = pattern_foo(time, uv.yx() / 1.15);
    let l = (uv2 * uv3).magnitude();
    (0.5 + 0.5 * (l.cos() - l.sin())).clamp(0.0, 1.0)
}

pub struct GameStage {
    current_frame: usize
}

impl GameStage {
    pub fn new() -> Self {
        GameStage {
            current_frame: 0,
        }
    }

    /// generate monochrome image
    fn fragment(&self, u: f32, v: f32) -> f32 {
        let t = self.current_frame as f32 / 60.0;
        let uv = Vec2{x: u, y: v} * 0.92 + Vec2 { x: 0.8, y: 0.4 };
        pattern(t, uv)
    }

    pub fn start(&mut self) {
        unsafe {
            *super::wasm4::PALETTE = [0x232e45, 0x3c5d75, 0x5eb2a0, 0xffd7b9];
        }
    }

    pub fn update(&mut self) {
        self.current_frame += 1;
    }

    pub fn render(&self) {
        let mut offset_0 = 0;
        let mut offset_1 = 40;
        for j in 0..80 {
            for i in (0..80).step_by(2) {
                let (u0, u1) = (i as f32 / 79.0, (i + 1) as f32 / 79.0);
                let v = j as f32 / 79.0;
                let (f0, f1) = (12.0 * self.fragment(u0, v), 12.0 * self.fragment(u1, v));
                let f0 = f0.round().clamp(0.0, 12.0) as u8;
                let f1 = f1.round().clamp(0.0, 12.0) as u8;
                let (mut row0, mut row1) = if f1 == 12 {
                    (0b1111, 0b1111)
                } else {
                    let low_r = f1 / 4;
                    let (mut r0, mut r1) = ((low_r) << 2 | low_r, (low_r << 2) | low_r);
                    match f1 % 4 {
                        1 => {
                            r0 += 0b0100;
                        },
                        2 => {
                            r0 += 0b0001;
                            r1 += 0b0100;
                        },
                        3 => {
                            r0 += 0b0001;
                            r1 += 0b0101;
                        },
                        _ => ()
                    }
                    (r0, r1)
                };
                row0 *= 16;
                row1 *= 16;
                if f0 == 12 {
                    row0 = row0 | 0b1111;
                    row1 = row1 | 0b1111;
                } else {
                    let low_r = f0 / 4;
                    row0 = row0 | (low_r << 2) | low_r;
                    row1 = row1 | (low_r << 2) | low_r;
                    match f0 % 4 {
                        1 => {
                            row1 += 0b0100;
                        },
                        2 => {
                            row0 += 0b0001;
                            row1 += 0b0100;
                        },
                        3 => {
                            row1 += 0b0001;
                            row0 += 0b0101;
                        },
                        _ => ()
                    }
                }
                unsafe {
                    *(*super::wasm4::FRAMEBUFFER).get_unchecked_mut(offset_0) = row0;
                    *(*super::wasm4::FRAMEBUFFER).get_unchecked_mut(offset_1) = row1;
                }
                offset_0 += 1;
                offset_1 += 1;
            }
            offset_0 += 40;
            offset_1 += 40;
        }
    }
}