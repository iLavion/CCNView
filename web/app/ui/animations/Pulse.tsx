// app/ui/animations/Pulse.tsx

'use client';

interface PulseProps {
  size?: 'small' | 'medium' | 'large';
  color?: string;
  count?: number;
}

export default function Pulse({ size = 'medium', color = 'blue', count = 3 }: PulseProps) {
  // Determine size based on prop
  const sizeClass = {
    small: 'w-2 h-2',
    medium: 'w-3 h-3',
    large: 'w-4 h-4',
  }[size];

  // Determine color based on prop
  const colorClass = `bg-${color}-600`;
  
  return (
    <div className="flex space-x-2">
      {Array.from({ length: count }).map((_, i) => (
        <div key={i} className="flex items-center justify-center">
          <div 
            className={`${sizeClass} ${colorClass} rounded-full animate-pulse`}
            style={{ animationDelay: `${i * 0.15}s` }}
          />
        </div>
      ))}
    </div>
  );
}
