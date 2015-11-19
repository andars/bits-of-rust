##Bits of Rust

This is a clone of the "Just a Bit" Pebble watchface, written in rust using
pebble.rs. No clue if it works on anything but aplite without modification.


![screenshot](screenshot.png)


##Building
In order to build, clone [pebble.rs](https://github.com/andars/pebble.rs) to
somewhere on your computer and modify the `path` in Cargo.toml as appropriate.

Don't be fooled by the Cargo.toml, building is actually accomplished with
`./build.sh` rather than `cargo build`. Cargo is only used to build
dependencies.

**See the note about patching the pebble sdk on the pebble.rs readme.**

##Questions I ask myself but no one else has actually asked yet:

* **So this is all memory-safe and stuff because rust, right?**

  Lol no. Take note of all the `unsafe` blocks, both here and in `pebble.rs`,
  and then realize they were written by me. I just recently fixed a bug in
  which I accidentally dereferenced an integer. Not sure if any of rust's
  benefits have survived.

* **Why on earth do I do [this](https://github.com/andars/bits-of-rust/blob/399577414a797fbc79277fd02e3c6bc3479320cb/wscript#L48)
  and [this](https://github.com/andars/bits-of-rust/blob/master/build.sh#L14) instead of some legitimate build process?**

  Reasons. Number one reason is that I can still use `pebble build` because
  I didn't want to have to rewrite pebble's build process. Also, I still have
  no clue how `waf` works and it seems not-awesome.

* **This is seeming less and less worth the trouble...**

  Hmm yeah you might be right. It *does* however tell the time, albeit in the
  cryptic encoding of the 'just a bit' face. Also, if I can figure out how to
  write a wrapper library that is actually safe, it might turn out pretty well
  in the end.

##Size comparison

In my builds, the C implementation of this watchface clocks in at 920 bytes, and
this version is (currently) at ~~1674~~ 1328 bytes. There are probably some tricks I could 
find to decrease that, because I haven't tried anything past rustc's `opt-level` yet.

#reminder
![electrical](http://binscorner.com/mails/f/fwd-no-problem-there-i-fixed-it/part-008.jpeg)
