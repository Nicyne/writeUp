<div id="top"></div>

<br />
<div align="center">
  <a href="https://github.com/nicyne/writeup">
      <img src="https://upload.wikimedia.org/wikipedia/commons/4/48/Markdown-mark.svg" width=124 height=124 alt="Logo">
  </a>

  <h3 align="center">writeUP (v.0.5.0)</h3>

 <p align="center">
    This is a work in progress.
    <br />
    <a href="https://github.com/nicyne/writeup/issues">Report a Bug ☠️</a>
    ·
    <a href="https://github.com/nicyne/writeup/issues">Request a Feature 📝</a>
  </p>
</div>

<!-- ABOUT -->

## About The Project 📢

WriteUp is a markdown note taking application written in react (frontend) and rust (backend). This is a project done by a two man team from Germany calling itself Nicyne.

---

### Why writeUp?

#### Support for Markdown

WriteUp follows the CommonMark-Spec with custom additions such as GFM and unicode-emoji-support so that your notes will always stay readable und well structured.

#### Easy organization

Using tags writeUp offers a simple and dynamic overview over both your own notes and those that have been shared with you.

Keeping track has never been this easy!

#### Simple Sharing (IN DEVELOPMENT\*)

With but a few clicks you can open up your note to your friends or colleagues. Give them the permission to read your note, let them edit it, or revoke their access again.

It's your note after all!

\*This feature may be unavailable or only partially available during development.

#### Fast and Reliable

Built on ReactJS and written in Typescript writeUp's frontend offers a reliable and intuitive interface to manage your notes with. Combined with its blazing fast rust backend you'll never have to wait for a change to take place.

<p align="right">(<a href="#top">back to top</a>)</p>

## Build ⚙️

### Manual

> To build writeUp this way you have to install either npm or yarn + cargo.

1. Build the frontend

```sh
cd client

# install dependencies
yarn install
#or
npm install

# build the frontend
yarn build
#or
npm run build

cp -R build/ ../public/
cd ..
```

2. Build the backend (requires `clang` to be installed)

```sh
cargo build --release
cp target/release/writeUp ./
```

### Docker

> To build writeUp this way you have to install docker.

1. Run docker build

```
docker build .
```

### Docker-compose

> To build writeUp this way you have to install docker and docker-compose.

1. Run docker-compose up

```sh
docker-compose up -d

# to rebuild run
docker-compose down
docker-compose up -d --build
```

## Configuration 📝

writeUp assumes you're setting a few environment variables. Should those not exists the application will crash telling you which ones you missed. Some - albeit not all - can generate default values.

The following variables have to / can be set:

```env
# Database URI
DB_URI: mongo
# Database Port
DB_PORT: 27017
# Name of the database user
DB_USER: root
# Password for the database user
DB_PASSWD: example


# OPTIONAL VARIABLES

# Application port, defaults to 8080
API_PORT: 8080

# Key used for signups during the development phase of this project.
# This key will not be used in the final stage.
BETA_KEY: B757B

# Secret used as a increasing password security.
# Although optional it is highly recommended to use a static secure value.
# This secret should not be changed after it has been used once.
PASSWD_SECRET: passwdSecret

# Secret used for the encoding of the session-id (has to be at least 64bytes)
SESSION_SECRET: sessionSecretsessionSecretsessionSecretsessionSecretsessionSecrets

# Secret reserved for for future development.
SHARE_SECRET: shareSecret

# Environment the application is running in.
# This variable must only be set if the environment
# is not PRODUCTION
ENVIRONMENT: DEVELOPMENT
```

<p align="right">(<a href="#top">back to top</a>)</p>

## Running writeUp in headless mode 🔮

You can run the writeUp backend in a headless-mode. In this mode the `public`-directory containing the frontend files will be ignored.

1. Run

```sh
./writeUp --headless
```

## Built With 🛠️

- [Rust][rust]
- [React][react]

<p align="right">(<a href="#top">back to top</a>)</p>

[rust]: https://www.rust-lang.org
[react]: https://reactjs.org
