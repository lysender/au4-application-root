const webpack = require('webpack');
const path = require('path');
const CleanWebpackPlugin = require('clean-webpack-plugin').CleanWebpackPlugin;

module.exports = {
  entry: './src/single-spa.config.js',
  output: {
    filename: 'single-spa.config.js',
    path: path.resolve(__dirname, 'dist/root-config'),
    library: 'root-config',
    libraryTarget: 'amd',
  },
  mode: 'production',
  module: {
    rules: [
      {parser: {System: false}},
      {
        test: /\.js$/,
        exclude: [path.resolve(__dirname, 'node_modules')],
        loader: 'babel-loader'
      }
    ],
  },
  resolve: {
    modules: [
      __dirname,
      'node_modules',
    ],
  },
  plugins: [
    new CleanWebpackPlugin(),
  ],
  externals: [
    /^lodash$/,
    /^single-spa$/
  ],
};
