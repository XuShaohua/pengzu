// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "controllers/shell.h"

#include <QCommandLineOption>
#include <QCommandLineParser>
#include <QDebug>
#include <QDir>
#include <QDirIterator>
#include <QFileInfo>

#include "formats/epub_parser.h"
#include "formats/mobi_parser.h"
#include "formats/pdf_parser.h"

bool ParseCmdlineOption(const QStringList& args) {
  QCommandLineParser parser;
  parser.setApplicationDescription("Parse CIP metadata from ebook files");
  const auto help_option = parser.addHelpOption();
  const auto version_option = parser.addVersionOption();
  const QCommandLineOption dir_option({"d", "dir"}, "Read directory recursively");
  parser.addOption(dir_option);
  parser.addPositionalArgument("path", "Path to ebook file or directory");

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
  if (parser.isSet(dir_option)) {
    for (const QString& path: positionalArgs) {
      if (!ParseEbookDirectory(path)) {
        qWarning() << "Failed to parse files in directory:" << path;
        ok = false;
      }
    }
  } else {
    for (const QString& filepath: positionalArgs) {
      if (!ParseEbookFile(filepath)) {
        qWarning() << "Failed to parse file:" << filepath;
        ok = false;
      }
    }
  }

  return ok;
}

bool ParseEbookDirectory(const QString& path) {
  const QFileInfo dir_info(path);
  if (!dir_info.isDir()) {
    qWarning() << "Not a directory:" << path;
    return false;
  }
  if (!dir_info.isReadable() || !dir_info.isExecutable()) {
    qWarning() << "Directory not accessible:" << path;
    return false;
  }

  QDirIterator it(path, QDirIterator::Subdirectories);
  qDebug() << Q_FUNC_INFO << path;
  while (it.hasNext()) {
    const QString filepath = it.next();
    const QFileInfo info(filepath);
    if (info.isFile()) {
      qDebug() << filepath;
      ParseEbookFile(filepath);
    }
  }

  return true;
}

bool ParseEbookFile(const QString& filepath) {
  QFileInfo info(filepath);
  if (!info.exists()) {
    qWarning() << "File not found:" << filepath;
    return false;
  }
  if (!info.isFile()) {
    qWarning() << "Not a generic file:" << filepath;
    return false;
  }
  if (!info.isReadable()) {
    qWarning() << "File not readable:" << filepath;
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
