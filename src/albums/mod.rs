// This file is part of Albums.
//
// Copyright (c) 2024 Max Rodriguez
// All rights reserved.
//
// Albums is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Albums is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Albums.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct AlbumsView(ObjectSubclass<imp::AlbumsView>)
        @extends gtk::Widget, adw::Bin;
}

impl AlbumsView {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for AlbumsView {
    fn default() -> Self {
        Self::new()
    }
}
