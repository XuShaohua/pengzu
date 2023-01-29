// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/util.h"

#include <QDebug>
#include <QProcess>
#include <QTemporaryFile>

bool HtmlToText(const QString& html, QString& text) {
  QTemporaryFile temporary_file;
  temporary_file.setAutoRemove(true);
  if (!temporary_file.open()) {
    qWarning() << "Failed to open temp file:" << temporary_file.fileName();
    return false;
  }
  temporary_file.write(html.toUtf8());
  temporary_file.flush();
  QProcess process;
  process.setProgram("html2text");
  process.setArguments({"-utf8", temporary_file.fileName()});
  process.start();
  if (!process.waitForFinished()) {
    qWarning() << "html2text time out!";
    return false;
  }

  if (process.exitCode() != 0) {
    qWarning() << process.error();
    qWarning() << process.readAllStandardError();
    return false;
  }
  const QByteArray bytes = process.readAllStandardOutput();
  // Take first 2k bytes.
  text = QString::fromLocal8Bit(bytes).left(2048);
  return true;
}