# CLI Tool for management Card on Trello

## Roadmap

| feature                   | state |
| ------------------------- | ----- |
| add card                  | ✅    |
| add card from json        | ✅    |
| get all cards             | ✅    |
| get card by id            | ✅    |
| get all cards by step     | ✅    |
| get all cards by label    | ✅    |
| remove all card           | ✅    |
| remove card by id         | 🚨    |
| remove all cards by step  | 🚨    |
| remove all cards by label | 🚨    |
| move card to next step    | 🚨    |
| move card to back step    | 🚨    |
| edit card by id           | 🚨    |
| remove card by id         | 🚨    |

## Set up your `.env`

```bash
TOKEN=
API_KEY=
SECRET_KEY=
BASE_URL=
BOARD_ID=
```

## Make simple CRUD

### CREATE

- add a card to the board

```bash
tc add card \
    --name my_card \
    --label my_label \
    --step my_step
```

### READ

- get all cards from the board

```bash
tc get card --all
```

- get all cards by step

```bash
tc get card --step my_steps
```

- get all cards by label

```bash
tc get card --label my_label
```

- get a card by id

```bash
tc get card --id $CARD_ID
```

### UPDATE

- move card to next step

```bash
tc move card next --id $CARD_ID
```

- move card to back step

```bash
tc move card back --id $CARD_ID
```

- update card by id

```bash
tc edit card --id $CARD_ID \
    --name new_name \
    --labe new_label \
    --step new_step
```

### DELETE

- remove a card by id

```bash
tc remove card --id $CARD_ID
```

- remove a card by step

```bash
tc remove card --step my_step
```

- remove a card by label

```bash
tc remove card --label my_label
```

> [!WARNING]
> 🚨 Be careful with this command

- remove all card

```bash
tc remove card --all
```

## Make a massive CRUD

- your json

```json
[
  {
    "name": "Name your card",
    "label": "YourLabel",
    "step": "YourStep"
  },
  {
    "name": "Name of other card",
    "label": "YourLabel",
    "step": "YourStep"
  }
]
```

- add multiple cards from json

```bash
tc add card --from $JSON_PATH
```
