# Helix fork keymap specification

This is the complete specification of the default normal, select and insert mode keymap of this Helix fork. It describes the keymap as it is implemented in `helix-term/src/keymap/default.rs` (the `keymap!` macros). When the keymap changes, this file, `book/src/keymap.md` and the tests change with it. Read `CLAUDE.md` for the rest of the fork (fuzzy search, `buffer_lines_picker`, the bindable `repeat` command).

## Mental scheme

- **Owner's conventions.** `ijkl` are up / left / down / right (not `hjkl`). There is no Vim compatibility goal. Single-character motions are not bound, because word motions are used instead.
- **Minor modes.** Almost every binding lives in a small named minor mode opened by a prefix key, so a binding can be guessed without having learned it. Most bindings are exactly two keys, and there are no longer sequences except `wn{s,v}` and the textobject / surround follow-up keys.
- **Prefix keys never double as commands.** No key sequence is a prefix of another sequence.
- **Capital letter = broader or alternate.** Within a mode, the capital version of a key extends the selection, acts in a broader context, or is an alternate action: `mr` selects regex matches and `mR` splits on them; `fs` is the document symbol picker and `fS` the workspace one; `gD` / `hD` are first / last diagnostic.
- **`g` is previous, `h` is next.** Wherever they appear as a pair (the Prev / Next prefixes, the character jumps `gg` / `hh`, the `m` rotations) the key on the left of the home row index finger goes back and its neighbour goes forward. `G` / `H` are the same motions extending the selection.
- **Regex is always `r` / `R`.**
- **Fingers.** Keys are preferred on the index, middle and ring fingers over the inner index column and the pinky. Mappings that will almost never be used are not worth a key.
- **Search results are shared.** `gs` / `hs` navigate the kept fuzzy matches if there are any, otherwise the regex search. This is the behavior of `search_prev` / `search_next` themselves; they walk the fuzzy matches kept by the last fuzzy search until another motion drops them. The `fuzzy_search_next` / `fuzzy_search_prev` family stays unbound.

## Minor modes

| Prefix | Infobox title | Purpose |
|---|---|---|
| `f` | Find | pickers and searching |
| `<space>` | Portal | jump anywhere: search jumps, lines, screen, files, buffers |
| `g` / `h` | Prev / Next | `g` goes to the previous one, `h` to the next one; each row is a pair |
| `G` / `H` | Extend Prev / Next | the same motions as `g` / `h`, but the selection is extended by the object instead of the cursor moving |
| `m` | Match | create and manipulate selections |
| `s` | Symbol | everything the language server provides, except pickers |
| `t` | Tree | tree-sitter nodes: `i k j l` = up, down, left, right in the tree |
| `w` | Window | splits, window focus and view placement (`ijkl` = up, left, down, right) |
| `C-` | Ctrl | standard editor chords: save, quit and the system clipboard |
| (none) | Normal | the single-key core |

Infobox titles are the names in this table (`Normal mode`, `Select mode` and `Insert mode` for the top levels, `New split scratch buffer` for `wn`).

## Bindings

Keys are written as typed (`<space>` is the space bar). Where a row lists several keys and several commands separated by " / ", they correspond in order.

### Find (`f`): pickers and searching

| Keys | Command | Action |
|---|---|---|
| `ff` | file_picker | File picker (workspace root) |
| `fF` | file_picker_in_current_directory | File picker (working directory) |
| `fe` | file_explorer | File explorer |
| `fE` | file_explorer_in_current_buffer_directory | File explorer at the current buffer's directory |
| `fb` | buffer_picker | Buffer picker |
| `fj` | jumplist_picker | Jumplist picker |
| `fl` | buffer_lines_picker | Buffer lines picker (fuzzy line picker) |
| `fc` | changed_file_picker | Changed-files picker (git) |
| `fg` | global_search | Global search / grep in the workspace |
| `fs` / `fS` | lsp_or_syntax_symbol_picker / lsp_or_syntax_workspace_symbol_picker | Symbol picker / workspace symbol picker |
| `fd` / `fD` | diagnostics_picker / workspace_diagnostics_picker | Diagnostics picker / workspace diagnostics picker |
| `fr` | last_picker | Resume last picker |
| `fp` | command_palette | Command palette |
| `f*` | search_selection | Use selection as search pattern (no word boundaries) |

