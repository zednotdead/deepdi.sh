import { reactRouter } from '@react-router/dev/vite';
import { defineConfig } from 'vite';
import svgr from 'vite-plugin-svgr';
import UnpluginTypia from '@ryoppippi/unplugin-typia/vite';

export default defineConfig({
  define: {
    global: {},
  },
  build: {
    commonjsOptions: {
      transformMixedEsModules: true,
    },
  },
  server: {
    proxy: {
      '/v1/traces': 'http://localhost:4318',
    },
  },
  plugins: [
    reactRouter(),
    svgr(),
    UnpluginTypia(),
  ],
});
