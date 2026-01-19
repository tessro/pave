**tess:**

what's the meta for software documentation in the ai agent era? a framework that's easy for agents to author within, is there such a thing?

---
**ChatGPT 5.2 Thinking:**

Yeah — there is a “meta” emerging, and it’s basically:

Docs aren’t for humans anymore; they’re for a human+agent pair doing work.
Which means: small, structured, testable, and easy to update.

Here’s a framework that works really well in practice and is very agent-authorable.

⸻

The new meta: “Docs as interfaces”

Treat documentation like you treat APIs:
	•	Precise contracts
	•	Small surfaces
	•	Versioned
	•	Validated
	•	Optimized for retrieval + execution

Agents don’t need prose. They need ground truth, constraints, examples, and how to verify.

⸻

A doc framework that agents can author: PAVED

This is my go-to structure for “agent-native” docs:

1) P — Purpose

What is this thing? What problem does it solve?
	•	1–3 sentences
	•	include non-goals

Example

“This service runs scheduled background jobs. It does not do real-time event processing.”

⸻

2) A — API / Interface

How do you use it? What are the entry points?
	•	CLI commands
	•	HTTP endpoints
	•	library calls
	•	config keys
	•	file formats

Agents thrive on tables and schemas here.

⸻

3) V — Verification

How do you know it’s working?

This is the #1 missing thing in most docs, and it’s everything for agents.

Include:
	•	“golden” commands
	•	expected output snippets
	•	healthcheck endpoints
	•	invariants
	•	how to run tests
	•	“common failure → diagnosis”

⸻

4) E — Examples

Concrete copy/paste examples, ideally minimal:
	•	1 happy path
	•	1 realistic path
	•	1 failure path

Agents use examples as “shape matching” to produce correct output.

⸻

5) D — Decisions

Short rationale + constraints:
	•	why this design exists
	•	what must not change
	•	tradeoffs

This prevents agents from “refactoring your intent away.”

⸻

The authoring meta: “Leaf docs” + “Index docs”

Agents do best when docs are:

Leaf docs (small + atomic)
	•	one concept
	•	one workflow
	•	one component
	•	one decision

Index docs (routing + map)
	•	“Start here”
	•	links to leaf docs
	•	a 30-second mental model

This mirrors how agents retrieve: they want a map, then a target chunk.

⸻

The formatting meta: “Docs that compile”

If you want agents to reliably author and maintain docs, make the format lintable:

Recommended structure per doc file
	•	frontmatter / metadata (optional)
	•	sections with stable headings
	•	bulleted constraints
	•	code blocks that run

Add doc quality gates
	•	“Every doc must include Verification”
	•	“Every public module must have one Example”
	•	“No doc > 300 lines; split instead”
	•	“Every example has expected output”

Agents can follow rules like these extremely well.

⸻

The best agent-friendly doc types (in priority order)

1) Runbooks

“Do X safely.”

Great because they’re procedural and verifiable.

2) Golden paths

“How we do this here.”

Great because they constrain choices.

3) Contracts / Schemas

OpenAPI / protobuf / JSON schema / config schema.

Agents are shockingly good when you give them structure.

4) Decision records (ADRs)

Short and high-signal.

⸻

What to avoid (old meta)

These fail in agent workflows:
	•	big narrative docs (“Architecture Overview” that’s 20 pages)
	•	“tribal knowledge” prose
	•	docs without verification steps
	•	docs that describe outcomes but not constraints
	•	docs that require reading 10 other docs first

⸻

The killer trick: “Make docs executable”

If you do only one thing, do this:

Put commands in docs that actually run
	•	make test
	•	make lint
	•	make smoke
	•	./scripts/verify_<thing>.sh
	•	curl localhost:…

Agents can:
	•	propose changes
	•	then validate them
	•	then update docs

Without verification hooks, they hallucinate correctness.

⸻

A concrete template you can standardize on

component.md
	•	Purpose
	•	Interface
	•	Configuration
	•	Verification
	•	Examples
	•	Gotchas
	•	Decisions / invariants

runbook_<task>.md
	•	When to use this
	•	Preconditions
	•	Steps
	•	Rollback
	•	Verification
	•	Escalation

adr_<title>.md
	•	Context
	•	Decision
	•	Consequences
	•	Alternatives considered

This is “agent-easy” because it’s repetitive and checklisted.

⸻

If you want one sentence to pin on the wall

Write docs like you’re training a careful junior engineer who can run commands but can’t read minds.

That’s basically the AI agent era.

⸻

If you want, I can turn this into a drop-in repo standard: folder layout + templates + a doc-lint checklist that agents can follow (and that humans won’t hate).