### Portal (`<space>`): jump anywhere

| Keys | Command | Action |
|---|---|---|
| `<space>f` | fuzzy_search | Fuzzy jump in the visible lines (the default fuzzy mode) |
| `<space>F` | fuzzy_search_global | Fuzzy jump in the whole document |
| `<space>i` / `<space>k` | fuzzy_search_up / fuzzy_search_down | Fuzzy jump in the visible lines above / below the cursor |
| `<space>r` / `<space>R` | search / rsearch | Regex search forward / backward |
| `<space>w` | goto_word | Jump to word by label |
| `<space>m` | match_brackets | Matching bracket |
| `<space>l` | goto_line_end | Line end |
| `<space>j` | goto_first_nonwhitespace | First non-blank of line |
| `<space>u` / `<space>o` | goto_file_start / goto_last_line | File start / end |
| `<space>t` / `<space>c` / `<space>b` | goto_window_top / goto_window_center / goto_window_bottom | Top / middle / bottom of screen |
| `<space>a` / `<space>A` | goto_last_accessed_file / goto_last_modified_file | Last accessed / last modified file |
| `<space>.` | goto_last_modification | Last modification in the file |
| `<space>p` | goto_file | Open the file path under the cursor |
| `<space>n` | goto_line | Go to line number |
| `<space>left` / `<space>right` | goto_previous_buffer / goto_next_buffer | Previous / next buffer (the arrows after the Portal prefix; `gb` / `hb` do the same) |

Column 0 (`goto_line_start`) has no key apart from `home`.

### Prev / Next (`g` / `h`): `g` goes to the previous one, `h` to the next one

| Keys | Command | Action |
|---|---|---|
| `ge` / `he` | undo / redo | Undo / redo |
| `gE` / `hE` | earlier / later | Earlier / later in the undo history |
| `gg` / `hh` | find_prev_char_ignore_case / find_next_char_ignore_case | Move the cursor to the previous / next occurrence of a character, ignoring case (then the character key); only moves, the selection is cleared down to a cursor on the found character |
| `gs` / `hs` | search_prev / search_next | Previous / next search result (kept fuzzy matches, else regex) |
| `gd` / `hd` | goto_prev_diag / goto_next_diag | Previous / next diagnostic |
| `gD` / `hD` | goto_first_diag / goto_last_diag | First / last diagnostic |
| `gf` / `hf` | goto_prev_function / goto_next_function | Previous / next function |
| `ga` / `ha` | goto_prev_parameter / goto_next_parameter | Previous / next argument |
| `gt` / `ht` | goto_prev_class / goto_next_class | Previous / next type or class |
| `gc` / `hc` | goto_prev_comment / goto_next_comment | Previous / next comment |
| `gp` / `hp` | goto_prev_paragraph / goto_next_paragraph | Previous / next paragraph |
| `gG` / `hG` | goto_prev_change / goto_next_change | Previous / next git change |
| `gm` / `hm` | goto_first_change / goto_last_change | First / last git change (modification) |
| `gj` / `hj` | jump_backward / jump_forward | Jumplist back / forward |
| `gb` / `hb` | goto_previous_buffer / goto_next_buffer | Previous / next buffer |
| `gv` / `hv` | page_up / page_down | Page up / down |

### Extend Prev / Next (`G` / `H`): the same motions, extending the selection

