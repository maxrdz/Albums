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

use crate::i18n::gettext_f;
use crate::utils::get_app_cache_directory;
use adw::gtk;
use adw::prelude::*;
use gettextrs::gettext;
use glib::{g_critical, g_error};
use glib_macros::clone;
use gtk::{gio, glib};
use libadwaita as adw;

use crate::globals::*;
use crate::vcs::VCS_TAG;

glib::wrapper! {
    pub struct Albums(ObjectSubclass<imp::Albums>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Albums {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        // The reason we have a separate action per theme is for allowing the
        // user to be able to set the application theme via keyboard shortcuts.
        let system_theme_action = gio::ActionEntry::builder("system-theme")
            .state(true.to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    app.set_adwaita_color_scheme(adw::ColorScheme::Default);
                    app.change_action_state("dark-theme", &false.to_variant());
                    app.change_action_state("light-theme", &false.to_variant());
                    action.set_state(&true.to_variant());
                },
            )
            .build();
        let light_theme_action = gio::ActionEntry::builder("light-theme")
            .state(false.to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    app.set_adwaita_color_scheme(adw::ColorScheme::ForceLight);
                    app.change_action_state("system-theme", &false.to_variant());
                    app.change_action_state("dark-theme", &false.to_variant());
                    action.set_state(&true.to_variant());
                },
            )
            .build();
        let dark_theme_action = gio::ActionEntry::builder("dark-theme")
            .state(false.to_variant())
            .activate(
                move |app: &Self, action: &gio::SimpleAction, _: Option<&glib::Variant>| {
                    app.set_adwaita_color_scheme(adw::ColorScheme::ForceDark);
                    app.change_action_state("system-theme", &false.to_variant());
                    app.change_action_state("light-theme", &false.to_variant());
                    action.set_state(&true.to_variant());
                },
            )
            .build();

        let choose_album_dir_action = gio::ActionEntry::builder("choose-library-directory")
            .activate(move |_: &Self, _, _| ())
            .build();
        let configure_action = gio::ActionEntry::builder("configure")
            .parameter_type(Some(glib::VariantTy::INT32))
            .activate(move |_: &Self, _, _| ())
            .build();
        let clear_cache_action = gio::ActionEntry::builder("clear-app-cache")
            .activate(move |app: &Self, _, _| app.show_clear_app_cache_prompt())
            .build();

        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        self.add_action_entries([
            system_theme_action,
            light_theme_action,
            dark_theme_action,
            choose_album_dir_action,
            configure_action,
            clear_cache_action,
            about_action,
            quit_action,
        ]);
    }

    fn set_adwaita_color_scheme(&self, color_scheme: adw::ColorScheme) {
        let adw_style_manager: adw::StyleManager = adw::StyleManager::default();
        adw_style_manager.set_color_scheme(color_scheme);
    }

    fn show_clear_app_cache_prompt(&self) {
        let window: gtk::Window = self.active_window().unwrap();

        let alert_dialog: adw::AlertDialog = adw::AlertDialog::builder()
            .heading(gettext("Clear App Cache?"))
            .body(gettext("Are you sure you want to clear Albums' cache? This may result in a slower start up on the next launch."))
            .build();

        alert_dialog.add_responses(&[("cancel", &gettext("Cancel")), ("clear", &gettext("Clear Cache"))]);
        alert_dialog.set_response_appearance("clear", adw::ResponseAppearance::Destructive);

        alert_dialog.connect_response(
            None,
            clone!(@weak self as s => move |_: &adw::AlertDialog, response: &str| {
                if response == "clear" {
                    glib::spawn_future_local(async move {
                        let app_cache_dir: String = get_app_cache_directory();

                        if let Err(io_error) = async_fs::remove_dir_all(&app_cache_dir).await {
                            match io_error.kind() {
                                std::io::ErrorKind::NotFound => (),
                                std::io::ErrorKind::PermissionDenied => g_critical!(
                                    "Application",
                                    "Insufficient permissions to clear cache directory."
                                ),
                                _ => g_error!(
                                    "Application",
                                    "Received an unexpected error kind after trying to clear the cache."
                                ),
                            }
                        }
                    });
                }
            }),
        );
        alert_dialog.present(&window);
    }

    fn show_about(&self) {
        let window: gtk::Window = self.active_window().unwrap();

        let about: adw::AboutDialog = adw::AboutDialog::builder()
            .application_icon(APP_INFO.app_id)
            .application_name(gettext("Albums"))
            .developer_name(APP_INFO.app_author)
            .version({
                if DEVELOPMENT_BUILD {
                    VCS_TAG
                } else {
                    APP_INFO.app_version
                }
            })
            .issue_url(format!("{}/issues", APP_INFO.app_repo).as_str())
            .developers(APP_INFO.authors)
            .artists(APP_INFO.artists.to_vec())
            //.documenters(APP_INFO.documenters.to_vec())
            .copyright(APP_INFO.copyright)
            .license(APP_INFO.license)
            .license_type(APP_INFO.license_type)
            .comments(format!(
                "{}\n\n{} (Git SHA1): {}",
                &gettext(
                    // TRANSLATORS: Generated POT file will have lots of whitespace.
                    // This is due to code linting. You can remove the whitespace in your PO file.
                    "A free and open source photo/video album app for Linux mobile, \
                        built with GTK4 and libadwaita, designed to be well integrated \
                        with GNOME technologies and mobile devices running Phosh.\
                        \n\nReleased under the GNU General Public License version 3.0."
                ),
                &gettext("Build Revision"),
                VCS_TAG
            ))
            .build();

        about.set_release_notes(
            "<p>\
          Initial release of Albums; Following the GNOME release schedule \
          as of GNOME version 46.1.\
        </p>",
        );

        about.add_credit_section(
            Some(&gettext("Powered by the following technologies")),
            &[
                &gettext_f(
                    "The GNOME Project {WEBSITE}",
                    &[("WEBSITE", "https://www.gnome.org")],
                ),
                "GTK https://gtk.org/",
                "Libadwaita https://gnome.pages.gitlab.gnome.org/libadwaita/",
                "FFmpeg https://ffmpeg.org/",
                "smol-rs https://github.com/smol-rs",
            ],
        );

        if let Ok(bytes) =
            gio::resources_lookup_data("/com/maxrdz/Albums/LEGAL.txt", gio::ResourceLookupFlags::NONE)
        {
            about.add_legal_section(
                &gettext("Open Source Software Licenses"),
                None,
                gtk::License::Custom,
                Some(String::from_utf8(bytes.to_vec()).unwrap().as_str()),
            );
        }
        about.present(&window)
    }
}
