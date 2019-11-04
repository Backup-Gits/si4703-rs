# Rust Si4703 FM Radio Turner (Receiver) Driver

<!--TODO
[![crates.io](https://img.shields.io/crates/v/si4703.svg)](https://crates.io/crates/si4703)
[![Docs](https://docs.rs/si4703/badge.svg)](https://docs.rs/si4703)
-->
[![Build Status](https://travis-ci.org/eldruin/si4703-rs.svg?branch=master)](https://travis-ci.org/eldruin/si4703-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/si4703-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/si4703-rs?branch=master)

This is a platform agnostic Rust driver for the Si4703 and Si4702 FM radio turners
(receivers) using the [`embedded-hal`] traits and I2C.

This driver allows you to:
- Enable/disable the device. See: `enable()`.
- Mute/unmute. See: `mute()`.
- Configure seek. See: `configure_seek()`.
- Seek with/without STC interrupts. See: `seek_with_stc_int_pin()`.
- Tune a frequency with/without STC interrupts. See: `tune_with_stc_int_pin()`.
- Set volume. See: `set_volume()`.
- Set band. See: `set_band()`.
- Set channel spacing. See: `set_channel_spacing()`.
- Set the GPIO1, GPIO2 and GPIO3 function/status. See: `set_gpio1()`.
- Enable/disable softmute. See: `enable_softmute()`.
- Enable/disable auto gain control. See: `enable_auto_gain_control()`.
- Enable/disable oscillator. See: `enable_oscillator()`.
- Enable/disable STC interrupts. See: `enable_stc_interrupts()`.
- Enable/disable RDS. See: `enable_rds()`.
- Enable/disable audio High-Z. See: `enable_audio_high_z()`.
- Set de-emphasis. See: `set_deemphasis()`.
- Set stereo to mono blend level. See: `set_stereo_to_mono_blend_level()`.
- Set stereo/mono output mode. See: `set_output_mode()`.
- Get channel. See: `get_channel()`.
- Get device ID. See: `get_device_id()`.
- Get chip ID. See: `get_chip_id()`.
- Reset and select I2C communication using several methods. See: `reset_and_select_i2c_method1()`.

<!--TODO
[Introductory blog post](https://blog.eldruin.com/...)
-->

## The devices

The Si4702/03-C19 extends Silicon Laboratories Si4700/Si4701 FM tuner
family, and further increases the ease and attractiveness of adding FM
radio reception to mobile devices through small size and board area,
minimum component count, flexible programmability, and superior, proven
performance.

The device offers significant programmability, and caters to the
subjective nature of FM listeners and variable FM broadcast environments
world-wide through a simplified programming interface and
mature functionality.

The Si4703-C incorporates a digital processor for the European Radio Data
System (RDS) and the US Radio Broadcast Data System (RBDS) including all
required symbol decoding, block synchronization, error detection, and
error correction functions.

RDS enables data such as station identification and song name to be
displayed to the user. The Si4703-C offers a detailed RDS view and a
standard view, allowing adopters to selectively choose granularity of RDS
status, data, and block errors.

Datasheets:
- [Si4702/Si4703](https://www.silabs.com/documents/public/data-sheets/Si4702-03-C19.pdf)

Further documentation:
- [Si4700/01/02/03 Programmer's Guide](https://www.silabs.com/documents/public/application-notes/AN230.pdf)
- [Using RDS/RBDS with the Si4701/03](https://www.silabs.com/documents/public/application-notes/AN243.pdf)
- [Si47xx Programming Guide](https://www.silabs.com/documents/public/application-notes/AN332.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

<!--TODO
In the following example an instance of the device Si4703 will be created.
Other devices can be created with similar methods like:
`Si4703::new_si4702(...)`.
-->
Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

<!-- TODO
```rust
```
-->

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/si4703-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
