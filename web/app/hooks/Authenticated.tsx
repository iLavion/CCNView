// app/hooks/Authenticator.tsx

'use client';

export default function isAuthenticated(): boolean {
  if (typeof window !== 'undefined') {
    const token = localStorage.getItem('token');
    return !!token;
  }
  return false;
}
