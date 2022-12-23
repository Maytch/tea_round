use bevy::prelude::*;

pub fn get_movement_input(keyboard_input: &Res<Input<KeyCode>>) -> (f32, f32) {
    let mut x = 0.0;
    let mut y = 0.0;
    
    if keyboard_input.pressed(KeyCode::W) {
        y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        x += 1.0;
    }

    // Account for Diagonals
    if x != 0.0 && y != 0.0 {
        x *= (2 as f32).sqrt() / (2 as f32);
        y *= (2 as f32).sqrt() / (2 as f32);
    }

    return (x, y);
}