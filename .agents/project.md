# Project

## Concepts

### `educator` package

- Must have dependencies:
  - `openai-utils`

### GeneratePresentationCommand

- Must be callable as `talk generate`
- Must have fields:
  - `topic: String`
- Must have methods:
  - `run`
    - Must call `get_response_from_openai` (from `openai-utils`)
  - `prompt(topic: &str) -> String`
