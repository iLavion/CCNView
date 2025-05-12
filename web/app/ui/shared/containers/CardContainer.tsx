interface CardContainerProps {
  children?: React.ReactNode;
  className?: string;
  active?: boolean;
}

export function CardContainer({ children, className, active = true }: CardContainerProps) {
  return (
    <div
      className={`flex flex-col items-center justify-center text-center bg-invalsia-black-900 text-invalsia-500 px-12 py-6 rounded-xs border-2 ${
        active ? 'border-invalsia-500' : 'border-invalsia-gray-500'
      } ${className}`}
    >
      {children}
    </div>
  );
}
