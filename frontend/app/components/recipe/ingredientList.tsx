import { Link } from 'react-router';
import { clsx } from 'clsx';
import type { IngredientUnitDTO } from 'common/bindings/IngredientUnitDTO';
import type { IngredientWithAmountDTO } from 'common/bindings/IngredientWithAmountDTO';
import convert from 'convert-units';
import { formatQuantity } from 'format-quantity';
import type { DetailedHTMLProps, FC, HTMLAttributes, PropsWithChildren } from 'react';
import * as Tooltip from '@radix-ui/react-tooltip';
import { CircleHelpIcon, GlassWaterIcon, WeightIcon } from 'lucide-react';

import Spoon from '~/icons/spoon.svg?react';

type UnitIconProps = { unit: IngredientUnitDTO['_type'] };

export const UnitIcon: FC<UnitIconProps> = ({ unit }) => {
  switch (unit) {
    case 'mililiters':
    case 'cups':
      return <GlassWaterIcon />;
    case 'teaspoons':
      return <Spoon />;
    case 'grams':
      return <WeightIcon />;
    default:
      return '';
  }
};

const mapIngredientUnit = (ingredientUnit: IngredientUnitDTO): { unit: string; val: number } => {
  let unit: { unit: string; val: number };

  switch (ingredientUnit._type) {
    case 'cups':
      unit = convert(ingredientUnit.amount).from('cup').toBest();
      break;
    case 'grams':
      unit = convert(ingredientUnit.amount).from('g').toBest();
      break;
    case 'teaspoons':
      unit = convert(ingredientUnit.amount)
        .from('tsp')
        .toBest({
          exclude: [
            'mm3', 'cm3', 'm3', 'km3', 'in3', 'fl-oz', 'pnt', 'qt', 'gal', 'ft3', 'yd3',
          ],
          cutOffNumber: 1,
        });
      break;
    case 'mililiters':
      unit = convert(ingredientUnit.amount).from('ml').toBest();
      break;
    default:
      unit = {
        unit: ingredientUnit.amount.unit,
        val: ingredientUnit.amount.amount,
      };
  }

  return unit;
};

type IngredientListElementProps =
    PropsWithChildren<
      DetailedHTMLProps<
        HTMLAttributes<HTMLUListElement>,
        HTMLUListElement
      >
    >;

function IngredientListElement({ children, className, ...props }: IngredientListElementProps) {
  return (
    <ul
      className={
        clsx(
          'flex flex-col',
          'w-full',
          'border-background-800 border-solid border rounded',
          'shadow-primary-900 shadow-md',
          'text-md',
          className,
        )
      }
      {...props}
    >
      {children}
    </ul>
  );
}

type IngredientListProps = { ingredients: IngredientWithAmountDTO[]; className: string };

export function IngredientList({ ingredients, className }: IngredientListProps) {
  return (
    <IngredientListElement className={clsx(className)}>
      {ingredients.map(ingredientWithAmount => (
        <IngredientListItem
          ingredient={ingredientWithAmount}
          key={ingredientWithAmount.ingredient.id}
        />
      ))}
    </IngredientListElement>
  );
}

export function IngredientListItem({ ingredient }: { ingredient: IngredientWithAmountDTO }) {
  const unit = mapIngredientUnit(ingredient.amount);
  const amount = `${formatQuantity(unit.val, true)} ${unit.unit}`;

  return (
    <Tooltip.Root>
      <li
        className="last:border-none border-b-2 border-background-900 \
        hover:bg-background-900 transition-colors"
      >
        <Link
          className="flex flex-row justify-between font-body p-2"
          to={`/ingredient/${ingredient.ingredient.id}`}
        >
          <div className="flex flex-row">
            <b className="capitalize mr-2">
              {ingredient.ingredient.name}
            </b>
            {ingredient.notes && (
              <Tooltip.Trigger>
                <CircleHelpIcon className="text-primary-300" />
              </Tooltip.Trigger>
            )}
          </div>
          <div className="flex flex-row items-baseline">
            {ingredient.optional && <span className="font-semibold text-text-400 text-sm">(optional)</span>}
            <div className="flex flex-row items-center">
              <UnitIcon unit={ingredient.amount._type} />
              <span className="ml-2">{amount}</span>
            </div>
          </div>
        </Link>

        {ingredient.notes && (
          <Tooltip.Portal>
            <Tooltip.Content className="bg-background-700 p-2 rounded">
              {ingredient.notes}
              <Tooltip.Arrow className="fill-background-700" />
            </Tooltip.Content>
          </Tooltip.Portal>
        )}
      </li>
    </Tooltip.Root>
  );
}
