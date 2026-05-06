# Template Information

This crate and its contents are a template that can be used by
[`cargo-generate`][1] to create a new crate for you.  **EVERY TIME** you build
your project, the contents of this file will be overwritten by the output of
the [`cargo-readme`][2] command.  **READ THE INSTRUCTIONS FOR IT SO YOU
UNDERSTAND WHERE YOU SHOULD PUT YOUR README'S CONTENTS!**  If you fail to do
so, you **will** be unhappy at some point!

# Tools That This Template Uses

This template uses numerous tools to simplify the development process.  The
tools that are currently use are described below.


## `cargo audit`

[`cargo-audit`][3] is a tool that is able to check your dependencies against
the [RustSec Advisory Database][5] for vulnerabilities.

## `cargo auditable`

[`cargo auditable`][4] embeds the versions of the crates you use to build your
binaries into the binaries themselves.  [`cargo-audit`][3] can then look within
the binaries to find what was used.  Combined with proper code signing and
other cryptographic methods, this can help downstream users verify that the
binary they are using is safe to use.

## `cargo make`

[`cargo make`][6] is a task runner.  It is configured (by default) via the
`Makefile.toml` file that is at the root of the repository.  The generated
files should be able to compile using the command `cargo make`.  If you wish to
learn about other targets that are already defined, run `cargo make help`, and
the top-level targets will be described.

## `cargo generate`

[`cargo generate`][7] This is the tool that you will use to convert this
template into a usable Rust project.  The configuration file for it is
`cargo-generate.toml`, but this file is not copied over into the output of this
tool.  In short, to find the file, you will need to clone this repository
first.

## `git cliff`

[`git cliff`][8] is a highly configurable change log file generator.  To
configure it, modify the settings in `cliff.toml` at the root of the
repository.  Note that the generated project's `Makefile.toml` (see the section
on `cargo make` above) doesn't use this tool because it only works with
[`git`][9], and you may not wish to use it with your project.  If you do want to
generate a changelog file as a part of your release process, modify the
`Makefile.toml` as appropriate to add in a new task for `git cliff.`

## `cargo readme`

[`cargo readme`][10] is a tool to generate your `README.md` file from comments
within your `lib.rs` or `main.rs` file as is appropriate.  It is configured via
template files.  The default template file in this project is `README.tpl`.  If
you use [`cargo make`][6] to build your project, then the contents of this
`README.md` file will be overwritten by the template each and every time you
build.

## `cargo udeps`

[`cargo udeps`][11] is a tool to find unused dependencies within your project.
It requires the `nightly` rust toolchain to work correctly.  If you don't
already have that toolchain installed, then when you try to build the generated
project via [`cargo make`][6] your build will fail.  You will need to either
modify `Makefile.toml` to not use this tool, or you will need to install the
`nightly` toolchain to solve this issue.

## `cargo clippy`

[`cargo clippy`][17] is a built-in tool that ships with the rust toolchains.
Look in the `lints.clippy` section of `Cargo.toml` to see what lints are
activated by default.

## `cargo deny`

[`cargo deny`][18] is another linter for your dependencies.  It is configured
via the `deny.toml` file in the root of the crate.

## ``mdbook

[`mdbook`][12] is used to generate higher-level documentation.  The template has
the start of a book in the `docs/user_guide` directory.  It is configured via
the file at `docs/book.toml`.

## `rustfmt`

[`rustfmt`][13] is part of the Rust toolchain.  This project defines a
`.rustfmt.toml` file to ensure a consistent style.  Note that the template uses
features only available on the `nightly` channel.  If you do not have the
`nightly` tools installed, then your build will fail.  You will need to either
modify the `.rustfmt.toml` file, or install the `nightly` toolchain to solve
this issue.

## `taplo`

[`taplo`][14] is multi-purpose tool for working with [TOML][15] files in the
same way that [`jq`][16] is for JSON files.  This project uses it to format all
TOML files in a standard format.  The configuration file uses is
`.taplo.toml`.

## `cargo llvm-cov`

[`cargo llvm-cov`][19] is able to generate code-coverage reports for the
project.  This is useful to verify that your tests are exercising all portions
of the code.

## `cargo deadlinks`

[`cargo deadlinks`][20] parses the output of `cargo doc` to ensure that all
links within the generated documentation have a target that exists.  Basically,
it spiders all of your generated docs to check for liveness of the targets,
producing warnings on any targets that don't exist or don't respond.

## `mdbook-linkcheck`

[`mdbook-linkcheck`][21] is another link checker, but specifically designed for
the [`mdbook`][12] tool.

## `cargo bolero`

[`cargo bolero`][22] is used to test code that uses [`bolero`][23] within its
tests.  It uses the services of different fuzzers to really exercise your code.
The generated output of this template includes the `cargo make bolero` target,
which will exercise each test it finds for 10 minutes.  This can be useful for
testing all of your code in relatively short bursts, or if you have a
low-priority process running on your CT server that runs constantly.  As the
tool runs, it will produce new files within your repository.  All of the files
it produces are kept within individual `__fuzz__` directories.  Commit these to
your repository so that your teammates can see the bugs that CT is catching.

[1]: https://crates.io/crates/cargo-generate
[2]: https://crates.io/crates/cargo-readme
[3]: https://crates.io/crates/cargo-audit
[4]: https://crates.io/crates/cargo-auditable
[5]: https://github.com/RustSec/advisory-db/
[6]: https://crates.io/crates/cargo-make
[7]: https://crates.io/crates/cargo-generate
[8]: https://crates.io/crates/git-cliff
[9]: https://git-scm.com/
[10]: https://crates.io/crates/cargo-readme
[11]: https://crates.io/crates/cargo-udeps
[12]: https://rust-lang.github.io/mdBook
[13]: https://rust-lang.github.io/rustfmt/
[14]: https://crates.io/crates/taplo-cli
[15]: https://toml.io/en
[16]: https://jqlang.github.io/jq/
[17]: https://doc.rust-lang.org/stable/clippy/index.html
[18]: https://crates.io/crates/cargo-deny
[19]: https://crates.io/crates/cargo-llvm-cov
[20]: https://crates.io/crates/cargo-deadlinks
[21]: https://crates.io/crates/mdbook-linkcheck
[22]: https://crates.io/crates/cargo-bolero
[23]: https://crates.io/crates/bolero
