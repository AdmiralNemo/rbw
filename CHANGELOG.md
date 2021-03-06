# Changelog

## Unreleased

### Fixed

* `rbw` now no longer requires the `XDG_RUNTIME_DIR` environment variable to be
  set.

## [0.4.1] - 2020-05-28

### Fixed

* More improved error messages.

## [0.4.0] - 2020-05-28

### Added

* Authenticator-based two-step login is now supported.

### Fixed

* Correctly handle password retries when entering an invalid password on the
  official Bitwarden server.
* Fix hang when giving an empty string to pinentry.
* The error message from the server is now shown when logging in fails.

## [0.3.5] - 2020-05-25

### Fixed

* Terminal-based pinentry methods should now work correctly (Glandos).
* Further error message improvements.

## [0.3.4] - 2020-05-24

### Fixed

* Handle edge case where a URI entry is set for a cipher but that entry has a
  null URI string (Adrien CLERC).

## [0.3.3] - 2020-05-23

### Fixed

* Set the correct default lock timeout when first creating the config file.
* Add a more useful error when `rbw` is run without being configured first.
* Don't throw an error when attempting to configure the base url before
  configuring the email.
* More improvements to error output.

## [0.3.2] - 2020-05-23

### Fixed

* Improve warning and error output a bit.

## [0.3.1] - 2020-05-23

### Fixed

* Fix option parsing for `rbw list --fields` and `rbw <add|generate> --uri`
  which was inadvertently broken in the previous release.

## [0.3.0] - 2020-05-22

### Fixed

* Better error message if the agent fails to start after daemonizing.
* Always automatically upgrade rbw-agent on new releases.
* Changing configuration now automatically drops in-memory keys (this should
  avoid errors when switching between different servers or accounts).
* Disallow setting `lock_timeout` to `0`, since this will cause the agent to
  immediately drop the decrypted keys before they can be used for decryption,
  even within a single run of the `rbw` client.

## [0.2.2] - 2020-05-17

### Fixed

* Fix syncing from the official Bitwarden server (thanks the_fdw).

### Added

* Added a couple example scripts to the repository for searching using fzf and
  rofi. Contributions and improvements welcome!

## [0.2.1] - 2020-05-03

### Fixed

* Properly maintain folder and URIs when editing an entry.

## [0.2.0] - 2020-05-03

### Added

* Multi-server support - you can now switch between multiple different
  bitwarden servers with `rbw config set base_url` without needing to
  redownload the password database each time.
* `rbw config unset` to reset configuration items back to the default
* `rbw list` and `rbw get` now support card, identity, and secure note entry
  types

### Fixed

* `rbw` is now able to decrypt secrets from organizations you are a member of.
* `rbw stop-agent` now waits for the agent to exit before returning.

### Changed

* Move to the `ring` crate for a bunch of the cryptographic functionality.
* The agent protocol is now versioned, to allow for seamless updates.

## [0.1.1] - 2020-05-01

### Fixed

* Some packaging changes.

## [0.1.0] - 2020-04-20

### Added

* Initial release
