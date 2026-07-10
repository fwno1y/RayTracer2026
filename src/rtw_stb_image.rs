use std::env;
use std::path::{Path, PathBuf};

pub struct RtwImage {
    image_width: u32,
    image_height: u32,
    bytes_per_pixel: u8,
    data: Vec<u8>,
}
impl RtwImage {
    const BYTES_PER_PIXEL: u8 = 3;
    pub fn empty() -> RtwImage {
        RtwImage {
            image_width: 0,
            image_height: 0,
            bytes_per_pixel: Self::BYTES_PER_PIXEL,
            data: Vec::new(),
        }
    }
    pub fn new(image_filename: &str) -> RtwImage {
        let mut image = Self::empty();
        let imagedir = env::var("RTW_IMAGES").ok();

        let search_paths = Self::build_search_paths(image_filename, imagedir);
        for path in search_paths {
            if image.load_from_path(&path) {
                return image;
            }
        }

        eprintln!("ERROR: Could not load image file '{}'.", image_filename);
        image
    }
    fn build_search_paths(filename: &str, imagedir: Option<String>) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        // 环境变量目录
        if let Some(dir) = imagedir {
            paths.push(PathBuf::from(dir).join(filename));
        }
        // 当前目录
        paths.push(PathBuf::from(filename));
        // images/ 子目录
        paths.push(PathBuf::from("images").join(filename));
        // ../images/, ../../images/, ... 最多 6 级
        for i in 1..=6 {
            let mut p = PathBuf::new();
            for _ in 0..i {
                p.push("..");
            }
            p.push("images");
            p.push(filename);
            paths.push(p);
        }
        paths
    }
    fn load_from_path(&mut self, path: &Path) -> bool {
        match image::open(path) {
            Ok(dyn_img) => {
                let rgb_img = dyn_img.to_rgb8(); // 强制转换为 RGB 8-bit
                let (w, h) = rgb_img.dimensions();
                self.image_width = w;
                self.image_height = h;
                self.data = rgb_img.into_raw(); // 获取 Vec<u8>，顺序为 R,G,B 每像素
                true
            }
            Err(_) => false,
        }
    }
    pub fn width(&self) -> i32 {
        if self.data.is_empty() {
            0
        } else {
            self.image_width as i32
        }
    }
    pub fn height(&self) -> i32 {
        if self.data.is_empty() {
            0
        } else {
            self.image_height as i32
        }
    }
    pub fn pixel_data(&self, x: u32, y: u32) -> &[u8] {
        const MAGENTA: [u8; 3] = [255, 0, 255];
        if self.data.is_empty() {
            return &MAGENTA;
        }
        let x = Self::clamp(self, x, 0, self.image_width);
        let y = Self::clamp(self, y, 0, self.image_height);
        let idx = (y * self.image_width + x) as usize * self.bytes_per_pixel as usize;
        &self.data[idx..idx + self.bytes_per_pixel as usize]
    }
    fn clamp(&self, x: u32, low: u32, high: u32) -> u32 {
        if x < low {
            return low;
        }
        if x < high {
            return x;
        }
        high - 1
    }
    #[allow(dead_code)]
    fn float_to_byte(value: f32) -> u8 {
        if value <= 0.0 {
            0
        } else if value >= 1.0 {
            255
        } else {
            (256.0 * value) as u8
        }
    }
}