| Keys | Command | Action |
|---|---|---|
| `Gs` / `Hs` | extend_search_prev / extend_search_next | Extend by previous / next search result |
| `Gf` / `Hf` | extend_goto_prev_function / extend_goto_next_function | Extend by previous / next function |
| `Ga` / `Ha` | extend_goto_prev_parameter / extend_goto_next_parameter | Extend by previous / next argument |
| `Gt` / `Ht` | extend_goto_prev_class / extend_goto_next_class | Extend by previous / next type or class |
| `Gc` / `Hc` | extend_goto_prev_comment / extend_goto_next_comment | Extend by previous / next comment |
| `Gp` / `Hp` | extend_goto_prev_paragraph / extend_goto_next_paragraph | Extend by previous / next paragraph |
| `Gd` / `Hd` | extend_goto_prev_diag / extend_goto_next_diag | Extend by previous / next diagnostic |
| `GG` / `HG` | extend_goto_prev_change / extend_goto_next_change | Extend by previous / next git change |
| `Gg` / `Hh` | extend_prev_char_ignore_case / extend_next_char_ignore_case | Extend the selection to the previous / next occurrence of a character, ignoring case (keeps the anchor) |

The plain `g` / `h` motions only move the cursor in normal mode. The `extend_goto_*` commands share their implementation with the plain ones through an `extend: bool` parameter (`goto_ts_object_impl`, `goto_next_diag_impl`, `goto_next_change_impl`, ...).

### Match (`m`): create and manipulate selections

| Keys | Command | Action |
|---|---|---|
| `mr` / `mR` | select_regex / split_selection | Select regex matches inside selections / split on them |
| `mn` | split_selection_on_newline | Split selection on newlines |
| `mg` / `mh` | rotate_selections_backward / rotate_selections_forward | Rotate the main selection back / forward |
| `mG` / `mH` | rotate_selection_contents_backward / rotate_selection_contents_forward | Rotate selection contents back / forward |
| `mf` / `mF` | flip_selections / ensure_selections_forward | Flip cursor and anchor / ensure the selection is forward |
| `mm` / `mM` | merge_selections / merge_consecutive_selections | Merge selections / merge consecutive selections |
| `mk` / `mK` | keep_selections / remove_selections | Keep / remove selections matching a regex |
| `m,` | remove_primary_selection | Remove the primary selection |
| `ml` / `mL` | extend_to_line_bounds / shrink_to_line_bounds | Take entire lines / shrink back from line bounds |
| `mt` | trim_selections | Trim whitespace from the selection |
| `m=` | align_selections | Align selections in columns |
| `mj` / `mJ` | join_selections / join_selections_space | Join lines / join lines with a space |
| `ma` / `mi` | select_textobject_around / select_textobject_inner | Select around / inside a textobject (then the object key) |
| `mw` / `mW` | select_word / select_long_word | Select the word / WORD under the cursor (shortcut for `miw` / `miW`, no object key) |
| `ms` / `mc` / `md` | surround_add / surround_replace / surround_delete | Surround: add / change (replace) / delete |

### Symbol (`s`): everything the language server provides, except pickers

| Keys | Command | Action |
|---|---|---|
| `sd` / `sD` | goto_definition / goto_declaration | Go to definition (goes to the implementation if there is no definition) / declaration |
| `st` | goto_type_definition | Go to type definition |
| `si` | goto_implementation | Go to implementation (goes to the definition if there is no implementation) |
| `sr` | goto_reference | Go to references |
| `sh` | hover | Hover documentation |
| `sc` | rename_symbol | Rename symbol (change) |
| `sa` | code_action | Code action |
| `sf` / `sF` | format_selections / format_buffer | Format selections / the entire file |
| `ss` | select_references_to_symbol_under_cursor | Select all references to the symbol under the cursor |

### Tree (`t`): tree-sitter nodes, `i k j l` = up, down, left, right in the tree

| Keys | Command | Action |
|---|---|---|
| `ti` / `tk` | expand_selection / shrink_selection | Expand to the parent node / shrink to a child node |
| `tj` / `tl` | select_prev_sibling / select_next_sibling | Select previous / next sibling |
| `ta` / `tc` | select_all_siblings / select_all_children | Select all siblings / all children |
| `te` / `tb` | move_parent_node_end / move_parent_node_start | Move to the end / start of the parent node |

