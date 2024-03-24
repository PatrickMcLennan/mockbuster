"use strict";
const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

/**
 * TODO:
 *
 * - Better optimize prod WASM builds (WASM code splitting? IDK)
 * - Watching the /components dir on each package is really innefficient (see: watchDirectories on every WasmPackPlugin).
 *   Find a more efficient way to recompile only what's necessary when /components changes
 */

const VIEWS = [
  { js: `homeView`, rust: `home_view` },
  { js: `loginView`, rust: `login_view` },
  { js: `movieView`, rust: `movie_view` },
  { js: `profileView`, rust: `profile_view` },
  { js: `recentlyRentedView`, rust: `recently_rented_view` },
  { js: `searchView`, rust: `search_view` },
  { js: `topTenView`, rust: `top_ten_view` },
];

const IS_PROD = process.env.NODE_ENV === `production`;

module.exports = {
  entry: {
    ...VIEWS.reduce(
      (all, { js, rust }) => ({
        ...all,
        [js]: path.resolve(__dirname, `./views/${rust}/${js}.ts`),
      }),
      {},
    ),
    index: path.resolve(__dirname, `./assets/scripts/index.ts`),
    serviceWorker: path.resolve(__dirname, `./assets/scripts/serviceWorker.ts`),
  },
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: path.resolve(__dirname, `./server/assets`),
  },
  mode: process.env.NODE_ENV,
  module: {
    rules: [
      {
        test: /\.(ts|tsx|js|jsx)$/,
        exclude: /(node_modules)/,
        use: `swc-loader`,
      },
      {
        test: /\.(png|svg|jpg|jpeg|gif)$/i,
        type: "asset/resource",
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/i,
        type: "asset/resource",
      },
      {
        test: /\.(s|c)ss$/,
        use: ["style-loader", MiniCssExtractPlugin.loader, "css-loader", "postcss-loader", "sass-loader"],
      },
    ],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        {
          from: "node_modules/bootstrap/dist/js/bootstrap.bundle.min.js",
          to: path.resolve(__dirname, "./server/assets/bootstrap.js"),
        },
        !IS_PROD && {
          from: "node_modules/bootstrap/dist/js/bootstrap.bundle.min.js.map",
          to: path.resolve(__dirname, "./server/assets/bootstrap.min.js.map"),
        },
        {
          from: "node_modules/bootstrap/dist/css/bootstrap.css",
          to: path.resolve(__dirname, "./server/assets/bootstrap.css"),
        },
        {
          from: `./manifest.json`,
          to: path.resolve(__dirname, `./server/assets/manifest.json`),
        },
        {
          from: `./assets/logo.svg`,
          to: path.resolve(__dirname, `./server/assets/logo.svg`),
        },
        !IS_PROD && {
          from: "node_modules/bootstrap/dist/css/bootstrap.css.map",
          to: path.resolve(__dirname, "./server/assets/bootstrap.css.map"),
        },
      ].filter(Boolean),
    }),
    new MiniCssExtractPlugin({
      filename: "[name].css",
    }),
    ...VIEWS.map(
      ({ rust }) =>
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, `./views/${rust}`),
          outName: `index`,
          args: "--log-level warn",
          extraArgs: IS_PROD ? "--no-typescript" : ``,
          mode: process.env.NODE_ENV,
          target: `web`,
          watchDirectories: [
            path.resolve(__dirname, `./views/${rust}/src`),
            path.resolve(__dirname, `./views/components`),
          ],
        }),
    ),
  ],
  resolve: {
    extensions: [".js", ".jsx", ".ts", ".tsx"],
  },
};
