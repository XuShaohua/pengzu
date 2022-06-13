// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <QGuiApplication>

#include "pdf/pdf_reader.h"

int main(int argc, char** argv) {
  QGuiApplication app(argc, argv);

  const auto filename = QGuiApplication::arguments().at(1);
  PdfReader reader;
  if (reader.load(filename)) {
    reader.readPage(4);
  }

  return QGuiApplication::exec();
}