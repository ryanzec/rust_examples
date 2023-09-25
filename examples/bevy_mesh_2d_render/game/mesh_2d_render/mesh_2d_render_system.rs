use crate::game::global_resource_load::MeshMaterial;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const SPRITE_SIZE: f32 = 64.0;
const TILE_TEXTURE_SIZE: f32 = 0.5;

pub fn mesh_2d_render_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_material: Res<MeshMaterial>,
) -> () {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let map_dimensions = (102, 102);
    let sprite_count = map_dimensions.0 * map_dimensions.1;
    let mut vertices = vec![[0.0, 0.0, 0.0]; sprite_count * 4];
    let normals = vec![[0.0, 0.0, 1.0]; sprite_count * 4];
    let mut uvs = vec![[0.0, 0.0]; sprite_count * 4];
    let mut triangles = vec![0u32; sprite_count * 6];

    for i in 0..(sprite_count) {
        let tile_index = i % 4;

        let tile_position: (usize, usize) = (tile_index % 2, tile_index / 2);
        let position_x_offset: f32 = (i % map_dimensions.0) as f32 * SPRITE_SIZE;
        let position_y_offset: f32 = (i / map_dimensions.0) as f32 * SPRITE_SIZE;

        // needed for vertices and uvs since each sprite needs 4 per
        let index_offset_4 = i * 4;

        // needed for vertices and uvs since each sprite needs 4 per
        let index_offset_6 = i * 6;
        let triangle_index_offset = (i + 1) * 4;

        // add the vertices for the 2 triangles that will make up the sprite
        vertices[index_offset_4 + 0] = [position_x_offset, position_y_offset, 0.0];
        vertices[index_offset_4 + 1] = [position_x_offset + SPRITE_SIZE, position_y_offset, 0.0];
        vertices[index_offset_4 + 2] = [position_x_offset, position_y_offset + SPRITE_SIZE, 0.0];
        vertices[index_offset_4 + 3] = [
            position_x_offset + SPRITE_SIZE,
            position_y_offset + SPRITE_SIZE,
            0.0,
        ];

        // uvs 0x0 is located at top left of the texture
        uvs[index_offset_4 + 0] = [
            tile_position.0 as f32 * TILE_TEXTURE_SIZE,
            tile_position.1 as f32 * TILE_TEXTURE_SIZE + TILE_TEXTURE_SIZE,
        ];
        uvs[index_offset_4 + 1] = [
            tile_position.0 as f32 * TILE_TEXTURE_SIZE + TILE_TEXTURE_SIZE,
            tile_position.1 as f32 * TILE_TEXTURE_SIZE + TILE_TEXTURE_SIZE,
        ];
        uvs[index_offset_4 + 2] = [
            tile_position.0 as f32 * TILE_TEXTURE_SIZE,
            tile_position.1 as f32 * TILE_TEXTURE_SIZE,
        ];
        uvs[index_offset_4 + 3] = [
            tile_position.0 as f32 * TILE_TEXTURE_SIZE + TILE_TEXTURE_SIZE,
            tile_position.1 as f32 * TILE_TEXTURE_SIZE,
        ];

        // println!(
        //     "({}) {}x{} | {:?}",
        //     tile_index, position_x_offset, position_y_offset, tile_position
        // );
        // println!(
        //     "{:?}:{:?}:{:?}:{:?}",
        //     uvs[index_offset_4 + 0],
        //     uvs[index_offset_4 + 1],
        //     uvs[index_offset_4 + 2],
        //     uvs[index_offset_4 + 3]
        // );

        // triangle 1 which is bottom right -> top right  -> top left
        triangles[index_offset_6 + 0] = (triangle_index_offset - 4) as u32;
        triangles[index_offset_6 + 1] = (triangle_index_offset - 3) as u32;
        triangles[index_offset_6 + 2] = (triangle_index_offset - 1) as u32;

        // triangle 2 which is bottom right -> top left -> bottom left
        triangles[index_offset_6 + 3] = (triangle_index_offset - 4) as u32;
        triangles[index_offset_6 + 4] = (triangle_index_offset - 1) as u32;
        triangles[index_offset_6 + 5] = (triangle_index_offset - 2) as u32;
    }

    // the positions are the vertices for the triangles
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // the indices are the ordering of the vertices to create the triangle properly
    mesh.set_indices(Some(Indices::U32(triangles)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::from_xyz(
            ((map_dimensions.0 as isize / 2) * 64 * -1) as f32,
            ((map_dimensions.1 as isize / 2) * 64 * -1) as f32,
            0.0,
        ),
        material: mesh_material.0.clone(),
        ..default()
    });
}
