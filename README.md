# mockbuster

A WIP application for friends to rate, share and discuss movies.

## Q & A's

### Tech TL;DR?

mockbuster is primarily an [Actix Web](https://actix.rs/) application glueing databases to a UI for users to rate, share and discuss movies.

#### FE TL;DR

The UI follows a **Server Side Rendered, Client Side Hydrated** strategy. Each page is built off of [yew](https://yew.rs/) components whose compile targets are both static strings (for HTML) and dynamic [WASM](https://webassembly.org/) components (for client side rendering). On each `GET` request, [actix](https://actix.rs/) renders out each page of yew components and responds with the complete Document, the traditional SSR approach. Once rendered on the client, that Document requests it's associated [WASM](https://webassembly.org/) counterpart code via `<script />` tags, which builds an identical V-DOM to replace the server generated HTML. This approach splits the appropriate responsibilites across server and client, allowing for SEO + fast response times from our server while still achieving a highly interactive & performant UI that scores highly across all [Core Web Vital](https://developers.google.com/search/docs/appearance/core-web-vitals) metrics.

Example / explanation using the [LoginView](https://github.com/PatrickMcLennan/mockbuster/tree/main/views/login_view) page:

- The main component is defined + written to be compatible with both server + client scenarios ([login_view](https://github.com/PatrickMcLennan/mockbuster/blob/main/views/login_view/login_view.rs#L11-L155)).
- That same component is then also exported via [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) for [WASM](https://webassembly.org/) environments ([run_login_view](https://github.com/PatrickMcLennan/mockbuster/blob/main/views/login_view/login_view.rs#L157-L161)), with explicit instructions to hydrate an existing version of itself within a DOM.
- On any `GET` request to `/login`, a Document is generated + sent in its correct state via the server rendering of `login_view` ([get](https://github.com/PatrickMcLennan/mockbuster/blob/main/server/routes/login/get.rs))
- Once sent to the client, that document immediately makes a request to fetch + run it's associated WASM ([get](https://github.com/PatrickMcLennan/mockbuster/blob/main/server/routes/login/get.rs#L36), [loginView](https://github.com/PatrickMcLennan/mockbuster/blob/main/views/login_view/loginView.ts))
- Once `loginView.js` is executed, the DOM sent by the servers rendering of `login_view` will be hydrated by the WASM code within `run_login_view`, replacing it with an identical V-DOM in markup that contains all event listeners & other UI logic.

#### SSR WASM? Sounds unnecessarily fancy. Why not just use react / handlebars / something sane?

No (good) reason in particular - as a Rust fan this stack is a POC to check out Rusts web development ecosystem / ergonomics. A goal of mockbuster was to write as little client side TypeScript as possible - no matter how painful / inefficient - and get a sense of if this stack is production ready. (I'm aware using [bootstrap](https://getbootstrap.com/) is sort of cheating here, but you get the idea.)

### What's the full stack?

#### Front End

- [bootstrap](https://getbootstrap.com/) for all styles + UI logic (dropdowns, modals, etc)
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) for generating WASM code from [yew](https://yew.rs/)
- [web-sys](https://crates.io/crates/web-sys) for binding the Browser API to WASM
- [reqwasm](https://crates.io/crates/reqwasm) as an HTTP client
- [webpack](https://webpack.js.org/) for bundling FE dependencies (bootstrap, etc)
  - [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/) for bundling WASM components via webpack
- [yew](https://yew.rs/) Client side templating

#### Back End

- [Actix Web](https://actix.rs/) for document, asset + REST requests
- [SeaORM](https://github.com/SeaQL/sea-orm) ORM & all interfacing with postgres
- [yew](https://yew.rs/) Server side templating

#### Storage

- [postgres](https://www.postgresql.org/) for the main + relational data store.
- [redis](https://redis.io/) for expensive ephemeral data such as sessions

#### Networking

- [Docker](https://docs.docker.com/) for Postgres + Redis containers
- [docker-compose](https://docs.docker.com/compose/) for composability

#### Shared

- [yew](https://yew.rs/) for UI components, compiled on both server + client (see [FE TL;DR](https://github.com/PatrickMcLennan/mockbuster#fe-tldr)).
- [validators](https://crates.io/crates/validators) for shared validation logic across server + client code.

## Developing

Development uses https [as it's needed for ServiceWorkers](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API#:~:text=Service%20workers%20only%20run%20over%20HTTPS%2C%20for%20security%20reasons.%20Most%20significantly%2C%20HTTP%20connections%20are%20susceptible%20to%20malicious%20code%20injection%20by%20man%20in%20the%20middle%20attacks%2C%20and%20such%20attacks%20could%20be%20worse%20if%20allowed%20access%20to%20these%20powerful%20APIs). You'll need to generate an SSL certificate + private key within the `nginx` dir:

```bash
## Install mkcert && generate trusted self-signed certs
brew install mkcert;
mkcert -install;
mkcert localhost;
```

Rename `localhost.pem` to `certificate.crt` and `localhost-key.pem` to `private.key`, then move both of those files into `/nginx`. Double click `certificate.crt` to open it, and follow the instructions to have your machine trust it.

> After this step, macOs + linux users may need to restart their machine for Chromium browsers to trust this cert.

Then generate the keys needed for [VAPID](https://datatracker.ietf.org/doc/html/draft-thomson-webpush-vapid) and insert them in the `.env` file. You can visit a site such as [this](https://www.attheminute.com/ca/vapid-key-generator) to have keys generated for you.

Then,

```bash
## Install Rust deps
cargo install cargo-watch;
## Install ORM
cargo install sea-orm-cli;
## Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; // https://rustwasm.github.io/wasm-pack/installer/;
## Start containers
docker-compose up -d;
## Run migrations
sea-orm-cli migrate up;
## Generate models
sea-orm-cli generate entity -o ./models/generated --with-serde both;
## Install FE toolchain
yarn;
```

Once installed, run these 2 build processes in parallel.

```bash
cargo watch -x run; ## from within /server
yarn compile:dev:watch; ## from root
```

All Rust & TS code across FE & BE will recompile on save, and the server will restart.
