// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use std::path::Path;

use crate::error::Error;

fn convert<P: AsRef<Path>>(
    path: P,
    webp_path: P,
    small_webp_path: P,
    small_width: u32,
    small_height: u32,
) -> Result<(), Error> {
    let img = ImageReader::open(path)?;
    let img = img.decode()?;
    img.save(webp_path)?;
    let small_img = img.resize(small_width, small_height, FilterType::CatmullRom);
    small_img.save(small_webp_path)?;
    Ok(())
}

pub fn convert_cover<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let webp_path = path.as_ref().with_extension("webp");
    let small_webp_path = path
        .as_ref()
        .with_file_name("small_cover")
        .with_extension("webp");
    convert(path.as_ref(), &webp_path, &small_webp_path, 270, 400)
}
