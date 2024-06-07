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

use crate::application::AlbumsApplication;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct AlbumsApplicationWindow(ObjectSubclass<imp::AlbumsApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

#[gtk::template_callbacks]
impl AlbumsApplicationWindow {
    pub fn new(application: &AlbumsApplication) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn app(&self) -> Option<AlbumsApplication> {
        self.application().and_downcast()
    }

    fn setup_gactions(&self) {
        let settings_action = gio::ActionEntry::builder("settings")
            .activate(move |win: &Self, _, _| {
                // User can trigger this action from a viewer navigation page,
                // in which we have to ensure all nav pages are popped back
                // to the root page so the user can see the preferences page.
                win.imp().window_navigation.pop_to_tag("window");
                win.imp().master_stack.set_visible_child_name("preferences");
            })
            .build();

        self.add_action_entries([settings_action]);
    }

    #[template_callback]
    fn master_stack_child_visible(&self) {
        let media_grid_imp = self.imp().library_view.imp().media_grid.imp();

        if let Some(child_name) = self.imp().master_stack.visible_child_name() {
            if child_name == self.imp().library_page.name().unwrap() {
                // If the photo grid has no model, load the photo library now.
                if media_grid_imp.photo_grid_view.model().is_none() {
                    self.imp().library_view.load_library();
                }
            }
        }
    }
}
