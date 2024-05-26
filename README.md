# rtl-sdr-rust

`cargo run --bin rtl_test`

Logging is implemented with `env_logger`.  Set the log level via the environment variable `RUST_LOG`.  Valid values are info, warn, error, debug, trace.

`udev` rules are required to allow a non-root user to claim the usb interface.  Copy `rtl-sdr.rules` to `/etc/udev/rules.d/99-rtl-sdr.rules` and run `sudo udevadm control --reload-rules && sudo udevadm trigger`