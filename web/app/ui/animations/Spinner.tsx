// app/ui/animations/Spinner.tsx

'use client';

interface SpinnerProps {
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl';
  color?: string;
}

export default function Spinner({ size = 'md', color = 'currentColor' }: SpinnerProps) {
  // Determine size based on prop
  const sizeClass = {
    'xs': 'w-3 h-3',
    'sm': 'w-4 h-4',
    'md': 'w-8 h-8',
    'lg': 'w-12 h-12',
    'xl': 'w-16 h-16',
    '2xl': 'w-24 h-24',
    '3xl': 'w-32 h-32',
    '4xl': 'w-40 h-40',
    '5xl': 'w-48 h-48',
    '6xl': 'w-56 h-56',
    '7xl': 'w-64 h-64',
    '8xl': 'w-72 h-72',
  }[size];

  return (
    <div className="flex justify-center items-center">
      <div 
        className={`${sizeClass} border-4 border-transparent border-t-invalsia-500 rounded-full animate-spin`}
        style={{ borderTopColor: color !== 'currentColor' ? color : undefined }}
        role="status"
        aria-label="loading"
      />
    </div>
  );
}
