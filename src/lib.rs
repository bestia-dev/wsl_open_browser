use unwrap::unwrap;

pub fn open_browser(arg_1: &str) {
    // absolute Linux path
    let mut arg = arg_1.to_string();
    if arg_1.starts_with("/mnt/c/") {
        // Linux: /mnt/c/Users/Luciano/Downloads\index.html
        // Win10: c:\Users\Luciano\Downloads\index.html
        arg = format!(r#"c:\{}"#, arg_1.trim_start_matches("/mnt/c/"));
    } else if arg_1.starts_with("/") {
        // Linux: /home/luciano/index.html
        // Win10: \\wsl$\Debian\home\luciano\index.html
        arg = format!(r#"\\wsl$\Debian{}"#, arg_1);
    }
    // call the symbolic link for chrome
    // ln -s "/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" /usr/bin/chrome_in_win
    // /usr/bin/chrome_in_win
    unwrap!(std::process::Command::new("/usr/bin/chrome_in_win")
        .arg(arg)
        .spawn());
}
