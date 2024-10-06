# Introduction

The directories within this project are my personal templates for creating new
Rust projects.  To use them, you will first need to install the [Rust][1] 
`stable` and `nightly` toolchains on your system. Then you will need to install
[`cargo-generate`][2] and [`cargo-make`][3].  

Once these are installed, you should be able to execute the following to get a
new project from the templates:

```rust
cargo generate -g git@github.com:cfkaran2/cargo_generate_templates.git
```

and [`cargo-generate`][2] should handle the rest for you, contacting GitHub to
get the templates and ask you questions to fill in the blanks in the template
files.

After you've created your basic project, execute the following:

```rust
cargo make help
```

If all goes well, then you will eventually have a help message pop up telling
you what build targets are currently available.  Note that the configuration
file for [`cargo-make`][3](`Makefile.toml`) is set up to use a wide variety of
tools.  If you don't have them installed on your system, then [`cargo-make`][3]
will download, build, and install them on your system the first time you call
`cargo make`.  This can take a very long time and a lot of space.  Once the
tools are installed though, further builds should be much quicker.

Have fun, and happy coding!

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

## License

The templates as they exist in this repository are licensed under the
[Apache 2.0 license][4].  Once you use [`cargo-generate`][2] to create your new
project, you may change the license of your code to whatever you wish.  

[1]: https://www.rust-lang.org/
[2]: https://crates.io/crates/cargo-generate
[3]: https://crates.io/crates/cargo-make
[4]: https://www.apache.org/licenses/LICENSE-2.0
