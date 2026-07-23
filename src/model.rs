use crate::hittable_list::HittableList;
use crate::material::{DiffuseLight, Lambertian, Material, Metal};
use crate::sphere::Sphere;
use crate::texture::ImageTexture;
use crate::triangle::Triangle;
use crate::vec3::{Point3, Vec3};
use crate::vec3color::Color;
use ahash::AHashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::sync::Arc;
use tobj::{LoadOptions, load_obj_buf};

// ==========================================================
// 跨版本编译适配器 (Version Compatibility Layers for tobj)
// 用于自动抹平 tobj v3 和 v4 在材质字段上的类型差异
// ==========================================================

trait ToOptString {
    fn to_opt_string(&self) -> Option<String>;
}
impl ToOptString for String {
    fn to_opt_string(&self) -> Option<String> {
        Some(self.clone())
    }
}
impl ToOptString for Option<String> {
    fn to_opt_string(&self) -> Option<String> {
        self.clone()
    }
}

trait ToFloat3 {
    fn to_float3(&self) -> [f32; 3];
}
impl ToFloat3 for [f32; 3] {
    fn to_float3(&self) -> [f32; 3] {
        *self
    }
}
impl ToFloat3 for Option<[f32; 3]> {
    fn to_float3(&self) -> [f32; 3] {
        self.unwrap_or([0.0, 0.0, 0.0])
    }
}

trait ToF32 {
    fn to_f32(&self) -> f32;
}
impl ToF32 for f32 {
    fn to_f32(&self) -> f32 {
        *self
    }
}
impl ToF32 for Option<f32> {
    fn to_f32(&self) -> f32 {
        self.unwrap_or(0.0)
    }
}

trait IntoMaterialVec {
    fn into_material_vec(self) -> Vec<tobj::Material>;
}
impl IntoMaterialVec for Vec<tobj::Material> {
    fn into_material_vec(self) -> Vec<tobj::Material> {
        self
    }
}
impl IntoMaterialVec for Result<Vec<tobj::Material>, tobj::LoadError> {
    fn into_material_vec(self) -> Vec<tobj::Material> {
        self.unwrap_or_else(|err| {
            eprintln!(
                "警告：解析 MTL 材质文件失败，将使用兜底材质。错误: {:?}",
                err
            );
            Vec::new()
        })
    }
}

// ==========================================================
// 业务几何加载函数
// ==========================================================

