// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "controllers/shell.h"

#include <QCommandLineOption>
#include <QCommandLineParser>
#include <QDebug>

#include "formats/epub_reader.h"
#include "formats/mobi_reader.h"
#include "formats/pdf_reader.h"

bool ParseCmdlineOption(const QStringList& args) {
  QCommandLineParser parser;
  const auto help_option = parser.addHelpOption();
  const auto version_option = parser.addVersionOption();
  parser.setApplicationDescription("Parse CIP metadata from ebook files");
  parser.addPositionalArgument("filepath", "Path to ebook file");
  parser.process(args);
  if (parser.isSet(version_option)) {
    parser.showVersion();
  }
  if (parser.isSet(help_option)) {
    parser.showHelp(0);
  }
  const QStringList positionalArgs = parser.positionalArguments();
  if (positionalArgs.isEmpty()) {
    parser.showHelp(1);
  }

  return true;
}

bool ParsePdfFile(const QString& filepath) {
  PdfReader reader;
  if (reader.load(filepath)) {
    QString text;
    const bool ok = reader.readPage(4, text);
    return ok;
  }

  return false;
}

bool ParseMObiFile(const QString& filepath) {
  MobiReader reader;
  if (reader.load(filepath)) {
    const auto pages = reader.numPages();
    qDebug() << "page num:" << pages;
    QString text;
    const bool ok = reader.readPage(1, text);
    qDebug() << "ok:" << ok;
    qDebug() << "text:\n" << text;
    return true;
  }

  return false;
}

bool ParseEPubFile(const QString& filepath) {
  EpubReader reader;
  if (reader.load(filepath)) {
    const auto pages = reader.numPages();
    qDebug() << "page num:" << pages;
    QString text;
    const bool ok = reader.readPage(1, text);
    qDebug() << "ok:" << ok;
    qDebug() << "text:\n" << text;
    return true;
  }

  return false;
}
