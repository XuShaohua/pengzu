// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "pdf/pdf_reader.h"

#include <poppler-qt5.h>

PdfReader::PdfReader(QObject* parent) : QObject(parent) {

}

bool PdfReader::load(const QString& filepath) {

  return false;
}
