/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
  rewrites: async () => [
    {
      source: '/api/:path*',
      destination: 'http://backend:3000/api/:path*',
    },
  ],
};

export default nextConfig;
