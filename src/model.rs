use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec3::Point3;
use std::sync::Arc;
use tobj::LoadOptions;

#[allow(dead_code)]
pub fn load_obj(path: &str, default_mat: Arc<dyn Material>) -> HittableList {
    let mut world = HittableList::new();
    // 加载 .obj 文件（忽略材质文件，或可后续扩展）
    let (models, _materials) =
        tobj::load_obj(path, &LoadOptions::default()).expect("Failed to load OBJ file");

    for model in models {
        let mesh = model.mesh;
        let indices = mesh.indices;
        let positions = mesh.positions; // 连续三元素为 x,y,z
        // 遍历三角形
        for chunk in indices.chunks(3) {
            if let [i0, i1, i2] = chunk {
                let v0 = Point3::new_vec3(
                    positions[3 * *i0 as usize] as f64,
                    positions[3 * *i0 as usize + 1] as f64,
                    positions[3 * *i0 as usize + 2] as f64,
                );
                let v1 = Point3::new_vec3(
                    positions[3 * *i1 as usize] as f64,
                    positions[3 * *i1 as usize + 1] as f64,
                    positions[3 * *i1 as usize + 2] as f64,
                );
                let v2 = Point3::new_vec3(
                    positions[3 * *i2 as usize] as f64,
                    positions[3 * *i2 as usize + 1] as f64,
                    positions[3 * *i2 as usize + 2] as f64,
                );
                let tri = Triangle::new(v0, v1, v2, default_mat.clone());
                world.add(Arc::new(tri));
            }
        }
    }
    world
}