### Window (`w`): splits, focus and view placement

| Keys | Command | Action |
|---|---|---|
| `wj` / `wl` / `wi` / `wk` | jump_view_left / jump_view_right / jump_view_up / jump_view_down | Focus window left / right / up / down |
| `wJ` / `wL` / `wI` / `wK` | swap_view_left / swap_view_right / swap_view_up / swap_view_down | Swap window left / right / up / down |
| `ws` / `wv` | hsplit / vsplit | Horizontal / vertical split |
| `wq` / `wo` | wclose / wonly | Close window / keep only this window |
| `ww` / `wt` | rotate_view / transpose_view | Rotate view / transpose view |
| `wf` / `wF` | goto_file_hsplit / goto_file_vsplit | Open the file under the cursor in a horizontal / vertical split |
| `wns` / `wnv` | hsplit_new / vsplit_new | New scratch buffer in a horizontal / vertical split |
| `wc` / `wu` / `wd` | align_view_center / align_view_top / align_view_bottom | Put the cursor line in the center / top / bottom of the view |

### Ctrl (`C-`): standard editor chords

| Keys | Command | Action |
|---|---|---|
| `C-s` | save_file (runs `:write`) | Save file |
| `C-q` | quit_view (runs `:quit`) | Quit |
| `C-c` | yank_to_clipboard | Copy selections to the system clipboard |
| `C-x` | cut_to_clipboard (`yank_to_clipboard`, then `delete_selection_noyank`) | Cut selections to the system clipboard |
| `C-v` | paste_clipboard_after | Paste from the system clipboard |

### Normal: the single-key core

