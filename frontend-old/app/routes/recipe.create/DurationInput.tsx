import { formatISODuration } from 'date-fns';
import { parse } from 'tinyduration';
import { ChangeEvent, useId, useState } from 'react';
import { clsx } from 'clsx';

import { Input } from '~/components/form/input';

interface DurationInputProps {
  className?: string;
  name?: string;
  value?: string;
  onChange?: (value: string) => void;
}

export function DurationInput(props: DurationInputProps) {
  const id = useId();
  const [hours, setHours] = useState((props.value && parse(props.value).hours) || 0);
  const [minutes, setMinutes] = useState((props.value && parse(props.value).minutes) || 0);

  const updateHours = (ev: ChangeEvent<HTMLInputElement>) => {
    const h = parseInt(ev.target.value || '0');
    setHours(h);

    const duration = formatISODuration({ hours: h, minutes });
    props.onChange?.(duration);
  };

  const updateMinutes = (ev: ChangeEvent<HTMLInputElement>) => {
    const m = parseInt(ev.target.value || '0');
    setMinutes(m > 60 ? 59 : m);

    const duration = formatISODuration({ hours, minutes: m });
    props.onChange?.(duration);
  };

  return (
    <div className={clsx('grid grid-cols-2 gap-4', props.className)}>
      <div className="flex flex-row gap-2 items-center">
        <Input
          type="number"
          value={hours}
          min={0}
          onChange={updateHours}
          className="flex-grow"
          id={`${id}-hours`}
        />
        <label htmlFor={`${id}-hours`}>
          hours
        </label>
      </div>
      <div className="flex flex-row gap-2 items-center">
        <Input
          type="number"
          value={minutes}
          min={0}
          max={59}
          onChange={updateMinutes}
          className="flex-grow"
          id={`${id}-minutes`}
        />
        <label htmlFor={`${id}-minutes`}>
          minutes
        </label>
      </div>
    </div>
  );
}
