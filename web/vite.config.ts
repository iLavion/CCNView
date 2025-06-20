import { reactRouter } from '@react-router/dev/vite';
import { defineConfig } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';
import path from 'path';

export default defineConfig({
  plugins: [reactRouter(), tsconfigPaths()],
  css: {
    postcss: './postcss.config.js',
  },
  resolve: {
    alias: {
      '~': path.resolve(__dirname, './app'),
    },
  },
});
