# Contributing to Uuid64ts

:+1::tada: First off, thanks for taking the time to contribute! :tada::+1:

## Code of Conduct

Please note that this project has a [Contributor Covenant Code of Conduct].
By participating in this project you agree to abide by its terms.
Instances of abusive, harassing, or otherwise unacceptable behavior may be
reported to the community leaders responsible for enforcement at
[dragonrun1@gmail.com](mailto:dragonrun1@gmail.com).

## Styleguides

### Documentation

TODO

### Git Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line
* When only changing documentation, include `[ci skip]` in the commit title
* Consider starting the commit message with an applicable emoji from [gitmoji].
  You can find a useful cheatsheet at [kapeli].

### Rust Styleguide

Please run
```shell
cargo fmt
```
on all code submissions.
There is a `.rustfmt.toml` file included in the root directory with a few
settings that should be picked up automatically and make your code fit right in.

All Rust code is linted with [Clippy].

Running
```shell
cargo +nightly clippy --all-features
```
will help polish the code before the CI workflow sees it and possibly starts
screaming :scream: at you and makes you :facepalm: facepalm or something because
you missed it.

### Test Styleguide

All tests have been moved into the `tests` module instead of having them inside
each file with the code as is common in Rust.
Each code module should have it's own submodule within the `tests` module for
it's tests.

Most test names are expected to start with `it_` but if `describe_` seems better
please use it.
All test function names should use _lower_snake_case_ as is common in Rust.

[Clippy]: https://github.com/rust-lang/rust-clippy
[Contributor Covenant Code of Conduct]: CODE_OF_CONDUCT.md
[gitmoji]: https://gitmoji.dev/
[kapeli]: https://kapeli.com/cheat_sheets/Gitmoji.docset/Contents/Resources/Documents/index
