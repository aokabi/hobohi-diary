import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: 'standalone',
  // Docker内でのAPIアクセスを許可
  experimental: {
    serverActions: {
      allowedOrigins: ['localhost:3000', 'localhost:9001'],
    },
  },
};

export default nextConfig;
