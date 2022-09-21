## Development environment setup
Install from: https://code.visualstudio.com/

Adding extensions, Setting up environment:
Rust Language Server, Tabnine, Fonts (Ligatures), Terminal (Oh-My-Zsh)
```sh
# Rust Language Server - https://github.com/rust-lang/rls
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Update
rustup update
# Install Rust Language Server
rustup component add rls rust-analysis rust-src
```

### Extensions
Tabnine (Code Completion) - https://www.tabnine.com/install/vscode
Powerline Fonts - https://github.com/powerline/fonts

### Shells and Etc
iTerm2 - https://www.iterm2.com/
Oh-My-Zsh - https://ohmyz.sh/
Powerlevel10k - https://github.com/romkatv/powerlevel10k

### Webserver
hostr - https://www.npmjs.com/package/hostr
```sh
$ pwd
/Users/elliottf/src/anyhow_turbofish/v3/json
$ npm install -g hostr
$ hostr --port 8080
```

### Crates used
https://docs.rs/reqwest/0.11.11/reqwest/index.html
https://docs.rs/json/0.12.4/json/index.html
https://docs.rs/anyhow/1.0.62/anyhow/index.html
