// app/ui/shared/containers/FlexContainer.tsx

interface FlexContainerProps {
  children?: React.ReactNode;
  className?: string;
}

export function FlexContainer({ children, className }: FlexContainerProps) {
  return (
    <div className={`flex flex-col items-center justify-center w-full ${className}`}>
      {children}
    </div>
  );
}
