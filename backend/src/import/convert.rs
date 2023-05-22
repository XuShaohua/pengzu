// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{Error, ErrorKind};

const IMAGE_MAGIC_PROGRAM: &str = "convert";

/// Covert image format with `image` crate.
///
/// Does not relay on external programs.
fn convert<P: AsRef<Path>>(in_path: P, out_path: P) -> Result<(), Error> {
    let img = ImageReader::open(in_path)?;
    let img = img.decode()?;
    img.save(out_path)?;
    Ok(())
}

/// Covert image format and resize with `image` crate.
///
/// Does not relay on external programs.
fn resize<P: AsRef<Path>>(in_path: P, out_path: P, width: u32, height: u32) -> Result<(), Error> {
    let img = ImageReader::open(in_path)?;
    let img = img.decode()?;
    let small_img = img.resize(width, height, FilterType::CatmullRom);
    small_img.save(out_path)?;
    Ok(())
}

/// Covert image format with image magic program.
///
/// Features:
///  - Produce smaller webp images.
///  - Higher performance.
fn convert_im<P: AsRef<Path>>(in_path: P, out_path: P) -> Result<(), Error> {
    let in_path_ref = in_path.as_ref();
    let out_path_ref = out_path.as_ref();
    let mut cmd = Command::new(IMAGE_MAGIC_PROGRAM)
        .arg(in_path_ref.as_os_str())
        .arg(out_path_ref.as_os_str())
        .spawn()?;
    let status = cmd.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::from_string(
            ErrorKind::IoError,
            format!("Failed to convert image from {in_path_ref:?} to {out_path_ref:?}"),
        ))
    }
}

/// Resize image with image magic program.
fn resize_im<P: AsRef<Path>>(
    in_path: P,
    out_path: P,
    width: u32,
    height: u32,
) -> Result<(), Error> {
    let in_path_ref = in_path.as_ref();
    let out_path_ref = out_path.as_ref();
    let scale = format!("{width}x{height}");
    let mut cmd = Command::new(IMAGE_MAGIC_PROGRAM)
        .arg(in_path_ref.as_os_str())
        .arg("-scale")
        .arg(&scale)
        .arg(out_path_ref.as_os_str())
        .spawn()?;
    let status = cmd.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::from_string(
            ErrorKind::IoError,
            format!(
                "Failed to resize image {in_path_ref:?} to {out_path_ref:?} with scale {scale}"
            ),
        ))
    }
}

pub fn convert_cover<P: AsRef<Path>>(path: P) -> Result<(PathBuf, PathBuf), Error> {
    let webp_path = path.as_ref().with_extension("webp");
    let small_webp_path = path
        .as_ref()
        .with_file_name("small_cover")
        .with_extension("webp");
    let width = 270;
    let height = 400;

    if let Err(err) = convert_im(path.as_ref(), &webp_path) {
        log::warn!("ImageMagic convert failed: {err:?}");
        convert(path.as_ref(), &webp_path)?;
    }
    if let Err(err) = resize_im(path.as_ref(), &small_webp_path, width, height) {
        log::warn!("ImageMagic resize failed: {err:?}");
        resize(path.as_ref(), &small_webp_path, width, height)?;
    }

    Ok((webp_path, small_webp_path))
}
