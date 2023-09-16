use bevy::{
    prelude::*, 
    window::PrimaryWindow, 
    sprite::MaterialMesh2dBundle, 
    render::{
        render_resource::PrimitiveTopology,
        mesh::Indices
    }};

use crate::{
    board::coord::*,
    ui::board::BoardUITransform,
};

const ARROW_COLOR_1: Color = Color::rgba(0.96, 0.68, 0.19, 0.6);
const ARROW_COLOR_2: Color = Color::rgba(0.96, 0.24, 0.24, 0.6);
const ARROW_COLOR_3: Color = Color::rgba(0.17, 0.81, 0.32, 0.6);
const ARROW_COLOR_4: Color = Color::rgba(0.24, 0.59, 0.87, 0.6);
const ARROW_DEPTH: f32 = 0.3;
const ARROW_LINE_WIDTH: f32 = 20.0;
const ARROW_HEAD_SIZE: f32 = 8.0;
const ARROW_FLAT_HEAD: bool = true;

#[derive(Component)]
pub struct ArrowDrawer {
    pub is_drawing: bool,
    pub start_coord: Coord,
}

#[derive(Component)]
pub struct Arrow {}

pub fn spawn_arrow_drawer(
    mut commands: Commands,
) {
    commands.spawn(
        ArrowDrawer {
            is_drawing: false,
            start_coord: Coord::new(0, 0),
        }
    );
}


pub fn update_arrows(
    mut commands: Commands,
    mut arrow_drawer_query: Query<&mut ArrowDrawer>,
    arrows_query: Query<Entity, With<Arrow>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    board_transform: Res<BoardUITransform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(mpos) = window_query.single().cursor_position() {
        if let Ok(mut arrow_drawer) = arrow_drawer_query.get_single_mut() {
            if buttons.just_pressed(MouseButton::Right) {
                match board_transform.get_hovered_square(mpos) {
                    Some(sqr) => { arrow_drawer.start_coord = sqr; arrow_drawer.is_drawing = true; },
                    None => { arrow_drawer.is_drawing = false; }
                }
            }
            if arrow_drawer.is_drawing && buttons.just_released(MouseButton::Right) {
                if let Some(end_coord) = board_transform.get_hovered_square(mpos) {
                    arrow_drawer.is_drawing = false;
                    let col = if keyboard.pressed(KeyCode::ShiftLeft) { ARROW_COLOR_2 } else {
                        if keyboard.pressed(KeyCode::ControlLeft) { ARROW_COLOR_3 } else {
                            if keyboard.pressed(KeyCode::AltLeft) { ARROW_COLOR_4 } else { ARROW_COLOR_1 }}};
                    // create arrow
                    let start_pos = board_transform.pos_from_coord(arrow_drawer.start_coord);
                    let end_pos = board_transform.pos_from_coord(end_coord);

                    commands.spawn((MaterialMesh2dBundle {
                        mesh: meshes.add(create_arrow_mesh(Vec2::ZERO, end_pos - start_pos, board_transform.sqr_size).into()).into(),
                        material: materials.add(ColorMaterial::from(col)),
                        transform: Transform::from_xyz(start_pos.x, start_pos.y, ARROW_DEPTH + 0.01 + arrows_query.iter().len() as f32 * 0.01),
                        ..default()
                    }, Arrow {}));
                }
            }
            if buttons.just_pressed(MouseButton::Left) {
                for arrow_entity in arrows_query.iter() {
                    commands.entity(arrow_entity).despawn();
                }
            }
        }
    }
}

fn create_arrow_mesh(mut start: Vec2, mut end: Vec2, sqr_size: f32) -> Mesh {
    let forward = (end - start).normalize();
    let perp = forward.perp();
    let actual_head_size = ARROW_LINE_WIDTH * 2.0 + ARROW_HEAD_SIZE;
    let head_back_amount = if ARROW_FLAT_HEAD { 0.0 } else { 0.35 };
    end -= forward * actual_head_size;
    start += forward * sqr_size * 0.45;

    let vertices = [
        start - perp * ARROW_LINE_WIDTH / 2.0,
        start + perp * ARROW_LINE_WIDTH / 2.0,
        end - perp * ARROW_LINE_WIDTH / 2.0,
        end + perp * ARROW_LINE_WIDTH / 2.0,
        end + forward * actual_head_size,
        end - forward * actual_head_size * head_back_amount - perp * actual_head_size / 2.0,
        end - forward * actual_head_size * head_back_amount + perp * actual_head_size / 2.0,
    ];

    let indices = Indices::U32(vec![0, 2, 1, 1, 2, 3, 2, 5, 4, 2, 4, 3, 3, 4, 6]);

    let positions: Vec<_> = vertices.iter().map(|vert| [vert.x, vert.y, 0.0]).collect();
    let normals = vec![[0.0, 0.0, 1.0]; 7];
    let uvs = vec![[0.0, 0.0]; 7];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}