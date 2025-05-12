// app/routes/Error.tsx

import { useRouteError, isRouteErrorResponse } from 'react-router';
import type { Route } from './+types/Error';
import { ErrorPage } from '~/pages/Error';
import { siteMetadata } from '~/config/MetaData';

export function meta({ params, data }: Route.MetaArgs) {
  // Safe access to ensure we don't get undefined values
  const errorCode = params?.code || (data?.code ? String(data.code) : '404');
  const title = `Error ${errorCode} - ${siteMetadata.title}`;
  const description = 'An unexpected error occurred';
  const bots = 'noindex,nofollow';

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
}

export async function loader({ params }: Route.LoaderArgs) {
  // Default to 404 for catch-all routes when no code is provided
  const code = params?.code ? parseInt(params.code, 10) : 404;
  return { code };
}

export default function ErrorRoute() {
  const error = useRouteError();
  const code = extractErrorCode(error);
  return <ErrorPage code={code} />;
}

// Error boundary component
export function ErrorBoundary() {
  const error = useRouteError();
  const code = extractErrorCode(error);
  return <ErrorPage code={code} />;
}

// More robust error code extraction
function extractErrorCode(error: unknown): number {
  if (!error) return 404;
  if (isRouteErrorResponse(error)) return Number(error.status || 404);
  if (error && typeof error === 'object') {
    if ('code' in error && error.code) return Number(error.code);
    if ('status' in error && error.status) return Number(error.status);
  }
  if (error instanceof Error) return 500;
  return 404;
}
