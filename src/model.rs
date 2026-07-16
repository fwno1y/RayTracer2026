use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec3::Point3;
use ahash::AHashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::sync::Arc;
use tobj::{LoadOptions, load_obj_buf};

#[allow(dead_code)]
pub fn load_obj_(path: &PathBuf, default_mat: Arc<dyn Material>) -> HittableList {
    // 1. 读取整个文件到内存
    let mut file = File::open(path).expect("Failed to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file");

    // 2. 包装成 Cursor 以提供 Read trait
    let mut cursor = Cursor::new(&data);

    // 3. 使用 load_obj_buf 解析，忽略材质
    let (models, _materials) = load_obj_buf(
        &mut cursor,
        &LoadOptions::default(),
        |_| -> Result<(Vec<tobj::Material>, AHashMap<String, usize>), tobj::LoadError> {
            Ok((Vec::new(), AHashMap::new())) // 忽略所有材质文件
        },
    )
    .expect("Failed to parse OBJ data");

    let mut world = HittableList::new();

    for model in models {
        let mesh = model.mesh;
        let indices = mesh.indices;
        let positions = mesh.positions;

        for chunk in indices.chunks(3) {
            if let [i0, i1, i2] = chunk {
                let v0 = Point3::new_vec3(
                    positions[3 * (*i0 as usize)] as f64,
                    positions[3 * (*i0 as usize) + 1] as f64,
                    positions[3 * (*i0 as usize) + 2] as f64,
                );
                let v1 = Point3::new_vec3(
                    positions[3 * (*i1 as usize)] as f64,
                    positions[3 * (*i1 as usize) + 1] as f64,
                    positions[3 * (*i1 as usize) + 2] as f64,
                );
                let v2 = Point3::new_vec3(
                    positions[3 * (*i2 as usize)] as f64,
                    positions[3 * (*i2 as usize) + 1] as f64,
                    positions[3 * (*i2 as usize) + 2] as f64,
                );
                let tri = Triangle::new(v0, v1, v2, default_mat.clone());
                world.add(Arc::new(tri));
            }
        }
    }
    world
}
