## Keymap

- [Normal mode](#normal-mode)
  - [Minor modes](#minor-modes)
  - [Find mode](#find-mode)
  - [Portal mode](#portal-mode)
  - [Prev / Next mode](#prev--next-mode)
  - [Match mode](#match-mode)
  - [Symbol mode](#symbol-mode)
  - [Tree mode](#tree-mode)
  - [Window mode](#window-mode)
  - [View mode](#view-mode)
  - [Extras mode](#extras-mode)
  - [Single-key core](#single-key-core)
  - [Popups](#popups)
    - [Popup](#popup)
    - [Completion Menu](#completion-menu)
    - [Signature-help Popup](#signature-help-popup)
- [Insert mode](#insert-mode)
- [Select / extend mode](#select--extend-mode)
- [Picker](#picker)
- [Prompt](#prompt)

> 💡 Mappings marked (**LSP**) require an active language server for the file.

> 💡 Mappings marked (**TS**) require a tree-sitter grammar for the file type.

> ⚠️ Some terminals' default key mappings conflict with Helix's. If any of the mappings described on this page do not work as expected, check your terminal's mappings to ensure they do not conflict. See the [wiki](https://github.com/helix-editor/helix/wiki/Terminal-Support) for known conflicts.

## Normal mode

Normal mode is the default mode when you launch helix. You can return to it from other modes by pressing the `Escape` key.

> 💡 This fork uses a different default keymap than upstream Helix: `ijkl` are up / left / down / right, and almost everything else lives in small named minor modes opened by a prefix key. Within a mode the capital letter of a key extends, acts in a broader context or is an alternate action (`mr` selects regex matches, `mR` splits on them). Prefix keys never double as commands.

> Hereafter, `<n>` represents an integer by typing a sequence of digits.

### Minor modes

| Key     | Mode                                | Purpose                                                       |
| -----   | ----                                | -------                                                       |
| `f`     | [Find](#find-mode)                  | Pickers and searching                                         |
| `Space` | [Portal](#portal-mode)              | Jump anywhere: search jumps, characters, lines, screen, files |
| `u`     | [Prev](#prev--next-mode)            | Go to the previous one (undo, match, diagnostic, function, ...) |
| `o`     | [Next](#prev--next-mode)            | Go to the next one (redo, match, diagnostic, function, ...)   |
| `m`     | [Match](#match-mode)                | Create and manipulate selections                              |
| `s`     | [Symbol](#symbol-mode)              | Everything the language server provides, except pickers       |
| `t`     | [Tree](#tree-mode)                  | Tree-sitter nodes: `i` `k` `j` `l` = up, down, left, right    |
| `w`     | [Window](#window-mode)              | Splits and window focus                                       |
| `z`     | [View](#view-mode)                  | Scrolling and cursor line placement                           |
| `g`     | [Extras](#extras-mode)              | Clipboard, comments, debugger                                 |
| `v`     | [Select mode](#select--extend-mode) | Extend selections                                             |
| `:`     |                                     | Command mode                                                  |

These modes (except command mode) can be configured by
[remapping keys](https://docs.helix-editor.com/remapping.html#minor-modes). Note that binding a single key
(for example `u = "..."`) replaces the whole minor mode behind that key.

### Find mode

Pickers and searching.

| Key | Description | Command |
| --- | ----------- | ------- |
| `ff` | File picker (workspace root) | `file_picker` |
| `fF` | File picker (working directory) | `file_picker_in_current_directory` |
| `fe` | File explorer | `file_explorer` |
| `fE` | File explorer at current buffer’s directory | `file_explorer_in_current_buffer_directory` |
| `fb` | Buffer picker | `buffer_picker` |
| `fj` | Jumplist picker | `jumplist_picker` |
| `fl` | Buffer lines picker | `buffer_lines_picker` |
| `fc` | Changed-files picker (git) | `changed_file_picker` |
| `fg` | Global search / grep in workspace | `global_search` |
| `fs` | Symbol picker (LSP or syntax) | `lsp_or_syntax_symbol_picker` |
| `fS` | Workspace symbol picker | `lsp_or_syntax_workspace_symbol_picker` |
| `fd` | Diagnostics picker | `diagnostics_picker` |
| `fD` | Workspace diagnostics picker | `workspace_diagnostics_picker` |
| `fr` | Resume last picker | `last_picker` |
| `fp` | Command palette | `command_palette` |
| `f*` | Use selection as search pattern (no word boundaries) | `search_selection` |

### Portal mode

Jump anywhere: search jumps, characters, lines, screen, files.

| Key | Description | Command |
| --- | ----------- | ------- |
| `<space>f` | Fuzzy jump in visible lines | `fuzzy_search` |
| `<space>F` | Fuzzy jump in the whole document | `fuzzy_search_global` |
| `<space>i` | Fuzzy jump in lines above the cursor | `fuzzy_search_up` |
| `<space>k` | Fuzzy jump in lines below the cursor | `fuzzy_search_down` |
| `<space>r` | Regex search forward | `search` |
| `<space>R` | Regex search backward | `rsearch` |
| `<space>w` | Jump to word by label | `goto_word` |
| `<space>m` | Matching bracket | `match_brackets` |
| `<space>u` / `<space>o` | Find previous / next character | `find_prev_char` / `find_next_char` |
| `<space>U` / `<space>O` | Till previous / next character | `till_prev_char` / `find_till_char` |
| `<space>j` / `<space>l` | Line start / end | `goto_line_start` / `goto_line_end` |
| `<space>h` | First non-blank of line | `goto_first_nonwhitespace` |
| `<space>g` / `<space>G` | File start / end | `goto_file_start` / `goto_last_line` |
| `<space>t` / `<space>c` / `<space>b` | Top / middle / bottom of screen | `goto_window_top` / `goto_window_center` / `goto_window_bottom` |
| `<space>a` / `<space>A` | Last accessed / last modified file | `goto_last_accessed_file` / `goto_last_modified_file` |
| `<space>.` | Last modification in file | `goto_last_modification` |
| `<space>p` | Open file path under cursor | `goto_file` |

### Prev / Next mode

U goes to the previous one, o to the next one; each row is a pair.

| Key | Description | Command |
| --- | ----------- | ------- |
| `ue` / `oe` | Undo / redo | `undo` / `redo` |
| `uh` / `oh` | Earlier / later in undo history | `earlier` / `later` |
| `uu` / `oo` | Previous / next match (the kept fuzzy matches if there are any, otherwise the regex search) | `search_prev` / `search_next` |
| `uU` / `oO` | Extend selection to previous / next match (same fuzzy / regex dispatch) | `extend_search_prev` / `extend_search_next` |
| `ud` / `od` | Previous / next diagnostic | `goto_prev_diag` / `goto_next_diag` |
| `uD` / `oD` | First / last diagnostic | `goto_first_diag` / `goto_last_diag` |
| `um` / `om` | Previous / next function (method) | `goto_prev_function` / `goto_next_function` |
| `ua` / `oa` | Previous / next argument | `goto_prev_parameter` / `goto_next_parameter` |
| `ut` / `ot` | Previous / next type or class | `goto_prev_class` / `goto_next_class` |
| `uc` / `oc` | Previous / next comment | `goto_prev_comment` / `goto_next_comment` |
| `up` / `op` | Previous / next paragraph | `goto_prev_paragraph` / `goto_next_paragraph` |
| `ug` / `og` | Previous / next git change | `goto_prev_change` / `goto_next_change` |
| `uG` / `oG` | First / last git change | `goto_first_change` / `goto_last_change` |
| `uj` / `oj` | Jumplist back / forward | `jump_backward` / `jump_forward` |
| `ub` / `ob` | Previous / next buffer | `goto_previous_buffer` / `goto_next_buffer` |
| `uv` / `ov` | Page up / down | `page_up` / `page_down` |

### Match mode

Create and manipulate selections.

| Key | Description | Command |
| --- | ----------- | ------- |
| `mr` | Select regex matches inside selections | `select_regex` |
| `mR` | Split selection on regex matches | `split_selection` |
| `mn` | Split selection on newlines | `split_selection_on_newline` |
| `mu` / `mo` | Rotate main selection back / forward | `rotate_selections_backward` / `rotate_selections_forward` |
| `mU` / `mO` | Rotate selection contents back / forward | `rotate_selection_contents_backward` / `rotate_selection_contents_forward` |
| `mc` / `mC` | Copy selection onto next / previous line | `copy_selection_on_next_line` / `copy_selection_on_prev_line` |
| `mf` | Flip cursor and anchor | `flip_selections` |
| `mF` | Ensure selection is forward | `ensure_selections_forward` |
| `mm` | Merge selections | `merge_selections` |
| `mM` | Merge consecutive selections | `merge_consecutive_selections` |
| `mk` | Keep selections matching regex | `keep_selections` |
| `mK` | Remove selections matching regex | `remove_selections` |
| `m,` | Remove primary selection | `remove_primary_selection` |
| `mx` | Shrink selection to line bounds | `shrink_to_line_bounds` |
| `mt` | Trim whitespace from selection | `trim_selections` |
| `m=` | Align selections in columns | `align_selections` |
| `mj` / `mJ` | Join lines / join lines with a space | `join_selections` / `join_selections_space` |
| `ma` / `mi` | Select around / inside textobject, then the object key | `select_textobject_around` / `select_textobject_inner` |
| `ms` / `mS` / `md` | Surround: add / replace / delete | `surround_add` / `surround_replace` / `surround_delete` |

### Symbol mode

Everything the language server provides, except pickers.

| Key | Description | Command |
| --- | ----------- | ------- |
| `sd` | Go to definition | `goto_definition` |
| `sD` | Go to declaration | `goto_declaration` |
| `st` | Go to type definition | `goto_type_definition` |
| `si` | Go to implementation | `goto_implementation` |
| `sr` | Go to references | `goto_reference` |
| `sh` | Hover documentation | `hover` |
| `sc` | Rename symbol (change) | `rename_symbol` |
| `sa` | Code action | `code_action` |
| `sf` | Format selections | `format_selections` |
| `ss` | Select all references to symbol under cursor | `select_references_to_symbol_under_cursor` |

### Tree mode

Tree-sitter nodes: i k j l = up, down, left, right in the tree.

| Key | Description | Command |
| --- | ----------- | ------- |
| `ti` | Expand selection to parent node | `expand_selection` |
| `tk` | Shrink selection to child node | `shrink_selection` |
| `tj` | Select previous sibling | `select_prev_sibling` |
| `tl` | Select next sibling | `select_next_sibling` |
| `ta` | Select all siblings | `select_all_siblings` |
| `tc` | Select all children | `select_all_children` |
| `te` | Move to end of parent node | `move_parent_node_end` |
| `tb` | Move to start of parent node | `move_parent_node_start` |

### Window mode

Splits and window focus (ijkl = up, left, down, right).

| Key | Description | Command |
| --- | ----------- | ------- |
| `wj` / `wl` / `wi` / `wk` | Focus window left / right / up / down | `jump_view_left` / `jump_view_right` / `jump_view_up` / `jump_view_down` |
| `wJ` / `wL` / `wI` / `wK` | Swap window left / right / up / down | `swap_view_left` / `swap_view_right` / `swap_view_up` / `swap_view_down` |
| `ws` / `wv` | Horizontal / vertical split | `hsplit` / `vsplit` |
| `wq` / `wo` | Close window / keep only this window | `wclose` / `wonly` |
| `ww` / `wt` | Rotate view / transpose view | `rotate_view` / `transpose_view` |
| `wf` / `wF` | Open file under cursor in h / v split | `goto_file_hsplit` / `goto_file_vsplit` |
| `wns` / `wnv` | New scratch buffer in h / v split | `hsplit_new` / `vsplit_new` |

### View mode

Scrolling and cursor line placement.

| Key | Description | Command |
| --- | ----------- | ------- |
| `zz` / `zc` | Center cursor line vertically | `align_view_center` |
| `zt` / `zb` | Cursor line to top / bottom of screen | `align_view_top` / `align_view_bottom` |
| `zi` / `zk` | Scroll view up / down | `scroll_up` / `scroll_down` |

### Extras mode

Clipboard, comments, debugger (the leftovers of the old space menu).

| Key | Description | Command |
| --- | ----------- | ------- |
| `gy` / `gY` | Yank selections / main selection to system clipboard | `yank_to_clipboard` / `yank_main_selection_to_clipboard` |
| `gp` / `gP` | Paste system clipboard after / before | `paste_clipboard_after` / `paste_clipboard_before` |
| `gR` | Replace selections with clipboard | `replace_selections_with_clipboard` |
| `gc` / `gC` | Toggle comments / block comments | `toggle_comments` / `toggle_block_comments` |
| `gd` | Debugger submenu (experimental, contents untouched) | N/A |

### Single-key core

The high-frequency core, no prefix.

| Key | Description | Command |
| --- | ----------- | ------- |
| `i` / `k` | Move up / down (visual line) | `move_visual_line_up` / `move_visual_line_down` |
| `j` / `l` | Previous / next word start | `move_prev_word_start` / `move_next_word_start` |
| `J` / `L` | Previous / next WORD start | `move_prev_long_word_start` / `move_next_long_word_start` |
| `I` / `K` | Half page up / down (cursor and view) | `page_cursor_half_up` / `page_cursor_half_down` |
| `h` / `H` | Insert before selection / at line start | `insert_mode` / `insert_at_line_start` |
| `a` / `A` | Append after selection / at line end | `append_mode` / `insert_at_line_end` |
| `n` / `N` | Open new line below / above | `open_below` / `open_above` |
| `v` | Select (extend) mode | `select_mode` |
| `x` / `X` | Select line / extend to line bounds | `extend_line_below` / `extend_to_line_bounds` |
| `;` | Collapse selection onto a single cursor | `collapse_selection` |
| `,` | Keep only primary selection | `keep_primary_selection` |
| `%` | Select entire file | `select_all` |
| `*` | Search for selection (word boundaries) | `search_selection_detect_word_boundaries` |
| `d` / `c` / `y` / `p` / `P` | Delete / change / yank / paste after / paste before | `delete_selection` / `change_selection` / `yank` / `paste_after` / `paste_before` |
| `r` / `R` | Replace with char / with yanked text | `replace` / `replace_with_yanked` |
| `<` / `>` | Unindent / indent | `unindent` / `indent` |
| `G` | Go to line number | `goto_line` |
| `q` / `Q` | Replay / record macro | `replay_macro` / `record_macro` |
| `"` | Select register | `select_register` |
| `&#124;` / `!` / `$` | Shell pipe / insert output / keep where command succeeds | `shell_pipe` / `shell_insert_output` / `shell_keep_pipe` |
| `:` | Command mode | `command_mode` |
| `.` | Repeat last command (including the text typed in insert mode) | `repeat` |

Other keys kept from upstream:

| Key                  | Description                                        | Command                 |
| -----                | -----------                                        | -------                 |
| `Home`, `End`        | Move to the start / end of the line                | `goto_line_start` / `goto_line_end` |
| `Ctrl-b`, `PageUp`   | Move page up                                       | `page_up`               |
| `Ctrl-f`, `PageDown` | Move page down                                     | `page_down`             |
| `Ctrl-u`, `Ctrl-d`   | Move cursor and page half page up / down           | `page_cursor_half_up` / `page_cursor_half_down` |
| `Ctrl-i`, `Tab`      | Jump forward on the jumplist                       | `jump_forward`          |
| `Ctrl-o`             | Jump backward on the jumplist                      | `jump_backward`         |
| `Ctrl-s`             | Save the current selection to the jumplist         | `save_selection`        |
| `Ctrl-c`             | Toggle comments                                    | `toggle_comments`       |
| `Ctrl-a`, `Ctrl-x`   | Increment / decrement object (number) under cursor | `increment` / `decrement` |
| `Ctrl-z`             | Suspend Helix and return to the shell              | `suspend`               |

> 💡 The fuzzy search commands `fuzzy_search_next`, `fuzzy_search_prev`, `extend_fuzzy_search_next` and `extend_fuzzy_search_prev` are unbound: `uu`, `oo`, `uU` and `oO` navigate the kept fuzzy matches (after `<space>f` and friends) and the regex search otherwise.

### Popups

#### Popup

Displays documentation for item under cursor. Remapping currently not supported.

| Key      | Description |
| ----     | ----------- |
| `Ctrl-u` | Scroll up   |
| `Ctrl-d` | Scroll down |

#### Completion Menu

Displays documentation for the selected completion item. Remapping currently not supported.

| Key                         | Description                      |
| ----                        | -----------                      |
| `Shift-Tab`, `Ctrl-p`, `Up` | Previous entry                   |
| `Tab`, `Ctrl-n`, `Down`     | Next entry                       |
| `Enter`                     | Close menu and accept completion |
| `Ctrl-c`                    | Close menu and reject completion |

Any other keypresses result in the completion being accepted.

#### Signature-help Popup

Displays the signature of the selected completion item. Remapping currently not supported.

| Key     | Description        |
| ----    | -----------        |
| `Alt-p` | Previous signature |
| `Alt-n` | Next signature     |

## Insert mode

Accessed by typing `i` in [normal mode](#normal-mode).

Insert mode bindings are minimal by default. Helix is designed to
be a modal editor, and this is reflected in the user experience and internal
mechanics. Changes to the text are only saved for undos when
escaping from insert mode to normal mode.

> 💡 New users are strongly encouraged to learn the modal editing paradigm
> to get the smoothest experience.

| Key                                         | Description                 | Command                  |
| -----                                       | -----------                 | -------                  |
| `Escape`                                    | Switch to normal mode       | `normal_mode`            |
| `Ctrl-s`                                    | Commit undo checkpoint      | `commit_undo_checkpoint` |
| `Ctrl-x`                                    | Autocomplete                | `completion`             |
| `Ctrl-r`                                    | Insert a register content   | `insert_register`        |
| `Ctrl-w`, `Alt-Backspace`                   | Delete previous word        | `delete_word_backward`   |
| `Alt-d`, `Alt-Delete`                       | Delete next word            | `delete_word_forward`    |
| `Ctrl-u`                                    | Delete to start of line     | `kill_to_line_start`     |
| `Ctrl-k`                                    | Delete to end of line       | `kill_to_line_end`       |
| `Ctrl-h`, `Backspace`, `Shift-Backspace`    | Delete previous char        | `delete_char_backward`   |
| `Ctrl-d`, `Delete`                          | Delete next char            | `delete_char_forward`    |
| `Ctrl-j`, `Enter`                           | Insert new line             | `insert_newline`         |
| `Tab`                                       | [Smart tab] (configurable)  | `smart_tab`              |
| `Shift-Tab`                                 | Insert tab                  | `insert_tab`             |

[Smart tab]: ./editor.md#editorsmart-tab-section

These keys are not recommended, but are included for new users less familiar
with modal editors.

| Key                                         | Description                 | Command                  |
| -----                                       | -----------                 | -------                  |
| `Up`                                        | Move to previous line       | `move_line_up`           |
| `Down`                                      | Move to next line           | `move_line_down`         |
| `Left`                                      | Backward a char             | `move_char_left`         |
| `Right`                                     | Forward a char              | `move_char_right`        |
| `PageUp`                                    | Move one page up            | `page_up`                |
| `PageDown`                                  | Move one page down          | `page_down`              |
| `Home`                                      | Move to line start          | `goto_line_start`        |
| `End`                                       | Move to line end            | `goto_line_end_newline`  |

As you become more comfortable with modal editing, you may want to disable some
insert mode bindings. You can do this by editing your `config.toml` file.

```toml
[keys.insert]
up = "no_op"
down = "no_op"
left = "no_op"
right = "no_op"
pageup = "no_op"
pagedown = "no_op"
home = "no_op"
end = "no_op"
```

## Select / extend mode

Accessed by typing `v` in [normal mode](#normal-mode).

Select mode echoes Normal mode, but changes any movements to extend
selections rather than replace them. The word motions (`j`, `l`, `J`, `L`),
vertical motions (`i`, `k`), the Portal jumps (`<space>u`, `<space>o`, `<space>U`,
`<space>O`, `<space>w`, `<space>j`, `<space>l`, `<space>g`, `<space>G`) and the
tree motions `te` and `tb` extend the selection instead of moving it.

`uu` and `oo` extend the selection to the previous / next match instead of
replacing it, so toggling this mode before pressing them makes it possible to
keep the current selection and selectively add matches to it.

## Picker

Keys to use within picker. Remapping currently not supported.
See the documentation page on [pickers](./pickers.md) for more info.
[Prompt](#prompt) keybinds also work in pickers, except where they conflict with picker keybinds.

| Key                          | Description                                                |
| -----                        | -------------                                              |
| `Shift-Tab`, `Up`, `Ctrl-p`  | Previous entry                                             |
| `Tab`, `Down`, `Ctrl-n`      | Next entry                                                 |
| `PageUp`, `Ctrl-u`           | Page up                                                    |
| `PageDown`, `Ctrl-d`         | Page down                                                  |
| `Home`                       | Go to first entry                                          |
| `End`                        | Go to last entry                                           |
| `Enter`                      | Open selected                                              |
| `Alt-Enter`                  | Open selected in the background without closing the picker |
| `Ctrl-s`                     | Open horizontally                                          |
| `Ctrl-v`                     | Open vertically                                            |
| `Ctrl-t`                     | Toggle preview                                             |
| `Escape`, `Ctrl-c`           | Close picker                                               |

## Prompt

Keys to use within prompt, Remapping currently not supported.

| Key                                         | Description                                                             |
| -----                                       | -------------                                                           |
| `Escape`, `Ctrl-c`                          | Close prompt                                                            |
| `Alt-b`, `Ctrl-Left`                        | Backward a word                                                         |
| `Ctrl-b`, `Left`                            | Backward a char                                                         |
| `Alt-f`, `Ctrl-Right`                       | Forward a word                                                          |
| `Ctrl-f`, `Right`                           | Forward a char                                                          |
| `Ctrl-e`, `End`                             | Move prompt end                                                         |
| `Ctrl-a`, `Home`                            | Move prompt start                                                       |
| `Ctrl-w`, `Alt-Backspace`, `Ctrl-Backspace` | Delete previous word                                                    |
| `Alt-d`, `Alt-Delete`, `Ctrl-Delete`        | Delete next word                                                        |
| `Ctrl-u`                                    | Delete to start of line                                                 |
| `Ctrl-k`                                    | Delete to end of line                                                   |
| `Backspace`, `Ctrl-h`, `Shift-Backspace`    | Delete previous char                                                    |
| `Delete`, `Ctrl-d`                          | Delete next char                                                        |
| `Ctrl-s`                                    | Insert a word under doc cursor, may be changed to Ctrl-r Ctrl-w later   |
| `Ctrl-p`, `Up`                              | Select previous history                                                 |
| `Ctrl-n`, `Down`                            | Select next history                                                     |
| `Ctrl-r`                                    | Insert the content of the register selected by following input char     |
| `Tab`                                       | Select next completion item                                             |
| `BackTab`                                   | Select previous completion item                                         |
| `Enter`                                     | Open selected                                                           |
