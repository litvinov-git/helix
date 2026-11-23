# tele-helix

**tele-helix** is my personal fork of the [Helix](https://helix-editor.com) editor (a
[Kakoune](https://github.com/mawww/kakoune) / [Neovim](https://github.com/neovim/neovim)
inspired modal editor written in Rust). It keeps Helix's editing model (selection first,
multiple selections, tree-sitter, built-in LSP) and changes how you get around.

## The idea: teleport, don't walk

Most editors teach you a large vocabulary of motions: word, WORD, paragraph, matching
bracket, `f`/`t` with a count, and so on. Each one makes you work out where the cursor will
land before you press the key.

Search is *imperative*: you look at where you want to be, type a few characters of what you
see, and you are there. tele-helix prioritizes that: fuzzy search, regex search, pickers
and jump labels come first, and the mentally heavier motions are kept short, predictable
and out of the way.

- **Fuzzy jump** (`<space>f`): type a few characters, the cursor jumps to the best match in
  the visible lines while you type; `Esc` puts everything back, `Enter` commits.
- **Pickers for everything** you would otherwise navigate to: files, buffers, jumplist,
  symbols, diagnostics, git changes, and the lines of the current buffer (`fl`).
- **Jump labels** (`<space>w`), regex search (`<space>r` / `<space>R`), character jumps
  (`gg` / `hh`), and the usual structural jumps (functions, diagnostics, changes, ...).

## The keymap

The default normal / select / insert keymap is replaced by a minor-mode keymap. Its goal is
to be **intuitive in the sense that you can guess a binding you have never typed**, instead
of looking it up. It gets there by being a small, consistent *vocabulary* rather than a
list of unrelated shortcuts:

- **A prefix key is an action family** (the "verb"). It opens a small named **minor mode**
  and the info box shows its name and keys: `f` Find, `<space>` Portal (jump), `g` / `h`
  Previous / Next, `G` / `H` Extend, `m` Match (selections), `s` Symbol (language
  server), `t` Tree, `w` Window.
- **The key after it is a direction or an object** (the "noun"), and it means the same thing
  nearly everywhere it appears (there are a few unavoidable exceptions, such as `md`,
  surround delete):
  - **Directions:** `g` is previous and `h` is next, wherever they form a pair. `i` / `k`
    are up / down and `j` / `l` are left / right (`ijkl`, not `hjkl`), also inside `t` and `w`.
  - **Objects:** `f` is a function, `a` an argument, `t` a type or class, `c` a comment,
    `p` a paragraph, `d` a diagnostic, `s` a search result, `r` a regex.
- **A capital letter means extend, a broader context, or an alternate action.**

So you compose instead of memorizing. `gf` is the previous function, `hf` the next one,
`Gf` extends the selection to the previous function, and `maf` selects around a function.
`gd` / `hd` walk diagnostics, `fd` opens the diagnostics picker, `fD` the workspace one.
`<space>r` is a regex jump, `mr` selects regex matches and `mR` splits on them. Learn a
noun once and it works with most verbs.

Some structure rules keep the vocabulary clean: bindings are mostly **two keys**, a prefix
key never doubles as a command, and no sequence is a prefix of another, so there is never
a pause or an ambiguity while typing. There is no Vim compatibility goal.

| Prefix | Mode | Purpose |
|---|---|---|
| `f` | Find | pickers and searching |
| `<space>` | Portal | jump anywhere: fuzzy / regex search, labels, lines, screen, files, buffers |
| `g` / `h` | Prev / Next | previous / next: search result, diagnostic, function, change, buffer, undo, ... |
| `G` / `H` | Extend Prev / Next | the same motions, extending the selection |
| `m` | Match | create and manipulate selections, textobjects, surround |
| `s` | Symbol | everything the language server provides |
| `t` | Tree | tree-sitter nodes |
| `w` | Window | splits and view placement |
| `C-` | Ctrl | save, quit, system clipboard |

A few choices worth knowing about:

- **Word motions only move the cursor.** `j` / `l` (and `J` / `L` for WORDs) go to the
  previous word start / next word end without selecting the span they crossed; `mw` / `mW`
  select the word / WORD under the cursor. `gg` / `hh` (find a character, ignoring case)
  behave the same way.
- **`u` / `o` insert before / after the selection and clear it.**
- **`I` / `K` move half a page and keep the cursor line centered**, for smooth fast scrolling.
- **`sd` / `si`** go to the definition / implementation, and fall back to each other when
  there is nothing to go to.
- **`gs` / `hs`** walk the matches of the last search, fuzzy or regex, whichever you used.

The complete keymap, with every command, is in [`keymap-v2.md`](./keymap-v2.md) (and in
[`book/src/keymap.md`](./book/src/keymap.md)). The implementation is
[`helix-term/src/keymap/default.rs`](./helix-term/src/keymap/default.rs).

## Other features

- **Fuzzy search** (`fuzzy_search*`), a fuzzy analog of `/` built on Helix's own `nucleo`
  matcher. Several matches per line, whitespace in the query ignored, smart case
  respected, history in register `~`. Variants search the visible lines, only above or only
  below the cursor, or the whole document.
- **Buffer lines picker** (`fl`): a fuzzy picker over the lines of the buffer as it is in
  memory, unsaved changes included.
- **`repeat`** (`.`): a bindable command that repeats the last command, including its
  following keys (`ms(`, `f` + a character, ...) and a sequence binding as a whole, and
  replays what was typed in insert mode.

## Building

You need a Rust toolchain. Then:

```sh
git clone https://github.com/litvinov-git/tele-helix
cd tele-helix
cargo build --release
ln -sfn ../../runtime target/release/runtime   # themes, grammars, queries
./target/release/hx
```

The binary is still called `hx` and the configuration still lives in `~/.config/helix`,
so the fork replaces upstream Helix rather than running next to it unless you rename the
binary. For other build options (grammars, packaging), see the upstream
[installation documentation](https://docs.helix-editor.com/install.html).

## About this fork

- This is **my personal fork**, built for how I work, not a general-purpose distribution.
- It tracks upstream Helix: I merge `helix-editor/helix` `master` regularly and never
  rewrite published history. The diff against upstream is kept small and lives in a few
  places: the keymap, the fuzzy search, the buffer lines picker, `repeat`, and a handful of
  small commands.
- **I rely heavily on [Claude Code](https://claude.com/claude-code)** to write the code. I
  review what it writes, and I strive to keep the code lean and close to upstream's style.
  If you see something that is not, that is a bug in my review.
- **You can use anything in this repository freely and never credit me for any of it.**
  The parts that are Helix's remain under Helix's license
  ([MPL 2.0](./LICENSE)), which is not mine to waive; everything I added is yours to take
  without attribution.

## Credits

All the hard parts (the editing model, tree-sitter integration, the language server
client, the renderer, ...) are the work of the [Helix](https://github.com/helix-editor/helix)
contributors, and Helix in turn follows the design of Kakoune. If you want an editor to
actually rely on, use Helix.
