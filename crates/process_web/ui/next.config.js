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
  async redirects() {
    return [
      {
        source: '/',
        destination: '/manage/collection-setting',
        permanent: false,
      },
    ]
  },
};

module.exports = nextConfig;
