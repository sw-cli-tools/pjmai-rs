# Groups Design Proposal (Simplified)

## Core Concept

**Groups are inferred, not created.** A group exists automatically when you have projects under a common parent directory. The "group" is simply a way to view and scope your projects.

```
~/github/
├── softwarewrighter/    ← group "softwarewrighter" (inferred)
│   ├── project1/
│   └── project2/
├── sw-cli-tools/        ← group "sw-cli-tools" (inferred)
│   └── pjmai-rs/
└── personal/         ← group "personal" (inferred)
    └── dotfiles/
```

**Group aliases** are the only manual part - you can give friendly names to inferred groups:
- `softwarewrighter` → alias `work`
- `sw-cli-tools` → alias `cli`
- `personal` → alias `home`

---

## How Groups Are Inferred

When you add a project, the group is derived from its path:

```bash
adpj pjmai -f ~/github/sw-cli-tools/pjmai-rs
# → Group inferred: "sw-cli-tools"
# → Group path: ~/github/sw-cli-tools

adpj webapp -f ~/code/mighty/webapp
# → Group inferred: "mighty"
# → Group path: ~/code/mighty
```

**Inference rule:** The group is the immediate parent directory name of the project.

---

## Commands

| Alias | Command | Description |
|-------|---------|-------------|
| `lsgp` | `pjmai group list` | List all groups with project counts |
| `shgp` | `pjmai group show` | Show current group info |
| `prgp` | `pjmai group prompt` | Print current group name (for shell prompt) |
| - | `pjmai group alias` | Add/remove/list group aliases |

**No `adgp`/`rmgp`** - groups exist as long as they have projects.
**No `chgp`** - current group is implied by current project.

---

## Command Details

### `pjmai group list` (alias: `lsgp`)

List all inferred groups:

```bash
$ lsgp
GROUP             PROJECTS  PATH
> sw-cli-tools    3         ~/github/sw-cli-tools
  softwarewrighter 5        ~/github/softwarewrighter
  personal     2         ~/github/personal
  mighty          4         ~/code/mighty

# With aliases shown
$ lsgp --aliases
GROUP              ALIAS     PROJECTS  PATH
> sw-cli-tools     cli       3         ~/github/sw-cli-tools
  softwarewrighter work      5         ~/github/softwarewrighter
  personal      home  2         ~/github/personal
  mighty           proto     4         ~/code/mighty
```

The `>` marks the current group (derived from current project).

### `pjmai group show` (alias: `shgp`)

Show details about current group (or specified group):

```bash
$ shgp
Group: sw-cli-tools
Alias: cli
Path: ~/github/sw-cli-tools
Projects (3):
  > pjmai-rs    ~/github/sw-cli-tools/pjmai-rs
    other-tool  ~/github/sw-cli-tools/other-tool
    lib-common  ~/github/sw-cli-tools/lib-common

$ shgp work  # or shgp softwarewrighter
Group: softwarewrighter
Alias: work
Path: ~/github/softwarewrighter
Projects (5):
  ...
```

### `pjmai group prompt` (alias: `prgp`)

Print current group name for shell prompt:

```bash
$ prgp
sw-cli-tools

# Or the alias if one exists
$ prgp --alias
cli

# Empty if no current project
$ prgp
(no output)
```

**Shell prompt integration:**
```bash
PROMPT='[$(prpj):$(prgp)] %~ $ '
# Result: [pjmai-rs:cli] ~/github/sw-cli-tools/pjmai-rs $
```

### `pjmai group alias`

Manage group aliases:

```bash
# Add alias
pjmai group alias sw-cli-tools cli
pjmai group alias softwarewrighter work
pjmai group alias personal home
pjmai group alias mighty proto

# List aliases
pjmai group alias --list
# sw-cli-tools → cli
# softwarewrighter → work
# ...

# Remove alias
pjmai group alias sw-cli-tools --remove
```

---

## Updated `lspj` Behavior

```bash
# Show all projects (current behavior, unchanged)
lspj

# Filter by group (use name or alias)
lspj --group cli
lspj --group sw-cli-tools  # same result

# Show current group's projects only
lspj --group .  # "." means current group
# or maybe:
lspj -g         # short form, implies current group
```

**Key insight:** We don't need `--all` because `lspj` defaults to showing everything. The `--group` flag is opt-in filtering.

---

## Data Model

### Config Structure

```toml
# ~/.pjmai/config.toml
version = "0.1.0"
current_project = "pjmai-rs"

# Group aliases (the only manual part)
[group_aliases]
sw-cli-tools = "cli"
softwarewrighter = "work"
personal = "home"
mighty = "proto"

# Projects (unchanged)
[[project]]
name = "pjmai-rs"
[project.action]
file_or_dir = "~/github/sw-cli-tools/pjmai-rs"
# Note: no explicit "group" field needed - inferred from path
```

