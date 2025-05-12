// app/routes/Home.tsx

import type { MetaFunction } from 'react-router';
import type { Route } from './+types/PrivacyPolicy';
import { siteMetadata } from '~/config/MetaData';
import HomePage from '~/pages/legal/PrivacyPolicy';
import { useTranslation } from 'react-i18next';

export const meta: MetaFunction = () => {
  const title = `Privacy Policy - ${siteMetadata.title}`;
  const description = siteMetadata.description || 'Welcome to our website!';
  const bots = 'index,follow';

  return [
    { title },
    { name: 'description', content: description },
    { name: 'robots', content: bots },
    { name: 'googlebot', content: bots },
    { name: 'revisit-after', content: 'never' },
    { name: 'X-Robots-Tag', content: bots, httpEquiv: 'X-Robots-Tag' },
    { property: 'og:type', content: 'website' },
    { property: 'og:title', content: title },
    { property: 'og:description', content: description },
    { property: 'og:site_name', content: title },
    { name: 'twitter:card', content: 'summary' },
    { name: 'twitter:title', content: title },
    { name: 'twitter:description', content: description },
  ];
};

export default function HomeRoute() {
  return <HomePage />;
}
