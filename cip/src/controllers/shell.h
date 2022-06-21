// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CIP_CIP_SRC_CONTROLLERS_SHELL_H_
#define CIP_CIP_SRC_CONTROLLERS_SHELL_H_

#include <QStringList>

bool ParseCmdlineOption(const QStringList& args);

bool ParsePdfFile(const QString& filepath);

bool ParseMObiFile(const QString& filepath);

bool ParseEPubFile(const QString& filepath);

#endif  // CIP_CIP_SRC_CONTROLLERS_SHELL_H_