### Rust Types

```rust
/// Updated registry
pub struct ProjectsRegistry {
    pub version: String,
    pub current_project: ProjectName,
    pub project: Vec<ChangeToProject>,
    pub stack: Vec<ProjectName>,

    // NEW: Group aliases only
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub group_aliases: HashMap<String, String>,  // group_name → alias
}

/// Inferred group (not stored, computed at runtime)
pub struct InferredGroup {
    pub name: String,           // e.g., "sw-cli-tools"
    pub alias: Option<String>,  // e.g., "cli"
    pub path: PathBuf,          // e.g., ~/github/sw-cli-tools
    pub projects: Vec<String>,  // project names in this group
}
```

---

## Group Inference Algorithm

```rust
fn infer_group(project_path: &str) -> Option<InferredGroup> {
    let path = Path::new(project_path);
    let parent = path.parent()?;
    let group_name = parent.file_name()?.to_str()?;

    Some(InferredGroup {
        name: group_name.to_string(),
        path: parent.to_path_buf(),
        ..
    })
}

fn get_all_groups(registry: &ProjectsRegistry) -> Vec<InferredGroup> {
    let mut groups: HashMap<String, InferredGroup> = HashMap::new();

    for project in &registry.project {
        if let Some(mut group) = infer_group(&project.action.file_or_dir) {
            // Add alias if exists
            group.alias = registry.group_aliases.get(&group.name).cloned();

            // Add project to group
            groups.entry(group.name.clone())
                .or_insert(group)
                .projects.push(project.name.clone());
        }
    }

    groups.into_values().collect()
}
```

---

## Migration from Current `group` Field

Currently `ProjectMetadata` has a `group: Option<String>` field. Options:

**Option A: Keep it as override**
- If set, use it instead of inferred group
- Useful for projects that don't fit the directory pattern

**Option B: Remove it**
- Groups are always inferred
- Simpler, but less flexible

**Option C: Deprecate it**
- Still read it for backwards compatibility
- New projects don't use it
- Eventually remove

**Recommendation:** Option A - keep as optional override for edge cases.

```bash
# Normal: group inferred from path
adpj myproj -f ~/github/owner/myproj
# → group: owner

# Override: explicit group
adpj myproj -f /some/weird/path --group work
# → group: work (explicit, not inferred)
```

---

## Implementation Plan

### Phase 1: Core Group Commands

1. Add `group_aliases` field to `ProjectsRegistry`
2. Implement group inference function
3. Implement `pjmai group list` (`lsgp`)
4. Implement `pjmai group show` (`shgp`)
5. Implement `pjmai group prompt` (`prgp`)
6. Add shell aliases

### Phase 2: Group Aliases

1. Implement `pjmai group alias` subcommand
2. Update `lspj --group` to accept aliases
3. Update `shgp` to accept aliases

### Phase 3: Integration

1. Update `lspj --group .` for current group
2. Update shell prompt examples in docs
3. Consider: show group in `lspj` output?

---

## Example Workflow

```bash
# Add some projects (groups inferred automatically)
adpj pjmai -f ~/github/sw-cli-tools/pjmai-rs
adpj assist -f ~/github/softwarewrighter/assist
adpj dotfiles -f ~/github/personal/dotfiles

# See all groups
$ lsgp
GROUP              PROJECTS  PATH
  sw-cli-tools     1         ~/github/sw-cli-tools
  softwarewrighter 1         ~/github/softwarewrighter
  personal      1         ~/github/personal

# Add friendly aliases
pjmai group alias sw-cli-tools cli
pjmai group alias softwarewrighter work
pjmai group alias personal home

# Switch to a project
chpj pjmai

# See current group
$ prgp
sw-cli-tools

$ prgp --alias
cli

# Show group details
$ shgp
Group: sw-cli-tools
Alias: cli
Path: ~/github/sw-cli-tools
Projects (1):
  > pjmai-rs

# Filter lspj by group
$ lspj --group cli
[cli] sw-cli-tools (1 project)
  > pjmai-rs  ~/github/sw-cli-tools/pjmai-rs

# Show current group's projects
$ lspj --group .
[cli] sw-cli-tools (1 project)
  > pjmai-rs  ~/github/sw-cli-tools/pjmai-rs
```

---

## Summary

| Aspect | Behavior |
|--------|----------|
| Group creation | Automatic (inferred from project paths) |
| Group deletion | Automatic (when no projects remain) |
| Group aliases | Manual (`pjmai group alias`) |
| Current group | Implied by current project |
| `lspj` default | All projects (unchanged) |
| `lspj --group X` | Filter by group name or alias |
| `chpj` | Unchanged - globally unique nicknames |

**Simplicity wins:**
- No `adgp`/`rmgp` - groups just exist
- No `chgp` - group follows project
- Only aliases are manual
- Everything else is inferred
