const path = require("path");

const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  entry: "./bootstrap.js",
  module: {
    rules: [
      {
        test: /\.ts?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.js'],
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
  module: {},
  plugins: [new CopyPlugin({ patterns: ["index.html"] })],
};
