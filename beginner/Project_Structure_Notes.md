# Basic components
- Packages
    - created when running `cargo new`
    - contains one or more crates
    - contains cargo.toml file
    - must have
        - at least 1 crate
        - no more than one library crate
        - unlimited number of binary crates
- Crates
    - tree of modules that product library or exe
- Modules
    - let you control organization, scope, & privacy of code

- when running cargo new
    - creates src dir with main.rs file
        - rust sees that as root crate for binary
    - if lib.rs file present in src dir
        - root crate for library
        - can create new library crate using `cargo new --lib PACKAGENAME`
    - common to have both in a CLI app
    - to add more binary crates within package/project
        - `mkdir src/bin` (create subdir within src dir)
        - needs a `fn main() {}` to be valid

- to run binary crates
    - can't use `cargo run` as no binary specified
    - need to specify binary like
        - `cargo run --bin my_package`

# Modules
- organize code for readability
- controls scope & privacy
- contains
    - functions
    - structs
    - enums
    - traits
    - etc
- explicitly defined using `mod` keyword
    - not mapped to file system (not like Python -> from packages.utils import helperfuncs)
    - designed to be flexible & straight forward for conditional compliation

# Modules in different files
- for module with no submodules
    - create new .rs file
    - copy/paste content into new file
    - reference new file using `mod NEWFILENAME;`
- for modules with submodules
    - two ways to do this
        1. create new dir with same name
            - create new file called `mod.rs`
                - move content from old file & paste into mod.rs
            - create new file called `models.rs` (or whatever the struct/enum/etc is)
                - move content into that file
            - in mod.rs replace models declartion 
            - do the same with the original file
        2. create new file called `auth_utils.rs` (or whatever the name is)
            - copy what would be in the mod.rs file into auth_utils.rs
            - keep the auth_utils dir & submodules
            - makes for a bit cleaner/less confusing code (removes the many `mod.rs` files)
- use statements are also helpful for re-exporting
    - add the `pub` keyword
- accessing library (authservice) from binary crate
    - create `main.rs`
    - add `fn main() {}`
    - setup use declartions `use authservice::{Credentials, authenticate}`
    ```rust
    fn main() {
        // below won't work unless credentails fields (un & pw) are either declared pub or impl new() block set
        let cred: Credentials = Credentials{username:"logan".to_owned(),password:"123".to_owned()};
        auth_service::authenticate(creds);
    }
    ```

# Importing/Using Dependencies
- [crates.io](https://crates.io)
- find package & copy usage
- paste into cargo.toml file
- using dependency in code
    - `use rand::prelude::*` -> brings all prelude modules into scope

# Publishing package
- login to crates.io
- create api token
- copy token
- in terminal -> `cargo login APITOKEN`
- in terminal
    - `cargo publish`
    - must commit changes to github first
    - to publish, neet metadata
        - name (must be unique)
        - version
        - edition
        - description
        - license
- once published, it is permanent

# Structuring with Workspaces
- used when want to reference many sub projects (eg blog project with api, frontend, shared projects)
- create new dir & add Cargo.toml file
    - in toml file add `[workspace]` to declare workspace
    - add `members = []` & add the names of the projects within the workspace
    - create each crate within dir `cargo new --vcs none api` (--vcs none doesn't init a .git file)``