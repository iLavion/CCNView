// app/pages/Error.tsx

'use client';
import { useState, useEffect } from 'react';
import { FlexContainer } from '~/ui/shared/containers/FlexContainer';
import { CardContainer } from '~/ui/shared/containers/CardContainer';

type ErrorProps = {
    code?: number;
    message?: string;
    details?: string;
    readMoreUrl?: string;
};

export function ErrorPage({ code, message = 'Page not found', details, readMoreUrl }: ErrorProps) {
    const [mounted, setMounted] = useState(false);
    
    useEffect(() => {
        setMounted(true);
    }, []);

    if (!mounted) return null;

    return (
        <FlexContainer>
            <div className="w-full h-full flex flex-col items-center justify-center p-4">
                <CardContainer
                    active={true}
                    className="shadow-[0_0_25px_rgba(var(--invalsia-500),0.5)] dark:shadow-[0_0_35px_rgba(var(--color-invalsia-500),0.6)]"
                >
                    <h1 className="text-6xl font-normal">
                        {readMoreUrl ? (
                            <a href={readMoreUrl} className="cursor-pointer hover:underline">
                                {code}
                            </a>
                        ) : (
                            code
                        )}
                    </h1>
                    <p className="text-2xl font-normal">{message}</p>
                    {details && <p className="text-sm text-gray-500">{details}</p>}
                </CardContainer>
            </div>
        </FlexContainer>
    );
}
