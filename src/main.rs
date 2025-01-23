use macroquad::{audio, prelude::*};

async fn run_app() {
    let texture: Texture2D = load_texture("chillguy.png").await.unwrap();
    let sound1 = audio::load_sound("chillsong.wav").await.unwrap();

    audio::play_sound_once(&sound1);

    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;  // Quitter la boucle pour redémarrer l'application
        }

        clear_background(LIGHTGRAY);
        draw_texture(&texture, 0., 0., WHITE);
        draw_text("Just a Chill Guy", 20.0, 20.0, 30.0, RED);
        next_frame().await;
    }
}

#[macroquad::main("Texture")]
async fn main() {
    loop {
        run_app().await;
    }
}

