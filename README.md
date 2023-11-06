# mockbuster

A WIP application for friends to rate, share and discuss movies.

## Q & A's

### Tech TL;DR?

mockbuster is primarily an [Actix Web](https://actix.rs/) application glueing databases to a UI for users to rate, share and discuss movies.

#### FE TL;DR
The UI follows a **Server Side Rendered, Client Side Hydrated** strategy.  Each page is built off of [yew](https://yew.rs/) components whose compile targets are both static strings (for HTML) and dynamic [WASM](https://webassembly.org/) components (for client side rendering).  On each `GET` request, [actix](https://actix.rs/) renders out each page of yew components and responds with the complete Document, the traditional SSR approach.  Once rendered on the client, that Document requests it's associated [WASM](https://webassembly.org/) counterpart code via `<script />` tags, which builds an identical V-DOM to replace the server generated HTML. This approach splits the appropriate responsibilites across server and client, allowing for SEO + fast response times from our server while still achieving a highly interactive & performant UI that scores highly across all [Core Web Vital](https://developers.google.com/search/docs/appearance/core-web-vitals) metrics.

Example / explanation using the [LoginView](https://github.com/PatrickMcLennan/mockbuster/tree/main/views/login_view) page:
  - The main component is defined + written to be compatible with both server + client scenarios ([login_view](https://github.com/PatrickMcLennan/mockbuster/blob/main/views/login_view/login_view.rs#L11-L155)).
  - That same component is then also exported via [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) for [WASM](https://webassembly.org/) environments ([run_login_view](https://github.com/PatrickMcLennan/mockbuster/blob/main/views/login_view/login_view.rs#L157-L161)), with explicit instructions to hydrate an existing version of itself within a DOM.
  - On any `GET` request to `/login`, a Document is generated + sent in its correct state via the server rendering of `login_view` ([get](https://github.com/PatrickMcLennan/mockbuster/blob/main/server/routes/login/get.rs))
  - Once sent to the client, that document immediately makes a request to fetch + run it's associated WASM ([get](https://github.com/PatrickMcLennan/mockbuster/blob/main/server/routes/login/get.rs#L36), [loginView](https://github.com/PatrickMcLennan/mockbuster/blob/main/views/login_view/loginView.ts))
  - Once `loginView.js` is executed, the DOM sent by the servers rendering of `login_view` will be hydrated by the WASM code within `run_login_view`, replacing it with an identical V-DOM in markup that contains all event listeners & other UI logic.
  
  #### SSR WASM?  Sounds unnecessarily fancy.  Why not just use react / handlebars / something sane?
  No (good) reason in particular - as a Rust fan this stack is a POC to check out Rusts web development ecosystem / ergonomics.  A goal of mockbuster was to write as little client side TypeScript as possible - no matter how painful / inefficient -  and get a sense of if this stack is production ready.  (I'm aware using [bootstrap](https://getbootstrap.com/) is sort of cheating here, but you get the idea.)

### What's the full stack?
#### Front End
- [bootstrap](https://getbootstrap.com/) for all styles + UI logic (dropdowns, modals, etc)
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) for generating WASM code invokable from client side JS
- [web-sys](https://crates.io/crates/web-sys) for binding the Browser API to WASM
- [reqwasm](https://crates.io/crates/reqwasm) as an HTTP client
- [webpack](https://webpack.js.org/) for bundling FE dependencies (bootstrap, etc)
  - [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/) for bundling WASM components via webpack

#### Back End
- [Actix Web](https://actix.rs/) for document, asset + REST requests
- [SeaORM](https://github.com/SeaQL/sea-orm) ORM & all interfacing with postgres

#### Networking
- [Docker](https://docs.docker.com/) for Postgres + Redis containers
- [docker-compose](https://docs.docker.com/compose/) for composability

#### Shared
- [yew](https://yew.rs/) for UI components, compiled on both server + client (see [FE TL;DR](https://github.com/PatrickMcLennan/mockbuster#fe-tldr)).
- [validators](https://crates.io/crates/validators) for shared validation logic across server + client code.



## Developing

  ```bash
  cargo install cargo-watch;
  cargo install sea-orm-cli;
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; // https://rustwasm.github.io/wasm-pack/installer/
  docker-compose up -d;
  sea-orm-cli migrate up;
  sea-orm-cli generate entity -o ./db_models/generated;
  yarn;
  ```
Once installed, run these 2 build processes in parallel **from the project root**.
  ```bash
  cargo watch -x run; // Recompile binaries + libraries on changes
  yarn compile:dev:watch; // Recompile FE assets + further compile Rust components into WASM on changes
  ```

You should now have recompilation of all Rust, TS + CSS styles on respective file changes (hot reloading coming one day...<sup><sup>maybe</sup></sup>).

All styles + JS should be imported via the `src/assets/entry.ts`.