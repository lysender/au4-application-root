
const path = require('path');

module.exports = {
  entry: './src/single-spa.config.js',
  output: {
    filename: 'single-spa.config.js',
    path: path.resolve(__dirname, 'dist/root-config'),
    library: {
      name: 'rootConfig',
      type: 'amd',
    },
    clean: true,
  },
  mode: 'production',
  externals: [
    /^lodash$/,
    /^single-spa$/
  ],
};
