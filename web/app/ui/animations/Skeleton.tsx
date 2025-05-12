// app/ui/animations/Skeleton.tsx

'use client';

interface SkeletonProps {
  count?: number;
  height?: string;
  width?: string;
  className?: string;
}

export default function Skeleton({ 
  count = 1, 
  height = 'h-4', 
  width = 'w-full', 
  className = '' 
}: SkeletonProps) {
  return (
    <div className="w-full space-y-2">
      {Array.from({ length: count }).map((_, i) => (
        <div 
          key={i}
          className={`${height} ${width} bg-gray-200 dark:bg-gray-700 rounded-md animate-pulse ${className}`}
        />
      ))}
    </div>
  );
}
