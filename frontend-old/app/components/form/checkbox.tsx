import { clsx } from 'clsx';
import type { DetailedHTMLProps, InputHTMLAttributes, PropsWithChildren } from 'react';
import { forwardRef } from 'react';

export type CheckboxType = DetailedHTMLProps<InputHTMLAttributes<HTMLInputElement>, HTMLInputElement>;

export const Checkbox = forwardRef<HTMLInputElement, CheckboxType>(({ className, ...props }, ref) => {
  return (
    <input type="checkbox" id="gluten" value="gluten" className={clsx(className)} ref={ref} {...props} />
  );
});

Checkbox.displayName = 'Checkbox';

type CheckboxRowProps = CheckboxType & {
  labelClassName?: string;
  checkboxClassName?: string;
};

export const CheckboxRow = forwardRef<HTMLInputElement, PropsWithChildren<CheckboxRowProps>>(
  ({
    className, children, id, checkboxClassName, labelClassName, ...props
  }, ref) => {
    return (
      <div className={clsx('flex flex-row justify-between', className)}>
        <label className={clsx('flex flex-row items-center', labelClassName)} htmlFor={id}>{children}</label>
        <Checkbox ref={ref} id={id} className={clsx(checkboxClassName)} {...props} />
      </div>
    );
  });

CheckboxRow.displayName = 'CheckboxRow';
