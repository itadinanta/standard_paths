//! # Standard Paths
//!
//! A Rust library providing methods for accessing standard paths
//! on the local filesystem (config, cache, user directories and etc.).
//!
//! It's a port of [QStandardPaths](https://doc.qt.io/qt-5/qstandardpaths.html)
//! class of the Qt framework.
//!
//! ### Usage
//! ```
//! extern crate standard_paths;
//!
//! use standard_paths::*;
//! use standard_paths::LocationType::*;
//!
//! fn main() {
//!     let sl = StandardPaths::new_with_names("app", "org");
//!     println!("{:?}", sl.writable_location(AppLocalDataLocation));
//! }
//! ```

#[macro_use]
mod macros;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
use unix::*;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows::*;

use std::env;
use std::path::PathBuf;


/// Enumerates the standard location type.
///
/// Is used to call
/// [StandardPaths::writable location](struct.StandardPaths.html#method.writable_location) and
/// [StandardPaths::find_executable_in_paths](struct.StandardPaths.html#method.find_executable_in_paths).
///
/// Some of the values are used to acquire user-specific paths,
/// some are application-specific and some are system-wide.
#[derive(Debug, Clone, PartialEq)]
pub enum LocationType {
    /// The user's home directory.
    ///
    /// * On Unix systems it's equal to the `$HOME` environment variable.
    /// * On the last Windows operating systems it's equal to the `%HomePath%`
    /// environment variable.
    HomeLocation,
    /// The user's desktop directory.
    DesktopLocation,
    /// The user's documents directory.
    DocumentsLocation,
    /// The directory for the user's downloaded files.
    ///
    /// This is a generic value. On Windows if no such directory exists,
    /// the directory for storing user documents is returned.
    DownloadLocation,
    /// The user's movies and videos directory.
    MoviesLocation,
    /// The user's music, recordings and other audio files directory.
    MusicLocation,
    /// The user's pictures, photos and screenshots directory.
    PicturesLocation,
    /// The user's applications directory.
    ///
    /// It might contain either executables, desktop files, or shortcuts.
    ///
    /// It's a platform-specific value.
    ApplicationsLocation,
    /// The user's fonts directory.
    FontsLocation,
    /// The directory for the runtime communication files (like Unix local sockets).
    ///
    /// This is a generic value. It could returns
    /// [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None)
    /// on some systems.
    RuntimeLocation,
    /// A directory for storing temporary files.
    ///
    /// It might be application-specific, user-specific or system-wide.
    TempLocation,
    /// The directory for the persistent data shared across applications.
    ///
    /// This is a generic value.
    GenericDataLocation,
    /// The persistent application data directory.
    ///
    /// This is an application-specific directory.
    /// On the Windows operating system, this returns the roaming path.
    AppDataLocation,
    /// The local settings directory.
    ///
    /// This is a Windows-specific value.
    /// On all other platforms, it returns the same value as
    /// [AppDataLocation](enum.LocationType.html#variant.AppDataLocation).
    AppLocalDataLocation,
    /// The directory for the user-specific cached data shared across applications.
    ///
    /// This is a generic value. It could returns
    /// [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None)
    /// from the appropriate methods if the system has no concept of shared cache.
    GenericCacheLocation,
    /// The user-specific cached data directory.
    ///
    /// This is an application-specific directory.
    AppCacheLocation,
    /// The user-specific configuration files directory.
    ///
    /// This may be either a generic value or application-specific.
    ConfigLocation,
    /// The user-specific configuration files directory.
    /// shared between multiple applications.
    ///
    /// This is a generic value.
    GenericConfigLocation,
    /// The user-specific configuration files directory.
    ///
    /// This is an application-specific value.
    AppConfigLocation
}

/// Stores application and organization names and provides all the crate methods.
pub struct StandardPaths {
    /// Application name.
    app_name: String,
    /// organization name.
    organisation_name: String
}

impl StandardPaths {

    /// Constructs a new `StandardPaths` with the application name
    /// derived from the `CARGO_PKG_NAME` variable.
    pub fn new() -> StandardPaths {
        StandardPaths {
            app_name: match env::var("CARGO_PKG_NAME") {
                Ok(name) => name,
                _ => String::new()
            },
            organisation_name: String::new()
        }
    }

    /// Constructs a new `StandardPaths` with the provided `app` and `organization` names.
    pub fn new_with_names(app: &'static str, organisation: &'static str) -> StandardPaths {
        StandardPaths {
            app_name: app.into(),
            organisation_name: organisation.into()
        }
    }

    /// Append application suffix to the `path`.
    ///
    /// For example `~/.config` -> `~/.config/org/app`.
    ///
    /// # Arguments
    /// * `path` - a mutable `PathBuf` to which the app suffix should be appended.
    fn append_organization_and_app(&self, path: &mut PathBuf) {
        if !self.organisation_name.is_empty() {
            path.push(&self.organisation_name);
        }
        if !self.app_name.is_empty() {
            path.push(&self.app_name);
        }
    }

    /// Returns the directory where files of type `location` should be written to.
    ///
    /// Note: the returned path can be a directory that does not exist.
    ///
    /// Returns [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None)
    /// if the location cannot be determined.
    ///
    /// # Arguments
    /// * `location` - location type.
    pub fn writable_location(&self, location: LocationType) -> Option<PathBuf> {
        self.writable_location_impl(location)
    }

    /// Returns all the directories of type `location`.
    ///
    /// The vector of locations is sorted by priority, starting with
    /// [self.writable location](struct.StandardPaths.html#method.writable_location)
    /// if it can be determined.
    ///
    /// Returns [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None)
    /// if no locations for the provided type are defined.
    ///
    /// # Arguments
    /// * `location` - location type.
    pub fn standard_locations(&self, location: LocationType) -> Option<Vec<PathBuf>> {
        self.standard_locations_impl(location)
    }

    /// Returns the absolute file path to the executable with `name` in the system path.
    ///
    /// It also could be used to check a path to be an executable.
    ///
    /// Internally it calls the
    /// [self.find_executable_in_paths](struct.StandardPaths.html#method.find_executable_in_paths)
    /// method with the system path as the `paths` argument. On most operating systems
    /// the system path is determined by the `PATH` environment variable.
    ///
    /// Note: on Windows the executable extensions from the `PATHEXT` environment variable
    /// are automatically appended to the `name` if it doesn't contain any extension.
    ///
    /// Returns [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None)
    /// if no executables are found or if the provided path is not executable.
    ///
    /// # Arguments
    /// * `name` - the name of the searched executable or an absolute path
    /// which should be checked to be executable.
    pub fn find_executable<S>(name: S) -> Option<Vec<PathBuf>>
    where S: Into<String> {
        let paths: Vec<PathBuf> = Vec::new();
        StandardPaths::find_executable_in_paths(name, paths)
    }

    /// Returns the absolute file path to the executable with `name` in the provided `paths`.
    ///
    /// Note: on Windows the executable extensions from the `PATHEXT` environment variable
    /// are automatically appended to the `name` if it doesn't contain any extension.
    ///
    /// Returns [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None)
    /// if no executables are found or if the provided path is not executable.
    ///
    /// # Arguments
    /// * `name` - the name of the searched executable or an absolute path
    /// which should be checked to be executable.
    /// * `paths` - the directories where to search for the executable.
    pub fn find_executable_in_paths<S>(name: S, paths: Vec<PathBuf>) -> Option<Vec<PathBuf>>
    where S: Into<String> {
        find_executable_in_paths_impl(name, paths)
    }
}
