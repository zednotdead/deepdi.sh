import { clsx } from 'clsx';
import type { FC, PropsWithChildren } from 'react';

export const Centered: FC<PropsWithChildren<{ className?: string }>> = ({ className, children }) => {
  return (
    <div className={clsx('max-w-screen-md mx-auto', className)}>
      {children}
    </div>
  );
};
