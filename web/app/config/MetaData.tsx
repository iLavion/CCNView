// app/config/MetaData.tsx

'use client';
// Basic site information
export const siteMetadata = {
  title: 'Invalsia',
  author: 'Invalsia Team',
  siteUrl: 'https://invalsia.com',
};

// Social media and contact information
export const contactInfo = {
  social: {
    twitter: 'https://twitter.com/invalsia',
    facebook: 'https://facebook.com/invalsia',
    instagram: 'https://instagram.com/invalsia',
    github: 'https://github.com/invalsia',
    linkedin: 'https://linkedin.com/company/invalsia',
  },
};

// Images and assets
export const assets = {
  logo: '/images/logo.svg',
  favicon: '/favicon.ico',
  ogImage: '/images/og-image.jpg',
};

// Open Graph default config
export const openGraph = {
  type: 'website',
  image: {
    url: `${siteMetadata.siteUrl}${assets.ogImage}`,
    width: 512,
    height: 512,
    alt: siteMetadata.title,
  },
};

// Twitter card default config
export const twitter = {
  handle: '@invalsia',
  site: '@invalsia',
  cardType: 'summary_large_image',
};
