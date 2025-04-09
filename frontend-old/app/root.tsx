/// <reference types="vite-plugin-svgr/client" />
import { Links, Meta, Outlet, Scripts, ScrollRestoration, useMatches } from 'react-router';
import '@fontsource-variable/raleway';
import '@fontsource-variable/playfair-display';
import { Suspense, type FC, type PropsWithChildren } from 'react';
import * as Tooltip from '@radix-ui/react-tooltip';
import { Toaster } from 'sonner';

import type { Handle } from './utils/types';
import { TopBar } from './components/topBar';
import './tailwind.css';

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body className="bg-background-950 font-body">
        {children}
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

const Providers: FC<PropsWithChildren> = ({ children }) => (
  <Tooltip.Provider>
    <Suspense fallback={null}>
      <Toaster
        toastOptions={{
          classNames: {
            toast: 'text-body shadow-md shadow-black-100',
            default: 'bg-background-950 border-background-800',
          },
        }}
      />
    </Suspense>
    {children}
  </Tooltip.Provider>
);

export default function App() {
  const matches = useMatches();

  const showNoTopBar = matches.some(({ handle }) => !!(handle as Handle | undefined)?.['noBar']);

  return (
    <Providers>
      {!showNoTopBar && <TopBar />}
      <div className="pt-16">
        <Outlet />
      </div>
    </Providers>
  );
}
