// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CIP_CIP_SRC_FORMATS_EPUB_PARSER_H_
#define CIP_CIP_SRC_FORMATS_EPUB_PARSER_H_

#include <QString>

bool ParseEpubFile(const QString& filepath);

bool ParseEpubMetadata(const QString& filepath, const QString& text);

#endif  // CIP_CIP_SRC_FORMATS_EPUB_PARSER_H_
