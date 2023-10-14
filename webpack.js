'use strict';
const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  entry: {
    shared: path.resolve(__dirname, `./server/assets/shared.ts`),
    homeView: path.resolve(__dirname, `./views/home_view/homeView.ts`),
    loginView: path.resolve(__dirname, `./views/login_view/loginView.ts`),
  },
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: path.resolve(__dirname, `./server/assets/dist`),
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
     }
    ],
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: '[name].css',
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, `./views/login_view`),
      outName: `login_view`,
      args: '--log-level warn',
      extraArgs: process.env.NODE_ENV === `production` ? '--no-typescript' : ``,
      mode: process.env.NODE_ENV,
      target: `web`,
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, `./views/home_view`),
      outName: `home_view`,
      args: '--log-level warn',
      extraArgs: process.env.NODE_ENV === `production` ? '--no-typescript' : ``,
      mode: process.env.NODE_ENV,
      target: `web`,
    }),
  ],
  resolve: {
    extensions: ['.js', '.jsx', '.ts', '.tsx'],
  },
};