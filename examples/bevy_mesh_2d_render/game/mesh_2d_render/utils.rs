const TILE_SIZE: f32 = 64.0;
const TILE_TEXTURE_SIZE: f32 = 0.5;
const TILESET_SIZE: f32 = 128.0;
const WORLD_SIZE: u32 = 500;

pub fn get_uvs_for_tile(tile_texture_x: u32, tile_texture_y: u32) -> Vec<[f32; 2]> {
    let mut uvs = vec![[0.0, 0.0]; 4];

    // uvs 0x0 is located at top left of the texture
    uvs[0] = [
        tile_texture_x as f32 * TILE_TEXTURE_SIZE,
        tile_texture_y as f32 * TILE_TEXTURE_SIZE + TILE_TEXTURE_SIZE,
    ];
    uvs[1] = [
        tile_texture_x as f32 * TILE_TEXTURE_SIZE + TILE_TEXTURE_SIZE,
        tile_texture_y as f32 * TILE_TEXTURE_SIZE + TILE_TEXTURE_SIZE,
    ];
    uvs[2] = [
        tile_texture_x as f32 * TILE_TEXTURE_SIZE,
        tile_texture_y as f32 * TILE_TEXTURE_SIZE,
    ];
    uvs[3] = [
        tile_texture_x as f32 * TILE_TEXTURE_SIZE + TILE_TEXTURE_SIZE,
        tile_texture_y as f32 * TILE_TEXTURE_SIZE,
    ];

    return uvs;
}

pub fn get_tile_index(tile_position_x: u32, tile_position_y: u32) -> u32 {
    return tile_position_x * (tile_position_y * WORLD_SIZE);
}

pub fn get_triangle_for_tile(tile_index: u32) -> Vec<u32> {
    let triangle_index_offset = (tile_index + 1) * 4;
    let mut triangles = vec![0, 0, 0, 0, 0, 0];

    // triangle 1 which is bottom right -> top right  -> top left
    triangles[0] = (triangle_index_offset - 4);
    triangles[1] = (triangle_index_offset - 3);
    triangles[2] = (triangle_index_offset - 1);

    // triangle 2 which is bottom right -> top left -> bottom left
    triangles[3] = (triangle_index_offset - 4);
    triangles[4] = (triangle_index_offset - 1);
    triangles[5] = (triangle_index_offset - 2);

    return triangles;
}

pub fn get_tile_vertices(tile_index: u32) -> Vec<[f32; 3]> {
    let mut vertices = vec![[0.0, 0.0, 0.0]; 4];
    let position_x_offset: f32 = (tile_index % WORLD_SIZE) as f32 * TILE_SIZE;
    let position_y_offset: f32 = (tile_index / WORLD_SIZE) as f32 * TILE_SIZE;

    // add the vertices for the 2 triangles that will make up the sprite
    vertices[0] = [position_x_offset, position_y_offset, 0.0];
    vertices[1] = [position_x_offset + TILE_SIZE, position_y_offset, 0.0];
    vertices[2] = [position_x_offset, position_y_offset + TILE_SIZE, 0.0];
    vertices[3] = [
        position_x_offset + TILE_SIZE,
        position_y_offset + TILE_SIZE,
        0.0,
    ];

    return vertices;
}
