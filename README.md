[comment]: # (auto_md_to_doc_comments segment start A)

# wsl_open_browser

[comment]: # (auto_cargo_toml_to_md start)

**Inside WSL2 opens a html file in the browser that is in Win10. The local file path is transformed between Linux and Win10 notation.**  
***[repository](https://github.com/LucianoBestia/wsl_open_browser); version: 2021.816.1157  date: 2021-08-16 authors: Luciano Bestia***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-23-green.svg)](https://github.com/LucianoBestia/wsl_open_browser/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-108-blue.svg)](https://github.com/LucianoBestia/wsl_open_browser/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-11-purple.svg)](https://github.com/LucianoBestia/wsl_open_browser/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/wsl_open_browser/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/wsl_open_browser/)

[comment]: # (auto_lines_of_code end)

[![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_auto_lib/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/cargo_auto_lib/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/cargo_auto_lib/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/cargo_auto_lib/)  

Inside WSL2 I want to open a html file in the browser. But the browser is in Win10.  
This is useful in Rust for the documentation:

```bash
cargo doc --open
```

## Open URL in Win10 Browser

I prepared a symbolic link to the chrome.exe.

```bash
ln -s "/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" /usr/bin/chrome_in_win
# now I can open a page like this
chrome_in_win http://github.com
# I want to open it with xdg-open
# xdg-open is the "Open any file" for linux
# add this to the end of ~/.bashrc
export BROWSER='/usr/bin/chrome_in_win'
# now I can use
xdg-open http://google.com
```

This works great for URL, but it does not work for local files, because Linux and Windows see the same file with different paths.

## Open local file in Win10 Browser

For example:  
Linux: `/home/luciano/index.html`  
Win10: `\\wsl$\Debian\home\luciano\index.html`  
OR:  
Linux: `/mnt/c/Users/Luciano/Downloads\index.html`  
Win10: `c:\Users\Luciano\Downloads\index.html`  

I need a way to transform the path prior to call the browser.  
Let's make a Rust CLI for that.

## Development

This is a simple binary. For good habit I separated the lib from the bin.  
Use [cargo-auto](https://crates.io/crates/cargo-auto) to run automation tasks: build, release, doc, copy_to_usr_bin,....  
To build and run with parameters use this:  

```bash
cargo run -- http://google.com
cargo run -- /home/luciano/index.html
cargo run -- /mnt/c/Users/Luciano/Downloads/index.html
```

After `cargo auto copy_to_usr_bin` we can now open the browser like this:  

```bash
wsl_open_browser http://google.com
wsl_open_browser /home/luciano/index.html
wsl_open_browser /mnt/c/Users/Luciano/Downloads/index.html
```

## Open with xdg-open

But I want to use xdg-open. For that I need to set the environment variable BROWSER. To make it persistent, copy this line to ~/.bashrc.

```bash
export BROWSER='/usr/bin/wsl_open_browser'
```

Finally we can open the browser like this:

```bash
xdg-open http://google.com
xdg-open /home/luciano/index.html
xdg-open /mnt/c/Users/Luciano/Downloads/index.html
```

and the Rust documentation works fine:

```bash
cargo doc --open
```

[comment]: # (auto_md_to_doc_comments segment end A)
