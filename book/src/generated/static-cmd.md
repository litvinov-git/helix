| Name | Description | Default keybinds |
| --- | --- | --- |
| `no_op` | Do nothing |  |
| `move_char_left` | Move left | normal: `` <left> ``, insert: `` <left> `` |
| `move_char_right` | Move right | normal: `` <right> ``, insert: `` <right> `` |
| `move_line_up` | Move up |  |
| `move_line_down` | Move down |  |
| `move_visual_line_up` | Move up | normal: `` i ``, `` <up> ``, insert: `` <up> `` |
| `move_visual_line_down` | Move down | normal: `` k ``, `` <down> ``, insert: `` <down> `` |
| `extend_char_left` | Extend left | select: `` <left> `` |
| `extend_char_right` | Extend right | select: `` <right> `` |
| `extend_line_up` | Extend up |  |
| `extend_line_down` | Extend down |  |
| `extend_visual_line_up` | Extend up | select: `` i ``, `` <up> `` |
| `extend_visual_line_down` | Extend down | select: `` k ``, `` <down> `` |
| `copy_selection_on_next_line` | Copy selection on next line | normal: `` mc ``, select: `` mc `` |
| `copy_selection_on_prev_line` | Copy selection on previous line | normal: `` mC ``, select: `` mC `` |
| `move_next_word_start` | Move to start of next word | insert: `` <C-right> `` |
| `move_prev_word_start` | Move to start of previous word | normal: `` j ``, insert: `` <C-left> `` |
| `move_next_word_end` | Move to end of next word | normal: `` l `` |
| `move_prev_word_end` | Move to end of previous word |  |
| `move_next_long_word_start` | Move to start of next long word |  |
| `move_prev_long_word_start` | Move to start of previous long word | normal: `` J `` |
| `move_next_long_word_end` | Move to end of next long word | normal: `` L `` |
| `move_prev_long_word_end` | Move to end of previous long word |  |
| `move_next_sub_word_start` | Move to start of next sub word |  |
| `move_prev_sub_word_start` | Move to start of previous sub word |  |
| `move_next_sub_word_end` | Move to end of next sub word |  |
| `move_prev_sub_word_end` | Move to end of previous sub word |  |
| `move_parent_node_end` | Move to end of the parent node | normal: `` te `` |
| `move_parent_node_start` | Move to beginning of the parent node | normal: `` tb `` |
| `extend_next_word_start` | Extend to start of next word |  |
| `extend_prev_word_start` | Extend to start of previous word | select: `` j `` |
| `extend_next_word_end` | Extend to end of next word | select: `` l `` |
| `extend_prev_word_end` | Extend to end of previous word |  |
| `extend_next_long_word_start` | Extend to start of next long word |  |
| `extend_prev_long_word_start` | Extend to start of previous long word | select: `` J `` |
| `extend_next_long_word_end` | Extend to end of next long word | select: `` L `` |
| `extend_prev_long_word_end` | Extend to end of prev long word |  |
| `extend_next_sub_word_start` | Extend to start of next sub word |  |
| `extend_prev_sub_word_start` | Extend to start of previous sub word |  |
| `extend_next_sub_word_end` | Extend to end of next sub word |  |
| `extend_prev_sub_word_end` | Extend to end of prev sub word |  |
| `extend_parent_node_end` | Extend to end of the parent node | select: `` te `` |
| `extend_parent_node_start` | Extend to beginning of the parent node | select: `` tb `` |
| `find_till_char` | Move till next occurrence of char |  |
| `find_next_char` | Move to next occurrence of char | normal: `` <space>h `` |
| `extend_till_char` | Extend till next occurrence of char |  |
| `extend_next_char` | Extend to next occurrence of char | normal: `` <space>H ``, select: `` <space>H ``, `` <space>h `` |
| `till_prev_char` | Move till previous occurrence of char |  |
| `find_prev_char` | Move to previous occurrence of char | normal: `` <space>g `` |
| `extend_till_prev_char` | Extend till previous occurrence of char |  |
| `extend_prev_char` | Extend to previous occurrence of char | normal: `` <space>G ``, select: `` <space>G ``, `` <space>g `` |
| `repeat_last_motion` | Repeat last motion |  |
| `repeat` | Repeat the last command | normal: `` . ``, select: `` . `` |
| `replace` | Replace with new char | normal: `` r ``, select: `` r `` |
| `switch_case` | Switch (toggle) case |  |
| `switch_to_uppercase` | Switch to uppercase |  |
| `switch_to_lowercase` | Switch to lowercase |  |
| `page_up` | Move page up | normal: `` gv ``, `` <C-b> ``, `` <pageup> ``, select: `` gv ``, `` <C-b> ``, `` <pageup> ``, insert: `` <pageup> `` |
| `page_down` | Move page down | normal: `` hv ``, `` <C-f> ``, `` <pagedown> ``, select: `` hv ``, `` <C-f> ``, `` <pagedown> ``, insert: `` <pagedown> `` |
| `half_page_up` | Move half page up |  |
| `half_page_down` | Move half page down |  |
| `page_cursor_up` | Move page and cursor up |  |
| `page_cursor_down` | Move page and cursor down |  |
| `page_cursor_half_up` | Move page and cursor half up | normal: `` I ``, `` <C-u> ``, select: `` I ``, `` <C-u> `` |
| `page_cursor_half_down` | Move page and cursor half down | normal: `` K ``, `` <C-d> ``, select: `` K ``, `` <C-d> `` |
| `select_all` | Select whole document | normal: `` % ``, select: `` % `` |
| `select_regex` | Select all regex matches inside selections | normal: `` mr ``, select: `` mr `` |
| `split_selection` | Split selections on regex matches | normal: `` mR ``, select: `` mR `` |
| `split_selection_on_newline` | Split selection on newlines | normal: `` mn ``, select: `` mn `` |
| `merge_selections` | Merge selections | normal: `` mm ``, select: `` mm `` |
| `merge_consecutive_selections` | Merge consecutive selections | normal: `` mM ``, select: `` mM `` |
| `search` | Search for regex pattern | normal: `` <space>r ``, select: `` <space>r `` |
| `rsearch` | Reverse search for regex pattern | normal: `` <space>R ``, select: `` <space>R `` |
| `search_next` | Select next search match | normal: `` hs `` |
| `search_prev` | Select previous search match | normal: `` gs `` |
| `extend_search_next` | Add next search match to selection | normal: `` Hs ``, select: `` Hs ``, `` hs `` |
| `extend_search_prev` | Add previous search match to selection | normal: `` Gs ``, select: `` Gs ``, `` gs `` |
| `fuzzy_search` | Fuzzy search in visible lines | normal: `` <space>f ``, select: `` <space>f `` |
| `fuzzy_search_down` | Fuzzy search in visible lines from the cursor line down | normal: `` <space>k ``, select: `` <space>k `` |
| `fuzzy_search_up` | Fuzzy search in visible lines from the cursor line up | normal: `` <space>i ``, select: `` <space>i `` |
| `fuzzy_search_global` | Fuzzy search in the whole document | normal: `` <space>F ``, select: `` <space>F `` |
| `fuzzy_search_next` | Select next fuzzy search match |  |
| `fuzzy_search_prev` | Select previous fuzzy search match |  |
| `extend_fuzzy_search_next` | Add next fuzzy search match to selection |  |
| `extend_fuzzy_search_prev` | Add previous fuzzy search match to selection |  |
| `search_selection` | Use current selection as search pattern | normal: `` f* ``, select: `` f* `` |
| `search_selection_detect_word_boundaries` | Use current selection as the search pattern, automatically wrapping with `\b` on word boundaries | normal: `` * ``, select: `` * `` |
| `make_search_word_bounded` | Modify current search to make it word bounded |  |
| `global_search` | Global search in workspace folder | normal: `` fg ``, select: `` fg `` |
| `extend_line` | Select current line, if already selected, extend to another line based on the anchor |  |
| `extend_line_below` | Select current line, if already selected, extend to next line | normal: `` x ``, select: `` x `` |
| `extend_line_above` | Select current line, if already selected, extend to previous line |  |
| `select_line_above` | Select current line, if already selected, extend or shrink line above based on the anchor |  |
| `select_line_below` | Select current line, if already selected, extend or shrink line below based on the anchor |  |
| `extend_to_line_bounds` | Extend selection to line bounds | normal: `` ml ``, select: `` ml `` |
| `shrink_to_line_bounds` | Shrink selection to line bounds | normal: `` mL ``, select: `` mL `` |
| `delete_selection` | Delete selection | normal: `` d ``, select: `` d `` |
| `delete_selection_noyank` | Delete selection without yanking |  |
| `change_selection` | Change selection | normal: `` c ``, select: `` c `` |
| `change_selection_noyank` | Change selection without yanking |  |
| `collapse_selection` | Collapse selection into single cursor | normal: `` ; ``, select: `` ; `` |
| `flip_selections` | Flip selection cursor and anchor | normal: `` mf ``, select: `` mf `` |
| `ensure_selections_forward` | Ensure all selections face forward | normal: `` mF ``, select: `` mF `` |
| `insert_mode` | Insert before selection | normal: `` u ``, select: `` u `` |
| `append_mode` | Append after selection | normal: `` o ``, select: `` o `` |
| `command_mode` | Enter command mode | normal: `` : ``, select: `` : `` |
| `file_picker` | Open file picker | normal: `` ff ``, select: `` ff `` |
| `file_picker_in_current_buffer_directory` | Open file picker at current buffer's directory |  |
| `file_picker_in_current_directory` | Open file picker at current working directory | normal: `` fF ``, select: `` fF `` |
| `file_explorer` | Open file explorer in workspace root | normal: `` fe ``, select: `` fe `` |
| `file_explorer_in_current_buffer_directory` | Open file explorer at current buffer's directory | normal: `` fE ``, select: `` fE `` |
| `file_explorer_in_current_directory` | Open file explorer at current working directory |  |
| `code_action` | Perform code action | normal: `` sa ``, select: `` sa `` |
| `buffer_picker` | Open buffer picker | normal: `` fb ``, select: `` fb `` |
| `jumplist_picker` | Open jumplist picker | normal: `` fj ``, select: `` fj `` |
| `buffer_lines_picker` | Open picker of the lines of the current buffer | normal: `` fl ``, select: `` fl `` |
| `symbol_picker` | Open symbol picker |  |
| `syntax_symbol_picker` | Open symbol picker from syntax information |  |
| `lsp_or_syntax_symbol_picker` | Open symbol picker from LSP or syntax information | normal: `` fs ``, select: `` fs `` |
| `changed_file_picker` | Open changed file picker | normal: `` fc ``, select: `` fc `` |
| `select_references_to_symbol_under_cursor` | Select symbol references | normal: `` ss ``, select: `` ss `` |
| `workspace_symbol_picker` | Open workspace symbol picker |  |
| `syntax_workspace_symbol_picker` | Open workspace symbol picker from syntax information |  |
| `lsp_or_syntax_workspace_symbol_picker` | Open workspace symbol picker from LSP or syntax information | normal: `` fS ``, select: `` fS `` |
| `diagnostics_picker` | Open diagnostic picker | normal: `` fd ``, select: `` fd `` |
| `workspace_diagnostics_picker` | Open workspace diagnostic picker | normal: `` fD ``, select: `` fD `` |
| `last_picker` | Open last picker | normal: `` fr ``, select: `` fr `` |
| `insert_at_line_start` | Insert at start of line | normal: `` U ``, select: `` U `` |
| `insert_at_line_end` | Insert at end of line | normal: `` O ``, select: `` O `` |
| `open_below` | Open new line below selection | normal: `` n ``, select: `` n `` |
| `open_above` | Open new line above selection | normal: `` N ``, select: `` N `` |
| `normal_mode` | Enter normal mode | normal: `` <esc> ``, select: `` v ``, insert: `` <esc> `` |
| `select_mode` | Enter selection extend mode | normal: `` v `` |
| `exit_select_mode` | Exit selection mode | select: `` <esc> `` |
| `goto_definition` | Goto definition | normal: `` sd ``, select: `` sd `` |
| `goto_declaration` | Goto declaration | normal: `` sD ``, select: `` sD `` |
| `add_newline_above` | Add newline above |  |
| `add_newline_below` | Add newline below | normal: `` <tab> ``, select: `` <tab> `` |
| `goto_type_definition` | Goto type definition | normal: `` st ``, select: `` st `` |
| `goto_implementation` | Goto implementation | normal: `` si ``, select: `` si `` |
| `goto_file_start` | Goto line number `<n>` else file start | normal: `` <space>u `` |
| `goto_file_end` | Goto file end |  |
| `extend_to_file_start` | Extend to line number `<n>` else file start | select: `` <space>u `` |
| `extend_to_file_end` | Extend to file end |  |
| `goto_file` | Goto files/URLs in selections | normal: `` <space>p ``, select: `` <space>p `` |
| `goto_file_hsplit` | Goto files in selections (hsplit) | normal: `` wf ``, select: `` wf `` |
| `goto_file_vsplit` | Goto files in selections (vsplit) | normal: `` wF ``, select: `` wF `` |
| `goto_reference` | Goto references | normal: `` sr ``, select: `` sr `` |
| `goto_window_top` | Goto window top | normal: `` <space>t ``, select: `` <space>t `` |
| `goto_window_center` | Goto window center | normal: `` <space>c ``, select: `` <space>c `` |
| `goto_window_bottom` | Goto window bottom | normal: `` <space>b ``, select: `` <space>b `` |
| `goto_last_accessed_file` | Goto last accessed file | normal: `` <space>a ``, select: `` <space>a `` |
| `goto_last_modified_file` | Goto last modified file | normal: `` <space>A ``, select: `` <space>A `` |
| `goto_last_modification` | Goto last modification | normal: `` <space>. ``, select: `` <space>. `` |
| `goto_line` | Goto line | normal: `` <space>n ``, select: `` <space>n `` |
| `goto_last_line` | Goto last line | normal: `` <space>o `` |
| `extend_to_last_line` | Extend to last line | select: `` <space>o `` |
| `goto_first_diag` | Goto first diagnostic | normal: `` gD ``, select: `` gD `` |
| `goto_last_diag` | Goto last diagnostic | normal: `` hD ``, select: `` hD `` |
| `goto_next_diag` | Goto next diagnostic | normal: `` hd `` |
| `goto_prev_diag` | Goto previous diagnostic | normal: `` gd `` |
| `extend_goto_next_diag` | Extend selection to next diagnostic | normal: `` Hd ``, select: `` Hd ``, `` hd `` |
| `extend_goto_prev_diag` | Extend selection to previous diagnostic | normal: `` Gd ``, select: `` Gd ``, `` gd `` |
| `goto_next_change` | Goto next change | normal: `` hg `` |
| `goto_prev_change` | Goto previous change | normal: `` gg `` |
| `extend_goto_next_change` | Extend selection to next change | normal: `` Hg ``, select: `` Hg ``, `` hg `` |
| `extend_goto_prev_change` | Extend selection to previous change | normal: `` Gg ``, select: `` Gg ``, `` gg `` |
| `goto_first_change` | Goto first change | normal: `` gG ``, select: `` gG `` |
| `goto_last_change` | Goto last change | normal: `` hG ``, select: `` hG `` |
| `goto_line_start` | Goto line start | normal: `` <home> ``, insert: `` <home> `` |
| `goto_line_end` | Goto line end | normal: `` <end> ``, `` <space>l `` |
| `goto_column` | Goto column |  |
| `extend_to_column` | Extend to column |  |
| `goto_next_buffer` | Goto next buffer | normal: `` hb ``, select: `` hb `` |
| `goto_previous_buffer` | Goto previous buffer | normal: `` gb ``, select: `` gb `` |
| `goto_line_end_newline` | Goto newline at line end | insert: `` <end> `` |
| `goto_first_nonwhitespace` | Goto first non-blank in line | normal: `` <space>j `` |
| `trim_selections` | Trim whitespace from selections | normal: `` mt ``, select: `` mt `` |
| `extend_to_line_start` | Extend to line start | select: `` <home> `` |
| `extend_to_first_nonwhitespace` | Extend to first non-blank in line | select: `` <space>j `` |
| `extend_to_line_end` | Extend to line end | select: `` <end> ``, `` <space>l `` |
| `extend_to_line_end_newline` | Extend to line end |  |
| `signature_help` | Show signature help |  |
| `smart_tab` | Insert tab if all cursors have all whitespace to their left; otherwise, run a separate command. | insert: `` <tab> `` |
| `insert_tab` | Insert tab char | insert: `` <S-tab> `` |
| `insert_newline` | Insert newline char | insert: `` <C-j> ``, `` <ret> `` |
| `insert_char_interactive` | Insert an interactively-chosen char |  |
| `append_char_interactive` | Append an interactively-chosen char |  |
| `delete_char_backward` | Delete previous char | insert: `` <C-h> ``, `` <backspace> ``, `` <S-backspace> `` |
| `delete_char_forward` | Delete next char | insert: `` <C-d> ``, `` <del> `` |
| `delete_word_backward` | Delete previous word | insert: `` <C-w> ``, `` <A-backspace> ``, `` <C-backspace> `` |
| `delete_word_forward` | Delete next word | insert: `` <A-d> ``, `` <A-del> ``, `` <C-del> `` |
| `kill_to_line_start` | Delete till start of line | insert: `` <C-u> `` |
| `kill_to_line_end` | Delete till end of line | insert: `` <C-k> `` |
| `undo` | Undo change | normal: `` ge ``, select: `` ge ``, insert: `` <C-z> `` |
| `redo` | Redo change | normal: `` he ``, select: `` he `` |
| `earlier` | Move backward in history | normal: `` gh ``, select: `` gh `` |
| `later` | Move forward in history | normal: `` hh ``, select: `` hh `` |
| `commit_undo_checkpoint` | Commit changes to new checkpoint |  |
| `yank` | Yank selection | normal: `` y ``, select: `` y `` |
| `yank_to_clipboard` | Yank selections to clipboard | normal: `` <C-c> ``, select: `` <C-c> ``, insert: `` <C-c> `` |
| `cut_to_clipboard` | Cut selections to clipboard | normal: `` <C-x> ``, select: `` <C-x> ``, insert: `` <C-x> `` |
| `save_file` | Write changes to disk | normal: `` <C-s> ``, select: `` <C-s> ``, insert: `` <C-s> `` |
| `format_buffer` | Format the entire file | normal: `` sF ``, select: `` sF `` |
| `quit_view` | Close the current view | normal: `` <C-q> ``, select: `` <C-q> ``, insert: `` <C-q> `` |
| `yank_to_primary_clipboard` | Yank selections to primary clipboard |  |
| `yank_joined` | Join and yank selections |  |
| `yank_joined_to_clipboard` | Join and yank selections to clipboard |  |
| `yank_main_selection_to_clipboard` | Yank main selection to clipboard |  |
| `yank_joined_to_primary_clipboard` | Join and yank selections to primary clipboard |  |
| `yank_main_selection_to_primary_clipboard` | Yank main selection to primary clipboard |  |
| `replace_with_yanked` | Replace with yanked text | normal: `` R ``, select: `` R `` |
| `replace_selections_with_clipboard` | Replace selections by clipboard content |  |
| `replace_selections_with_primary_clipboard` | Replace selections by primary clipboard |  |
| `paste_after` | Paste after selection | normal: `` p ``, select: `` p `` |
| `paste_before` | Paste before selection | normal: `` P ``, select: `` P `` |
| `paste_clipboard_after` | Paste clipboard after selections | normal: `` <C-v> ``, select: `` <C-v> ``, insert: `` <C-v> `` |
| `paste_clipboard_before` | Paste clipboard before selections |  |
| `paste_primary_clipboard_after` | Paste primary clipboard after selections |  |
| `paste_primary_clipboard_before` | Paste primary clipboard before selections |  |
| `indent` | Indent selection | normal: `` <gt> ``, select: `` <gt> `` |
| `unindent` | Unindent selection | normal: `` <lt> ``, select: `` <lt> `` |
| `format_selections` | Format selection | normal: `` sf ``, select: `` sf `` |
| `join_selections` | Join lines inside selection | normal: `` mj ``, select: `` mj `` |
| `join_selections_space` | Join lines inside selection and select spaces | normal: `` mJ ``, select: `` mJ `` |
| `keep_selections` | Keep selections matching regex | normal: `` mk ``, select: `` mk `` |
| `remove_selections` | Remove selections matching regex | normal: `` mK ``, select: `` mK `` |
| `align_selections` | Align selections in column | normal: `` m= ``, select: `` m= `` |
| `keep_primary_selection` | Keep primary selection | normal: `` , ``, select: `` , `` |
| `remove_primary_selection` | Remove primary selection | normal: `` m, ``, select: `` m, `` |
| `completion` | Invoke completion popup |  |
| `hover` | Show docs for item under cursor | normal: `` sh ``, select: `` sh `` |
| `toggle_comments` | Comment/uncomment selections | normal: `` / ``, select: `` / `` |
| `toggle_line_comments` | Line comment/uncomment selections |  |
| `toggle_block_comments` | Block comment/uncomment selections | normal: `` ? ``, select: `` ? `` |
| `rotate_selections_forward` | Rotate selections forward | normal: `` mh ``, select: `` mh `` |
| `rotate_selections_backward` | Rotate selections backward | normal: `` mg ``, select: `` mg `` |
| `rotate_selection_contents_forward` | Rotate selection contents forward | normal: `` mH ``, select: `` mH `` |
| `rotate_selection_contents_backward` | Rotate selections contents backward | normal: `` mG ``, select: `` mG `` |
| `reverse_selection_contents` | Reverse selections contents |  |
| `expand_selection` | Expand selection to parent syntax node | normal: `` ti ``, select: `` ti `` |
| `shrink_selection` | Shrink selection to previously expanded syntax node | normal: `` tk ``, select: `` tk `` |
| `select_next_sibling` | Select next sibling in the syntax tree | normal: `` tl ``, select: `` tl `` |
| `select_prev_sibling` | Select previous sibling the in syntax tree | normal: `` tj ``, select: `` tj `` |
| `select_all_siblings` | Select all siblings of the current node | normal: `` ta ``, select: `` ta `` |
| `select_all_children` | Select all children of the current node | normal: `` tc ``, select: `` tc `` |
| `jump_forward` | Jump forward on jumplist | normal: `` hj ``, select: `` hj `` |
| `jump_backward` | Jump backward on jumplist | normal: `` gj ``, `` <C-o> ``, select: `` gj ``, `` <C-o> `` |
| `save_selection` | Save current selection to jumplist |  |
| `jump_view_right` | Jump to right split | normal: `` wl ``, select: `` wl `` |
| `jump_view_left` | Jump to left split | normal: `` wj ``, select: `` wj `` |
| `jump_view_up` | Jump to split above | normal: `` wi ``, select: `` wi `` |
| `jump_view_down` | Jump to split below | normal: `` wk ``, select: `` wk `` |
| `swap_view_right` | Swap with right split | normal: `` wL ``, select: `` wL `` |
| `swap_view_left` | Swap with left split | normal: `` wJ ``, select: `` wJ `` |
| `swap_view_up` | Swap with split above | normal: `` wI ``, select: `` wI `` |
| `swap_view_down` | Swap with split below | normal: `` wK ``, select: `` wK `` |
| `transpose_view` | Transpose splits | normal: `` wt ``, select: `` wt `` |
| `rotate_view` | Goto next window | normal: `` ww ``, select: `` ww `` |
| `rotate_view_reverse` | Goto previous window |  |
| `hsplit` | Horizontal bottom split | normal: `` ws ``, select: `` ws `` |
| `hsplit_new` | Horizontal bottom split scratch buffer | normal: `` wns ``, select: `` wns `` |
| `vsplit` | Vertical right split | normal: `` wv ``, select: `` wv `` |
| `vsplit_new` | Vertical right split scratch buffer | normal: `` wnv ``, select: `` wnv `` |
| `wclose` | Close window | normal: `` wq ``, select: `` wq `` |
| `wonly` | Close windows except current | normal: `` wo ``, select: `` wo `` |
| `select_register` | Select register | normal: `` " ``, select: `` " `` |
| `insert_register` | Insert register | insert: `` <C-r> `` |
| `copy_between_registers` | Copy between two registers |  |
| `align_view_middle` | Align view middle |  |
| `align_view_top` | Align view top | normal: `` wu ``, select: `` wu `` |
| `align_view_center` | Align view center | normal: `` wc ``, select: `` wc `` |
| `align_view_bottom` | Align view bottom | normal: `` wd ``, select: `` wd `` |
| `scroll_up` | Scroll view up |  |
| `scroll_down` | Scroll view down |  |
| `match_brackets` | Goto matching bracket | normal: `` <space>m ``, select: `` <space>m `` |
| `surround_add` | Surround add | normal: `` ms ``, select: `` ms `` |
| `surround_replace` | Surround replace | normal: `` mS ``, select: `` mS `` |
| `surround_delete` | Surround delete | normal: `` md ``, select: `` md `` |
| `select_textobject_around` | Select around object | normal: `` ma ``, select: `` ma `` |
| `select_textobject_inner` | Select inside object | normal: `` mi ``, select: `` mi `` |
| `goto_next_function` | Goto next function | normal: `` hf `` |
| `goto_prev_function` | Goto previous function | normal: `` gf `` |
| `extend_goto_next_function` | Extend selection to next function | normal: `` Hf ``, select: `` Hf ``, `` hf `` |
| `extend_goto_prev_function` | Extend selection to previous function | normal: `` Gf ``, select: `` Gf ``, `` gf `` |
| `goto_next_class` | Goto next type definition | normal: `` ht `` |
| `goto_prev_class` | Goto previous type definition | normal: `` gt `` |
| `extend_goto_next_class` | Extend selection to next type definition | normal: `` Ht ``, select: `` Ht ``, `` ht `` |
| `extend_goto_prev_class` | Extend selection to previous type definition | normal: `` Gt ``, select: `` Gt ``, `` gt `` |
| `goto_next_parameter` | Goto next parameter | normal: `` ha `` |
| `goto_prev_parameter` | Goto previous parameter | normal: `` ga `` |
| `extend_goto_next_parameter` | Extend selection to next parameter | normal: `` Ha ``, select: `` Ha ``, `` ha `` |
| `extend_goto_prev_parameter` | Extend selection to previous parameter | normal: `` Ga ``, select: `` Ga ``, `` ga `` |
| `goto_next_comment` | Goto next comment | normal: `` hc `` |
| `goto_prev_comment` | Goto previous comment | normal: `` gc `` |
| `extend_goto_next_comment` | Extend selection to next comment | normal: `` Hc ``, select: `` Hc ``, `` hc `` |
| `extend_goto_prev_comment` | Extend selection to previous comment | normal: `` Gc ``, select: `` Gc ``, `` gc `` |
| `goto_next_test` | Goto next test |  |
| `goto_prev_test` | Goto previous test |  |
| `goto_next_xml_element` | Goto next (X)HTML element |  |
| `goto_prev_xml_element` | Goto previous (X)HTML element |  |
| `goto_next_entry` | Goto next pairing |  |
| `goto_prev_entry` | Goto previous pairing |  |
| `goto_next_paragraph` | Goto next paragraph | normal: `` hp `` |
| `goto_prev_paragraph` | Goto previous paragraph | normal: `` gp `` |
| `extend_goto_next_paragraph` | Extend selection to next paragraph | normal: `` Hp ``, select: `` Hp ``, `` hp `` |
| `extend_goto_prev_paragraph` | Extend selection to previous paragraph | normal: `` Gp ``, select: `` Gp ``, `` gp `` |
| `dap_launch` | Launch debug target |  |
| `dap_restart` | Restart debugging session |  |
| `dap_toggle_breakpoint` | Toggle breakpoint |  |
| `dap_continue` | Continue program execution |  |
| `dap_pause` | Pause program execution |  |
| `dap_step_in` | Step in |  |
| `dap_step_out` | Step out |  |
| `dap_next` | Step to next |  |
| `dap_variables` | List variables |  |
| `dap_terminate` | End debug session |  |
| `dap_edit_condition` | Edit breakpoint condition on current line |  |
| `dap_edit_log` | Edit breakpoint log message on current line |  |
| `dap_switch_thread` | Switch current thread |  |
| `dap_switch_stack_frame` | Switch stack frame |  |
| `dap_enable_exceptions` | Enable exception breakpoints |  |
| `dap_disable_exceptions` | Disable exception breakpoints |  |
| `shell_pipe` | Pipe selections through shell command | normal: `` \| ``, select: `` \| `` |
| `shell_pipe_to` | Pipe selections into shell command ignoring output |  |
| `shell_insert_output` | Insert shell command output before selections | normal: `` ! ``, select: `` ! `` |
| `shell_append_output` | Append shell command output after selections |  |
| `shell_keep_pipe` | Filter selections with shell predicate | normal: `` $ ``, select: `` $ `` |
| `suspend` | Suspend and return to shell | normal: `` <C-z> ``, select: `` <C-z> `` |
| `rename_symbol` | Rename symbol | normal: `` sc ``, select: `` sc `` |
| `increment` | Increment item under cursor | normal: `` + ``, select: `` + `` |
| `decrement` | Decrement item under cursor | normal: `` <minus> ``, select: `` <minus> `` |
| `record_macro` | Record macro | normal: `` Q ``, select: `` Q `` |
| `replay_macro` | Replay macro | normal: `` q ``, select: `` q `` |
| `command_palette` | Open command palette | normal: `` fp ``, select: `` fp `` |
| `goto_word` | Jump to a two-character label | normal: `` <space>w `` |
| `extend_to_word` | Extend to a two-character label | select: `` <space>w `` |
| `goto_next_tabstop` | Goto next snippet placeholder |  |
| `goto_prev_tabstop` | Goto next snippet placeholder |  |
| `rotate_selections_first` | Make the first selection your primary one |  |
| `rotate_selections_last` | Make the last selection your primary one |  |
