# wsl_open_browser

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
To build and run with parameters use this:  
```bash
cargo run -- http://google.com
cargo run -- /home/luciano/index.html
cargo run -- /mnt/c/Users/Luciano/Downloads/index.html
```
For the final build:  
```bash
cargo build --release
# experimentally using strip to make the binary 10x smaller:
strip target/release/wsl_open_browser
```
Maybe the file needs to be marked as executable, I am not sure:
```bash
chmod a+x target/release/wsl_open_browser
```
Copy the file `target/release/wsl_open_browser` to `/usr/bin`.  
Now we can open the browser like this:  
```bash
wsl_open_browser http://google.com
wsl_open_browser /home/luciano/index.html
wsl_open_browser /mnt/c/Users/Luciano/Downloads/index.html
```
## Open with xdg-open
But I want to use xdg-open. For that I need to set the environment variable BROWSER. To make it persistant, copy this line to ~/.bashrc.
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