# lookup

A Unix-style CLI tool to fetch dictionary definitions and Wikipedia summaries.

![demo](demo.gif)

## Installation

```bash
cargo build --release
cp target/release/lookup /usr/local/bin/
```

## Usage

```bash
lookup <term>
```

### Examples

```bash
$ lookup hello
Definitions:
1. (noun) "Hello!" or an equivalent greeting.
2. (verb) To greet with "hello".
3. (interjection) A greeting (salutation) said when meeting someone or acknowledging someone's arrival or presence.

Wikipedia:
Hello (often "hi" in modern English for frequent use) is a salutation or greeting in the English language...

$ lookup "spruce goose"
Definitions:
N/A

Wikipedia:
The Hughes H-4 Hercules (commonly known as the Spruce Goose) is a prototype strategic airlift flying boat...
```

## Output

- **Definitions** - From [Free Dictionary API](https://dictionaryapi.dev/), showing part of speech and meaning
- **Wikipedia** - Introductory summary from Wikipedia

Shows "N/A" when no results are found (common for proper nouns in the dictionary).

## Unix Philosophy

- Plain text output (no colors)
- Pipeable to `less`, `grep`, etc.
- Exit codes: 0 success, 1 error
- Errors to stderr

```bash
lookup "rust programming" | less
lookup hello | grep noun
```

## APIs Used

| Source | Endpoint | Auth |
|--------|----------|------|
| Dictionary | `api.dictionaryapi.dev` | None |
| Wikipedia | `en.wikipedia.org/w/api.php` | None |

## License

MIT
