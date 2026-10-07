> ## Documentation Index
> Fetch the complete documentation index at: https://docs.parallel.ai/llms.txt
> Use this file to discover all available pages before exploring further.

# Statefulness

> Multi-turn conversations with the Responses API using `previous_response_id`

<Note>
  If Zero Data Retention (ZDR) is enabled for your Parallel organization, statefulness is
  disabled. ZDR accounts do not retain any response data server-side, so
  `previous_response_id` cannot resolve a prior turn.
</Note>

The Responses API is stateful: it supports multi-turn conversations through the standard OpenAI
`previous_response_id` parameter. Pass the `id` of a prior response and the model inherits
that conversation's context, so follow-up questions can use pronouns and references
("its population", "the second one") without restating the original question.

## Usage

Create an initial response, then reference its `id` in the follow-up:

<CodeGroup>
  ```bash cURL theme={"system"}
  # First turn
  curl https://api.parallel.ai/v1/responses \
    -H "Authorization: Bearer $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "model": "parallel",
      "input": "What is the capital of Australia?",
      "reasoning": {"effort": "low"}
    }'
  # => {"id": "resp_abc123", ..., "output": [{"type": "message", "content": [{"type": "output_text", "text": "Canberra"}]}]}

  # Follow-up turn
  curl https://api.parallel.ai/v1/responses \
    -H "Authorization: Bearer $PARALLEL_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{
      "model": "parallel",
      "input": "And what is its population?",
      "reasoning": {"effort": "low"},
      "previous_response_id": "resp_abc123"
    }'
  ```

  ```python Python theme={"system"}
  import os
  from openai import OpenAI

  client = OpenAI(
      api_key=os.environ["PARALLEL_API_KEY"],
      base_url="https://api.parallel.ai/v1",
  )

  first = client.responses.create(
      model="parallel",
      input="What is the capital of Australia?",
      reasoning={"effort": "low"},
  )
  print(first.output_text)  # Canberra

  follow_up = client.responses.create(
      model="parallel",
      input="And what is its population?",
      reasoning={"effort": "low"},
      previous_response_id=first.id,
  )
  print(follow_up.output_text)  # Canberra's population is about ...
  ```

  ```typescript TypeScript theme={"system"}
  import OpenAI from "openai";

  const client = new OpenAI({
    apiKey: process.env.PARALLEL_API_KEY,
    baseURL: "https://api.parallel.ai/v1",
  });

  const first = await client.responses.create({
    model: "parallel",
    input: "What is the capital of Australia?",
    reasoning: { effort: "low" },
  });
  console.log(first.output_text); // Canberra

  const followUp = await client.responses.create({
    model: "parallel",
    input: "And what is its population?",
    reasoning: { effort: "low" },
    previous_response_id: first.id,
  });
  console.log(followUp.output_text);
  ```
</CodeGroup>

## Building a research chat loop

Chain each turn to the previous response id to build an interactive research session:

```python Python theme={"system"}
previous_id = None
while True:
    question = input("Ask: ")
    response = client.responses.create(
        model="parallel",
        input=question,
        reasoning={"effort": "medium"},
        previous_response_id=previous_id,
    )
    print(response.output_text)
    previous_id = response.id
```

## Notes

* Responses are stored server-side to support follow-ups — you don't need to pass
  `store: true` (the field is accepted but has no effect; see
  [OpenAI Responses Compatibility](/responses-api/openai-compatibility)).
* Each turn performs fresh web research; the conversation context informs what to research,
  and answers stay grounded in live sources.
* Tiers can vary across turns — for example, open with a `high`-effort research question and
  ask cheap `low`-effort follow-ups.
