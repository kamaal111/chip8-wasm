/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  webpack: (config) => {
    return {
      ...config,
      experiments: {
        ...config.experiments,
        asyncWebAssembly: true,
        syncWebAssembly: true,
      },
    };
  },
};

module.exports = nextConfig;
