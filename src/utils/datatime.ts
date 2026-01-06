import dayjs from 'dayjs';

export type Format = 'YYYY-MM-DD';

export function datetime(source: string, format: Format = 'YYYY-MM-DD') {
  let value = dayjs(source);

  return value.format(format);
}
