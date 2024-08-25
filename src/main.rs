use macroquad::audio::{load_sound, play_sound_once};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use player::Player;

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
    ACTIVE, INACTIVE, PAUSED
}

#[macroquad::main(window_conf)]
async fn main() {
    const LASER_SCALE: f32 = 0.6;
    const ENEMY_SCALE: f32 = 0.5;

    let mut game_state = GameState::ACTIVE;
    let mut hit_dur = 0;
    let mut hit_toggle = false;
    let mut score = 0;
    let mut highest_score = 0;

    let mut player_proj = Vec::new();
    let mut enemy_proj: Vec<Vec2> = Vec::new();
    let mut enemies = Vec::new();

    set_pc_assets_folder("assets");

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
    let mut player = Player::new(load_texture("images/png/player.png").await.unwrap(),
                                 200., 200., 3, 0.5);
    let enemy = load_texture("images/png/enemyShip.png").await.unwrap();

    reset_enemies(&mut enemies);

    'outer: loop {
        if game_state == GameState::ACTIVE {
            let (mut pi, mut ei) = (0, 0);

            player.update(5.);
            if is_key_pressed(KeyCode::Space) {
                player_proj.push(Vec2::new(player.x + player.width() / 2., player.y));
                play_sound_once(&gun_sound);
            }
            if is_key_pressed(KeyCode::Escape) {
                game_state = GameState::PAUSED;
                continue;
            }

            clear_background(BLACK);

            while pi < player_proj.len() {
                let mut j = 0;
                let mut hit = false;

                draw_texture_ex(&laser, player_proj[pi].x, player_proj[pi].y, WHITE, texture_params!(laser, LASER_SCALE));
                player_proj[pi].y -= 9.;

                while j < enemies.len() {
                    let enemy_rec = dest_rec(&enemies[j], &enemy, ENEMY_SCALE);
                    let player_laser_rec = dest_rec(&player_proj[pi], &laser, LASER_SCALE);

                    if enemy_rec.overlaps(&player_laser_rec) {
                        play_sound_once(&enemy_hit_sound);
                        let x = gen_range(50., screen_width() - 50.);
                        enemies[j].x = x;
                        enemies[j].y -= gen_range(800., 1000.);
                        hit = true;
                        score += 1;
                    }
                    j += 1;
                }

                match player_proj[pi].y + laser.height() * LASER_SCALE < 0. || hit {
                    true => { player_proj.remove(pi); },
                    false => pi += 1
                }
            }

            while ei < enemy_proj.len() {
                let x = enemy_proj[ei].x;
                let mut hit = false;

                draw_texture_ex(&laser_en, x, enemy_proj[ei].y, WHITE, texture_params!(laser_en, LASER_SCALE));
                enemy_proj[ei].y += 5.;
                let y = enemy_proj[ei].y;

                if hit_dur == 0 {
                    let rec = Rect::new(x, y, scale_width(&laser_en, LASER_SCALE), scale_height(&laser_en, LASER_SCALE));
                    if rec.overlaps(&player.dest_rect()) {
                        play_sound_once(&player_hit_sound);
                        if player.lives_left==0 {
                            game_state = GameState::INACTIVE;
                            continue 'outer;
                        }
                        player.lives_left -= 1;
                        hit = true;
                        hit_dur = 160;
                    }
                }

                match enemy_proj[ei].y > screen_height() || hit {
                    true => { enemy_proj.remove(ei); },
                    false => ei += 1
                }
            }
            let lives_text = format!("Lives: {}", player.lives_left);
            let score_text = format!("Score: {}", score);

            match hit_dur != 0 {
                true => {
                    if hit_dur % 20 == 0 { hit_toggle = !hit_toggle };
                    match hit_toggle {
                        true => player.draw(Color::new(0., 0., 0., 0.)),
                        false => player.draw(BLUE)
                    }
                    hit_dur -= 1;
                },
                false => player.draw(BLUE)
            }

            enemies.iter_mut().for_each(|en| {
                en.y += 1.;

                if en.y % 100. < 1. && en.y > 0. {
                    enemy_proj.push(Vec2::new(en.x + scale_width(&enemy, ENEMY_SCALE) / 2., en.y));
                }

                if en.y > screen_height() {
                    let x = gen_range(50., screen_width() - 50.);
                    en.x = x;
                    en.y -= gen_range(800., 1000.);
                }
                draw_texture_ex(&enemy, en.x, en.y, WHITE, texture_params!(enemy, ENEMY_SCALE));
            });
            draw_text_ex(lives_text.as_str(), 10., 30., text_params!(font, 40, WHITE));
            draw_text_ex(score_text.as_str(), screen_width()-120., 30., text_params!(font, 40, WHITE));
        }
        else if game_state == GameState::PAUSED {
            let game_paused_dim = measure_text("GAME PAUSED", Some(&font), 100, 1.);
            let esc_dim = measure_text("Press space to resume", Some(&font), 65, 1.);
            let exit_dim = measure_text("Press Q to exit", Some(&font), 80, 1.);

            clear_background(BLACK);
            draw_text_ex("GAME PAUSED", (screen_width()-game_paused_dim.width)*0.5,
                         screen_height()*0.5-game_paused_dim.height*2.5, text_params!(font, 100, ORANGE));
            draw_text_ex("Press space to resume", (screen_width()-esc_dim.width)*0.5,
                         screen_height()*0.5-esc_dim.height*0.1, text_params!(font, 65, DARKPURPLE));
            draw_text_ex("Press Q to exit", (screen_width()-exit_dim.width)*0.5,
                         screen_height()*0.5+exit_dim.height*1.5, text_params!(font, 80, DARKPURPLE));

            if is_key_pressed(KeyCode::Space) {
                game_state = GameState::ACTIVE;
            }
            else if is_key_pressed(KeyCode::Q) {
                break;
            }
        }
        else {
            if score> highest_score {
                highest_score = score;
            }

            let score_text = format!("Highest Score: {}", highest_score);
            let game_over_dim = measure_text("GAME OVER", Some(&font), 100, 1.);
            let score_dim = measure_text(score_text.as_str(), Some(&font), 80, 1.);
            let continue_dim = measure_text("Press space to continue...", Some(&font), 58, 1.);

            clear_background(BLACK);
            draw_text_ex("GAME OVER!", (screen_width()-game_over_dim.width)*0.5,
                         screen_height()*0.5-game_over_dim.height*2.5, text_params!(font, 100, RED));
            draw_text_ex(score_text.as_str(), (screen_width()-score_dim.width)*0.5,
                         screen_height()*0.5-score_dim.height*0.1, text_params!(font, 80, DARKPURPLE));
            draw_text_ex("Press space to continue...", (screen_width()-continue_dim.width)*0.5,
                         screen_height()*0.5+continue_dim.height*1.5, text_params!(font, 58, DARKPURPLE));

            if is_key_pressed(KeyCode::Space) {
                score = 0;
                player.lives_left = 3;
                player.x = 200.;
                player.y = 200.;
                player_proj.clear();
                enemy_proj.clear();
                reset_enemies(&mut enemies);

                game_state = GameState::ACTIVE;
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
    texture.width()*scale
}

fn scale_height(texture: &Texture2D, scale: f32) -> f32 {
    texture.height()*scale
}

fn dest_rec(vec: &Vec2, texture: &Texture2D, scale: f32) -> Rect {
    Rect::new(vec.x, vec.y, texture.width()*scale, texture.height()*scale)
}

fn reset_enemies(enemies: &mut Vec<Vec2>) {
    enemies.clear();
    for i in 1..=10 {
        let index = i as f32;
        let x = gen_range(50., screen_width()-50.);
        let y = gen_range(index*-150., index*-50.);
        enemies.push(Vec2::new(x, y));
    }
}