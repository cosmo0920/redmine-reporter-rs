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
```

and then,

```bash
$ cargo build
$ cargo run -- <date (format: %Y-%m-%d)> # e.g.) cargo run -- 2016-06-02
```

### LICENSE

[MIT](LICENSE).
