/** @type {import('next').NextConfig} */
const isProd = process.env.NODE_ENV === "production";

const nextConfig = {
  async rewrites() {
    if (isProd) {
      return [
        {
          source: "/:path*",
          destination: `${process.env.NEXT_PUBLIC_API_HOST}/:path*`,
        },
      ];
    } else {
      return [
        {
          source: "/:path*",
          destination: `${process.env.NEXT_PUBLIC_API_HOST}/:path*`,
        },
      ];
    }
  },
};

module.exports = nextConfig;
