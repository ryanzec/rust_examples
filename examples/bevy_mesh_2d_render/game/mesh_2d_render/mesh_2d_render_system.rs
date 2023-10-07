use crate::game::global_resource_load::MeshMaterial;
use crate::game::mesh_2d_render::{get_tile_vertices, get_triangle_for_tile, get_uvs_for_tile};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

pub fn mesh_2d_render_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_material: Res<MeshMaterial>,
) -> () {
    let mut rng = rand::thread_rng();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let map_dimensions = (500, 500);
    let sprite_count = map_dimensions.0 * map_dimensions.1;
    let mut vertices = vec![[0.0, 0.0, 0.0]; sprite_count * 4];
    let normals = vec![[0.0, 0.0, 1.0]; sprite_count * 4];
    let mut uvs = vec![[0.0, 0.0]; sprite_count * 4];
    let mut triangles = vec![0u32; sprite_count * 6];

    for i in 0..(sprite_count) {
        let tile_index = rng.gen_range(0..4);
        let tile_position: (u32, u32) = (tile_index % 2, tile_index / 2);

        // needed for vertices and uvs since each sprite needs 4 per
        let index_offset_4 = i * 4;

        // needed for vertices and uvs since each sprite needs 4 per
        let index_offset_6 = i * 6;

        let tile_vertices = get_tile_vertices(i as u32);
        let tile_uvs = get_uvs_for_tile(tile_position.0, tile_position.1);
        let tile_triangles = get_triangle_for_tile(i as u32);

        // add the vertices for the 2 triangles that will make up the sprite
        vertices[index_offset_4 + 0] = tile_vertices[0];
        vertices[index_offset_4 + 1] = tile_vertices[1];
        vertices[index_offset_4 + 2] = tile_vertices[2];
        vertices[index_offset_4 + 3] = tile_vertices[3];

        uvs[index_offset_4 + 0] = tile_uvs[0];
        uvs[index_offset_4 + 1] = tile_uvs[1];
        uvs[index_offset_4 + 2] = tile_uvs[2];
        uvs[index_offset_4 + 3] = tile_uvs[3];

        triangles[index_offset_6 + 0] = tile_triangles[0];
        triangles[index_offset_6 + 1] = tile_triangles[1];
        triangles[index_offset_6 + 2] = tile_triangles[2];
        triangles[index_offset_6 + 3] = tile_triangles[3];
        triangles[index_offset_6 + 4] = tile_triangles[4];
        triangles[index_offset_6 + 5] = tile_triangles[5];
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
