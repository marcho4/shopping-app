import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  eslint: {
    // Отключаем проверку ESLint при сборке
    ignoreDuringBuilds: true,
  },
};

export default nextConfig;
