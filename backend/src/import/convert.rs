// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use std::path::Path;
use std::process::Command;

use crate::error::{Error, ErrorKind};

/// Covert image format with `image` crate.
///
/// Does not relay on external programs.
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

/// Covert image format with image magic program.
///
/// Features:
///  - Produce smaller webp images.
///  - Higher performance.
fn convert_with_image_magic<P: AsRef<Path>>(
    path: P,
    webp_path: P,
    small_webp_path: P,
    small_width: u32,
    small_height: u32,
) -> Result<(), Error> {
    const IMAGE_MAGIC_PROGRAM: &str = "convert";
    let mut cmd = Command::new(IMAGE_MAGIC_PROGRAM)
        .arg(path.as_ref().as_os_str())
        .arg(webp_path.as_ref().as_os_str())
        .spawn()?;
    let status = cmd.wait()?;
    if !status.success() {
        return Err(Error::from_string(
            ErrorKind::IoError,
            format!("Failed to convert image from {:?} to webp", path.as_ref()),
        ));
    }
    let scale = format!("{small_width}x{small_height}");
    let mut cmd2 = Command::new(IMAGE_MAGIC_PROGRAM)
        .arg(path.as_ref().as_os_str())
        .arg("-scale")
        .arg(scale)
        .arg(small_webp_path.as_ref().as_os_str())
        .spawn()?;
    let status = cmd2.wait()?;
    if !status.success() {
        return Err(Error::from_string(
            ErrorKind::IoError,
            format!(
                "Failed to convert image from {:?} to small cover",
                path.as_ref()
            ),
        ));
    }
    Ok(())
}

pub fn convert_cover<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let webp_path = path.as_ref().with_extension("webp");
    let small_webp_path = path
        .as_ref()
        .with_file_name("small_cover")
        .with_extension("webp");
    let width = 270;
    let height = 400;
    if let Err(err) =
        convert_with_image_magic(path.as_ref(), &webp_path, &small_webp_path, width, height)
    {
        log::warn!("ImageMagic failed: {err:?}");
        convert(path.as_ref(), &webp_path, &small_webp_path, width, height)
    } else {
        Ok(())
    }
}
