use macroquad::audio::{load_sound, play_sound_once};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use macroquad::ui::{root_ui, Skin};
use player::*;

mod player;

///text_params!(font, size, color)
macro_rules! text_params {
    ($font: ident, $size: literal, $color: expr) => {
        TextParams {
            font: Some(&$font),
            font_size: $size,
            color: $color,
            ..Default::default()
        }
    };
}

#[derive(PartialEq)]
enum GameState {
    Active,
    Inactive,
    Paused,
}

#[macroquad::main(window_conf)]
async fn main() {
    const LASER_SCALE: f32 = 0.6;
    const ENEMY_SCALE: f32 = 0.5;

    let mut game_state = GameState::Active;
    let mut hit_dur = 0;
    let mut hit_toggle = false;
    let mut score = 0;
    let mut highest_score = 0;

    let mut player_proj = Vec::new();
    let mut enemy_proj: Vec<Vec2> = Vec::new();
    let mut enemies = Vec::with_capacity(10);

    set_pc_assets_folder("assets");

    let font_bytes = load_file("font/edge.ttf").await.unwrap();

    let button_style = root_ui()
        .style_builder()
        .font(&font_bytes)
        .unwrap()
        .font_size(50)
        .color_clicked(PURPLE)
        .color(PURPLE)
        .color_hovered(DARKPURPLE)
        .background_margin(RectOffset::new(10.0, 10.0, 1.0, 1.0))
        .build();
    
    let ui_skin = Skin {
        button_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    let gun_sound = load_sound("sound/gun.ogg").await.unwrap();
    let player_hit_sound = load_sound("sound/player_hit.wav").await.unwrap();
    let enemy_hit_sound = load_sound("sound/enemy_hit.wav").await.unwrap();

    /*
    TODO: play_sound() pauses the window for aound 3secs. Have to do something with it.
    let bg_music = load_sound("sound/bg.ogg").await.unwrap();
    play_sound(&bg_music, PlaySoundParams {
        looped: true,
        volume: 0.5
    });
    */

    let font = load_ttf_font("font/edge.ttf").await.unwrap();
    let laser = load_texture("images/png/laserGreen.png").await.unwrap();
    let laser_en = load_texture("images/png/laserRed.png").await.unwrap();
    let mut player = Player::new(
        load_texture("images/png/player.png").await.unwrap(),
        200.,
        200.,
        3,
        0.5,
    );
    let enemy = load_texture("images/png/enemyShip.png").await.unwrap();

    reset_enemies(&mut enemies);

    loop {
        if game_state == GameState::Active {
            player.update(5.);

            if is_key_pressed(KeyCode::Space) && hit_dur == 0 {
                player_proj.push(Vec2::new(player.x + player.width() / 2., player.y));
                play_sound_once(&gun_sound);
            }
            if is_key_pressed(KeyCode::Escape) {
                game_state = GameState::Paused;
                continue;
            }

            clear_background(BLACK);

            player_proj.retain_mut(|ppj| {
                draw_texture_ex(
                    &laser,
                    ppj.x,
                    ppj.y,
                    WHITE,
                    draw_texture_params(&laser, LASER_SCALE),
                );
                ppj.y -= 9.;

                let mut hit = false;
                enemies.iter_mut().for_each(|enm| {
                    let enemy_rec = dest_rec(enm, &enemy, ENEMY_SCALE);
                    let player_laser_rec = dest_rec(ppj, &laser, LASER_SCALE);

                    if enemy_rec.overlaps(&player_laser_rec) {
                        play_sound_once(&enemy_hit_sound);
                        enm.x = gen_range(50., screen_width() - 50.);
                        enm.y -= gen_range(800., 1000.);
                        hit = true;
                        score += 1;
                    }
                });

                !(ppj.y + laser.height() * LASER_SCALE < 0. || hit)
            });

            enemy_proj.retain_mut(|epj| {
                draw_texture_ex(
                    &laser_en,
                    epj.x,
                    epj.y,
                    WHITE,
                    draw_texture_params(&laser_en, LASER_SCALE),
                );
                epj.y += 5.;

                let mut hit = false;
                if hit_dur == 0 {
                    let rec = Rect::new(
                        epj.x,
                        epj.y,
                        scale_width(&laser_en, LASER_SCALE),
                        scale_height(&laser_en, LASER_SCALE),
                    );
                    if rec.overlaps(&player.dest_rect()) {
                        play_sound_once(&player_hit_sound);
                        if player.lives_left == 0 {
                            game_state = GameState::Inactive;
                        }
                        player.lives_left -= 1;
                        hit = true;
                        hit_dur = 160;
                    }
                }

                !(epj.y > screen_height() || hit)
            });

            let lives_text = format!("Lives: {}", player.lives_left);
            let score_text = format!("Score: {}", score);
            let score_dim = measure_text(score_text.as_str(), Some(&font), 40, 1.);

            match hit_dur != 0 {
                true => {
                    if hit_dur % 20 == 0 {
                        hit_toggle = !hit_toggle
                    };
                    match hit_toggle {
                        true => player.draw(Color::new(0., 0., 0., 0.)),
                        false => player.draw(BLUE),
                    }
                    hit_dur -= 1;
                }
                false => player.draw(BLUE),
            }

            enemies.iter_mut().for_each(|en| {
                en.y += 1.;

                if en.y % 100. < 1. && en.y > 0. {
                    enemy_proj.push(Vec2::new(
                        en.x + scale_width(&enemy, ENEMY_SCALE) / 2.,
                        en.y,
                    ));
                }
                if en.y > screen_height() {
                    en.x = gen_range(50., screen_width() - 50.);
                    en.y -= gen_range(800., 1000.);
                }

                draw_texture_ex(
                    &enemy,
                    en.x,
                    en.y,
                    WHITE,
                    draw_texture_params(&enemy, ENEMY_SCALE),
                );
            });

            draw_text_ex(lives_text.as_str(), 10., 30., text_params!(font, 40, WHITE));
            draw_text_ex(
                score_text.as_str(),
                screen_width()-score_dim.width-10.,
                30.,
                text_params!(font, 40, WHITE),
            );
        }

        else if game_state == GameState::Paused {
            let game_paused_dim = measure_text("GAME PAUSED", Some(&font), 100, 1.);

            clear_background(BLACK);
            draw_text_ex(
                "GAME PAUSED",
                (screen_width() - game_paused_dim.width) * 0.5,
                screen_height() * 0.5 - game_paused_dim.height * 2.5,
                text_params!(font, 100, ORANGE),
            );
            
            if root_ui().button(vec2(screen_width()/2.-70., 330.), "Resume") {
                game_state = GameState::Active;
            }
            else if root_ui().button(vec2(screen_width()/2.-45., 400.), "Exit") {
                break;
            }
        }

        else {
            if score > highest_score {
                highest_score = score;
            }

            let score_text = format!("Highest Score: {}", highest_score);
            let game_over_dim = measure_text("GAME OVER", Some(&font), 100, 1.);
            let score_dim = measure_text(score_text.as_str(), Some(&font), 80, 1.);

            clear_background(BLACK);
            draw_text_ex(
                "GAME OVER!",
                (screen_width() - game_over_dim.width) * 0.5,
                screen_height() * 0.5 - game_over_dim.height * 4.5,
                text_params!(font, 100, RED),
            );
            draw_text_ex(
                score_text.as_str(),
                (screen_width() - score_dim.width) * 0.5,
                screen_height() * 0.5 - score_dim.height * 2.5,
                text_params!(font, 80, DARKPURPLE),
            );

            if root_ui().button(vec2(screen_width()/2.-50., 330.), "Play") {
                score = 0;
                player.lives_left = 3;
                player.x = 200.;
                player.y = 200.;
                player_proj.clear();
                enemy_proj.clear();
                reset_enemies(&mut enemies);

                game_state = GameState::Active;
            }
            else if root_ui().button(vec2(screen_width()/2.-50., 400.), "Exit") {
                break;
            }
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Shooter".to_string(),
        window_width: 700,
        window_height: 900,
        icon: None,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

fn scale_width(texture: &Texture2D, scale: f32) -> f32 {
    texture.width() * scale
}

fn scale_height(texture: &Texture2D, scale: f32) -> f32 {
    texture.height() * scale
}

fn dest_rec(vec: &Vec2, texture: &Texture2D, scale: f32) -> Rect {
    Rect::new(
        vec.x,
        vec.y,
        texture.width() * scale,
        texture.height() * scale,
    )
}

fn reset_enemies(enemies: &mut Vec<Vec2>) {
    enemies.clear();
    for i in 1..=10 {
        let index = i as f32;
        let x = gen_range(50., screen_width() - 50.);
        let y = gen_range(index * -150., index * -50.);
        enemies.push(Vec2::new(x, y));
    }
}
