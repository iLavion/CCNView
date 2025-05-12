// app/ui/shared/containers/DocsContainer.tsx

'use client';
import { FlexContainer } from '~/ui/shared/containers/FlexContainer';
import { CardContainer } from '~/ui/shared/containers/CardContainer';

interface DocsContainerProps {
  title: string;
  children: React.ReactNode;
  author: string;
  date: string;
}

export function DocsContainer({ title, children, author, date }: DocsContainerProps) {
  return (
    <FlexContainer>
      <div className="w-full h-full flex flex-col items-center justify-center p-4">
        <CardContainer active={true} className="shadow-[0_0_25px_rgba(var(--invalsia-500),0.5)] dark:shadow-[0_0_35px_rgba(var(--color-invalsia-500),0.6)]">
          <h1 className="text-6xl font-normal relative">{title}</h1>
          <p className="text-lg mt-4">{children}</p>
          <p className="text-sm mt-2">By: {author}</p>
          <p className="text-sm mt-2">Date: {date}</p>
        </CardContainer>
      </div>
    </FlexContainer>
  );
}
