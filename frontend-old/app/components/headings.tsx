import { clsx } from 'clsx';
import type { ElementType, FC, PropsWithChildren } from 'react';

import type { PolymorphicProps } from '~/utils/polymorphicProps';

export type HeadingType = React.DetailedHTMLProps<React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement>;

export const Title: FC<PropsWithChildren<HeadingType>> = ({ children, className, ...props }) => (
  <h1 className={clsx('text-4xl font-heading text-text-50', className)} {...props}>
    {children}
  </h1>
);

type HeadingProps<T extends ElementType> = PropsWithChildren<HeadingType & PolymorphicProps<T>>;

export function Heading<T extends ElementType>({
  as,
  children,
  className,
  ...props
}: HeadingProps<T>) {
  const Component = as || 'h2';

  return (
    <Component className={clsx('text-2xl font-heading text-text-50 mb-2', className)} {...props}>
      {children}
    </Component>
  );
}

type DateTextProps<T extends ElementType> = PropsWithChildren<HeadingType & PolymorphicProps<T>>;

export function DateText<T extends ElementType>({
  as,
  children,
  className,
  ...props
}: DateTextProps<T>) {
  const Component = as || 'h2';

  return (
    <Component className={clsx('text-md italic text-text-200', className)} {...props}>
      {children}
    </Component>
  );
}
