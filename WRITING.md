# Writing rules

VoxelWright help and UI text should be easy to scan and easy to act on.

## Use plain words

- Write for a reader around fifth-grade level when the subject allows it.
- Use short sentences. Most sentences should cover one idea.
- Start steps with an action: **Click**, **Select**, **Open**, or **Check**.
- Use the exact name shown in Studio.
- Define a needed technical word where it first appears.
- Give an example when a number or choice may be hard to picture.

## Say what happens next

- Tell the user what a button does.
- After a major step, say what the user should see.
- For an error, say what happened and the next safe action.
- Explain why an action is disabled.
- Put deep format details in the report or glossary, not in the main path.

## Remove filler

Delete:

- Generic welcome paragraphs
- Repeated summaries
- Hype such as “powerful,” “seamless,” or “revolutionary”
- Canned phrases such as “whether you are” and “let's dive in”
- Long lists of adjectives
- Claims that are not backed by a test
- Headings that contain only one obvious sentence

## Prefer result names in the UI

| Avoid               | Prefer               |
| ------------------- | -------------------- |
| Greedy Cuboids      | Fewest Parts         |
| Surface Shell       | Outside Only         |
| Solid Fill          | Fill Inside          |
| Geometry fallback   | Made as a full block |
| Appearance fallback | Used a backup color  |
| Fidelity report     | Import report        |

Keep the technical name in a tooltip, report, or glossary when it helps with bug reports.

## Check every page

1. Can a new user tell what to click first?
2. Does each step ask for one action?
3. Are hard words explained?
4. Does every warning say what to do?
5. Can any sentence be shorter without losing meaning?
6. Does the page sound like a person who used the tool?
