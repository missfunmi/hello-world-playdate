#![no_std]

extern crate alloc;
use crate::SpriteType::{Background, Dot, Player};
use alloc::boxed::Box;
use alloc::vec::Vec;
use anyhow::Error;
use crankstart::graphics::{rect_make, Bitmap, Graphics};
use crankstart::sprite::{Sprite, SpriteManager};
use crankstart::system::System;
use crankstart::{crankstart_game, Game, Playdate};
use crankstart_sys::{LCDBitmapFlip, PDButtons, PDRect};
use euclid::point2;

const SCREEN_WIDTH: f32 = 400.0;
const SCREEN_HEIGHT: f32 = 240.0;
const MOVE: f32 = 2.0;                    // We'll let the player move 2 pixels at a time
const PLAYER_SPRITE_WIDTH: f32 = 32.0;    // Width in pixels of player.png
const PLAYER_SPRITE_HEIGHT: f32 = 32.0;   // Height in pixels of player.png

struct HelloWorldGame {
    background_image: Bitmap,
    player: Sprite,
    dot_image: Bitmap,
    dots: Vec<Sprite>,
}

impl HelloWorldGame {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        let sprite_manager = SpriteManager::get_mut();

        // Load the background
        let background_image = Graphics::get().load_bitmap("assets/background")?;
        let background = Box::leak(Box::new(sprite_manager.new_sprite()?));
        background.set_bounds(&rect_make(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT))?;
        background.set_use_custom_draw()?;
        background.set_z_index(0)?;
        background.set_tag(Background as u8)?;
        sprite_manager.add_sprite(background)?;

        // Load the player
        let player_image = Graphics::get().load_bitmap("assets/player")?;
        let mut player = sprite_manager.new_sprite()?;
        player.set_image(player_image, LCDBitmapFlip::kBitmapUnflipped)?;
        player.move_to(200.0, 120.0)?;
        player.set_z_index(10)?;
        player.set_opaque(true)?;
        player.set_tag(Player as u8)?;
        sprite_manager.add_sprite(&player)?;

        // Load the dot
        let dot_image = Graphics::get().load_bitmap("assets/dot")?;
        let dots = Vec::new();

        let game = HelloWorldGame {
            background_image,
            player,
            dot_image,
            dots,
        };
        Ok(Box::new(game))
    }
}

impl Game for HelloWorldGame {
    fn update_sprite(
        &mut self,
        sprite: &mut Sprite,
        _playdate: &mut Playdate,
    ) -> Result<(), Error> {
        let tag = sprite.get_tag()?.into();
        match tag {
            Player => {
                let (current, _pushed, _released) = System::get().get_button_state()?;
                let current_position = sprite.get_position()?;
                let mut new_position = current_position.clone();

                if (current & PDButtons::kButtonUp) == PDButtons::kButtonUp {
                    new_position.1 -= MOVE;
                }
                if (current & PDButtons::kButtonDown) == PDButtons::kButtonDown {
                    new_position.1 += MOVE;
                }
                if (current & PDButtons::kButtonLeft) == PDButtons::kButtonLeft {
                    new_position.0 -= MOVE;
                }
                if (current & PDButtons::kButtonRight) == PDButtons::kButtonRight {
                    new_position.0 += MOVE;
                }

                new_position.0 = new_position.0.clamp(
                    0.0 + PLAYER_SPRITE_WIDTH / 2.0,
                    SCREEN_WIDTH - PLAYER_SPRITE_WIDTH / 2.0,
                );
                new_position.1 = new_position.1.clamp(
                    0.0 + PLAYER_SPRITE_HEIGHT / 2.0,
                    SCREEN_HEIGHT - PLAYER_SPRITE_HEIGHT / 2.0,
                );

                sprite.move_to(new_position.0, new_position.1)?;
            }
            _ => {}
        }

        sprite.mark_dirty()?;
        Ok(())
    }

    fn draw_sprite(
        &self,
        sprite: &Sprite,
        _bounds: &PDRect,
        _draw_rect: &PDRect,
        _playdate: &Playdate,
    ) -> Result<(), Error> {
        let tag = sprite.get_tag()?.into();
        match tag {
            Background => self
                .background_image
                .draw(point2(0, 0), LCDBitmapFlip::kBitmapUnflipped)?,
            _ => {}
        }
        Ok(())
    }

    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        let (current, pushed, _) = System::get().get_button_state()?;
        if (current & PDButtons::kButtonA) == PDButtons::kButtonA {
            let sprite_manager = SpriteManager::get_mut();
            let player_position = self.player.get_position()?;
            let mut dot = sprite_manager.new_sprite()?;
            dot.set_image(self.dot_image.clone(), LCDBitmapFlip::kBitmapUnflipped)?;
            dot.move_to(player_position.0, player_position.1-PLAYER_SPRITE_HEIGHT/2.0)?;
            dot.set_z_index(5)?;
            dot.set_opaque(true)?;
            dot.set_tag(Dot as u8)?;
            sprite_manager.add_sprite(&dot)?;
            self.dots.push(dot);
        } else if (pushed & PDButtons::kButtonB) == PDButtons::kButtonB {
            self.dots.clear();
        }
        Ok(())
    }
}

crankstart_game!(HelloWorldGame);

#[repr(u8)]
enum SpriteType {
    Background = 0,
    Player = 1,
    Dot = 2
}

impl From<u8> for SpriteType {
    fn from(tag: u8) -> Self {
        let sprite_type = match tag {
            0 => Background,
            1 => Player,
            _ => Dot
        };
        sprite_type
    }
}
