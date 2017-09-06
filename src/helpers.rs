use ggez::graphics;
use ggez::{Context, GameResult};
use cgmath::Vector2;
use animation::{PlayerAnimation, PlayerAnimationStatus, LeftRightImage};
use player::Player;
use assets::Assets;
use display::Screen;
use particles::Blood;
use rand;


pub fn player1_animation(ctx: &mut Context) -> GameResult<PlayerAnimation> {
    let r = PlayerAnimation {
        walk: [
            LeftRightImage { left: sprite(ctx, "/sprite_02.png")?, right: sprite(ctx, "/sprite_07.png")? },
            LeftRightImage { left: sprite(ctx, "/sprite_03.png")?, right: sprite(ctx, "/sprite_08.png")? },
        ],
        jump: LeftRightImage { left: sprite(ctx, "/sprite_04.png")?, right: sprite(ctx, "/sprite_09.png")? },
        fall: LeftRightImage { left: sprite(ctx, "/sprite_05.png")?, right: sprite(ctx, "/sprite_10.png")? },
        stand: LeftRightImage { left: sprite(ctx, "/sprite_01.png")?, right: sprite(ctx, "/sprite_06.png")? },
        time: 0. 
    };
    Ok(r)
}

pub fn player2_animation(ctx: &mut Context) -> GameResult<PlayerAnimation> {
    let r = PlayerAnimation {
        walk: [
            LeftRightImage { left: sprite(ctx, "/sprite_13.png")?, right: sprite(ctx, "/sprite_18.png")? },
            LeftRightImage { left: sprite(ctx, "/sprite_14.png")?, right: sprite(ctx, "/sprite_19.png")? },
        ],
        jump: LeftRightImage { left: sprite(ctx, "/sprite_15.png")?, right: sprite(ctx, "/sprite_20.png")? },
        fall: LeftRightImage { left: sprite(ctx, "/sprite_16.png")?, right: sprite(ctx, "/sprite_21.png")? },
        stand: LeftRightImage { left: sprite(ctx, "/sprite_12.png")?, right: sprite(ctx, "/sprite_17.png")? },
        time: 0. 
    };
    Ok(r)
}

fn sprite(ctx: &mut Context, s: &str) -> GameResult<graphics::Image> {
    let mut llama_s = graphics::Image::new(ctx, s)?;
    llama_s.set_filter(graphics::FilterMode::Nearest);
    Ok(llama_s)
}

fn player_image(player: &mut Player) -> &graphics::Image {
    let status = if player.velocity.y < 0. {
        PlayerAnimationStatus::Falling
    } else if player.velocity.y > 0. {
        PlayerAnimationStatus::Jumping
    } else if player.velocity.x != 0. {
        PlayerAnimationStatus::Walking
    } else {
        PlayerAnimationStatus::Standing
    };
    match status {
        PlayerAnimationStatus::Walking => {
                player.animation.time += 1. / 60.; // TODO
                if player.animation.time > PlayerAnimation::WALK_ANIMATION_CYCLE * 2. {
                    player.animation.time = 0.;
                    &player.animation.walk[0].face(&player.facing)
                } else if player.animation.time > PlayerAnimation::WALK_ANIMATION_CYCLE {
                    &player.animation.walk[0].face(&player.facing)
                } else {
                    &player.animation.walk[1].face(&player.facing)
                }
            },
        PlayerAnimationStatus::Standing => &player.animation.stand.face(&player.facing),
        PlayerAnimationStatus::Jumping => &player.animation.jump.face(&player.facing),
        PlayerAnimationStatus::Falling => &player.animation.fall.face(&player.facing), 
    }
}

pub fn kill(players: &mut [Player; 2], killer_index: usize, victim_index: usize) {
    players[killer_index].score += 1;
    players[killer_index].velocity.y *= -1.0;
    players[victim_index].position = random_position();
    println!("{} killed {}", killer_index, victim_index);
}

pub fn score_text(ctx: &mut Context, score: u32, assets: &mut Assets) -> GameResult<graphics::Text> {
    graphics::Text::new(ctx, &format!("{}", score), &assets.font)
}

pub fn draw_player(ctx: &mut Context,
               player: &mut Player,
               screen: &Screen)
               -> GameResult<()> {
    let pixel_position = screen.position_to_pixel(player.position);
    let size = screen.size_to_pixel(player.size);
    let player_image = player_image(player);

    graphics::draw_ex(ctx,
                      player_image,
                      graphics::DrawParam {
                          dest: graphics::Point {
                              x: pixel_position.x as f32,
                              y: pixel_position.y as f32,
                          },
                          scale: graphics::Point {
                              x: size.x as f32 / player_image.width() as f32,
                              y: size.x as f32 / player_image.height() as f32,
                          },
                          ..Default::default()
                      })?;
    Ok(())
}

pub fn draw_blood(ctx: &mut Context,
               blood: &Blood,
               screen: &Screen, 
               assets: &mut Assets)
               -> GameResult<()> {
    let pixel_position = screen.position_to_pixel(blood.position);
    let size = screen.size_to_pixel(blood.size);
    let blood_image = &assets.blood;

    graphics::draw_ex(ctx,
                      blood_image,
                      graphics::DrawParam {
                          dest: graphics::Point {
                              x: pixel_position.x as f32,
                              y: pixel_position.y as f32,
                          },
                          scale: graphics::Point {
                              x: size.x as f32 / blood_image.width() as f32,
                              y: size.x as f32 / blood_image.height() as f32,
                          },
                          ..Default::default()
                      })?;
    Ok(())
}

pub fn random_position() -> Vector2<f64> {
    Vector2::new(rand::random::<f64>() - 0.5, 0.)
}

pub fn random_blood_particle(position: Vector2<f64>) -> Blood {
    Blood {
        position: position,
        size: Vector2::new(0.02, 0.02),
        velocity: Vector2::new((rand::random::<f64>() - 0.5) / 50. , rand::random::<f64>() / 50.),
    }
}