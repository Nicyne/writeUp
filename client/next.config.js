/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  poweredByHeader: false,
  experimental: {
    outputStandalone: true,
  },
  async rewrites() {
    let url = process.env.API_URL ?? 'http://localhost:8080/api/';
    if (!url.endsWith('/')) url += '/';
    url += ':path*';

    return [
      {
        source: '/api/:path*',
        destination: url, // Proxy to Backend
      },
    ];
  },
};

module.exports = nextConfig;
