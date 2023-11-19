const path = require("path");

const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  experiments: {
    asyncWebAssembly: true,
  },
  module: {},
  plugins: [new CopyPlugin({ patterns: ["index.html", "styles.css"] })],
};
