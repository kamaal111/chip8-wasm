/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: false,
  output: "export",
  webpack: (config, { isServer }) => {
    const webpackConfig = {
      ...config,
      experiments: {
        ...config.experiments,
        asyncWebAssembly: true,
      },
    };

    const wasmOutputPath = "static/wasm/[modulehash].wasm";
    if (isServer) {
      return {
        ...webpackConfig,
        output: {
          ...webpackConfig.output,
          webassemblyModuleFilename: `../${wasmOutputPath}`,
        },
      };
    }

    return {
      ...webpackConfig,
      output: {
        ...webpackConfig.output,
        webassemblyModuleFilename: wasmOutputPath,
      },
    };
  },
};

module.exports = nextConfig;
