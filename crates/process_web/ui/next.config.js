const withMDX = require("@next/mdx")();
const { readFileSync } = require('fs');
const path = require('path');

const getConfig = () => {
  const filePath = path.resolve(__dirname, 'public/config.json');
  const rawConfig = readFileSync(filePath);
  return JSON.parse(rawConfig.toString());
};

const isProd = process.env.NODE_ENV === "production";

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "standalone",
  // Configure `pageExtensions` to include MDX files
  pageExtensions: ["js", "jsx", "mdx", "ts", "tsx"],
  // Optionally, add any other Next.js config below
  async rewrites() {
    const config = getConfig();
    if (isProd) {
      return [
        {
          source: "/api/:path*",
          destination: `${config.API_HOST}/:path*`,
        },
        {
          source: "/remote-auth/:path*",
          destination: `${config.REMOTE_AUTH_API_HOST}/:path*`,
        },
      ];
    } else {
      return [
        {
          source: "/api/:path*",

          destination: `${config.API_HOST}/:path*`,
        },
        {
          source: "/remote-auth/:path*",
          destination: `${config.REMOTE_AUTH_API_HOST}/:path*`,
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
