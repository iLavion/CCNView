// app/ui/animations/Progress.tsx

'use client';
import { useEffect, useState } from 'react';

interface ProgressProps {
  value?: number; // 0-100, if undefined it's indeterminate
  color?: string;
  height?: string;
  width?: string;
}

export default function Progress({ 
  value, 
  color = 'blue-600', 
  height = 'h-1',
  width = 'w-full'
}: ProgressProps) {
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    // If value is defined, use it
    if (value !== undefined) {
      setProgress(value);
      return;
    }

    // Otherwise create an indeterminate animation
    const timer = setInterval(() => {
      setProgress(prev => {
        if (prev >= 100) return 0;
        return prev + 1;
      });
    }, 10);

    return () => clearInterval(timer);
  }, [value]);

  return (
    <div className={`bg-gray-200 dark:bg-gray-700 rounded-full ${width} ${height} overflow-hidden`}>
      <div 
        className={`bg-${color} h-full rounded-full transition-all duration-300 ease-in-out`} 
        style={{ 
          width: `${progress}%`,
          transition: value === undefined ? 'width 0.5s ease-in-out' : undefined
        }}
        role="progressbar" 
        aria-valuenow={value !== undefined ? value : undefined}
        aria-valuemin={0}
        aria-valuemax={100}
      />
    </div>
  );
}
