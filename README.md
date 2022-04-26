[comment]: # (auto_md_to_doc_comments segment start A)

# wsl_open_browser

[comment]: # (auto_cargo_toml_to_md start)

**Inside WSL2 opens a html file in the browser that is in Win10. The local file path is transformed between Linux and Win10 notation.**  
***version: 2021.822.842  date: 2021-08-22 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/wsl_open_browser)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-44-green.svg)](https://github.com/bestia-dev/wsl_open_browser/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-113-blue.svg)](https://github.com/bestia-dev/wsl_open_browser/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-15-purple.svg)](https://github.com/bestia-dev/wsl_open_browser/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/wsl_open_browser/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/wsl_open_browser/)

[comment]: # (auto_lines_of_code end)

[![crates.io](https://img.shields.io/crates/v/cargo-auto.svg)](https://crates.io/crates/wsl_open_browser)
[![Documentation](https://docs.rs/wsl_open_browser/badge.svg)](https://docs.rs/wsl_open_browser/)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/wsl_open_browser.svg)](https://web.crev.dev/rust-reviews/crate/wsl_open_browser/)
[![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/wsl_open_browser/)
[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/wsl_open_browser/blob/master/LICENSE)
[![Rust](https://github.com/bestia-dev/wsl_open_browser/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/wsl_open_browser/)
[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Fwsl_open_browser&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

## Try it

Install it from crates.io and add 2 symbolic links and one env variable:

```bash
cargo install wsl_open_browser
ln -s "/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" /usr/bin/chrome_in_win
export BROWSER='/usr/bin/wsl_open_browser'
# I like to use the short command `www` to open a browser
ln -s "/usr/bin/wsl_open_browser" www

# go to a directory that contains a subdirectory with index.html
cd ~/rustprojects/wsl_open_browser
www docs/index.html
# or
www docs
# or go inside the directory that contains index.html
cd docs
www .
# or
www
# if you want to open an URL
www google.com
```

Congratulations! You have just opened a windows browser from WSL2.

## Motivation

Inside WSL2 I want to open a html file in the browser. But the browser is in Win10.  
This is useful in Rust for the documentation:

```bash
cargo doc --open
```

or for simply open a file in a browser from bash:

```bash
www index.html
```

## Open URL in Win10 Browser (first attempt)

I prepared a symbolic link to the chrome.exe.

```bash
ln -s "/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" /usr/bin/chrome_in_win
# now I can open a page like this
chrome_in_win http://github.com
# I want to open it with xdg-open
# xdg-open is the "Open any file" for linux
export BROWSER='/usr/bin/chrome_in_win'
# now I can use
xdg-open http://google.com
```

This works great for URL, but it does not work for local files, because Linux and Windows see the same file with different paths.  
The command `ln -s` is permanent and persistent. It makes a file that stays there forever.  
But `export BROWSER=` is NOT persistent. You need to add this command to `~/.bashrc` that runs it on every start of terminal.  

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
Use [cargo auto](https://crates.io/crates/cargo-auto) to run automation tasks: build, release, doc, copy_to_usr_bin,....  
After `cargo auto copy_to_usr_bin` we can now open the browser like this:  

```bash
wsl_open_browser http://google.com
wsl_open_browser /home/luciano/index.html
wsl_open_browser /mnt/c/Users/Luciano/Downloads/index.html
wsl_open_browser docs/index.html
```

## Open with xdg-open

But I want to use xdg-open.  
xdg-open is the "Open any file" for linux.  
For that I need to set the environment variable BROWSER.  
To make it persistent, copy this line to `~/.bashrc`.

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

## simply: `www docs` or `www .`

I am still not satisfied.  
I want to open the browser from Debian bash terminal with this simple syntax:  

```bash
www docs
www .
www google.com
```

I will make another alias:  

```bash
ln -s "/usr/bin/wsl_open_browser" www
```

The command `ln -s` is permanent and persistent. It makes a file that stays there forever.  

[comment]: # (auto_md_to_doc_comments segment end A)
