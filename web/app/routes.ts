// app/routes.ts
import { type RouteConfig, route, index, layout, } from '@react-router/dev/routes';

// Helper function to create common routes with prefixed IDs
function createCommonRoutes(prefix: string) {
  return [
    index('./routes/Home.tsx', { id: `${prefix}_home` }),
    route('privacy-policy', './routes/legal/PrivacyPolicy.tsx', {
      id: `${prefix}_privacy-policy`,
    }),
    route('terms-of-service', './routes/legal/TermsOfService.tsx', {
      id: `${prefix}_terms-of-service`,
    }),
    route(':code(\\d+)', './routes/Error.tsx', { id: `${prefix}_error_code` }),
  ];
}

export default [
  layout('./ui/layouts/MainLayout.tsx', [
    // Normal routes
    ...createCommonRoutes('root'),
    // Catch-all route for 404 errors
    route('*', './routes/Error.tsx', { id: 'root_splat' }),
  ]),
] satisfies RouteConfig;
