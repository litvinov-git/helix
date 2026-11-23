use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn insert_mode_cursor_position() -> anyhow::Result<()> {
    test(TestCase {
        in_text: String::new(),
        in_selection: Selection::single(0, 0),
        in_keys: "u".into(),
        out_text: String::new(),
        out_selection: Selection::single(0, 0),
        line_feed_handling: LineFeedHandling::AsIs,
    })
    .await?;

    test(("#[\n|]#", "u", "#[|\n]#")).await?;
    test(("#[\n|]#", "u<esc>", "#[|\n]#")).await?;
    test(("#[\n|]#", "u<esc>u", "#[|\n]#")).await?;

    Ok(())
}

/// Range direction is preserved when escaping insert mode to normal
#[tokio::test(flavor = "multi_thread")]
async fn insert_to_normal_mode_cursor_position() -> anyhow::Result<()> {
    test_plain_insert(("#[f|]#oo\n", "v<right><right>mf<esc>", "#[|foo]#\n")).await?;
    test_plain_insert((
        indoc! {"\
                #[f|]#oo
                #(b|)#ar"
        },
        "v<right><right>mf<esc>",
        indoc! {"\
                #[|foo]#
                #(|bar)#"
        },
    ))
    .await?;

    test_plain_insert((
        indoc! {"\
                #[f|]#oo
                #(b|)#ar"
        },
        "o",
        indoc! {"\
                #[fo|]#o
                #(ba|)#r"
        },
    ))
    .await?;

    test_plain_insert((
        indoc! {"\
                #[f|]#oo
                #(b|)#ar"
        },
        "o<esc>",
        indoc! {"\
                #[f|]#oo
                #(b|)#ar"
        },
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn surround_by_character() -> anyhow::Result<()> {
    // Only pairs matching the passed character count
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mi{",
        "(so [many {#[good|]#} text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mi[",
        "(so [#[many {good} text|]#] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mi(",
        "(#[so [many {good} text] here|]#)",
    ))
    .await?;

    // Works with characters that aren't pairs too
    test((
        "'so 'many 'go#[o|]#d' text' here'",
        "mi'",
        "'so 'many '#[good|]#' text' here'",
    ))
    .await?;
    test((
        "'so 'many 'go#[o|]#d' text' here'",
        "2mi'",
        "'so '#[many 'good' text|]#' here'",
    ))
    .await?;
    test((
        "'so \"many 'go#[o|]#d' text\" here'",
        "mi\"",
        "'so \"#[many 'good' text|]#\" here'",
    ))
    .await?;

    // Selection direction is preserved
    test((
        "(so [many {go#[|od]#} text] here)",
        "mi{",
        "(so [many {#[|good]#} text] here)",
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn surround_inside_pair() -> anyhow::Result<()> {
    // Works at first character of buffer
    // TODO: Adjust test when opening pair failure is fixed
    test(("#[(|]#something)", "mim", "#[(|]#something)")).await?;

    // Inside a valid pair selects pair
    test(("some (#[t|]#ext) here", "mim", "some (#[text|]#) here")).await?;

    // On pair character selects pair
    // TODO: Opening pair character is a known failure case that needs addressing
    // test(("some #[(|]#text) here", "mim", "some (#[text|]#) here")).await?;
    test(("some (text#[)|]# here", "mim", "some (#[text|]#) here")).await?;

    // No valid pair does nothing
    test(("so#[m|]#e (text) here", "mim", "so#[m|]#e (text) here")).await?;

    // Count skips to outer pairs
    test((
        "(so (many (go#[o|]#d) text) here)",
        "1mim",
        "(so (many (#[good|]#) text) here)",
    ))
    .await?;
    test((
        "(so (many (go#[o|]#d) text) here)",
        "2mim",
        "(so (#[many (good) text|]#) here)",
    ))
    .await?;
    test((
        "(so (many (go#[o|]#d) text) here)",
        "3mim",
        "(#[so (many (good) text) here|]#)",
    ))
    .await?;

    // Matching pairs outside selection don't match
    test((
        "((so)((many) go#[o|]#d (text))(here))",
        "mim",
        "((so)(#[(many) good (text)|]#)(here))",
    ))
    .await?;
    test((
        "((so)((many) go#[o|]#d (text))(here))",
        "2mim",
        "(#[(so)((many) good (text))(here)|]#)",
    ))
    .await?;

    // Works with mixed braces
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mim",
        "(so [many {#[good|]#} text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "2mim",
        "(so [#[many {good} text|]#] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "3mim",
        "(#[so [many {good} text] here|]#)",
    ))
    .await?;

    // Selection direction is preserved
    test((
        "(so [many {go#[|od]#} text] here)",
        "mim",
        "(so [many {#[|good]#} text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[|od]#} text] here)",
        "2mim",
        "(so [#[|many {good} text]#] here)",
    ))
    .await?;
    test((
        "(so [many {go#[|od]#} text] here)",
        "3mim",
        "(#[|so [many {good} text] here]#)",
    ))
    .await?;

    // Only pairs outside of full selection range are considered
    test((
        "(so (many (go#[od) |]#text) here)",
        "mim",
        "(so (#[many (good) text|]#) here)",
    ))
    .await?;
    test((
        "(so (many#[ (go|]#od) text) here)",
        "mim",
        "(so (#[many (good) text|]#) here)",
    ))
    .await?;
    test((
        "(so#[ (many (go|]#od) text) here)",
        "mim",
        "(#[so (many (good) text) here|]#)",
    ))
    .await?;
    test((
        "(so (many (go#[od) text) |]#here)",
        "mim",
        "(#[so (many (good) text) here|]#)",
    ))
    .await?;

    // Works with multiple cursors
    test((
        "(so (many (good) text) #[he|]#re\nso (many (good) text) #(|he)#re)",
        "mim",
        "(#[so (many (good) text) here\nso (many (good) text) here|]#)",
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn surround_around_pair() -> anyhow::Result<()> {
    // Works at first character of buffer
    // TODO: Adjust test when opening pair failure is fixed
    test(("#[(|]#something)", "mam", "#[(|]#something)")).await?;

    // Inside a valid pair selects pair
    test(("some (#[t|]#ext) here", "mam", "some #[(text)|]# here")).await?;

    // On pair character selects pair
    // TODO: Opening pair character is a known failure case that needs addressing
    // test(("some #[(|]#text) here", "mam", "some #[(text)|]# here")).await?;
    test(("some (text#[)|]# here", "mam", "some #[(text)|]# here")).await?;

    // No valid pair does nothing
    test(("so#[m|]#e (text) here", "mam", "so#[m|]#e (text) here")).await?;

    // Count skips to outer pairs
    test((
        "(so (many (go#[o|]#d) text) here)",
        "1mam",
        "(so (many #[(good)|]# text) here)",
    ))
    .await?;
    test((
        "(so (many (go#[o|]#d) text) here)",
        "2mam",
        "(so #[(many (good) text)|]# here)",
    ))
    .await?;
    test((
        "(so (many (go#[o|]#d) text) here)",
        "3mam",
        "#[(so (many (good) text) here)|]#",
    ))
    .await?;

    // Matching pairs outside selection don't match
    test((
        "((so)((many) go#[o|]#d (text))(here))",
        "mam",
        "((so)#[((many) good (text))|]#(here))",
    ))
    .await?;
    test((
        "((so)((many) go#[o|]#d (text))(here))",
        "2mam",
        "#[((so)((many) good (text))(here))|]#",
    ))
    .await?;

    // Works with mixed braces
    test((
        "(so [many {go#[o|]#d} text] here)",
        "mam",
        "(so [many #[{good}|]# text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "2mam",
        "(so #[[many {good} text]|]# here)",
    ))
    .await?;
    test((
        "(so [many {go#[o|]#d} text] here)",
        "3mam",
        "#[(so [many {good} text] here)|]#",
    ))
    .await?;

    // Selection direction is preserved
    test((
        "(so [many {go#[|od]#} text] here)",
        "mam",
        "(so [many #[|{good}]# text] here)",
    ))
    .await?;
    test((
        "(so [many {go#[|od]#} text] here)",
        "2mam",
        "(so #[|[many {good} text]]# here)",
    ))
    .await?;
    test((
        "(so [many {go#[|od]#} text] here)",
        "3mam",
        "#[|(so [many {good} text] here)]#",
    ))
    .await?;

    // Only pairs outside of full selection range are considered
    test((
        "(so (many (go#[od) |]#text) here)",
        "mam",
        "(so #[(many (good) text)|]# here)",
    ))
    .await?;
    test((
        "(so (many#[ (go|]#od) text) here)",
        "mam",
        "(so #[(many (good) text)|]# here)",
    ))
    .await?;
    test((
        "(so#[ (many (go|]#od) text) here)",
        "mam",
        "#[(so (many (good) text) here)|]#",
    ))
    .await?;
    test((
        "(so (many (go#[od) text) |]#here)",
        "mam",
        "#[(so (many (good) text) here)|]#",
    ))
    .await?;

    // Works with multiple cursors
    test((
        "(so (many (good) text) #[he|]#re\nso (many (good) text) #(|he)#re)",
        "mam",
        "#[(so (many (good) text) here\nso (many (good) text) here)|]#",
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn match_around_closest_ts() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            r#"fn main() {testing!{"f#[|oo]#)"};}"#,
            "mam",
            r#"fn main() {testing!{#[|"foo)"]#};}"#,
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            r##"fn main() { let _ = ("#[|1]#23", "#(|1)#23"); } "##,
            "3mam",
            r##"fn main() #[|{ let _ = ("123", "123"); }]# "##,
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            r##" fn main() { let _ = ("12#[|3", "12]#3"); } "##,
            "1mam",
            r##" fn main() { let _ = #[|("123", "123")]#; } "##,
        ),
    )
    .await?;

    Ok(())
}

/// Ensure the very initial cursor in an opened file is the width of
/// the first grapheme
#[tokio::test(flavor = "multi_thread")]
async fn cursor_position_newly_opened_file() -> anyhow::Result<()> {
    let test = |content: &str, expected_sel: Selection| -> anyhow::Result<()> {
        let file = helpers::temp_file_with_contents(content)?;
        let mut app = helpers::AppBuilder::new()
            .with_file(file.path(), None)
            .build()?;

        let (view, doc) = helix_view::current!(app.editor);
        let sel = doc.selection(view.id).clone();
        assert_eq!(expected_sel, sel);

        Ok(())
    };

    test("foo", Selection::single(0, 1))?;
    test("👨‍👩‍👧‍👦 foo", Selection::single(0, 7))?;
    test("", Selection::single(0, 0))?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn cursor_position_append_eof() -> anyhow::Result<()> {
    // Selection is forwards
    test_plain_insert(("#[foo|]#", "obar<esc>", "#[foobar|]#\n")).await?;

    // Selection is backwards
    test_plain_insert(("#[|foo]#", "obar<esc>", "#[foobar|]#\n")).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn select_mode_tree_sitter_next_function_is_union_of_objects() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            indoc! {"\
                #[/|]#// Increments
                fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
            "},
            "hfvhf",
            indoc! {"\
                /// Increments
                #[fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }|]#
            "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn select_mode_tree_sitter_prev_function_unselects_object() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            indoc! {"\
                /// Increments
                #[fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }|]#
            "},
            "vgf",
            indoc! {"\
                /// Increments
                #[fn inc(x: usize) -> usize { x + 1 }|]#
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
            "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn select_mode_tree_sitter_prev_function_goes_backwards_to_object() -> anyhow::Result<()> {
    // Note: the anchor stays put and the head moves back.
    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            indoc! {"\
                /// Increments
                fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
                /// Identity
                #[fn ident(x: usize) -> usize { x }|]#
            "},
            "vgf",
            indoc! {"\
                /// Increments
                fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                #[|fn dec(x: usize) -> usize { x - 1 }
                /// Identity
                ]#fn ident(x: usize) -> usize { x }
            "},
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            indoc! {"\
                /// Increments
                fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
                /// Identity
                #[fn ident(x: usize) -> usize { x }|]#
            "},
            "vgfgf",
            indoc! {"\
                /// Increments
                #[|fn inc(x: usize) -> usize { x + 1 }
                /// Decrements
                fn dec(x: usize) -> usize { x - 1 }
                /// Identity
                ]#fn ident(x: usize) -> usize { x }
            "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn find_char() -> anyhow::Result<()> {
    test(("he#[l|]#lo\nhello", "hhl", "hel#[l|]#o\nhello")).await?;
    test(("hel#[l|]#o\nhello", "hhl", "hello\nhe#[l|]#lo")).await?;
    test(("hel#[l|]#o\nhello", "hhx", "hel#[l|]#o\nhello")).await?;
    test(("he#[l|]#lo\nhello", "2hhl", "hello\nhe#[l|]#lo")).await?;
    test(("#[h|]#ello\nhello", "9hhl", "#[h|]#ello\nhello")).await?;

    test_extra_keys(("h#[e|]#llo\nhello", "<A-t>l", "h#[el|]#lo\nhello")).await?;
    test_extra_keys(("he#[l|]#lo\nhello", "<A-t>l", "he#[llo\nhe|]#llo")).await?;
    test_extra_keys(("hel#[l|]#o\nhello", "<A-t>l", "hel#[lo\nhe|]#llo")).await?;
    test_extra_keys(("hel#[l|]#o\nhello", "<A-t>x", "hel#[l|]#o\nhello")).await?;
    test_extra_keys(("he#[l|]#lo\nhello", "2<A-t>l", "he#[llo\nhel|]#lo")).await?;
    test_extra_keys(("#[h|]#ello\nhello", "9<A-t>l", "#[h|]#ello\nhello")).await?;

    test(("hello\nhel#[l|]#o", "ggl", "hello\nhe#[l|]#lo")).await?;
    test(("hello\nhe#[l|]#lo", "ggl", "hel#[l|]#o\nhello")).await?;
    test(("hello\n#[h|]#ello", "ggx", "hello\n#[h|]#ello")).await?;
    test(("hello\nhel#[l|]#o", "2ggl", "hel#[l|]#o\nhello")).await?;
    test(("hello\nhell#[o|]#", "9ggl", "hello\nhell#[o|]#")).await?;

    test_extra_keys(("hello\nhell#[o|]#", "<A-T>l", "hello\nhel#[|lo]#")).await?;
    test_extra_keys(("hello\nhel#[l|]#o", "<A-T>l", "hell#[|o\nhell]#o")).await?;
    test_extra_keys(("hello\nhe#[l|]#lo", "<A-T>l", "hell#[|o\nhel]#lo")).await?;
    test_extra_keys(("hello\n#[h|]#ello", "<A-T>x", "hello\n#[h|]#ello")).await?;
    test_extra_keys(("hello\nhel#[l|]#o", "2<A-T>l", "hel#[|lo\nhell]#o")).await?;
    test_extra_keys(("hello\nhell#[o|]#", "9<A-T>l", "hello\nhell#[o|]#")).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn find_char_ignore_case() -> anyhow::Result<()> {
    test(("#[h|]#ello World", "hhw", "hello #[W|]#orld")).await?;
    test(("#[h|]#ello World", "hhW", "hello #[W|]#orld")).await?;
    test(("hello Wor#[l|]#d", "ggH", "#[h|]#ello World")).await?;
    test(("#[h|]#ello World", "2hho", "hello W#[o|]#rld")).await?;
    // The extending versions keep the anchor of the selection.
    test(("#[he|]#llo World", "Hhw", "#[hello W|]#orld")).await?;
    test(("hello #[Wo|]#rld", "GgH", "#[|hello W]#orld")).await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn find_char_line_ending() -> anyhow::Result<()> {
    test_extra_keys(("on#[e|]#\ntwo\n", "hh<ret>", "one#[\n|]#two\n")).await?;
    test_extra_keys(("one#[\n|]#two\n", "hh<ret>", "one\ntwo#[\n|]#")).await?;
    test_extra_keys(("one\n#[t|]#wo\n", "hh<ret>", "one\ntwo#[\n|]#")).await?;
    test_extra_keys(("one#[\n|]#", "hh<ret>", "one#[\n|]#")).await?;
    test_extra_keys(("#[o|]#ne\ntwo\n", "2hh<ret>", "one\ntwo#[\n|]#")).await?;
    test_extra_keys(("#[o|]#ne\ntwo\n", "9hh<ret>", "#[o|]#ne\ntwo\n")).await?;

    test_extra_keys(("o#[n|]#e\ntwo\n", "<A-t><ret>", "o#[ne|]#\ntwo\n")).await?;
    test_extra_keys(("on#[e|]#\ntwo\n", "<A-t><ret>", "on#[e\ntwo|]#\n")).await?;
    test_extra_keys(("one#[\n|]#two\n", "<A-t><ret>", "one#[\ntwo|]#\n")).await?;
    test_extra_keys(("one#[\n|]#", "<A-t><ret>", "one#[\n|]#")).await?;
    test_extra_keys(("on#[e|]#\n", "<A-t><ret>", "on#[e|]#\n")).await?;
    test_extra_keys(("#[o|]#ne\ntwo\n", "2<A-t><ret>", "#[one\ntwo|]#\n")).await?;
    test_extra_keys(("#[o|]#ne\ntwo\n", "9<A-t><ret>", "#[o|]#ne\ntwo\n")).await?;

    test_extra_keys((
        "one\ntwo\n#[t|]#hree\n",
        "gg<ret>",
        "one\ntwo#[\n|]#three\n",
    ))
    .await?;
    test_extra_keys((
        "one\ntwo#[\n|]#three\n",
        "gg<ret>",
        "one#[\n|]#two\nthree\n",
    ))
    .await?;
    test_extra_keys((
        "one\ntw#[o|]#\nthree\n",
        "gg<ret>",
        "one#[\n|]#two\nthree\n",
    ))
    .await?;
    test_extra_keys(("o#[n|]#e\n", "gg<ret>", "o#[n|]#e\n")).await?;
    test_extra_keys(("#[o|]#ne\n", "gg<ret>", "#[o|]#ne\n")).await?;
    test_extra_keys(("one\ntwo\nth#[r|]#ee\n", "2gg\n", "one#[\n|]#two\nthree\n")).await?;
    test_extra_keys(("one\ntwo\nth#[r|]#ee\n", "9gg\n", "one\ntwo\nth#[r|]#ee\n")).await?;

    test_extra_keys((
        "one\ntwo\nth#[r|]#ee\n",
        "<A-T><ret>",
        "one\ntwo\n#[|thr]#ee\n",
    ))
    .await?;
    test_extra_keys((
        "one\ntwo\n#[t|]#hree\n",
        "<A-T><ret>",
        "one\n#[|two\nt]#hree\n",
    ))
    .await?;
    test_extra_keys((
        "one\ntwo#[\n|]#three\n",
        "<A-T><ret>",
        "one\n#[|two\n]#three\n",
    ))
    .await?;
    test_extra_keys(("o#[n|]#e\n", "<A-T><ret>", "o#[n|]#e\n")).await?;
    test_extra_keys(("#[o|]#ne\n", "<A-T><ret>", "#[o|]#ne\n")).await?;
    test_extra_keys((
        "one\ntwo\nth#[r|]#ee\n",
        "2<A-T>\n",
        "one\n#[|two\nthr]#ee\n",
    ))
    .await?;
    test_extra_keys((
        "one\ntwo\nth#[r|]#ee\n",
        "9<A-T>\n",
        "one\ntwo\nth#[r|]#ee\n",
    ))
    .await?;

    test_extra_keys((
        indoc! {
            "\
            one
            #[|t]#wo
            three"
        },
        "<A-T><ret><space>l<right>2hh<ret>",
        indoc! {
            "\
            one
            two#[
            |]#three"
        },
    ))
    .await?;

    test_extra_keys((
        indoc! {
            "\
            #[|o]#ne
            two
            three"
        },
        "hh<ret>2<A-t><ret><space>j<A-T><ret>gg<ret>",
        indoc! {
            "\
            one#[
            |]#two
            three"
        },
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn repeat_find_char() -> anyhow::Result<()> {
    test((
        indoc! {
            "\
            #[o|]#ne two
            one two"
        },
        "hht.",
        indoc! {
            "\
            one two
            one #[t|]#wo"
        },
    ))
    .await?;

    test((
        indoc! {
            "\
            #[o|]#ne two
            one two
            "
        },
        "hh<ret>.",
        indoc! {
            "\
            one two
            one two#[
            |]#"
        },
    ))
    .await?;

    test((
        indoc! {
            "\
            #[o|]#ne two
            one two
            "
        },
        "hhthh<ret>.",
        indoc! {
            "\
            one two
            one two#[
            |]#"
        },
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_surround_replace() -> anyhow::Result<()> {
    test((
        indoc! {"\
            (#[|a]#)
            "},
        "mcm{",
        indoc! {"\
            {#[|a]#}
            "},
    ))
    .await?;

    test((
        indoc! {"\
            (#[a|]#)
            "},
        "mcm{",
        indoc! {"\
            {#[a|]#}
            "},
    ))
    .await?;

    test((
        indoc! {"\
            {{

            #(}|)#
            #[}|]#
            "},
        "mcm)",
        indoc! {"\
            ((

            #()|)#
            #[)|]#
            "},
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_surround_delete() -> anyhow::Result<()> {
    test((
        indoc! {"\
            (#[|a]#)
            "},
        "mdm",
        indoc! {"\
            #[|a]#
            "},
    ))
    .await?;

    test((
        indoc! {"\
            (#[a|]#)
            "},
        "mdm",
        indoc! {"\
            #[a|]#
            "},
    ))
    .await?;

    test((
        indoc! {"\
            {{

            #(}|)#
            #[}|]#
            "},
        "mdm",
        "\n\n#(\n|)##[\n|]#",
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn tree_sitter_motions_work_across_injections() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>let #[|x]# = 1;</script>",
            "ti",
            "<script>let #[|x = 1]#;</script>",
        ),
    )
    .await?;

    // When the full injected layer is selected, expand_selection jumps to
    // a more shallow layer.
    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>#[|let x = 1;]#</script>",
            "ti",
            "#[|<script>let x = 1;</script>]#",
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>let #[|x = 1]#;</script>",
            "tk",
            "<script>let #[|x]# = 1;</script>",
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>let #[|x]# = 1;</script>",
            "tl",
            "<script>let x #[=|]# 1;</script>",
        ),
    )
    .await?;

    test_with_config(
        AppBuilder::new().with_file("foo.html", None),
        (
            "<script>let #[|x]# = 1;</script>",
            "tj",
            "<script>#[|let]# x = 1;</script>",
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn search_abort_after_async_edit() -> anyhow::Result<()> {
    test_key_sequences(
        &mut AppBuilder::new()
            .with_input_text("one\ntwo #[x|]#\n")
            .build()?,
        vec![
            (
                Some("<space>r"),
                Some(&|_| {
                    // Simulate a formatter changing the document while the search prompt is open.
                    helix_term::job::dispatch_blocking(|editor, _| {
                        let (view, doc) = helix_view::current!(editor);
                        let transaction = helix_core::Transaction::change(
                            doc.text(),
                            std::iter::once((0, 4, None)),
                        );
                        doc.apply(&transaction, view.id);
                        doc.append_changes_to_history(view);
                    });
                }),
            ),
            (None, None),
            (
                Some("<esc>"),
                Some(&|app| {
                    let (view, doc) = helix_view::current_ref!(app.editor);
                    assert_eq!(doc.text(), "two x\n");
                    assert_eq!(doc.selection(view.id), &Selection::single(4, 5));
                }),
            ),
        ],
        false,
    )
    .await
}

// After an edit, `A-i` must not restore an `A-o` history range from before the edit.
#[tokio::test(flavor = "multi_thread")]
async fn shrink_selection_after_edit() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_file("foo.rs", None),
        (
            "let x = foo(#[b|]#ar);\n",
            "ti<space>uUzzzz<esc>%tk",
            "#[zzzzlet x = foo(bar);|]#\n",
        ),
    )
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn word_motions_only_move_the_cursor() -> anyhow::Result<()> {
    test(("#[h|]#ello world", "l", "hell#[o|]# world")).await?;
    test(("hello wor#[l|]#d", "j", "hello #[w|]#orld")).await?;
    test(("#[f|]#oo-bar baz", "L", "foo-ba#[r|]# baz")).await?;
    test(("foo-bar b#[a|]#z", "J", "foo-bar #[b|]#az")).await?;
    test(("#[h|]#ello world foo", "2l", "hello worl#[d|]# foo")).await?;
    // Select mode keeps extending.
    test(("#[h|]#ello world", "vl", "#[hello|]# world")).await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn select_word_shortcuts() -> anyhow::Result<()> {
    test(("hel#[l|]#o world", "mw", "#[hello|]# world")).await?;
    test(("foo-ba#[r|]# baz", "mw", "foo-#[bar|]# baz")).await?;
    test(("foo-ba#[r|]# baz", "mW", "#[foo-bar|]# baz")).await?;
    test(("one two #[t|]#hree", "2mw", "one two #[three|]#")).await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn insert_and_append_clear_the_selection() -> anyhow::Result<()> {
    test(("he#[llo|]# world", "uX<esc>", "heX#[|l]#lo world")).await?;
    test(("he#[llo|]# world", "oX<esc>", "helloX#[ |]#world")).await?;
    test((
        "#[he|]#llo #(wo|)#rld",
        "uX<esc>",
        "X#[|h]#ello X#(|w)#orld",
    ))
    .await?;
    Ok(())
}
