// app/ui/Background.tsx

import type { ReactNode } from 'react';

interface BackgroundProps {
  children: ReactNode;
  className?: string;
}

export default function Background({ children, className = '' }: BackgroundProps) {
  return (
    <div className={`relative min-h-screen w-full ${className}`}>
      {/* Grid with dots background */}
      <div className="absolute inset-0 w-full h-full">
        <div
          className="absolute inset-0 w-full h-full"
          style={{
            backgroundImage:
              'radial-gradient(rgba(39, 39, 56, 0.40) 1px, transparent 1px), linear-gradient(to right, rgba(0, 0, 0, 0.05) 1px, transparent 1px), linear-gradient(to bottom, rgba(0, 0, 0, 0.05) 1px, transparent 1px)',
            backgroundSize: '20px 20px, 20px 20px, 20px 20px',
            backgroundPosition: '0 0, 0 0, 0 0',
          }}
        />
      </div>

      {/* Content */}
      <div className="relative z-10 h-full w-full">{children}</div>
    </div>
  );
}
