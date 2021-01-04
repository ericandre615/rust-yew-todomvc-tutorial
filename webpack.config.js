const path = require('path');

module.exports = {
  entry: './static/main.js',
  devServer: {
    contentBase: './static',
    historyApiFallback: true,
  },
};

