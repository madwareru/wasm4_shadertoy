pub struct GameStage {
    rnd_gen: super::random::RndGen,
    current_frame: usize,
    fire_pixels: [u8; 80*80]
}

impl GameStage {
    pub fn new() -> Self {
        GameStage {
            rnd_gen: super::random::RndGen::new(),
            current_frame: 0,
            fire_pixels: [0; 80*80]
        }
    }

    /// generate monochrome image
    fn fragment(&self, u: usize, v: usize) -> u8 {
        self.fire_pixels[v * 80 + u]
    }

    pub fn start(&mut self) {
        unsafe {
            *super::wasm4::PALETTE = [0x140202, 0xd2281d, 0xe7a839, 0xeff39b];
        }
    }

    fn do_fire(&mut self) {
        for j in 0..79 {
            for i in 0..80 {
                let idx = j * 80 + i;
                let next_idx = idx + 80;
                let idx = (idx as i16 + self.rnd_gen.gen_range_i(-2..=2)).max(0) as usize;
                let next_pix = self.fire_pixels[next_idx];
                if next_pix == 0 {
                    self.fire_pixels[idx] = 0;
                } else if self.rnd_gen.gen_range(0..=99) >= 55 {
                    self.fire_pixels[idx] = next_pix.max(1) - 1;
                } else {
                    self.fire_pixels[idx] = next_pix;
                }
            }
        }
        for i in 0..80 {
            let idx = 79 * 80 + i;
            self.fire_pixels[idx] = if self.rnd_gen.gen_range(0..=99) >= 71 {
                self.rnd_gen.gen_range(9..=11) as u8
            } else {
                12
            };
        }
    }

    pub fn update(&mut self) {
        if self.current_frame % 4 == 0 {
            self.do_fire();
        }
        self.current_frame += 1;
    }

    pub fn render(&self) {
        let mut offset_0 = 0;
        let mut offset_1 = 40;
        for j in 0..80 {
            for i in (0..80).step_by(2) {
                let (f0, f1) = (self.fragment(i, j), self.fragment(i+1, j));
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