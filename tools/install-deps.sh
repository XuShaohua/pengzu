#!/bin/bash
# Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

set -xe

# For cip
sudo apt install -y \
  libepub-dev \
  libmobi-dev \
  libpoppler-qt5-dev \
  libzip-dev \
  html2text

sudo apt install -y \
  libpq-dev \
  libsqlite3-dev \
  libssl-dev
