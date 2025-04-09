import clsx from 'clsx';
import { DetailedHTMLProps, forwardRef, HTMLAttributes, PropsWithChildren } from 'react';

type ParagraphType = DetailedHTMLProps<HTMLAttributes<HTMLParagraphElement>, HTMLParagraphElement>;

type ErrorLineProps = PropsWithChildren<ParagraphType>;

export const ErrorLine = forwardRef<HTMLInputElement, ErrorLineProps>(
  function ErrorLine({ className, children, ...props }, ref) {
    return (
      <p ref={ref} className={clsx('text-red-700 font-bold my-1', className)} {...props}>{children}</p>
    );
  },
);
