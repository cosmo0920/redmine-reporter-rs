Redmine Reporter
===

[![Build Status](https://travis-ci.org/cosmo0920/redmine-reporter-rs.svg?branch=master)](https://travis-ci.org/cosmo0920/redmine-reporter-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/qb36vnkkpfa7w3yo/branch/master?svg=true)](https://ci.appveyor.com/project/cosmo0920/redmine-reporter-rs/branch/master)

A tiny Redmine issue reporter written in Rust language.

### How to Use

You have to prepare the following settings.toml in src/settings.toml.

```toml
[settings]
apikey = "<Your API KEY>"
redmine = "<Redmine URL>"
project_id = "<Project ID>"
tracker_id = "<Tracker ID>"
title_suffix = "suffix"
description = "description"
specify_deadline = (false|true)
```

and then,

```bash
$ cargo build
$ cargo run -- <date (format: %Y-%m-%d)> # e.g.) cargo run -- 2016-06-02
```
##### For Windows

If you use this tiny executable tool in Windows, please install Visual Studio 2015 and then rust compiler which is targeted for MSVC API and its package manager, which is called cargo via [rustup.rs](https://www.rustup.rs/).

You must install openssl which is suitable ABI.

If you use MSVC ABI rust compiler, please consider to install OpenSSL which is compiled with MSVC. In more detail, please refer this page: https://slproweb.com/products/Win32OpenSSL.html

If you execute this OpenSSL installer by default, you must set the following environment variables:

 * OPENSSL\_INCLUDE\_DIR=C:\OpenSSL\include
 * OPENSSL\_LIB\_DIR=C:\OpenSSL\lib
 * OPENSSL\_LIBS=ssleay32:libeay32

### LICENSE

[MIT](LICENSE).
