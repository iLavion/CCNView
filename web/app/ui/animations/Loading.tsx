// app/ui/animations/Loading.tsx

'use client';
import { useEffect, useState } from 'react';
import Spinner from './Spinner';
import Skeleton from './Skeleton';
import Pulse from './Pulse';
import Progress from './Progress';

type LoadingType = 'spinner' | 'skeleton' | 'pulse' | 'progress';

interface LoadingProps {
  type?: LoadingType;
  text?: string;
  fullScreen?: boolean;
}

export default function Loading({ type = 'spinner', text, fullScreen = false }: LoadingProps) {
  const loadingText = text;
  
  const containerClasses = fullScreen 
    ? 'fixed inset-0 flex items-center justify-center bg-white/80 dark:bg-gray-900/80 z-50' 
    : 'flex flex-col items-center justify-center p-4';
  
  return (
    <div className={containerClasses}>
      <div className="flex flex-col items-center gap-3">
        {type === 'spinner' && <Spinner />}
        {type === 'skeleton' && <Skeleton />}
        {type === 'pulse' && <Pulse />}
        {type === 'progress' && <Progress />}
        
        {loadingText && (
          <p className="text-sm text-gray-600 dark:text-gray-300 mt-2">{loadingText}</p>
        )}
      </div>
    </div>
  );
}