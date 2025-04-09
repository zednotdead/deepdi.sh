import { carton } from '@lucide/lab';
import { CarrotIcon, Icon, WheatOffIcon } from 'lucide-react';
import type { ReactElement } from 'react';

export const diets = (type: string = 'ingredient'): { id: string; name: string; icon?: ReactElement; friendly: string; unfriendly: string }[] => [
  {
    id: 'vegan',
    name: 'Vegan',
    icon: <CarrotIcon />,
    friendly: `This ${type} is vegan friendly`,
    unfriendly: `This ${type} is not vegan friendly`,
  },
  {
    id: 'vegetarian',
    name: 'Vegetarian',
    icon: <Icon iconNode={carton} />,
    friendly: `This ${type} is vegetarian friendly`,
    unfriendly: `This ${type} is not vegetarian friendly`,
  },
  {
    id: 'gluten_free',
    name: 'Gluten-free',
    icon: <WheatOffIcon />,
    friendly: `This ${type} is gluten free.`,
    unfriendly: `This ${type} contains gluten.`,
  },
];
