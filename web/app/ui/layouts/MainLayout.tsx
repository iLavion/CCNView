import React from 'react';
import { Outlet } from 'react-router';
import Background from '~/ui/Background';
import Navigation from '~/ui/shared/Navigation';
import Footer from '~/ui/shared/Footer';
import { OverlayProvider } from '~/utilities/Overlay';

function ErrorBoundary({ children }: { children: React.ReactNode }) {
  return <React.Suspense fallback={<div>Loading...</div>}>{children}</React.Suspense>;
}

export default function MainLayout() {
  return (
    <OverlayProvider>
      <Background>
        <div className="min-h-[100dvh] flex flex-col relative font-inter">
          <Navigation />
          <main className="w-full h-full flex flex-grow">
            <ErrorBoundary>
              <Outlet />
            </ErrorBoundary>
          </main>
          <Footer />
          <div id="o-root" className="absolute w-screen h-screen" />
        </div>
      </Background>
    </OverlayProvider>
  );
}