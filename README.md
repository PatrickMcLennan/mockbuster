# mockbuster

A WIP application for friends to rate, share and discuss movies.

## Q & A's

### Tech TL;DR?

mockbuster is primarily an [Actix Web](https://actix.rs/) application glueing databases to a UI for users to rate, share and discuss movies.

#### FE TL;DR
The UI follows a Server Side Rendered, Client Side Hydrated strategy.  Each page is built off of [yew](https://yew.rs/) components whose compile targets can be either static strings (for HTML) or dynamic [WASM](https://webassembly.org/) components.  On each `GET` request, [actix](https://actix.rs/) pre-renders out each page of yew components and responds with the complete Document.  Once rendered on the client, that Document requests it's associated [WASM](https://webassembly.org/) code via `<script />` tags, which then builds an identical V-DOM to replace the server generated HTML. This approach splits the appropriate responsibilites across server and client, allowing for SEO + fast response times by our server while achieving a highly interactive UI that scores great among [Core Web Vital](https://developers.google.com/search/docs/appearance/core-web-vitals) metrics.  
  - **SSR WASM?  Sounds unnecessarily fancy.  Why not just use react / handlebars / something sane?**
  <br />
    No (good) reason in particular - as a Rust fan this stack is a POC to check out Rusts web development ecosystem / ergonomics.  A goal of mockbuster was to write as little client side TypeScript as possible - no matter how painful / inefficient -  and get a sense of if this stack is production ready.

### What's the full stack?
#### Front End
- [yew](https://yew.rs/) for HTML templating & UI components
- [bootstrap](https://getbootstrap.com/) for all styles + UI logic (dropdowns, modals, etc)
- [web-sys](https://crates.io/crates/web-sys) for WASM <-> Browser API binding 
- [reqwasm](https://crates.io/crates/reqwasm) as an HTTP client
- [webpack](https://webpack.js.org/) for bundling FE dependencies (bootstrap, etc)
  - [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/) for bundling WASM components via webpack

#### Back End
- [Actix Web](https://actix.rs/) for document, asset + REST requests



## Developing

Install [cargo-watch](https://crates.io/crates/cargo-watch), [wasm-pack](https://github.com/rustwasm/wasm-pack) + other JS dependencies **from the project root**.
  ```bash
  cargo install cargo-watch;
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; // https://rustwasm.github.io/wasm-pack/installer/
  yarn;
  ```
Once installed, run these 2 build processes in parallel **from the project root**.
  ```bash
  cargo watch -x run; // Recompile binaries + libraries on changes
  yarn compile:dev:watch; // Recompile FE assets + further compile Rust components into WASM on changes
  ```

You should now have recompilation of all Rust, TS + CSS styles on respective file changes (hot reloading coming one day...<sup><sup>maybe</sup></sup>).

All styles + JS should be imported via the `src/assets/entry.ts`.