| Keys | Command | Action |
|---|---|---|
| `i` / `k` | move_visual_line_up / move_visual_line_down | Move up / down (visual line) |
| `j` / `l` | cursor_prev_word_start / cursor_next_word_end | Move the cursor to the previous word start / next word end (does not select the span) |
| `J` / `L` | cursor_prev_long_word_start / cursor_next_long_word_end | Move the cursor to the previous WORD start / next WORD end (does not select the span) |
| `I` / `K` | page_cursor_half_up_centered / page_cursor_half_down_centered | Half page up / down, then the cursor line is centered in the view (smooth, fast scrolling: the cursor stays in the middle) |
| `u` / `U` | insert_before_selection / insert_at_line_start | Insert at the start of the selection, which is cleared (each selection becomes a cursor) / at line start |
| `o` / `O` | append_after_selection / insert_at_line_end | Append after the end of the selection, which is cleared (each selection becomes a cursor) / at line end |
| `n` / `N` | open_below / open_above | Open a new line below / above |
| `tab` | add_newline_below | Add a new line below, stay in normal mode |
| `/` / `?` | toggle_comments / toggle_block_comments | Toggle comments / block comments |
| `.` | repeat | Repeat the last command (the fork's bindable repeat) |
| `v` | select_mode | Select (extend) mode |
| `x` | extend_line_below | Select line (extends to the next line if already selected) |
| `+` / `-` | increment / decrement | Increment / decrement the number under the cursor |
| `;` | collapse_selection | Collapse the selection onto a single cursor |
| `,` | keep_primary_selection | Keep only the primary selection |
| `%` | select_all | Select the entire file |
| `*` | search_selection_detect_word_boundaries | Search for the selection (word boundaries) |
| `d` / `c` / `y` / `p` / `P` | delete_selection / change_selection / yank / paste_after / paste_before | Delete / change / yank / paste after / paste before |
| `r` / `R` | replace / replace_with_yanked | Replace with a char / with the yanked text |
| `<` / `>` | unindent / indent | Unindent / indent |
| `q` / `Q` | replay_macro / record_macro | Replay / record macro |
| `"` | select_register | Select register |
| `\|` / `!` / `$` | shell_pipe / shell_insert_output / shell_keep_pipe | Shell pipe / insert output / keep where the command succeeds |
| `:` | command_mode | Command mode |

`page_cursor_half_*_centered` run `page_cursor_half_up` / `page_cursor_half_down` (`scroll(.., sync_cursor = true)`, which extends the selection in select mode) and then `align_view_center`. `find_*_char_ignore_case` / `extend_*_char_ignore_case` are the `find_char` family with a case-insensitive comparison; the plain ones leave a cursor on the found character instead of selecting the span (upstream `f` selects it), the extend ones keep upstream's extending behavior. `insert_before_selection` / `append_after_selection` are `insert_mode` / `append_mode` with every selection reduced to the one character the insertion happens at (the same result as for a one character selection). `select_word` / `select_long_word` run `select_textobject_inner` for `w` / `W` without waiting for the object key.

The word motions `j` `l` `J` `L` only move the cursor (`cursor_prev_word_start`, `cursor_next_word_end`, `cursor_prev_long_word_start`, `cursor_next_long_word_end`: the upstream word motion followed by collapsing the selection to the cursor). The upstream commands that select the span passed over (`move_prev_word_start`, `move_next_word_end`, `move_prev_long_word_start`, `move_next_long_word_end`) stay unbound; in select mode `j` `l` `J` `L` extend the selection as before.

Other keys kept from upstream: arrow keys (`up` / `down` = visual line motions, which do not scroll, `left` / `right` = char motions), `home` / `end`, `pageup` / `C-b` and `pagedown` / `C-f` (page up / down), `C-u` / `C-d` (half page), `C-o` (jump_backward), `C-z` (suspend), `esc` (normal_mode).

Unbound on purpose: the span-selecting word motions (see above), `a A h H e E w b W B t T f F s S C G X` and `u`/`o` as prefixes (`h` and `G` are prefixes now), the DAP debugger submenu, view scrolling (`scroll_up` / `scroll_down`), `add_newline_above`, `paste_clipboard_before`, `save_selection`, the case-sensitive character finds (`find_next_char`, `find_prev_char`, `extend_next_char`, `extend_prev_char`), till-character (`find_till_char`, `till_prev_char`), `copy_selection_on_next_line` / `copy_selection_on_prev_line` (`yp` covers it), `completion` outside insert mode, the stock `[` / `]` menus and the `C-w` window mode. The fuzzy next / prev family (`fuzzy_search_next`, `fuzzy_search_prev`, `extend_fuzzy_search_next`, `extend_fuzzy_search_prev`) is unbound because `gs` / `hs` / `Gs` / `Hs` already reach it.

## Select mode

Select mode is `normal.clone()` plus `merge_nodes` of the overrides below, so everything not listed is inherited unchanged (including `.`, `/` and `?`, the Ctrl chords, `f`, `m`, `s`, `w`, and the extending `G` / `H` mode). Every motion extends the selection:

| Keys | Command | Action |
|---|---|---|
| `i` / `k` / `up` / `down` | extend_visual_line_up / extend_visual_line_down | Extend up / down (visual line) |
| `left` / `right` | extend_char_left / extend_char_right | Extend by a character |
| `j` / `l` / `J` / `L` | extend_prev_word_start / extend_next_word_end / extend_prev_long_word_start / extend_next_long_word_end | Extend by word motions |
| `home` / `end` | extend_to_line_start / extend_to_line_end | Extend to line start / end |
| `gg` / `hh` | extend_prev_char_ignore_case / extend_next_char_ignore_case | Extend to previous / next character, ignoring case (same as `Gg` / `Hh`) |
| `<space>w` | extend_to_word | Extend to a word label |
| `<space>j` / `<space>l` | extend_to_first_nonwhitespace / extend_to_line_end | Extend to first non-blank / line end |
| `<space>u` / `<space>o` | extend_to_file_start / extend_to_last_line | Extend to file start / end |
| `gs` / `hs` | extend_search_prev / extend_search_next | Extend by previous / next search result |
| `gf` / `hf` `ga` / `ha` `gt` / `ht` `gc` / `hc` `gp` / `hp` `gd` / `hd` `gG` / `hG` | extend_goto_prev_* / extend_goto_next_* | Extend by the object of the motion |
| `te` / `tb` | extend_parent_node_end / extend_parent_node_start | Extend to the end / start of the parent node |
| `.` | repeat | Repeat the last command |
| `v` / `esc` | normal_mode / exit_select_mode | Leave select mode |

The regex, fuzzy and bracket portal jumps (`<space>r`, `<space>R`, `<space>f`, `<space>F`, `<space>i`, `<space>k`, `<space>m`) honor `editor.mode` themselves, so they extend in select mode without an override.

## Insert mode

Insert mode keeps the upstream bindings (`esc`, `C-r`, `C-w` / `A-backspace`, `A-d` / `A-del`, `C-u`, `C-k`, `C-h` / `backspace`, `C-d` / `del`, `C-j` / `ret`, `tab`, `S-tab`, the arrows, page and home / end keys) except for the ones listed below, and adds:

| Keys | Command | Action |
|---|---|---|
| `C-s` | save_file | Save file |
| `C-q` | quit_view | Quit |
| `C-c` | yank_to_clipboard | Copy selections to the system clipboard |
| `C-x` | cut_to_clipboard | Cut selections to the system clipboard |
| `C-v` | paste_clipboard_after | Paste from the system clipboard |
| `C-z` | undo | Undo (stay in insert mode) |
| `C-left` / `C-right` | move_prev_word_start / move_next_word_start | Move to previous / next word start |
| `C-backspace` / `C-del` | delete_word_backward / delete_word_forward | Delete the previous / next word |

`C-s` and `C-x` replace the upstream `commit_undo_checkpoint` and `completion`, which are unbound in insert mode. The chords share their commands with the Ctrl mode, so `save_file`, `quit_view` and `cut_to_clipboard` exist once.

## Implementation notes

- `save_file`, `quit_view` and `format_buffer` are thin static wrappers that run the `:write`, `:quit` and `:format` typable commands (`run_typable`).
- `cut_to_clipboard` yanks to `+` and then runs `delete_selection_noyank`.
- Only static commands can be bound in the `keymap!` macro.
- The keymap tests (`helix-term/tests/test/**`) are written against these default keys. Tests of commands that have no default key bind them through `test_extra_keys` (`A-s` = `save_selection`, `A-t` / `A-T` = `find_till_char` / `till_prev_char`).

## Open questions for the owner

- `C-s` no longer saves the selection (`save_selection` is unbound).
- Yank-main-selection, paste-before and replace-with-clipboard have no Ctrl chord, so they have no key.
- File start / end on `<space>u` / `<space>o` and the loss of a key for column 0 were picks made to resolve a clash with `<space>g` / `<space>h`, which are free now; `<space>g` / `<space>h` could take them back.
- Git change moved from `gg` / `hg` to `gG` / `hG` (first / last to `gm` / `hm`) and earlier / later from `gh` / `hh` to `gE` / `hE`, to free `gg` / `hh` for the character finds.
- `I` / `K` recenter after every half page jump; the half page is a guess at the best smooth motion, a smaller step (a quarter page) is a one-line change.
- Insert mode: `C-right` goes to the next word start; it could follow `l` and go to the word end instead.
- Insert mode: `move_prev_word_start` / `move_next_word_start` leave the word selected, so typing afterwards replaces it. A variant that only moves the cursor may feel more natural.
- Insert mode: `paste_clipboard_after` on `C-v` pastes after the cursor character; the before variant may feel better.
- Insert mode: `C-s` and `C-x` replace the undo checkpoint and the completion trigger, which now have no key there.
- Terminals often send `C-h` or `C-w` for `C-backspace` unless they support the kitty keyboard protocol, so that chord may not reach Helix.
- `tab` no longer jumps forward in the jumplist (`hj` does); `add_newline_above` has no key.
