import { clsx } from 'clsx';
import { forwardRef } from 'react';

type NativeButtonType = React.DetailedHTMLProps<React.ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement>;

export const IconButton = forwardRef<HTMLButtonElement, NativeButtonType>(
  function ButtonAddNew({ children, type, className, ...props }, ref) {
    return (
      <button
        className={clsx(
          'hover:bg-background-900 active:bg-background-800',
          'p-4 rounded-full flex-none',
          'w-14 h-14',
          className,
        )}
        type={type ?? 'button'}
        ref={ref}
        {...props}
      >
        {children}
      </button>
    );
  },
);
