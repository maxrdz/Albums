<img src="https://gitlab.gnome.org/maxrdz/gallery/-/raw/master/data/icons/com.maxrdz.Gallery.png" align="right" width="125em"/>

# Gallery

A free and open source photo/video gallery app for Linux mobile,
built with GTK4 and libadwaita, designed to be well integrated
with GNOME technologies and mobile devices.

## Software Requirements

- mount version 2.40+
- pango version 1.52+
- gstreamer 1.24+
- glib version 2.80+
- gio version 2.80+
- gtk4 1.13.4+
- libadwaita 1.5
- desktop-file-utils
- appstream
- appstream-glib
- Rustup (provides Cargo & rustc)
- Ninja build system
- Meson build system

## Building from Source

We use Meson/Ninja as the build system for Gallery.
The quickest way to build for release is to do the following:

```sh
meson setup build
meson compile -C build
meson install -C build
```

To build for debug:

```sh
meson setup builddevel -Dprofile=dev
meson compile -C builddevel
meson install -C builddevel
```

To uninstall the app build from your local system:
```sh
sudo ninja -C build uninstall
```
Replace `build` with the Meson build directory of the
application build version that you want to uninstall.

## Cross Compiling

For cross compiling, we use
[cross-rs](https://github.com/cross-rs/cross), which is a
drop-in replacement for Cargo. This tool allows the developer
to cross compile using **Docker** (or a Docker drop-in
replacement, such as [Podman](https://podman.io/))
instead of installing dependencies and additional pkg-conf
configuration on the build machine. On setup, Meson will check
that cross and docker, or an alternative, are installed.

To install the cross binary on your system user's cargo:
```sh
cargo install cross --git https://github.com/cross-rs/cross
```
NOTE: This will install the `cross` program under `~/.cargo/bin`.
Be sure to add `~/.cargo/bin` to your PATH so Meson can find it.

To setup a build that targets ARM64 GNU/Linux:

```sh
meson setup buildaarch64 -Dtarget=aarch64-unknown-linux-gnu
```

## License

Gallery is licensed under the GNU GPL version 3.0.
You can read the full license in the `COPYING` file.
