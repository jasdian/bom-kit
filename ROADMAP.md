# Project Roadmap

## Vision

Provide a lightweight BOM (Bill of Materials) toolkit that simplifies BOM creation, validation, and export.

## Goals (3–6 months)

- Core: Parse input component lists (CSV, JSON).
- Validation: Detect missing fields and duplicate components.
- Export: Generate standardized BOM formats (CSV, SPDX).
- CI: Add basic tests and continuous integration.

## Milestones

- CLI to load and validate CSV
- Basic unit tests
- Add JSON and SPDX export
- Improve validation rules
- Better error messages
- More test coverage, bug fixes

## Releases

- 0.1.1
  - core Unit (as count) BoM
  - component
  - part
  - quantity
  - BoM and its explosion
  - substitute
- 0.1.2
  - CSV parse + validation
  - where-used and cost analysis (part cost)
  - usage example
- 0.2.0
  - refactor codebase, to keep minimal `pub` access
  - stack usage (VecDeque), over recursive calls
  - Format exports (JSON, SPDX)
  - different quantity types
  - inventory (part/assembly recipe coverage)
  - tests coverage
- 0.3.0
  - assemble cost, calculation cost modes
  - Stability, documentation, CI
- 0.4.0
  - factory: assembly items production, production time estimations
- 0.5.0
  - pre-defined types: staff, electricity, storage/warehouse
  - staff modes: SUM cost over time / each worker
  - cost types: static/time-based

## Success Metrics

- Parse and validate >95% of sample BOMs without errors
- Automated tests covering critical paths (target 80%+)
- Users can generate SPDX from sample input in <30s

## Risks & Mitigations

- Ambiguous input formats → Provide clear input schema and examples
- SPDX complexity → Start with a minimal subset, expand iteratively

## Future Steps

- example CSV fixtures
- CLI parser
- different dimensions (m squared, etc.)
- more unit/quantity types
