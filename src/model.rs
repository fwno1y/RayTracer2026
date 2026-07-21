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

/// 物理顶点缩放、旋转与平移加载函数
/// 直接在加载阶段修改 OBJ 的三角面片顶点坐标，确保在世界空间中 100% 绝对物理对齐，消灭任何包围盒相交导致的 Bug
pub fn load_scaled_obj(
    path: &std::path::PathBuf,
    default_mat: Arc<dyn Material>,
    target_size: f64,
    translation: crate::vec3::Vec3,
    rotation_y_deg: f64,
) -> HittableList {
    let mut file = File::open(path).expect("Failed to open obj file");
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read obj file");
    let mut cursor = Cursor::new(&data);

    let (models, _) = load_obj_buf(&mut cursor, &LoadOptions::default(), |_| {
        Ok((Vec::new(), ahash::AHashMap::new()))
    })
    .expect("Failed to parse OBJ");

    let mut min_p = Point3::new_vec3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max_p = Point3::new_vec3(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    let mut has_points = false;

    for model in &models {
        let pos = &model.mesh.positions;
        for i in 0..(pos.len() / 3) {
            let x = pos[3 * i] as f64;
            let y = pos[3 * i + 1] as f64;
            let z = pos[3 * i + 2] as f64;
            min_p = Point3::new_vec3(min_p.x().min(x), min_p.y().min(y), min_p.z().min(z));
            max_p = Point3::new_vec3(max_p.x().max(x), max_p.y().max(y), max_p.z().max(z));
            has_points = true;
        }
    }

    let center = if has_points {
        (min_p + max_p) * 0.5
    } else {
        Point3::new_vec3(0.0, 0.0, 0.0)
    };
    let dx = max_p.x() - min_p.x();
    let dy = max_p.y() - min_p.y();
    let dz = max_p.z() - min_p.z();
    let max_dim = dx.max(dy).max(dz);
    let scale = if max_dim > 1e-5 {
        target_size / max_dim
    } else {
        1.0
    };

    let rad = rotation_y_deg.to_radians();
    let sin_t = rad.sin();
    let cos_t = rad.cos();

    let mut list = HittableList::new();
    for model in models {
        let mesh = model.mesh;
        let indices = mesh.indices;
        let positions = mesh.positions;
        for chunk in indices.chunks(3) {
            if let [i0, i1, i2] = chunk {
                let transform = |idx: usize| -> Point3 {
                    let x = positions[3 * idx] as f64 - center.x();
                    let y = positions[3 * idx + 1] as f64 - center.y();
                    let z = positions[3 * idx + 2] as f64 - center.z();

                    let sx = x * scale;
                    let sy = y * scale;
                    let sz = z * scale;

                    // 绕 Y 轴旋转
                    let rx = cos_t * sx + sin_t * sz;
                    let rz = -sin_t * sx + cos_t * sz;

                    Point3::new_vec3(
                        rx + translation.x(),
                        sy + translation.y(),
                        rz + translation.z(),
                    )
                };
                let v0 = transform(*i0 as usize);
                let v1 = transform(*i1 as usize);
                let v2 = transform(*i2 as usize);
                list.add(Arc::new(Triangle::new(v0, v1, v2, default_mat.clone())));
            }
        }
    }
    list
}
