# NONSENSE 

Write SQL queries with CSS 🤪

NONSENSE stands for "NONSENSE Organizes Nonsensical SQL Expressing Nonsensical Stylesheets Extensively"

If u keep asking ["SQL vs. CSS What’s the Difference? Which Is Better?"](https://web.archive.org/web/20230606193317/https://history-computer.com/sql-vs-css-whats-the-difference-which-is-better/), now don't have to anymore!!!!! 

## What??
It parses a CSS like syntax and transforms into CSS queries. 

Right now it can only generate basic select statements. 

```css
.users {
  name,
  id
}
```

Becomes

```sql
SELECT name, id FROM users;
```

## Usage

It reads from a file and writes to stdout

```bash
nonsense input.css > output.sql
```
