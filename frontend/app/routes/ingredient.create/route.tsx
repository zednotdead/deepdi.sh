import type { ActionFunctionArgs, MetaFunction } from 'react-router';
import { Form, useActionData, useNavigate, useSubmit } from 'react-router';
import { clsx } from 'clsx';
import { Controller, useForm } from 'react-hook-form';
import type { CreateIngredientDTO } from 'common/bindings/CreateIngredientDTO';
import { assert } from 'typia';
import type { SerializedEditorState } from 'lexical';
import type { IngredientDTO } from 'common/bindings/IngredientDTO';
import { PenLineIcon } from 'lucide-react';
import { useEffect } from 'react';
import { toast } from 'sonner';

import { Centered } from '~/components/centered';
import { Editor } from '~/components/editor';
import { editBorder } from '~/utils/classes';
import { Label } from '~/components/form/label';
import { CheckboxRow } from '~/components/form/checkbox';
import { renderToPlaintext } from '~/components/editor/renderPlaintext';
import { makeTitle } from '~/utils/makeTitle';
import { diets } from '~/components/ingredients/diets';
// import { Handle } from '~/utils/types';

export const meta: MetaFunction = () => [
  { title: makeTitle('Create a new ingredient') },
];

export async function action({ request }: ActionFunctionArgs) {
  const data = await request.formData();

  const parsed = {
    ...Object.fromEntries(data.entries()),
  };

  const ingredient = assert<CreateIngredientDTO>({
    name: parsed.name as string,
    description: parsed.description as string,
    diet_violations: (parsed.dietViolations as unknown as string).split(','),
  });

  const createdIngredient: IngredientDTO = await fetch(
    'http://localhost:8111/ingredient/create',
    { method: 'POST', body: JSON.stringify(ingredient), headers: { 'Content-Type': 'application/json' } },
  ).then(res => res.json());

  return {
    ingredient: createdIngredient,
  };
}

interface IngredientCreateForm {
  name: string;
  description: SerializedEditorState;
  dietViolations?: string[];
}

export default function CreateIngredientRoute() {
  const data = useActionData<typeof action>();
  const submit = useSubmit();
  const navigate = useNavigate();
  const { register, handleSubmit, control } = useForm<IngredientCreateForm>();

  const submitData = (data: IngredientCreateForm) => {
    const payload: CreateIngredientDTO = {
      ...data,
      description: JSON.stringify(data.description),
    };

    submit({ ...payload }, { method: 'post', action: '/ingredient/create' });
  };

  useEffect(() => {
    if (data?.ingredient) {
      toast(`Ingredient "${data?.ingredient.name}" was successfully created`, {
        richColors: true,
        action: {
          label: 'Open',
          onClick: () => navigate(`/ingredient/${data.ingredient.id}`),
        },
      });

      navigate('/ingredient');
    }
  }, [data, navigate]);

  return (
    <Centered>
      <Form
        onSubmit={handleSubmit(submitData)}
        className="flex flex-col p-2"
      >
        <div className="flex flex-col" aria-labelledby="name">
          <Label htmlFor="name">Ingredient name</Label>
          <div
            className={clsx([
              'flex flex-row justify-stretch items-end focus-within:bg-background-900', 'pb-2 pr-2',
              editBorder,
            ])}
          >
            <input
              id="name"
              className="text-4xl font-heading bg-transparent flex-grow outline-none"
              placeholder="Name"
              autoComplete="off"
              aria-autocomplete="none"
              required
              {...register('name')}
            />
            <span className="w-6 h-6 ml-2 flex-grow-0" aria-hidden="true">
              <PenLineIcon />
            </span>
          </div>
        </div>
        <fieldset className="flex flex-col mt-4 max-w-xs">
          <Label as="legend" className="mb-2">
            Violates a diet?
            {' '}
            <span className="text-xs font-normal text-text-300">(check means you can&apos;t eat it on that diet)</span>
          </Label>
          {diets().map(diet => (
            <CheckboxRow key={diet.id} id={diet.id} value={diet.id} className="mb-2 last:mb-0" {...register('dietViolations')}>
              <span className="mr-2" aria-disabled="true">{diet.icon}</span>
              {diet.name}
            </CheckboxRow>
          ))}
        </fieldset>
        <div className="flex flex-col mt-4" aria-labelledby="description">
          <Label htmlFor="description">Description</Label>
          <Controller
            name="description"
            control={control}
            rules={{
              required: true,
              validate: value => !!renderToPlaintext(value),
            }}
            render={({ field }) => (
              <Editor
                className={clsx('mt-2 prose p-2 outline-none focus-within:bg-background-900', editBorder)}
                name={field.name}
                value={field.value}
                onChange={field.onChange}
              />
            )}
          />
        </div>
        {/* TODO: add error messages */}
        <button
          type="submit"
          className={clsx(
            'bg-background-400 hover:bg-background-500 active:bg-background-400',
            'font-bold text-text-900 font-body uppercase',
            'w-fit ml-auto mt-3 p-2',
            'rounded-lg border-2 border-primary-400',
            'transition-colors',
          )}
        >
          Submit
        </button>
      </Form>
    </Centered>
  );
}

/*
export const handle: Handle = {
  noBar: true,
};
*/
