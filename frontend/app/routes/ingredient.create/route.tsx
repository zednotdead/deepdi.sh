import type { ActionFunctionArgs } from '@remix-run/node';
import { json } from '@remix-run/node';
import { Form, useActionData, useSubmit } from '@remix-run/react';
import { clsx } from 'clsx';
import type { FC, PropsWithChildren } from 'react';
import { Centered } from '~/components/centered';
import { Title } from '~/components/headings';
import Edit from '~/icons/edit.svg?react';
import { Controller, useForm } from 'react-hook-form';
import type { CreateIngredientDTO } from 'common/bindings/CreateIngredientDTO';
import { assert } from 'typia';
import { Editor } from '~/components/editor';
import type { SerializedEditorState } from 'lexical';
import type { IngredientDTO } from 'common/bindings/IngredientDTO';

const Label: FC<PropsWithChildren<{ for: string; className?: string }>> = ({ children, for: htmlFor, className }) => (
  <label htmlFor={htmlFor} className={clsx('font-heading text-xl font-semibold', className)}>
    {children}
  </label>
);

export async function action({ request }: ActionFunctionArgs) {
  const data = await request.formData();

  const parsed = {
    ...Object.fromEntries(data.entries()),
  };

  const ingredient = assert<CreateIngredientDTO>(parsed);

  const createdIngredient: IngredientDTO = await fetch(
    'http://localhost:8111/ingredient/create',
    { method: 'POST', body: JSON.stringify(ingredient), headers: { 'Content-Type': 'application/json' } },
  ).then(res => res.json());

  return json({
    ingredient: createdIngredient,
  });
}

interface IngredientCreateForm {
  name: string;
  description: SerializedEditorState;
  dietFriendly?: string[];
}

export default function CreateIngredientRoute() {
  const data = useActionData<typeof action>();
  const submit = useSubmit();
  const { register, handleSubmit, control } = useForm<IngredientCreateForm>();

  const submitData = (data: IngredientCreateForm) => {
    const payload: CreateIngredientDTO = {
      ...data,
      description: JSON.stringify(data.description),
    };

    submit({ ...payload }, { method: 'post', action: '/ingredient/create' });
  };

  return (
    <Centered>
      <Title className="text-2xl">Creating a new ingredient</Title>
      {data && <pre>{JSON.stringify(data, null, 2)}</pre>}
      <Form
        onSubmit={handleSubmit(submitData)}
        className="flex flex-col"
      >
        <Label for="name">Ingredient name</Label>
        <div
          className="flex flex-row justify-stretch items-end \
          pb-2 pr-2 \
          transition-colors \
          border-b-2 border-background-800 focus-within:border-primary-400"
        >
          <input
            id="name"
            className="text-4xl font-heading bg-background-950 flex-grow"
            placeholder="Name"
            autoComplete="off"
            aria-autocomplete="none"
            {...register('name')}
          />
          <span className="w-6 h-6 ml-2 flex-grow-0">
            <Edit />
          </span>
        </div>
        <Label for="description" className="mt-4">Description</Label>
        <Controller
          name="description"
          control={control}
          render={({ field }) => (
            <Editor
              className="mt-4 prose p-2 \
                  border-b-2 border-background-800 focus-within:border-primary-400"
              name={field.name}
              value={field.value}
              onChange={field.onChange}
            />
          )}
        />

        <button type="submit">Submit</button>
      </Form>
    </Centered>
  );
}
