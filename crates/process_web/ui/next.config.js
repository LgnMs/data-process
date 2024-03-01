const withMDX = require('@next/mdx')()
const isProd = process.env.NODE_ENV === "production";

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "standalone",
  // Configure `pageExtensions` to include MDX files
  pageExtensions: ['js', 'jsx', 'mdx', 'ts', 'tsx'],
  // Optionally, add any other Next.js config below
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
        source: "/",
        destination: "/manage/collection-setting",
        permanent: false,
      },
    ];
  },
};

module.exports = withMDX(nextConfig);
