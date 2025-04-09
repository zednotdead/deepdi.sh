import { clsx } from 'clsx';
import type { FC } from 'react';
import * as Tooltip from '@radix-ui/react-tooltip';
import { CircleCheckIcon, CircleXIcon } from 'lucide-react';

import { diets } from './diets';

export const DietList: FC<{ diets: string[]; type?: string; className?: string }> = ({
  diets: inputDiets,
  className,
  type,
}) => {
  const dietsDisplay = diets(type).map((diet) => {
    const violates = inputDiets.includes(diet.id);

    return ({
      ...diet,
      violates,
      description: violates ? diet.unfriendly : diet.friendly,
    });
  });

  return (
    <div className={clsx('mr-2', className)}>
      <div className="font-semibold font-heading border-b-2 pb-2 mb-2 border-background-700">Diets</div>
      <ul>
        {dietsDisplay.map(diet => (
          <li key={diet.id} className="flex flex-row justify-between items-center mb-2 last:mb-0">
            <span className="flex flex-row">
              <span className="mr-2">{diet.icon}</span>
              {diet.name}
            </span>
            <div>
              <Tooltip.Root>
                <Tooltip.Trigger>
                  {diet.violates
                    ? (
                        <CircleXIcon className="text-red-900" />
                      )
                    : (
                        <CircleCheckIcon className="text-green-900" />
                      )}
                </Tooltip.Trigger>
                <Tooltip.Portal>
                  <Tooltip.Content
                    className={
                      clsx(
                        'p-2 rounded',
                        diet.violates ? 'bg-red-300' : 'bg-background-700',
                      )
                    }
                  >
                    {diet.description}
                    <Tooltip.Arrow
                      className={
                        clsx(
                          diet.violates ? 'fill-red-300' : 'fill-background-700',
                        )
                      }
                    />
                  </Tooltip.Content>
                </Tooltip.Portal>
              </Tooltip.Root>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};
