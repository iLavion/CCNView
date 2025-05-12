// app/utilities/Token.tsx

// Get session token
export function getToken(): string | null {
    const token = localStorage.getItem('token');
    if (token) {
        return token;
    }
    return null;
}

// Set session token
export function setToken(token: string): void {
    localStorage.setItem('token', token);
}

// Remove session token
export function removeToken(): void {
    localStorage.removeItem('token');
}

// Check if token exists and is valid
export function isTokenValid(): boolean {
    const token = getToken();
    if (!token) return false;
    return true;
}