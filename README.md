# cronflake

Cronflake is a crate that provide utilities struct to work with cron experessioins

| Field        | Required | Allowed values  | Allowed special characters | Remarks                            |
| ------------ | -------- | --------------- | -------------------------- | ---------------------------------- |
| Minutes      | Yes      | 0–59            | \* , -                     |                                    |
| Hours        | Yes      | 0–23            | \* , -                     |                                    |
| Day of month | Yes      | 1–31            | \* , - ? L W               | ? L W only in some implementations |
| Month        | Yes      | 1–12 or JAN–DEC | \* , -                     |                                    |
| Day of week  | Yes      | 0–6 or SUN–SAT  | \* , - ? L #               | ? L # only in some implementations |
