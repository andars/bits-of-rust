##Bits of Rust

This is a clone of the "Just a Bit" Pebble watchface, written in rust using
pebble.rs. No clue if it works on anything but aplite without modification.


##Building
In order to build, clone [pebble.rs](https://github.com/andars/pebble.rs) to
somewhere on your computer and modify the `path` in Cargo.toml as appropriate.

Don't be fooled by the Cargo.toml, building is actually accomplished with
`./build.sh` rather than `cargo build`. Cargo is only used to build
dependencies.

**See the note about patching the pebble sdk on the pebble.rs readme.**

##Size comparison

In my builds, the C implementation of this watchface clocks in at 920 bytes, and
this version is (currently) at 1674 bytes. There are probably some tricks I could 
find to decrease that, because I haven't tried at all yet.

#reminder
![electrical](http://binscorner.com/mails/f/fwd-no-problem-there-i-fixed-it/part-008.jpeg)
