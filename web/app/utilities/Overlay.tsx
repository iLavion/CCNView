// app/utilities/Overlay.tsx
// Overlay component to manage overlays
import { createPortal } from 'react-dom';
import React, { useState, useEffect } from 'react';

declare global {
  interface Window {
    overlay?: {
      show: (content: React.ReactNode) => () => void;
    };
  }
}

// Preserve React context in overlay
const OverlayPortal = ({
  children,
  onClose,
}: {
  children: React.ReactNode;
  onClose: () => void;
}) => {
  const [container] = useState(() => {
    const div = document.createElement('div');
    div.className = 'relative flex flex-col justify-center items-center w-full h-full';
    return div;
  });

  useEffect(() => {
    const overlayRoot = document.getElementById('o-root');
    if (!overlayRoot) {
      console.error('Overlay root not found');
      return;
    }

    overlayRoot.appendChild(container);

    return () => {
      if (overlayRoot.contains(container)) {
        overlayRoot.removeChild(container);
      }
    };
  }, [container]);

  return createPortal(
    <div className="absolute z-10 flex flex-col items-center justify-center w-full h-full">
      {children}
    </div>,
    container
  );
};

// Provider component to render in your app
export function OverlayProvider({ children }: { children: React.ReactNode }) {
  const [overlayContent, setOverlayContent] = useState<React.ReactNode | null>(null);

  const showOverlay = (content: React.ReactNode) => {
    setOverlayContent(content);
    const overlayRoot = document.getElementById('o-root');
    if (overlayRoot) {
      overlayRoot.style.pointerEvents = 'auto';
      overlayRoot.style.zIndex = '50';
    }

    return () => setOverlayContent(null);
  };

  const hideOverlay = () => {
    setOverlayContent(null);

    // When hiding overlay, update styles to prevent click blocking
    const overlayRoot = document.getElementById('o-root');
    if (overlayRoot) {
      overlayRoot.style.pointerEvents = 'none';
      overlayRoot.style.zIndex = '-1';
    }
  };

  // Initialize overlay root styles and expose the API
  useEffect(() => {
    const overlayRoot = document.getElementById('o-root');
    if (overlayRoot) {
      overlayRoot.style.pointerEvents = 'none';
      overlayRoot.style.zIndex = '-1';
    }

    // THIS IS THE CRITICAL PART - Add to window
    window.overlay = { show: showOverlay };

    return () => {
      // Clean up when provider unmounts
      delete window.overlay;
    };
  }, []); // Empty dependency array ensures this runs only once

  return (
    <>
      {children}
      {overlayContent && <OverlayPortal onClose={hideOverlay}>{overlayContent}</OverlayPortal>}
    </>
  );
}

// Simple overlay utility that uses the global API
export const overlay = {
  show: (content: React.ReactNode) => {
    console.log('Attempting to show overlay');
    if (typeof window !== 'undefined' && window.overlay) {
      return window.overlay.show(content);
    } else {
      console.error('Overlay provider not initialized');
      return () => {};
    }
  },
};
