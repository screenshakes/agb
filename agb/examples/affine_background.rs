#![no_std]
#![no_main]

use agb::{
    display::{
        affine::AffineMatrixBackground,
        tiled::{AffineBackgroundSize, TiledMap},
        Priority,
    },
    fixnum::{num, Num},
    include_background_gfx,
};

include_background_gfx!(affine_tiles, "3f3f74", water_tiles => 256 "examples/water_tiles.png");

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let (gfx, mut vram) = gba.display.video.tiled1();
    let vblank = agb::interrupt::VBlank::get();

    let tileset = affine_tiles::water_tiles.tiles;

    vram.set_background_palettes(affine_tiles::PALETTES);

    let mut bg = gfx.affine(Priority::P0, AffineBackgroundSize::Background32x32);

    for y in 0..32u16 {
        for x in 0..32u16 {
            bg.set_tile(&mut vram, (x, y).into(), &tileset, 1);
        }
    }

    bg.commit(&mut vram);
    bg.set_visible(true);

    let mut rotation = num!(0.);
    let rotation_increase: Num<i32, 16> = num!(0.01);

    let mut input = agb::input::ButtonController::new();

    let mut scroll_x = 0;
    let mut scroll_y = 0;

    loop {
        input.update();
        scroll_x += input.x_tri() as i32;
        scroll_y += input.y_tri() as i32;

        let scroll_pos = (scroll_x, scroll_y).into();

        rotation += rotation_increase;
        rotation = rotation.rem_euclid(1.into());

        let transformation = AffineMatrixBackground::from_scale_rotation_position(
            (0, 0).into(),
            (1, 1).into(),
            rotation,
            scroll_pos,
        );

        bg.set_transform(transformation);

        vblank.wait_for_vblank();
        bg.commit(&mut vram);
    }
}
