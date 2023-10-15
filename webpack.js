'use strict';
const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

/**
 * TODO:
 * 
 * - Better optimize prod WASM builds (WASM code splitting? IDK)
 * - yarn compile::dev::watch seems to recompile on save but browser doesn't always reflect latest changes. ?
 * - Watching the /components dir on each package is really innefficient (see: watchDirectories on every WasmPackPlugin).
 *   Find a more efficient way to recompile only what's necessary when /components changes
 */

const IS_PROD = process.env.NODE_ENV === `production`;

module.exports = {
  entry: {
    homeView: path.resolve(__dirname, `./views/home_view/homeView.ts`),
    loginView: path.resolve(__dirname, `./views/login_view/loginView.ts`),
    profileView: path.resolve(__dirname, `./views/profile_view/profileView.ts`),
    recentlyRented: path.resolve(__dirname, `./views/recently_rented_view/recentlyRentedView.ts`),
    topTenView: path.resolve(__dirname, `./views/top_ten_view/topTenView.ts`),
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
        type: 'asset/resource', 
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/i,
        type: 'asset/resource',
      },
      {
        test: /\.(s|c)ss$/,
        use: ['style-loader', MiniCssExtractPlugin.loader, 'css-loader', 'postcss-loader', 'sass-loader']
      },
    ],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        {
          from: 'node_modules/bootstrap/dist/js/bootstrap.bundle.min.js',
          to: path.resolve(__dirname, './server/assets/bootstrap.js'),
        },
        !IS_PROD && {
          from: 'node_modules/bootstrap/dist/js/bootstrap.bundle.min.js.map',
          to: path.resolve(__dirname, './server/assets/bootstrap.min.js.map'),
        },
        {
          from: 'node_modules/bootstrap/dist/css/bootstrap.css',
          to: path.resolve(__dirname, './server/assets/bootstrap.css'),
        },
        !IS_PROD && {
          from: 'node_modules/bootstrap/dist/css/bootstrap.css.map',
          to: path.resolve(__dirname, './server/assets/bootstrap.css.map'),
        },
      ].filter(Boolean),
    }),
    new MiniCssExtractPlugin({
      filename: '[name].css',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, `./views/home_view`),
      outName: `home_view`,
      args: '--log-level warn',
      extraArgs: process.env.NODE_ENV === `production` ? '--no-typescript' : ``,
      mode: process.env.NODE_ENV,
      target: `web`,
      watchDirectories: [
        path.resolve(__dirname, `./views/home_view/src`),
        path.resolve(__dirname, `./views/components`)
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, `./views/login_view`),
      outName: `login_view`,
      args: '--log-level warn',
      extraArgs: process.env.NODE_ENV === `production` ? '--no-typescript' : ``,
      mode: process.env.NODE_ENV,
      target: `web`,
      watchDirectories: [
        path.resolve(__dirname, `./views/login_view/src`),
        path.resolve(__dirname, `./views/components`)
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, `./views/profile_view`),
      outName: `home_view`,
      args: '--log-level warn',
      extraArgs: process.env.NODE_ENV === `production` ? '--no-typescript' : ``,
      mode: process.env.NODE_ENV,
      target: `web`,
      watchDirectories: [
        path.resolve(__dirname, `./views/profile_view/src`),
        path.resolve(__dirname, `./views/components`)
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, `./views/recently_rented_view`),
      outName: `home_view`,
      args: '--log-level warn',
      extraArgs: process.env.NODE_ENV === `production` ? '--no-typescript' : ``,
      mode: process.env.NODE_ENV,
      target: `web`,
      watchDirectories: [
        path.resolve(__dirname, `./views/recently_rented_view/src`),
        path.resolve(__dirname, `./views/components`)
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, `./views/top_ten_view`),
      outName: `home_view`,
      args: '--log-level warn',
      extraArgs: process.env.NODE_ENV === `production` ? '--no-typescript' : ``,
      mode: process.env.NODE_ENV,
      target: `web`,
      watchDirectories: [
        path.resolve(__dirname, `./views/top_ten_view/src`),
        path.resolve(__dirname, `./views/components`)
      ]
    }),
  ],
  resolve: {
    extensions: ['.js', '.jsx', '.ts', '.tsx'],
  },
};