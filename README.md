# Riot Manifest (rman)

<div align="center">

[![github](https://img.shields.io/badge/github-ev3nvy/rman--rs-181717?logo=github&style=for-the-badge)][repository]
[![crates.io](https://img.shields.io/crates/v/rman?color=2B4D28&logo=rust&style=for-the-badge)][crates-io]
[![docs.rs](https://img.shields.io/badge/docs.rs-rman-D2991D?logo=docs.rs&style=for-the-badge)][docs-rs]
[![Discord](https://img.shields.io/discord/1007597805956780062?color=5865F2&label=discord&logo=discord&logoColor=FFFFFF&style=for-the-badge)][discord]

</div>

<div align="center">

[![crates.io](https://img.shields.io/crates/d/rman?style=for-the-badge)][crates-io]
[![crates.io](https://img.shields.io/crates/l/rman?style=for-the-badge)][crates-io]
[![docs.rs](https://img.shields.io/docsrs/rman?style=for-the-badge)][docs-rs]
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ev3nvy/rman-rs/ci.yml?branch=main&style=for-the-badge)][repository]

</div>

This is an [unofficial](#legal) rust implementation for parsing the [.manifest][manifest] file
format, and downloading containing files.

## About

The format was made by [RiotGames][riot-games] and is used by [RiotClient][riot-client] for
downloading game updates.

Layout of the [.manifest][manifest] file is as follows:
  - [file header (28 bytes)](src/parser/header.rs),
  - [zstd compressed data](src/parser/file.rs#60),
  - digital signature.

Decompressed [zstd][zstd] data is a binary [flatbuffer][flatbuffers] format. This crate uses
[generated code](src/generated/flatbuffer.rs) from a mostly complete schema over at
[this repository][rman-schema].

[rman-schema][rman-schema] repository is only added as a submodule as a way to track which schema
version is used.

## Status

This crate is ready for use as-is and should be able to parse all current and future
[.manifest][manifest] files, barring any changes to the format itself. If the library is unable
to parse any specific file, or if it breaks in the future, feel free to
[contribute](#contributing).

The [semver-major][semver] version of the crate will stay at 0, until functionality and purpose
of all of the fields in the flatbuffer schema is known. Besides that, the crate follows
[cargo's versioning guidelines](https://doc.rust-lang.org/cargo/reference/semver.html).

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rman = "0.3"
```

## Usage

See the [documentation][docs-rs] for examples and information about all of the
exposed API's.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Seeking help

If you need any help with using the library, or have any questions, feel free to open an issue,
or ask for assistance [on discord][discord].

## Documentation

You can find latest release documentation on [docs.rs][docs-rs]. Versions from `0.3.0` onwards are also
published to GitHub pages (e.g. https://ev3nvy.github.io/rman-rs/v0.3.0). You can also find
documentation for the latest unpublished version on [GitHub pages][github-pages-docs].

## Acknowledgements

- @moonshadow565 for the amazing work over at
[rman - set of CLI tools for rito manifest and bundle files][moonshadow565-rman]. Most of the
schema is based upon his work.
- @Morilli for creating [ManifestDownloader][morilli-manifest-downloader], which was my first
exposure to the inner workings of `.manifest` format, and for answering my question on discord.
- @Kurainu for creating [RMAN-Parse][kurainu-rman-parse] on which the early drafts of this project
were based on.

## Legal

Riot Games, VALORANT, and any associated logos are trademarks, service marks, and/or registered
trademarks of Riot Games, Inc.

This project is in no way affiliated with, authorized, maintained, sponsored or endorsed by Riot
Games, Inc or any of its affiliates or subsidiaries.

I, the project owner and creator, am not responsible for any legalities that may arise in the use
of this project. Use at your own risk.

<!-- Project links -->
[crates-io]: https://crates.io/crates/rman
[discord]: https://discord.gg/5QVVBKBvpQ
[docs-rs]: https://docs.rs/rman
[github-pages-docs]: https://ev3nvy.github.io/rman-rs
[repository]: https://github.com/ev3nvy/rman-rs


<!-- References -->
[flatbuffers]: https://github.com/google/flatbuffers
[flatbuffers-guide-building]: https://google.github.io/flatbuffers/flatbuffers_guide_building.html
[flatbuffers-guide-using-schema-compiler]: https://google.github.io/flatbuffers/flatbuffers_guide_using_schema_compiler.html
[flatbuffers-releases]: https://github.com/google/flatbuffers/releases
[manifest]: https://technology.riotgames.com/news/supercharging-data-delivery-new-league-patcher
[moonshadow565-rman]: https://github.com/moonshadow565/rman
[morilli-manifest-downloader]: https://github.com/Morilli/ManifestDownloader
[kurainu-rman-parse]: https://github.com/Kurainu/RMAN-Parse
[riot-client]: https://www.riotgames.com/en/news/new-riot-client-coming-soon
[riot-games]: https://www.riotgames.com
[rman-schema]: https://github.com/ev3nvy/rman-schema
[semver]: https://semver.org/
[zstd]: https://github.com/facebook/zstd
