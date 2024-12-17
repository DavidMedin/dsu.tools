### `dsu.tools`
A toolbox of tools useful to DSU students.

Authored by : Sara Solace, Irina Pecherskaia, David Medin, and Gabe Delle


### Software Dependancies
- Rust
  - Rustup
  - Cargo
- `sqlite3`
- `npm`

If you don't want to install `cargo` to compile the server's backend in Rust, just go to the "Release" tab on the right in Github to download the pre-built server binaries for Linux or Windows.

### Compiling
First, clone the project with
```bash
git clone https://github.com/davidmedin/dsu.tools
```
To build the project, you must make a sqlite database using `sqlite3` in the `database` directory. Then, run `generate.sh` to fill the database with the necessary tables.

Then, go to `dsutools-frontend` and install `npm`. Follow the `README.md` in that directory to continue and build the `dist` directory.

Finally, download the pre-built server binary into the `dsutools-backend` directory, and run it. The website should now be functional. The terminal output should print an address you can navigate to with a browser.
