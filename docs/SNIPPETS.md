# Snippets

## General zellij Commands

Setup the agent setup _(layout installed)_:

```cmd
zellij -n coding-agents -l zellij-agents-layout
```

Setup the agent setup _(layout not installed)_:

```cmd
zellij -n coding-agents -l zellij\zellij-agents-layout.kdl
```

Get the list of sessions to retrieve your session id:

```cmd
zellij list-sessions
```

Get the list of panes from a session to retrieve the pane id:

```cmd
zellij --session <session-id> action list-panes
```

## __Planner Agent:__ Plan Implementation of a Task

Prompt for __Planner Agent:__

```
Create a new session for the task <task-slug>. Afterwards create an implementation plan for the task and persist it in the created session log file.
```

Zellij command:

```cmd
zellij --session coding-agents action paste --pane-id <planner-pane-id> "Create a new session for the task <task-slug>. Afterwards create an implementation plan for the task and persist it in the created session log file."
zellij --session coding-agents action send-keys --pane-id <planner-pane-id> "Enter"
```

## __Implementer Agent:__ Implement the Implementation Plan of a Task

Prompt for __Implementer Agent:__

```
Implement the plan provided in the session log for the task <task-slug>.
```

Zellij command:

```cmd
zellij --session coding-agents action paste --pane-id <implementer-pane-id> "Implement the plan provided in the session log for the task <task-slug>."
zellij --session coding-agents action send-keys --pane-id <implementer-pane-id> "Enter"
```

## __Reviewer Agent:__ Review the Implementation of a Task

Prompt for __Reviewer Agent:__

```
The implementer has implemented the solution as described by the implementation plan in the session log for the task <task-slug>. Review the changes.
```

Zellij command:

```cmd
zellij --session coding-agents action paste --pane-id <reviewer-pane-id> "The implementer has implemented the solution as described by the implementation plan in the session log for the task <task-slug>. Review the changes."
zellij --session coding-agents action send-keys --pane-id <reviewer-pane-id> "Enter"
```
