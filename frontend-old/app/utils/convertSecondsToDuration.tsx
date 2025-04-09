import type { Duration } from 'date-fns';

const HOURS_IN_DAY = 24;
const MINUTES_IN_HOUR = 60;
const SECONDS_IN_MINUTE = 60;

const SECONDS_IN_HOUR = MINUTES_IN_HOUR * SECONDS_IN_MINUTE;
const SECONDS_IN_DAY = SECONDS_IN_HOUR * HOURS_IN_DAY;

export const convertSecondsToDuration = (secs: number): Duration => {
  let s = secs;

  const days = Math.floor(s / SECONDS_IN_DAY);
  s -= days * SECONDS_IN_DAY;

  const hours = Math.floor(s / SECONDS_IN_HOUR);
  s -= hours * SECONDS_IN_HOUR;

  const minutes = Math.floor(s / SECONDS_IN_MINUTE);

  const seconds = s % SECONDS_IN_MINUTE;

  return {
    seconds,
    minutes,
    hours,
    days,
  };
};
