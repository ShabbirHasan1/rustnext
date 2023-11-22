# rustnext
<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-osx%2Flinux%2Fwindows-blueviolet"></a>
</p>

- [Overview](#overview)
- [Usage](#usage)
- [Compile](#compile)
- [Details](#details)

# Overview

`rustnext` was designed to retrieve paths from the _buildManifest.js file that is used by the Next.js framework 🦀

## Usage

- To use, just enter the URL:
```sh
rustnext -u <URL>
```
- If you prefer, you can specify the protocol:
```sh
rustnext -u <URL> --proto http
```
# Compile
```sh
cargo build --release
```

# Details

```
Code that retrieves paths from the next.js framework

Usage: rustnext [OPTIONS] --url <URL>

Options:
  -u, --url <URL>      Insert URL
  -p, --proto <PROTO>  Insert protocol [default: https]
  -h, --help           Print help
  -V, --version        Print version
```

