// app/providers/Invalsia.tsx

'use client';

import axios from 'axios';

interface APIProps {
  v: number;
  t: string | null;
}

export const api = {
  get: (url: string, config = {}) => {
    const instance = createInstance();
    return instance.get(url, config);
  },
  post: (url: string, data = {}, config = {}) => {
    const instance = createInstance();
    return instance.post(url, data, config);
  },
  put: (url: string, data = {}, config = {}) => {
    const instance = createInstance();
    return instance.put(url, data, config);
  },
  delete: (url: string, config = {}) => {
    const instance = createInstance();
    return instance.delete(url, config);
  },
  patch: (url: string, data = {}, config = {}) => {
    const instance = createInstance();
    return instance.patch(url, data, config);
  }
};

function createInstance(props: Partial<APIProps> = {}) {
  const { t = null } = props;
  const isDev = process.env.NODE_ENV === 'development'
  console.log(process.env.NODE_ENV);

  return axios.create({
    baseURL: isDev ? `http://localhost:5174/` : `https://api.limovia.se/`,
    headers: {
      'Content-Type': 'application/json',
      'Authorization': t ? `Bearer ${t}` : undefined,
    },
  });
}