// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <QCoreApplication>

#include "controllers/shell.h"

int main(int argc, char** argv) {
  QCoreApplication app(argc, argv);
  QCoreApplication::setApplicationName("cip-parser");
  QCoreApplication::setApplicationVersion(CIP_VERSION);

  const bool ok = ParseCmdlineOption(QCoreApplication::arguments());
  return ok ? 0 : 1;
}