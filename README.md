Notes-CLI: A simple notes CLI
========================================

[![notes-cli on crates.io][cratesio-image]][cratesio]

[cratesio-image]: https://img.shields.io/crates/v/notes-cli.svg
[cratesio]: https://crates.io/crates/notes-cli

`notes-cli` aims to be an extremely simple note taking CLI with human readable output.
All notes are stored in a single file in plain text with a prepended local timestamp, and
with optional tags appended. Notes is designed to be used with a search tool such
as [ripgrep](https://github.com/BurntSushi/ripgrep)

The delimiter between timestamp, note, and tags is configurable, along with the notes
file path. Once set, they will be remembered. More in depth searching may be added in
the future, but one of my goals is to keep the notes file hand editable.

## Examples

#### Note with a tag, as well as delimiter and path setup
```
$ notes-cli "My first note." -t test-tag -p ~/Documents/notes.txt -d " ~ "
2020-03-02 16:53:28 ~ My first note. ~ test-tag
```

#### Notes file with several notes
```
$ cat ~/Documents/notes.txt 
2020-03-02 17:16:35 ~ Website that aggregates stock portfolios across brokerages. ~ idea, website
2020-03-02 17:17:31 ~ A note taking cli. ~ 
2020-03-02 17:17:55 ~ A simple note taking CLI in Rust. ~ 
2020-03-02 17:19:45 ~ Create a taxi tracking app. ~ idea, app
2020-03-02 17:47:16 ~ https://learningmusic.ableton.com/ ~ tutorial
2020-03-02 17:47:30 ~ Add a new blog about emulating gameboy sound to site. ~ blog
```

#### Searching by time
```
$ cat ~/Documents/notes.txt | rg "2020-03-02 17:17"
2020-03-02 17:17:31 ~ A note taking cli. ~ 
2020-03-02 17:17:55 ~ A simple note taking CLI in Rust. ~
```

#### Searching by tag
```
$ cat ~/Documents/notes.txt | rg "idea"
2020-03-02 17:16:35 ~ Website that aggregates stock portfolios across brokerages. ~ idea, website
2020-03-02 17:19:45 ~ Create a taxi tracking app. ~ idea, app
```

## Installing

```
cargo install notes-cli 
```
Be sure to have `~/.cargo/bin` on your path. The name `notes-cli` is used for uniqueness, but is very
verbose to type. I would recommend aliasing it something like `n` for the most frictionless experience.