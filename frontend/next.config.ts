import type { NextConfig } from "next";
import { env } from "process";

const nextConfig: NextConfig = {
  output: env.NODE_ENV === 'production' ? 'export' : 'standalone',
  // Docker内でのAPIアクセスを許可
  experimental: {
    serverActions: {
      allowedOrigins: ['localhost:3000', 'localhost:9001'],
    },
  },
};

export default nextConfig;
