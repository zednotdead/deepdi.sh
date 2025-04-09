import { clsx } from 'clsx';
import { DetailedHTMLProps, forwardRef, InputHTMLAttributes, ReactElement } from 'react';

import { editBorder } from '~/utils/classes';

type NativeInputProps = DetailedHTMLProps<InputHTMLAttributes<HTMLInputElement>, HTMLInputElement>;

interface InputProps extends NativeInputProps {
  inputClassName?: string;
  icon?: ReactElement;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(function Input(
  {
    inputClassName,
        className,
        icon,
        ...props
  },
  ref,
) {
  return (
    <div
      className={clsx([
        'flex flex-row justify-stretch items-end focus-within:bg-background-900', 'py-2 pr-2',
        editBorder,
        className,
      ])}
    >
      <input
        className={clsx('bg-transparent flex-grow outline-none w-full', inputClassName)}
        ref={ref}
        {...props}
      />
      {icon && (
        <span className="w-6 h-6 ml-2 flex-grow-0" aria-hidden="true">
          {icon}
        </span>
      )}
    </div>

  );
});
