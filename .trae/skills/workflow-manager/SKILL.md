---
name: "workflow-manager"
description: "Manages software development workflows (planning, coding, testing, review). Invoke when starting a new task, feature, or project phase to ensure structured execution."
---

# Workflow Manager

This skill helps manage the software development lifecycle by enforcing structured workflows, documentation, and context management. It acts as a project manager and technical lead, guiding the development process through defined stages.

## 1. Workflow Selection Strategy

When invoked, analyze the user's request to determine the most appropriate workflow type.

### A. Standard Software Engineering (Waterfall/V-Model)

- **Trigger**: "Build a complex system", "Strict requirements", "Critical infrastructure".
- **Steps**:
  1. **User Research**: Understand the problem space and user needs.
  2. **PRD (Product Requirement Document)**: Define features, scope, and acceptance criteria.
  3. **PRD Review**: Validate requirements with stakeholders.
  4. **Technical Design**: Architecture, database schema, API contracts.
  5. **Technical Review**: Validate technical feasibility and scalability.
  6. **Development**: Coding implementation.
  7. **Integration (联调)**: Combine modules and test interactions.
  8. **Code Submission**: Pull Request/Merge Request preparation.
  9. **Testing**: QA, Unit, Integration, E2E tests.
  10. **Experimental Launch**: Canary release, A/B testing.
  11. **Full Release**: Production deployment.

### B. Modern Agile (Scrum/Kanban)

- **Trigger**: "Iterate on a feature", "Fix a bug", "Refactor code", "MVP development".
- **Steps**:
  1. **Backlog Refinement**: Clarify task details and priority.
  2. **Sprint Planning**: Estimate effort and commit to scope.
  3. **Implementation (TDD)**: Write failing tests, then implementation code.
  4. **Code Review**: Peer review for quality and standards.
  5. **CI/CD**: Automated build, test, and deployment.
  6. **Retrospective**: Review what went well and what didn't.

### C. AI-Native Development (The "AI Era" Flow)

- **Trigger**: "Prototype with AI", "Automate this task", "Generate code", "Explore ideas".
- **Steps**:
  1. **Intent Analysis**: AI understands the high-level goal and context.
  2. **Context Assembly**: AI gathers relevant codebase context (RAG).
  3. **Prompt Engineering**: Crafting precise instructions for generation.
  4. **Solution Generation**: AI proposes multiple implementation paths.
  5. **User Selection/Refinement**: User selects the best path or refines the prompt.
  6. **Autonomous Implementation**: AI writes code, tests, and docs.
  7. **Self-Correction Loop**: AI fixes its own errors (lint/test failures).
  8. **Verification**: User verifies the outcome against intent.
  9. **Knowledge Distillation**: Update core memories/docs with learnings.

## 2. Directory Structure & Initialization

For every new workflow execution:

1. **Generate a Unique ID**: `{date}_{daily_seq}_{type}_{name}`
    - `date`: YYYYMMDD (e.g., 20231027)
    - `daily_seq`: 01, 02, etc. (increment if directory exists)
    - `type`: feat, fix, chore, docs, refactor, test, perf
    - `name`: short-description (kebab-case)
2. **Create Directory**: `{project_root}/.trae/workflows/{unique_id}/`
3. **Initialize Files**:
    - `context.md`: The central state file.
    - `00_plan.md`: Initial high-level plan.

## 3. Execution Rules (The "Sandwich" Method)

For **EACH STEP** in the chosen workflow:

1. **Pre-Survey (Planning)**:
    - **Before** starting the step execution.
    - Create/Update a step-specific plan (e.g., `01_research_plan.md`).
    - Define: Inputs, Outputs, Success Criteria, Dependencies, Risks.
2. **Execution**:
    - Perform the task (coding, writing docs, running tests).
    - Update `context.md` with progress and status changes.
3. **Post-Summary (Review)**:
    - **After** completing the step execution.
    - Create/Update a summary document (e.g., `01_research_summary.md`).
    - Record: Results, Decisions Made, Issues Encountered, Next Steps.

## 4. Context Management

Maintain `context.md` in the workflow directory. It must contain:

- **Meta**: Workflow ID, Start Time, Status (Pending, In Progress, Completed).
- **Parameters**: Project Root, Target Directories, Relevant Files.
- **Current Step**: Which step is active.
- **History**: Log of completed steps and links to their summary docs.
- **Variables**: Key decisions or data points carried over between steps.

## 5. Script Support (Conceptual)

When executing, ensure the following information is available to the context:

- `WORKFLOW_DIR`: The absolute path to the current workflow archive.
- `PROJECT_ROOT`: The root of the codebase.
- `PROJECT_INFO`: Summary of the project (from README or core memory).

## How to Use This Skill

1. **Trigger**: User says "Start a new feature", "Fix this bug", "Plan a refactor", or "Run a workflow".
2. **Action**:
    - Ask user for details if needed (Type, Name).
    - Create the directory structure.
    - Select the workflow based on complexity and type.
    - Begin the first step (Pre-Survey).
