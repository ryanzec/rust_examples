use crate::game::mesh_2d_render::get_uvs_for_tile;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::sprite::Mesh2dHandle;
use rand::Rng;

pub fn mesh_2d_update_system(
    mut mesh_2d_handle_query: Query<&Mesh2dHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    mouse_button: Res<Input<MouseButton>>,
) -> () {
    if mouse_button.just_pressed(MouseButton::Left) {
        let mesh_2d_handle = &mesh_2d_handle_query.get_single().unwrap().0;
        let mut mesh = meshes.get_mut(mesh_2d_handle).unwrap();
        let mut mesh_uvs = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();

        let VertexAttributeValues::Float32x2(mesh_uvs) = mesh_uvs else {
            panic!("Unexpected vertex format, expected Float32x2.");
        };

        let mut rng = rand::thread_rng();
        let tile_texture_position = (rng.gen_range(0..2), rng.gen_range(0..2));
        let tile_position = [0, 0];
        let tile_index = tile_position[0] + (tile_position[1] * 500);
        let new_uvs = get_uvs_for_tile(tile_texture_position.0, tile_texture_position.1);

        mesh_uvs[tile_index + 0] = new_uvs[0];
        mesh_uvs[tile_index + 1] = new_uvs[1];
        mesh_uvs[tile_index + 2] = new_uvs[2];
        mesh_uvs[tile_index + 3] = new_uvs[3];
    }
}
