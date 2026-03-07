# TODO — Future Features

## rmgp — Remove by Group

Remove all projects in a group at once. Useful for cleaning up an entire org's projects before re-scanning.

**Alias:** `rmgp`
**Command:** `pjmai group remove <name>`

```bash
rmgp sw-cli-tools         # "Remove 5 projects in group 'sw-cli-tools'? [y/N]"
rmgp sw-cli-tools -y      # skip prompt
rmgp .                    # remove all projects in current group
```

**Why:** When reorganizing projects (forking repos between orgs, renaming owners), you often want to clear one org's projects and re-scan, not nuke everything with `rmpj --all`.

**Implementation notes:**
- Add `Remove` variant to `GroupAction` enum in `args.rs`
- Add `group_remove` function in `command.rs` that uses `get_inferred_groups()` to find projects, prompts, then removes
- Add `rmgp` alias to `source-pjm.sh` and `aliases` output
- Should support `--yes`/`-y` for non-interactive use
- Should accept group name or alias (resolve via `find_group()`)
