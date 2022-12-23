use bevy::prelude::*;
use crate::libs::sprite::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

const PLAYER_SPEED: f32 = 2.5;
const PLAYER_SCALE: f32 = 2.0;
const PLAYER_SPRITE_SPEED: f32 = 0.2;

#[derive(Component)]
struct PlayerComponent {
    sprite_direction: SpriteDirection,
    sprite_handles: PlayerSpriteHandles,
    animation_timer: AnimationTimer,
}

struct PlayerSpriteHandles {
    pub idle_right: Handle<TextureAtlas>,
    pub idle_up: Handle<TextureAtlas>,
    pub idle_down: Handle<TextureAtlas>,
    pub run_right: Handle<TextureAtlas>,
    pub run_up: Handle<TextureAtlas>,
    pub run_down: Handle<TextureAtlas>,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let (player_sprite_handles, texture_atlas_handle) = load_player_sprites(&asset_server, texture_atlases);

    let player = PlayerComponent {
        sprite_direction: SpriteDirection::Down,
        sprite_handles: player_sprite_handles,
        animation_timer: AnimationTimer {
            timer: Timer::from_seconds(PLAYER_SPRITE_SPEED, TimerMode::Repeating)
        }
    };

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(PLAYER_SCALE)),
        ..default()
    })
    .insert(player);
}

fn load_player_sprites(
    asset_server: &Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) -> (PlayerSpriteHandles, Handle<TextureAtlas>) {
    let idle_right_sprite = asset_server.load("sprites/char_idle_right.png");
    let idle_up_sprite = asset_server.load("sprites/char_idle_up.png");
    let idle_down_sprite = asset_server.load("sprites/char_idle_down.png");
    let run_right_sprite = asset_server.load("sprites/char_run_right.png");
    let run_up_sprite = asset_server.load("sprites/char_run_up.png");
    let run_down_sprite = asset_server.load("sprites/char_run_down.png");
    
    let idle_right_atlas = TextureAtlas::from_grid(idle_right_sprite, Vec2::new(24.0, 32.0), 4, 1, None, None);
    let idle_up_atlas = TextureAtlas::from_grid(idle_up_sprite, Vec2::new(24.0, 32.0), 4, 1, None, None);
    let idle_down_atlas = TextureAtlas::from_grid(idle_down_sprite, Vec2::new(24.0, 32.0), 4, 1, None, None);
    let run_right_atlas = TextureAtlas::from_grid(run_right_sprite, Vec2::new(24.0, 32.0), 4, 1, None, None);
    let run_up_atlas = TextureAtlas::from_grid(run_up_sprite, Vec2::new(24.0, 32.0), 4, 1, None, None);
    let run_down_atlas = TextureAtlas::from_grid(run_down_sprite, Vec2::new(24.0, 32.0), 4, 1, None, None);

    let idle_right = texture_atlases.add(idle_right_atlas);
    let idle_up = texture_atlases.add(idle_up_atlas);
    let idle_down = texture_atlases.add(idle_down_atlas);
    let run_right = texture_atlases.add(run_right_atlas);
    let run_up = texture_atlases.add(run_up_atlas);
    let run_down = texture_atlases.add(run_down_atlas);

    let player_sprite_handles = PlayerSpriteHandles { 
        idle_right,  
        idle_up, 
        idle_down, 
        run_right, 
        run_up, 
        run_down: run_down.clone()
    };

    return (player_sprite_handles, run_down);
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut PlayerComponent, &mut Transform, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>
) {
    if let Ok((mut player, mut transform, mut sprite, mut texture_atlas_handle)) = query.get_single_mut() {
        let (x, y) = crate::libs::input::get_movement_input(&keyboard_input);

        let is_idle: bool = x == 0.0 && y == 0.0;

        update_sprite_direction(&mut player.sprite_direction, x, y);
        update_texture_atlas_handle(
            &mut texture_atlas_handle,
            &player.sprite_handles,
            &player.sprite_direction,
            &is_idle
        );
        sprite.flip_x = matches!(player.sprite_direction, SpriteDirection::Left);
        animate_sprite(
            time,
            texture_atlases,
            &mut player.animation_timer,
            &mut sprite,
            &texture_atlas_handle
        );

        transform.translation.x += (x * PLAYER_SPEED).round();
        transform.translation.y += (y * PLAYER_SPEED).round();
    }
}

fn update_texture_atlas_handle(
    texture_atlas_handle: &mut Handle<TextureAtlas>,
    sprite_handles: &PlayerSpriteHandles,
    sprite_direction: &SpriteDirection,
    is_idle: &bool,
) {
    if *is_idle {
        if matches!(sprite_direction, SpriteDirection::Left) {
            *texture_atlas_handle = sprite_handles.idle_right.clone();
            return;
        }
        if matches!(sprite_direction, SpriteDirection::Right) {
            *texture_atlas_handle = sprite_handles.idle_right.clone();
            return;
        }
        if matches!(sprite_direction, SpriteDirection::Up) {
            *texture_atlas_handle = sprite_handles.idle_up.clone();
            return;
        }
        if matches!(sprite_direction, SpriteDirection::Down) {
            *texture_atlas_handle = sprite_handles.idle_down.clone();
            return;
        }
    }

    if matches!(sprite_direction, SpriteDirection::Left) {
        *texture_atlas_handle = sprite_handles.run_right.clone();
        return;
    }
    if matches!(sprite_direction, SpriteDirection::Right) {
        *texture_atlas_handle = sprite_handles.run_right.clone();
        return;
    }
    if matches!(sprite_direction, SpriteDirection::Up) {
        *texture_atlas_handle = sprite_handles.run_up.clone();
        return;
    }
    if matches!(sprite_direction, SpriteDirection::Down) {
        *texture_atlas_handle = sprite_handles.run_down.clone();
        return;
    }
}

