# Chapter 14: Publishing a Rust Crate
* Covers customizing builds using build-profiles, publishing crates to Crates.io, organizing large projects with workspaces, installing binaries, and extending Cargo with custom commands.

* Release Profiles: Allows configuration of how program is compiled. There are two profiles: `dev` and `release`.
    ```cmd 
            $ cargo build // compile with dev profile
            $ cargo build --release
    ```
    * The release build is optimized, while the dev build isn't since it contains debug information.