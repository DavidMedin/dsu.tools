# dsu.tools Backend Server

The backend server for dsu.tools is written in Rust.

We use the web framework [Rocket](https://rocket.rs) for the web framework.

### Database Setup
Make sure you set up the database as outlined in the `dsutools-database/README.md` file.

### Installing Rust and Building

To install Rust, install the `rustup` tool and run `rustup install stable`.

To build the project, run `cargo build`.\
Or, to build and run the project in one step, run `cargo run`.\
Before running the server, make sure to build the frontend using Vite as mentioned in the `dsutools-frontend/README.md` file.\

### Server Configuration
This project uses Rocket to do a lot of Web related tasks for us. Rocket has its own configuration file, `Rocket.toml`. In there we can configure things like where the database file lives and what kind of database it is.
