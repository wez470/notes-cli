Notes: A simple notes CLI
========================================
Notes aims to be an extremely simple note taking CLI with human readable output.
All notes are stored in a single file in plain text with a prepended timestamp, and
with optional tags appended. Notes is designed to be used with a search tool such
as [ripgrep](https://github.com/BurntSushi/ripgrep)

The delimiter between timestamp, note, and tags is configurable, along with the notes
file path. More in depth searching may be added in the future.

## Examples

```
$ cargo run "Add a new blog about emulating gameboy sound to site." -t blog
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/notes 'Add a new blog about emulating gameboy sound to site.' -t blog`
2020-03-02 17:47:30 ~ Add a new blog about emulating gameboy sound to site. ~ blog
```
```
$ cat ~/Documents/notes.txt 
2020-03-02 17:16:35 ~ A website that aggregates stock portfolios across different brokerages. ~ idea, website
2020-03-02 17:17:31 ~ A note taking cli. ~ 
2020-03-02 17:17:55 ~ A simple note taking CLI in Rust. ~ 
2020-03-02 17:19:45 ~ Create a taxi tracking app. ~ idea, app
2020-03-02 17:47:16 ~ https://learningmusic.ableton.com/ ~ tutorial
2020-03-02 17:47:30 ~ Add a new blog about emulating gameboy sound to site. ~ blog
```
```
$ cat ~/Documents/notes.txt | rg "2020-03-02 17:17"
2020-03-02 17:17:31 ~ A note taking cli. ~ 
2020-03-02 17:17:55 ~ A simple note taking CLI in Rust. ~
```
```
$ cat ~/Documents/notes.txt | rg "idea"
2020-03-02 17:16:35 ~ A website that aggregates stock portfolios across different brokerages. ~ idea, website
2020-03-02 17:19:45 ~ Create a taxi tracking app. ~ idea, app
```
## Usage

```
cargo install TODO 
```