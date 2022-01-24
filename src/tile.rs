use std::ops::RangeInclusive;
use super::wasm4::*;

const HOR_STRIDE_FB: usize = 40;
const HOR_STRIDE_TILE: usize = 2;

const MIN_TILE_COORD: i32 = -1;
const MAX_TILE_COORD: i32 = 39;
const POSSIBLE_TILE_COORDINATE_RANGE: RangeInclusive<i32> = MIN_TILE_COORD..=MAX_TILE_COORD;

const MIN_SPRITE_COORD: i32 = -7;
const MAX_SPRITE_COORD: i32 = 159;
const POSSIBLE_SPRITE_COORDINATE_RANGE: RangeInclusive<i32> = MIN_SPRITE_COORD..=MAX_SPRITE_COORD;

/// 8x8 tile for
pub struct Tile {
    pub colors: [u8; 16],
    pub opacity_mask: [u8; 16]
}

impl Tile {
    /// blit tile at position (x, y)
    /// Since screen has resolution 160x160, logically tile positions are [0..19] for each axis.
    /// However, we also add a half tile possibility (meaning tiles could be shifted in 4 pixels
    /// by X or Y axis). So, in a result, tile would be drawn if X or Y lies in a [-1..39] range
    pub fn blit_as_tile(&self, x: i32, y: i32) {
        if !POSSIBLE_TILE_COORDINATE_RANGE.contains(&x) { return; }
        if !POSSIBLE_TILE_COORDINATE_RANGE.contains(&y) { return; }

        // Since colors are packed by four at a tile, this means FRAMEBUFFER is actually not 160x160,
        // but in fact 40x160. Our x position ideally match this, so, effectively, we would just affect
        // two bytes at a sprite in horizontal axis, or one at extreme cases:
        let (x_range_dst, x_range_src) = match x {
            MIN_TILE_COORD => (0..1, 1..2), // skip first byte
            MAX_TILE_COORD => (39..40, 0..1), // skip last byte
            _ => (x as usize.. x as usize + 2, 0..2)
        };

        // We need to unpack our vertical coordinate, since in y axis we are not doing anything "dense"
        let (y_range_dst, y_range_src) = match y {
            MIN_TILE_COORD => (0..4, 4..8), // skip first four rows
            MAX_TILE_COORD => (156..160, 0..4), // skip last four rows
            _ => (y as usize * 4 .. y as usize * 4 + 8, 0..8)
        };

        for (stride_dst, stride_src) in y_range_dst
            .zip(y_range_src)
            .map(|(d, s)| (d * HOR_STRIDE_FB, s * HOR_STRIDE_TILE)) {
            for (xd, xs) in x_range_dst.clone().zip(x_range_src.clone()) {
                unsafe {
                    let (src_value, src_mask) = (
                        *self.colors.get_unchecked(stride_src + xs),
                        *self.opacity_mask.get_unchecked(stride_src + xs)
                    );
                    let fb_value = (*FRAMEBUFFER).get_unchecked_mut(stride_dst + xd);
                    *fb_value = (*fb_value & (!src_mask)) | (src_value & src_mask);
                }
            }
        }
    }
    /// blit tile at 160x160 precision (pixelwise)
    /// this is more expensive than blit_tile
    pub fn blit_as_sprite(&self, x: i32, y: i32) {
        if !POSSIBLE_SPRITE_COORDINATE_RANGE.contains(&x) { return; }
        if !POSSIBLE_SPRITE_COORDINATE_RANGE.contains(&y) { return; }

        if x % 4 == 0 && y % 4 == 0 {
            // If we are at a position where we could draw cheaply, we will :)
            self.blit_as_tile(x / 4, y / 4);
            return;
        }

        let dst_x = x.min(MAX_SPRITE_COORD).max(0);
        let dst_y = y.min(MAX_SPRITE_COORD).max(0);

        let (src_x, span_size) = if dst_x > x { // x was lower than zero
            ((dst_x - x) as usize, 8 - (dst_x - x))
        } else if 160 - dst_x < 8 { // x was near right border of the screen
            (0, 160 - dst_x)
        } else {
            (0, 8)
        };

        let (src_y, span_count) = if dst_y > y { // y was lower than zero
            ((dst_y - y) as usize, 8 - (dst_y - y))
        } else if 160 - dst_y < 8 { // y was near bottom of the screen
            (0, 160 - dst_y)
        } else {
            (0, 8)
        };

        let (dst_x, dst_y) = (dst_x as usize, dst_y as usize);

        for (j_src, j_dst) in (src_y..src_y+span_count as usize).zip(dst_y..dst_y+span_count as usize) {
            for (i_src, i_dst) in (src_x..src_x+span_size as usize).zip(dst_x..dst_x+span_size as usize) {
                // We need to put pixel here. We will do it very ineffectively.
                // We will think about sprite drawing like an expensive operation which shouldn't occur many in the frame
                let real_src_idx = i_src / 4 + j_src * HOR_STRIDE_TILE;
                let src_mask = match i_src % 4 {
                    0 => 0b00_00_00_11,
                    1 => 0b00_00_11_00,
                    2 => 0b00_11_00_00,
                    3 => 0b11_00_00_00,
                    _ => unreachable!()
                };
                let mask_bit_is_set =
                    (unsafe { *(self.opacity_mask.get_unchecked(real_src_idx)) } & src_mask) != 0;
                if !mask_bit_is_set { continue; }

                let color_bits =
                    (unsafe { *(self.colors.get_unchecked(real_src_idx)) } & src_mask) >> ((i_src % 4) * 2);

                let xor_mask = match i_dst % 4 {
                    0 => 0b11_11_11_00,
                    1 => 0b11_11_00_11,
                    2 => 0b11_00_11_11,
                    3 => 0b00_11_11_11,
                    _ => unreachable!()
                };

                unsafe {
                    let real_dst_idx = i_dst / 4 + j_dst * HOR_STRIDE_FB;
                    let fb_value = (*FRAMEBUFFER).get_unchecked_mut(real_dst_idx);
                    *fb_value = (*fb_value & xor_mask) | color_bits << ((i_dst % 4) * 2)
                }
            }
        }
    }
}