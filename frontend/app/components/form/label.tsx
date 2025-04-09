import { clsx } from 'clsx';
import type { ElementType } from 'react';

import type { PolymorphicProps } from '~/utils/polymorphicProps';

type LabelType = React.DetailedHTMLProps<React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement>;

type LabelProps<T extends ElementType> = LabelType & PolymorphicProps<T>;

export function Label<T extends ElementType>({
  children,
  htmlFor,
  className,
  as,
  ...props
}: LabelProps<T>) {
  const Component = as || 'label';

  return (
    <Component htmlFor={htmlFor as string} className={clsx('font-heading text-2xl font-semibold', className)} {...props}>
      {children}
    </Component>
  );
}
