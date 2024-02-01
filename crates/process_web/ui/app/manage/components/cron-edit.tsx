import React, { useEffect, useState } from "react";
import { Cron, DefaultLocale } from "react-js-cron";
import 'react-js-cron/dist/styles.css'

export default function CronEdit(props: {value?: string, onChange?: (value: string) => void}) {
  // const [value, setValue] = useState(props.value ? props.value : "");

  return <Cron locale={LOCALE} value={props.value ? props.value : ""} setValue={(value: string) => {
    props.onChange?.(value)
  }} />
}


// TODO 翻译
export const LOCALE: DefaultLocale = {
  everyText: 'every',
  emptyMonths: 'every month',
  emptyMonthDays: 'every day of the month',
  emptyMonthDaysShort: 'day of the month',
  emptyWeekDays: 'every day of the week',
  emptyWeekDaysShort: 'day of the week',
  emptyHours: 'every hour',
  emptyMinutes: 'every minute',
  emptyMinutesForHourPeriod: 'every',
  yearOption: 'year',
  monthOption: 'month',
  weekOption: 'week',
  dayOption: 'day',
  hourOption: 'hour',
  minuteOption: 'minute',
  rebootOption: 'reboot',
  prefixPeriod: 'Every',
  prefixMonths: 'in',
  prefixMonthDays: 'on',
  prefixWeekDays: 'on',
  prefixWeekDaysForMonthAndYearPeriod: 'and',
  prefixHours: 'at',
  prefixMinutes: ':',
  prefixMinutesForHourPeriod: 'at',
  suffixMinutesForHourPeriod: 'minute(s)',
  errorInvalidCron: 'Invalid cron expression',
  clearButtonText: 'Clear',
  weekDays: [
    // Order is important, the index will be used as value
    'Sunday', // Sunday must always be first, it's "0"
    'Monday',
    'Tuesday',
    'Wednesday',
    'Thursday',
    'Friday',
    'Saturday',
  ],
  months: [
    // Order is important, the index will be used as value
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
  ],
  // Order is important, the index will be used as value
  altWeekDays: [
    'SUN', // Sunday must always be first, it's "0"
    'MON',
    'TUE',
    'WED',
    'THU',
    'FRI',
    'SAT',
  ],
  // Order is important, the index will be used as value
  altMonths: [
    'JAN',
    'FEB',
    'MAR',
    'APR',
    'MAY',
    'JUN',
    'JUL',
    'AUG',
    'SEP',
    'OCT',
    'NOV',
    'DEC',
  ],
}