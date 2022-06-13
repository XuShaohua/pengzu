// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <QGuiApplication>
#include <QDebug>

#include "formats/pdf_reader.h"
#include "formats/mobi_reader.h"
#include "formats/epub_reader.h"

int main(int argc, char** argv) {
  QGuiApplication app(argc, argv);

  const auto filename = QGuiApplication::arguments().at(1);
  EpubReader reader;
  if (reader.load(filename)) {
    const auto pages = reader.numPages();
    qDebug() << "page num:" << pages;
    QString text;
    const bool ok = reader.readPage(1, text);
    qDebug() << "ok:" << ok;
    qDebug() << "text:\n" << text;
  }

//  MobiReader reader;
//  if (reader.load(filename)) {
//    const auto pages = reader.numPages();
//    qDebug() << "page num:" << pages;
//    QString text;
//    const bool ok = reader.readPage(1, text);
//    qDebug() << "ok:" << ok;
//    qDebug() << "text:\n" << text;
//  }

//  PdfReader reader;
//  if (reader.load(filename)) {
//    QString text;
//    const bool _ok = reader.readPage(4, text);
//  }

  return QGuiApplication::exec();
}