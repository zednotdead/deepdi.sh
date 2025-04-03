import { Link } from 'react-router';
import { clsx } from 'clsx';
import * as NavigationMenuPrimitive from '@radix-ui/react-navigation-menu';
import { forwardRef, type PropsWithChildren } from 'react';
import { ChevronDownIcon } from 'lucide-react';

type AsChild = { asChild?: boolean };

function NavSectionWrapper({ children, asChild }: PropsWithChildren<AsChild>) {
  return (
    <NavigationMenuPrimitive.Item
      data-state="open"
      className="h-full"
      asChild={asChild}
    >
      {children}
    </NavigationMenuPrimitive.Item>
  );
}

function NavSectionButton({ children, asChild, noChevron }: PropsWithChildren<AsChild & { noChevron?: boolean }>) {
  return (
    <NavigationMenuPrimitive.Trigger
      asChild={asChild}
      className={clsx(
        'h-full',
        'px-3 text-md hover:bg-background-800',
        'text-md font-medium',
        'text-text-50',
        'focus:outline-none focus-visible:ring focus-visible:ring-purple-500 focus-visible:ring-opacity-75',
        'flex flex-row items-center gap-1',
        'group',
      )}
    >
      {children}
      {!noChevron && (
        <ChevronDownIcon className="h-5 w-5 group-hover:rotate-180 transition-transform" />
      )}
    </NavigationMenuPrimitive.Trigger>
  );
}

function _NavSectionButtonLink({ children, asChild }: PropsWithChildren<AsChild>) {
  return (
    <NavigationMenuPrimitive.Link
      asChild={asChild}
      className={clsx(
        'h-full',
        'flex flex-row items-center',
        'px-3 text-md hover:bg-background-800',
        'text-md font-medium',
        'text-text-50',
        'focus:outline-none focus-visible:ring focus-visible:ring-purple-500 focus-visible:ring-opacity-75',
      )}
    >
      {children}
    </NavigationMenuPrimitive.Link>
  );
}

type NavSectionContentProps = { className?: string };

function NavSectionContent({ children, className }: PropsWithChildren<NavSectionContentProps>) {
  return (
    <NavigationMenuPrimitive.Content
      className={clsx(
        'absolute w-auto top-0 left-0 rounded-lg',
      )}
    >
      <div className={clsx('p-3', className)}>
        <div className="w-full flex flex-col space-y-2">
          {children}
        </div>
      </div>
    </NavigationMenuPrimitive.Content>
  );
}

type NavSectionContentLinkProps = { external?: boolean; to: string; title: string };

function NavSectionContentLink(
  {
    children,
    external,
    to,
    title,
  }: PropsWithChildren<NavSectionContentLinkProps>,
) {
  const InnerLink = forwardRef<HTMLAnchorElement, PropsWithChildren<{ className?: string }>>(
    ({ children, className }, ref) => external
      ? <a ref={ref} href={to} className={className}>{children}</a>
      : <Link ref={ref} className={className} to={to}>{children}</Link>,
  );
  InnerLink.displayName = 'NavSectionContentLink_InnerLink';

  return (
    <NavigationMenuPrimitive.Link
      href="https://www.radix-ui.com"
      asChild
    >
      <InnerLink
        className={clsx(
          'w-full px-4 py-3 hover:bg-background-800 rounded-md',
          'focus:outline-none focus-visible:ring focus-visible:ring-purple-500 focus-visible:ring-opacity-75',
        )}
      >
        <span className="text-md font-semibold text-text-50">
          {title}
        </span>

        <div className="mt-1 text-md text-text-50">
          {children}
        </div>
      </InnerLink>
    </NavigationMenuPrimitive.Link>
  );
}

export function TopBar() {
  return (
    <div
      className={
        clsx(
          'w-full h-16',
          'border-b-2 border-primary-400',
          'flex flex-row items-center',
          'px-4',
          'fixed',
          'bg-background-950',
        )
      }
    >
      <Link
        to="/"
        className="flex flex-shrink-0 flex-grow-0 w-fit h-full border-r-2 border-inherit p-2 pr-6 items-center"
      >
        <span className="w-fit h-fit text-xl font-heading font-bold text-center">
          deepdi.sh
        </span>
      </Link>
      <NavigationMenuPrimitive.Root className="relative h-full *:h-full w-full">
        <NavigationMenuPrimitive.List
          className="flex flex-row px-2 space-x-2 font-body h-full items-center justify-center"
        >
          <NavSectionWrapper>
            <NavSectionButton>Ingredients</NavSectionButton>

            <NavSectionContent className="w-[19rem] lg:w-[23rem]">
              <NavSectionContentLink to="/ingredient" title="Ingredients">
                Look at a list of all ingredients in our repository
              </NavSectionContentLink>
              <NavSectionContentLink to="/ingredient/create" title="Add an ingredient">
                Have we forgotten about something? Click here to add a new ingredient.
              </NavSectionContentLink>
            </NavSectionContent>
          </NavSectionWrapper>
          <NavSectionWrapper>
            <NavSectionButton>Recipes</NavSectionButton>

            <NavSectionContent className="w-[18rem] lg:w-[22rem]">
              <NavSectionContentLink to="/recipe" title="Recipes">
                Have a look at all of our recipes!
              </NavSectionContentLink>
              <NavSectionContentLink to="/recipe/create" title="Add a new recipe">
                Got a recipe you want to share with us? Go here!
              </NavSectionContentLink>
            </NavSectionContent>
          </NavSectionWrapper>

          <NavigationMenuPrimitive.Indicator
            className={clsx(
              'z-20',
              'top-[100%] flex items-end justify-center h-2 overflow-hidden',
              'transition-[width_transform] duration-[250ms] ease-[ease]',
            )}
          >
            <div className="top-1 relative bg-background-900 w-2 h-2 rotate-45" />
          </NavigationMenuPrimitive.Indicator>
        </NavigationMenuPrimitive.List>

        <div
          className={clsx(
            'absolute flex justify-center',
            'w-full left-0',
          )}
          style={{
            perspective: '2000px',
          }}
        >
          <NavigationMenuPrimitive.Viewport
            className={clsx(
              'relative mt-2 shadow-lg rounded-md bg-background-900 overflow-hidden',
              'w-radix-navigation-menu-viewport',
              'h-radix-navigation-menu-viewport',
              'origin-[top_center] transition-[width_height] duration-300 ease-[ease]',
            )}
          />
        </div>
      </NavigationMenuPrimitive.Root>
    </div>
  );
}
