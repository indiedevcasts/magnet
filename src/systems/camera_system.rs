use crate::components::player::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};

pub fn fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    time: Res<Time>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;
        let (orthographic_projection, mut camera_transform) = camera_query.single_mut();
        for (_, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                // The boundaries used to clamp the camera in the level
                let (x_boundary_distance, y_boundary_distance) = (
                    orthographic_projection.right * orthographic_projection.scale,
                    orthographic_projection.top * orthographic_projection.scale,
                );
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let x = (player_translation.x).clamp(
                        x_boundary_distance,
                        level.px_wid as f32 - x_boundary_distance,
                    );
                    let y = (player_translation.y).clamp(
                        y_boundary_distance,
                        level.px_hei as f32 - y_boundary_distance,
                    );

                    let direction = Vec3::new(x, y, camera_transform.translation.z);

                    let smooth_damp = magnet::core::vec3::smooth_damp(
                        camera_transform.translation,
                        direction,
                        Vec3::ZERO,
                        0.2,
                        f32::INFINITY,
                        time.delta_seconds(),
                    );
                    camera_transform.translation = smooth_damp;
                }
            }
        }
    }
}
