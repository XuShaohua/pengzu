// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/util.h"

bool IsPlainCipPage(const QString& text) {
  return text.contains("图书在版编目") && text.contains("中国版本图书馆");
}