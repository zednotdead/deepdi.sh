import { Form, useActionData, useLoaderData, useNavigate, useSubmit, ActionFunctionArgs, MetaFunction } from 'react-router';
import { clsx } from 'clsx';
import { IngredientDTO } from 'common/bindings/IngredientDTO';
import { IngredientUnitDTO } from 'common/bindings/IngredientUnitDTO';
import type { RecipeDTO } from 'common/bindings/RecipeDTO';
import type { SerializedEditorState } from 'lexical';
import { PenLineIcon, TrashIcon } from 'lucide-react';
import { ChangeEvent, Fragment, useEffect, useState } from 'react';
import { Controller, FieldArrayWithId, useFieldArray, UseFieldArrayProps, useForm } from 'react-hook-form';
import { toast } from 'sonner';
import typia, { assert } from 'typia';
import { CreateRecipeDTO } from 'common/bindings/CreateRecipeDTO';
import { parse as parseDuration } from 'tinyduration';

import { DurationInput } from './DurationInput';

import { Centered } from '~/components/centered';
import { Editor } from '~/components/editor';
import { EMPTY_RTE } from '~/components/editor/utils';
import { renderToPlaintext } from '~/components/editor/renderPlaintext';
import { ErrorLine } from '~/components/form/error';
import { Label } from '~/components/form/label';
import { editBorder } from '~/utils/classes';
import { DashedButton } from '~/components/button/dashed';
import { IconButton } from '~/components/button/icon';
import { Heading } from '~/components/headings';
import { Input } from '~/components/form/input';
import { makeTitle } from '~/utils/makeTitle';

interface InternalIngredientWithAmount {
  ingredient_id?: string | null;
  amount: IngredientUnitDTO;
  optional?: boolean;
  notes?: string;
}

interface RecipeCreateForm {
  name: string;
  description: SerializedEditorState;
  steps: SerializedEditorState[];
  ingredients: InternalIngredientWithAmount[];
  servings: string;
  time: Record<string, string>;
}

function makeRequiredAndNotEmpty<T>(
  text: string,
  validateFn: (value: T) => boolean,
  rules?: UseFieldArrayProps['rules'],
) {
  return {
    ...rules,
    required: text,
    validate: (value: T) => {
      if (!validateFn(value)) {
        return text;
      }
    },
  };
}

function validateRTEContent(value: SerializedEditorState | null): boolean {
  return Boolean(value && renderToPlaintext(value)?.trim());
}

const MIN_AMOUNT = 1;

const UNITS: IngredientUnitDTO['_type'][] = ['cups', 'grams', 'other', 'teaspoons', 'mililiters'] as const;

const newButtonClass = clsx(
  'w-full h-20',
);

