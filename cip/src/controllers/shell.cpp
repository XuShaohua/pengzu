// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "controllers/shell.h"

#include <QCommandLineOption>
#include <QCommandLineParser>
#include <QDebug>
#include <QFileInfo>

#include "formats/epub_reader.h"
#include "formats/mobi_reader.h"
#include "formats/pdf_parser.h"

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

  bool ok = true;
  for (const QString& filepath: positionalArgs) {
    if (!ParseEbookFile(filepath)) {
      qWarning() << "Failed to parse file:" << filepath;
      ok = false;
    }
  }

  return ok;
}

bool ParseEbookFile(const QString& filepath) {
  QFileInfo info(filepath);
  if (!info.exists()) {
    qWarning() << "File not found:" << filepath;
    return false;
  }
  const QString extension_name = info.suffix().toLower();
  qDebug() <<  filepath << ", ext:" << extension_name;
  if (extension_name == "pdf") {
    return ParsePdfFile(filepath);
  }
  if (extension_name == "epub") {
    return ParseEpubFile(filepath);
  }
  if (extension_name == "mobi" ||
      extension_name == "azw" ||
      extension_name == "azw3") {
    return ParseMobiFile(filepath);
  }

  qWarning() << "Unsupported file:" << filepath;
  return false;
}


bool ParseMobiFile(const QString& filepath) {
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

bool ParseEpubFile(const QString& filepath) {
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
