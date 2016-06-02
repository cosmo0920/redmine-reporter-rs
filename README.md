Redmine Reporter
===

[![Build Status](https://travis-ci.org/cosmo0920/redmine-reporter-rs.svg?branch=master)](https://travis-ci.org/cosmo0920/redmine-reporter-rs)

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
$ cargo run -- [date (fmt: %Y-%m-%d)]
```

### LICENSE

[MIT](LICENSE).