export default function CreateRecipeRoute() {
  const data = useActionData<typeof action>();
  const { availableIngredients } = useLoaderData<typeof loader>();
  const submit = useSubmit();
  const navigate = useNavigate();
  const { register, handleSubmit, control, formState, setValue } = useForm<RecipeCreateForm>({
    reValidateMode: 'onSubmit',
    criteriaMode: 'all',
  });
  const steps = useFieldArray({
    name: 'steps',
    control,
    rules: {
      required: `Your recipe should have at least ${MIN_AMOUNT} steps!`,
      minLength: {
        value: MIN_AMOUNT,
        message: `Your recipe should have at least ${MIN_AMOUNT} steps!`,
      },
    },
  });

  const ingredients = useFieldArray({
    name: 'ingredients',
    control,
    rules: {
      required: `Your recipe should have at least ${MIN_AMOUNT} ingredients!`,
      minLength: {
        value: MIN_AMOUNT,
        message: `Your recipe should have at least ${MIN_AMOUNT} ingredients!`,
      },
    },
  });

  const submitData = (data: RecipeCreateForm) => {
    console.group('SUBMITTING');
    console.log('SUBMITTED', data);

    const time: Record<string, number> = Object.fromEntries(
      Object
        .entries(data.time)
        .map(([k, v]) => {
          const duration = parseDuration(v);
          let seconds = 0;
          seconds += (duration.hours ?? 0) * 60 * 60;
          seconds += (duration.minutes ?? 0) * 60;
          return [k, seconds];
        }),
    );

    const payload = assert<CreateRecipeDTO>({
      ingredients: data.ingredients.map(i => ({ ...i, optional: i.optional ?? false, notes: i.notes ?? null })),
      name: data.name,
      time,
      steps: data.steps.map(s => JSON.stringify(s)),
      description: JSON.stringify(data.description),
      servings: { exact: parseInt(data.servings) },
    } as CreateRecipeDTO);
    console.log('PAYLOAD', payload);
    console.groupEnd();

    submit(JSON.stringify(payload), { method: 'post', action: '/recipe/create', encType: 'application/json' });
  };

  useEffect(() => {
    if (data && 'recipe' in data && data.recipe) {
      toast(`Ingredient "${data?.recipe.name}" was successfully created`, {
        richColors: true,
        action: {
          label: 'Open',
          onClick: () => navigate(`/recipe/${data.recipe.id}`),
        },
      });

      navigate('/recipe');
    }
  }, [data, navigate]);

  const IngredientField = ({
    ingField,
    index,
  }: {
    ingField: FieldArrayWithId<RecipeCreateForm, 'ingredients', 'id'>;
    index: number;
  }) => {
    const [isOther, setIsOther] = useState(ingField.amount._type === 'other');
    return (
      <Fragment>
        {formState.errors.ingredients?.[index]?.ingredient_id && (
          <ErrorLine>{formState.errors.ingredients[index].ingredient_id.message}</ErrorLine>
        )}
        <div className={clsx(
          'grid',
          'gap-2 items-center',
          'sm:grid-row-1 sm:grid-cols-[6fr_1fr_auto_1fr_auto]',
          'grid-row-3 grid-cols-[1fr_auto_auto]',
        )}
        >
          {isOther
            ? (
                <div
                  className={
                    clsx(
                      'flex flex-row justify-between gap-4',
                      'col-span-3 sm:col-span-1',
                    )
                  }
                >
                  <Input
                    placeholder="Amount"
                    type="number"
                    step="any"
                    inputClassName="text-end"
                    {...register(`ingredients.${index}.amount.amount.amount`, { valueAsNumber: true })}
                  />
                  <Input
                    placeholder="Unit"
                    className="flex-grow"
                    type="text"
                    {...register(`ingredients.${index}.amount.amount.unit`)}
                  />
                </div>
              )
            : (
                <Input
                  placeholder="Amount"
                  autoComplete="off"
                  aria-autocomplete="none"
                  type="number"
                  step="any"
                  inputClassName="text-end"
                  className={
                    clsx(
                      'flex flex-row justify-between gap-4',
                      'col-span-2 sm:col-start-1 sm:col-end-2',
                    )
                  }
                  {...register(`ingredients.${index}.amount.amount`, { valueAsNumber: true })}
                />
              )}
          <select
            className={
              clsx(
                'min-w-16',
                { 'col-span-3 sm:col-span-1': isOther },
              )
            }
            {
              ...register(`ingredients.${index}.amount._type`,
                {
                  onChange: (e: ChangeEvent<HTMLSelectElement>) => {
                    const isOther = e.target.value === 'other';
                    setIsOther(isOther);

                    if (typeof ingField.amount.amount === 'object') {
                      setValue(`ingredients.${index}.amount.amount`, ingField.amount.amount.amount);
                    }
                    else {
                      setValue(`ingredients.${index}.amount.amount.amount`, ingField.amount.amount);
                    }
                  },
                },
              )
            }
          >
            {UNITS.map(u => (
              <option value={u} key={u}>{u}</option>
            ))}
          </select>
          <select
            defaultValue=""
            {...register(`ingredients.${index}.ingredient_id`, {
              required: 'You must pick an ingredient',
            })}
            className="flex-grow col-span-3 sm:col-span-1"
          >
            <option disabled value="">Select an ingredient</option>
            {availableIngredients.map(ing => (
              <option value={ing.id} key={ing.id}>{ing.name}</option>
            ))}
          </select>
          <IconButton
            onClick={() => ingredients.remove(index)}
            aria-label="Delete this ingredient"
            title="Delete this ingredient"
            className="col-span-3 sm:col-span-1 justify-self-end"
          >
            <TrashIcon />
          </IconButton>
        </div>
      </Fragment>
    );
  };

  return (
    <Centered>
      {data && 'errorCode' in data && (
        <div className="bg-red-100 mt-4 p-4 border-solid border-red-300 border-2 rounded">
          <Heading>An error has occured.</Heading>
          <pre>{JSON.stringify(data.body, null, 2)}</pre>
        </div>
      )}
      <Form
        onSubmit={handleSubmit(submitData)}
        className="flex flex-col p-2"
      >
        <div className="flex flex-col" aria-labelledby="name">
          <Label htmlFor="name">Recipe name</Label>
          {formState.errors.name && (
            <ErrorLine>{formState.errors.name.message}</ErrorLine>
          )}
          <Input
            id="name"
            className="text-4xl font-heading"
            placeholder="Name"
            autoComplete="off"
            aria-autocomplete="none"
            icon={<PenLineIcon />}
            {...register('name', { required: 'The name should not be empty' })}
          />
        </div>

        <div className="flex flex-col" aria-labelledby="servings">
          <Label htmlFor="servings">Servings</Label>
          {formState.errors.servings && (
            <ErrorLine>{formState.errors.servings.message}</ErrorLine>
          )}
          <Input
            id="servings"
            placeholder="Servings"
            autoComplete="off"
            aria-autocomplete="none"
            type="number"
            step="1"
            defaultValue={1}
            {...register('servings',
              {
                required: 'The amount of servings should be above 0',
                min: { value: 0, message: 'The amount of servings should be above 0' },
              },
            )}
          />
        </div>

        <div className="flex flex-col gap-2 mt-4">
          <Label className="mb-4">Time</Label>
          <div className="grid grid-cols-[1fr_2fr] grid-rows-2 gap-2">
            <Label as="span" className="text-xl mr-2 flex-none">Prep time</Label>
            <Controller
              name="time.Prep time"
              control={control}
              render={function DescriptionField({ field }) {
                return (
                  <DurationInput
                    value={field.value}
                    onChange={(v) => {
                      setValue(field.name, v);
                    }}
                    className="flex-grow"
                  />
                );
              }}
            />
            <Label as="span" className="text-xl mr-2 flex-none">Cook time</Label>
            <Controller
              name="time.Cook time"
              control={control}
              render={function DescriptionField({ field }) {
                return (
                  <DurationInput
                    value={field.value}
                    onChange={(v) => {
                      setValue(field.name, v);
                    }}
                    className="flex-grow"
                  />
                );
              }}
            />
          </div>
        </div>

        <div className="flex flex-col mt-4" aria-labelledby="description">
          <Label htmlFor="description">Description</Label>
          {formState.errors.description?.message && (
            <ErrorLine>{formState.errors.description.message}</ErrorLine>
          )}
          <Controller
            name="description"
            control={control}
            rules={makeRequiredAndNotEmpty('Description should not be empty', validateRTEContent)}
            render={function DescriptionField({ field }) {
              return (
                <>
                  <Editor
                    className={clsx('mt-2 prose p-2 outline-none focus-within:bg-background-900', editBorder)}
                    name={field.name}
                    value={field.value}
                    onChange={field.onChange}
                    ref={field.ref}
                  />
                </>
              );
            }}
          />
        </div>

        <div className="flex flex-col mt-4" aria-labelledby="ingredients">
          <Label as="p" id="ingredients">Ingredients</Label>
          {formState.errors.ingredients?.root && (
            <ErrorLine>{formState.errors.ingredients.root.message}</ErrorLine>
          )}
          {ingredients.fields.map((ingField, i) => (
            <IngredientField ingField={ingField} index={i} key={ingField.id} />
          ))}

          <DashedButton
            onClick={() => ingredients.append({ amount: { _type: 'grams', amount: 100 } })}
            className={
              clsx(
                newButtonClass,
                'mt-4',
                'before:inline before:content-["+"]',
                'before:pr-2 before:text-xl',
                'flex flex-row items-center justify-center',
              )
            }
          >
            Add a new ingredient
          </DashedButton>
        </div>

        <div className="flex flex-col mt-4" aria-labelledby="steps">
          <Label as="p" id="steps">Steps</Label>
          {formState.errors.steps?.root && (
            <ErrorLine>{formState.errors.steps?.root.message}</ErrorLine>
          )}
          {steps.fields.map((stepField, i) => (
            <Controller
              name={`steps.${i}`}
              control={control}
              key={stepField.id}
              rules={makeRequiredAndNotEmpty('This step must not be empty', validateRTEContent)}
              render={function StepField({ field, fieldState }) {
                return (
                  <Fragment>
                    <div className="flex flex-row justify-between items-center">
                      <Label className="text-xl" htmlFor={`steps.${i}`}>
                        Step
                        {' '}
                        {i + 1}
                      </Label>
                      <IconButton
                        onClick={() => steps.remove(i)}
                        aria-label="Delete this step"
                        title="Delete this step"
                        className="hover:bg-background-900 active:bg-background-800 p-4 rounded-full flex-none"
                      >
                        <TrashIcon />
                      </IconButton>
                    </div>
                    {fieldState.error && (
                      <ErrorLine>{fieldState.error.message}</ErrorLine>
                    )}
                    <Editor
                      id={`steps.${i}`}
                      className={clsx('mt-2 prose p-2 outline-none focus-within:bg-background-900', editBorder)}
                      name={field.name}
                      value={field.value}
                      onChange={field.onChange}
                      ref={field.ref}
                    />
                  </Fragment>
                );
              }}
            />
          ))}

          <DashedButton
            onClick={() => steps.append(EMPTY_RTE)}
            className={
              clsx(
                newButtonClass,
                'mt-4',
                'before:inline before:content-["+"]',
                'before:pr-2 before:text-xl',
                'flex flex-row items-center justify-center',
              )
            }
          >
            Add a new step
          </DashedButton>
        </div>

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

export const meta: MetaFunction = () => {
  return [
    { title: makeTitle('Create a recipe') },
  ];
};

export async function action({ request }: ActionFunctionArgs) {
  const data = assert<CreateRecipeDTO>(await request.json());

  const res = await fetch('http://localhost:8111/recipe/create',
    { method: 'POST', body: JSON.stringify(data), headers: { 'Content-Type': 'application/json' } },
  );
  const status = res.status;

  if (status >= 200 && status < 300) {
    const recipe = typia.assert<RecipeDTO>(await res.json());
    return {
      status,
      recipe,
    };
  }

  return {
    errorCode: status,
    body: await res.json() as unknown,
  };
}

export async function loader() {
  const res = await fetch('http://localhost:8111/ingredient');
  const ingredients: IngredientDTO[] = typia.assert<IngredientDTO[]>(await res.json());

  return {
    availableIngredients: ingredients,
  };
}