#[allow(dead_code)]
pub fn load_obj_(path: &PathBuf, default_mat: Arc<dyn Material>) -> HittableList {
    let mut file = File::open(path).expect("Failed to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read file");

    let mut cursor = Cursor::new(&data);

    let (models, _materials) = load_obj_buf(
        &mut cursor,
        &LoadOptions::default(),
        |_| -> Result<(Vec<tobj::Material>, AHashMap<String, usize>), tobj::LoadError> {
            Ok((Vec::new(), AHashMap::new()))
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

pub fn add_glowing_crystal(
    world: &mut HittableList,
    center: Point3,
    scale: f64,
    rot_y_deg: f64,
    glass_mat: Arc<dyn Material>,
    glow_color: Option<Color>,
) {
    let rad = rot_y_deg.to_radians();
    let sin_t = rad.sin();
    let cos_t = rad.cos();

    let pts = [
        Vec3::new_vec3(0.0, scale * 1.7, 0.0),
        Vec3::new_vec3(0.0, -scale * 1.7, 0.0),
        Vec3::new_vec3(scale, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, scale),
        Vec3::new_vec3(-scale, 0.0, 0.0),
        Vec3::new_vec3(0.0, 0.0, -scale),
    ];

    let transform = |v: Vec3| -> Point3 {
        let rx = cos_t * v.x() + sin_t * v.z();
        let rz = -sin_t * v.x() + cos_t * v.z();
        Point3::new_vec3(rx + center.x(), v.y() + center.y(), rz + center.z())
    };

    let v0 = transform(pts[0]);
    let v1 = transform(pts[1]);
    let v2 = transform(pts[2]);
    let v3 = transform(pts[3]);
    let v4 = transform(pts[4]);
    let v5 = transform(pts[5]);

    world.add(Arc::new(Triangle::new(v0, v3, v2, glass_mat.clone())));
    world.add(Arc::new(Triangle::new(v0, v4, v3, glass_mat.clone())));
    world.add(Arc::new(Triangle::new(v0, v5, v4, glass_mat.clone())));
    world.add(Arc::new(Triangle::new(v0, v2, v5, glass_mat.clone())));

    world.add(Arc::new(Triangle::new(v1, v2, v3, glass_mat.clone())));
    world.add(Arc::new(Triangle::new(v1, v3, v4, glass_mat.clone())));
    world.add(Arc::new(Triangle::new(v1, v4, v5, glass_mat.clone())));
    world.add(Arc::new(Triangle::new(v1, v5, v2, glass_mat.clone())));

    if let Some(color) = glow_color {
        let core_light = Arc::new(DiffuseLight::from_color(color));
        world.add(Arc::new(Sphere::new(center, scale * 0.3, core_light)));
    }
}

/// 支持加载 MTL 并具备「强制哑光模式」的 OBJ 导入函数
pub fn load_scaled_obj_with_mtl(
    path: &std::path::PathBuf,
    fallback_mat: Arc<dyn Material>,
    target_size: f64,
    translation: Vec3,
    rotation_y_deg: f64,
    force_matte: bool, // 【新增控制参数】若为 true 则屏蔽高光反光，使材质实心哑光化
) -> HittableList {
    let options = tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
    };

    let (models, materials_result) = tobj::load_obj(path, &options)
        .unwrap_or_else(|err| panic!("无法加载 OBJ 文件: {}\n错误信息: {:?}", path.display(), err));

    let materials = materials_result.into_material_vec();
    let obj_directory = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let mut converted_materials: Vec<Arc<dyn Material>> = Vec::new();

    for mat in &materials {
        let diffuse = mat.diffuse.to_float3();
        let diffuse_color = Color::new_vec3(
            diffuse[0].max(0.0) as f64,
            diffuse[1].max(0.0) as f64,
            diffuse[2].max(0.0) as f64,
        );

        let mut has_texture = false;
        let mut texture_str = String::new();

        if let Some(ref diffuse_tex) = mat.diffuse_texture.to_opt_string() {
            if !diffuse_tex.trim().is_empty() {
                let texture_path = obj_directory.join(diffuse_tex);
                if texture_path.exists() {
                    if let Some(texture_str_slice) = texture_path.to_str() {
                        texture_str = texture_str_slice.to_string();
                        has_texture = true;
                    }
                }
            }
        }

        if has_texture {
            converted_materials.push(Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
                &texture_str,
            )))));
        } else if force_matte {
            // 【处理核心】强制哑光，直接输出不透光的 diffuse_color 面片
            converted_materials.push(Arc::new(Lambertian::from_color(diffuse_color)));
        } else {
            let specular = mat.specular.to_float3();
            let specular_strength =
                (specular[0].abs() + specular[1].abs() + specular[2].abs()) / 3.0;

            if specular_strength > 0.05 {
                let shininess = mat.shininess.to_f32() as f64;
                let fuzz = (1.0 - shininess / 1000.0).clamp(0.02, 0.35);
                converted_materials.push(Arc::new(Metal::new(diffuse_color, fuzz)));
            } else {
                converted_materials.push(Arc::new(Lambertian::from_color(diffuse_color)));
            }
        }
    }

    // --------------------------------------------------
    // 数据缩放变换和渲染列表输出
    // --------------------------------------------------
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut min_z = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    let mut has_points = false;

    for model in &models {
        let positions = &model.mesh.positions;
        for vertex in positions.chunks_exact(3) {
            let x = vertex[0] as f64;
            let y = vertex[1] as f64;
            let z = vertex[2] as f64;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            min_z = min_z.min(z);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            max_z = max_z.max(z);
            has_points = true;
        }
    }

    let mut result = HittableList::new();
    if !has_points {
        return result;
    }

    let center_x = (min_x + max_x) * 0.5;
    let center_y = (min_y + max_y) * 0.5;
    let center_z = (min_z + max_z) * 0.5;

    let size_x = max_x - min_x;
    let size_y = max_y - min_y;
    let size_z = max_z - min_z;
    let max_dimension = size_x.max(size_y).max(size_z);

    let scale = if max_dimension > 1e-8 && target_size > 0.0 {
        target_size / max_dimension
    } else {
        1.0
    };

    let radians = rotation_y_deg.to_radians();
    let sin_y = radians.sin();
    let cos_y = radians.cos();

    let transform_vertex = |index: usize, positions: &[f32]| -> Option<Point3> {
        let base = index.checked_mul(3)?;
        if base + 2 >= positions.len() {
            return None;
        }

        let x = (positions[base] as f64 - center_x) * scale;
        let y = (positions[base + 1] as f64 - center_y) * scale;
        let z = (positions[base + 2] as f64 - center_z) * scale;

        let rotated_x = cos_y * x + sin_y * z;
        let rotated_z = -sin_y * x + cos_y * z;

        Some(Point3::new_vec3(
            rotated_x + translation.x(),
            y + translation.y(),
            rotated_z + translation.z(),
        ))
    };

    for model in models {
        let mesh = model.mesh;
        let positions = mesh.positions;
        let indices = mesh.indices;

        let material: Arc<dyn Material> = match mesh.material_id {
            Some(material_id) => converted_materials
                .get(material_id)
                .cloned()
                .unwrap_or_else(|| fallback_mat.clone()),
            None => fallback_mat.clone(),
        };

        for triangle_indices in indices.chunks_exact(3) {
            let i0 = triangle_indices[0] as usize;
            let i1 = triangle_indices[1] as usize;
            let i2 = triangle_indices[2] as usize;

            let Some(v0) = transform_vertex(i0, &positions) else {
                continue;
            };
            let Some(v1) = transform_vertex(i1, &positions) else {
                continue;
            };
            let Some(v2) = transform_vertex(i2, &positions) else {
                continue;
            };

            result.add(Arc::new(Triangle::new(v0, v1, v2, material.clone())));
        }
    }

    result
}
