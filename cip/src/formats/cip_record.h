// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CIP_CIP_SRC_FORMATS_CIP_RECORD_H_
#define CIP_CIP_SRC_FORMATS_CIP_RECORD_H_

#include <QString>
#include <QStringList>

bool IsPlainCipPage(const QString& text);

struct CipRecord {
  QString title{};
  QString original_title{};
  QStringList authors{};
  QString publisher{};
  QString pubdate{};
  QString isbn{};
  QString category_id{};
  QString cip_id{};
  QString price{};
};

bool ParseCipFromText(const QString& text, CipRecord& record);

#endif  // CIP_CIP_SRC_FORMATS_CIP_RECORD_H_
