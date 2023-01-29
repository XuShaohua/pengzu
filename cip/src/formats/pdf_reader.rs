// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use lopdf::{Document, Object};
use std::path::Path;

use crate::error::{Error, ErrorKind};

const IGNORE: &[&str] = &[
    "Length",
    "BBox",
    "FormType",
    "Matrix",
    "Resources",
    "Type",
    "XObject",
    "Subtype",
    "Filter",
    "ColorSpace",
    "Width",
    "Height",
    "BitsPerComponent",
    "Length1",
    "Length2",
    "Length3",
    "PTEX.FileName",
    "PTEX.PageNumber",
    "PTEX.InfoDict",
    "FontDescriptor",
    "ExtGState",
    "Font",
    "MediaBox",
    "Annot",
];

pub struct PdfReader {
    doc: Document,
}

fn filter_func(object_id: (u32, u16), object: &mut Object) -> Option<((u32, u16), Object)> {
    if IGNORE.contains(&object.type_name().unwrap_or_default()) {
        return None;
    }
    if let Ok(d) = object.as_dict_mut() {
        d.remove(b"Font");
        d.remove(b"Resources");
        d.remove(b"Producer");
        d.remove(b"ModDate");
        d.remove(b"Creator");
        d.remove(b"ProcSet");
        d.remove(b"XObject");
        d.remove(b"MediaBox");
        d.remove(b"Annots");
        if d.is_empty() {
            return None;
        }
    }
    Some((object_id, object.clone()))
}

impl PdfReader {
    /// Open a pdf file.
    ///
    /// # Errors
    /// Returns error if this pdf file does not found or is invalid.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let doc = Document::load_filtered(path, filter_func)?;
        if doc.is_encrypted() {
            return Err(Error::from_string(
                ErrorKind::InvalidPdfFile,
                "password is missing".to_string(),
            ));
        }
        Ok(Self { doc })
    }

    /// Get number of pages in pdf file.
    #[must_use]
    pub fn pages(&self) -> u32 {
        let mut page_counter = 0;
        for _page in self.doc.page_iter() {
            page_counter += 1;
        }
        page_counter
    }

    /// Read text content of specific page.
    ///
    /// # Errors
    /// Returns error if failed to read page text.
    pub fn read_page(&self, page: u32) -> Result<String, Error> {
        let text = self.doc.extract_text(&[page])?;
        Ok(text)
    }

    /// Read text content of multiple pages.
    ///
    /// # Errors
    /// Returns error if failed to read page text.
    pub fn read_pages(&self, pages: &[u32]) -> Result<String, Error> {
        self.doc.extract_text(pages).map_err(Into::into)
    }
}
