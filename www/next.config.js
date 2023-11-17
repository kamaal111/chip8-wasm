/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
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
