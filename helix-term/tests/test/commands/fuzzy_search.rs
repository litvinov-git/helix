use super::*;

use helix_term::config::{Config, ConfigLoadError};

fn fuzzy_search_config() -> Config {
    // The fuzzy search commands have no default bindings.
    let keys = String::from(
        r#"
        [keys.normal]
        "A-f" = "fuzzy_search"
        "A-d" = "fuzzy_search_down"
        "A-u" = "fuzzy_search_up"
        "A-g" = "fuzzy_search_global"
        "A-n" = "fuzzy_search_next"
        "A-p" = "fuzzy_search_prev"
        "#,
    );
    let mut config = Config::load(Ok(&keys), Err(ConfigLoadError::default())).unwrap();
    config.editor = helpers::test_editor_config();
    config
}

async fn test_fuzzy<T: Into<TestCase>>(test_case: T) -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_config(fuzzy_search_config()),
        test_case,
    )
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn fuzzy_search_selects_best_match() -> anyhow::Result<()> {
    test_fuzzy((
        indoc! {"\
            #[o|]#ne two
            xfxxxb
            foo_bar
            "},
        "<A-f>fb<ret>",
        indoc! {"\
            one two
            xfxxxb
            #[foo_b|]#ar
            "},
    ))
    .await?;

    // smart case
    test_fuzzy((
        indoc! {"\
            #[o|]#ne two
            foo_bar
            Foo_Bar
            "},
        "<A-f>FB<ret>",
        indoc! {"\
            one two
            foo_bar
            #[Foo_B|]#ar
            "},
    ))
    .await?;

    // whitespace in the query is ignored
    test_fuzzy((
        indoc! {"\
            #[o|]#ne two
            foo_bar
            "},
        "<A-g>foo bar<ret>",
        indoc! {"\
            one two
            #[foo_bar|]#
            "},
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn fuzzy_search_abort_and_no_match_keep_selection() -> anyhow::Result<()> {
    let text = indoc! {"\
        #[o|]#ne two
        foo_bar
        "};

    test_fuzzy((text, "<A-f>fb<esc>", text)).await?;
    test_fuzzy((text, "<A-f>qqq<ret>", text)).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn fuzzy_search_next_prev() -> anyhow::Result<()> {
    // multiple matches on one line, wrapping around
    let text = indoc! {"\
        #[x|]#x
        ab xx a_b xx ab
        ab
        "};

    test_fuzzy((
        text,
        "<A-f>ab<ret><A-n>",
        indoc! {"\
            xx
            ab xx #[a_b|]# xx ab
            ab
            "},
    ))
    .await?;

    test_fuzzy((
        text,
        "<A-f>ab<ret>3<A-n>",
        indoc! {"\
            xx
            ab xx a_b xx ab
            #[ab|]#
            "},
    ))
    .await?;

    test_fuzzy((
        text,
        "<A-f>ab<ret>4<A-n>",
        indoc! {"\
            xx
            #[ab|]# xx a_b xx ab
            ab
            "},
    ))
    .await?;

    test_fuzzy((
        text,
        "<A-g>ab<ret><A-p><A-p>",
        indoc! {"\
            xx
            ab xx a_b xx #[ab|]#
            ab
            "},
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn fuzzy_search_down_up() -> anyhow::Result<()> {
    let text = indoc! {"\
        foo_bar
        #[o|]#ne two
        fxb
        "};

    test_fuzzy((
        text,
        "<A-d>fb<ret>",
        indoc! {"\
            foo_bar
            one two
            #[fxb|]#
            "},
    ))
    .await?;

    test_fuzzy((
        text,
        "<A-u>fb<ret>",
        indoc! {"\
            #[foo_b|]#ar
            one two
            fxb
            "},
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn fuzzy_search_visible_lines_only() -> anyhow::Result<()> {
    // The test terminal is 150 rows high, the match is far below that.
    let text = format!("#[o|]#ne two\n{}foo_bar\n", "xx\n".repeat(300));

    test_fuzzy((text.clone(), "<A-f>fb<ret>", text.clone())).await?;
    test_fuzzy((
        text,
        "<A-g>fb<ret>",
        format!("one two\n{}#[foo_b|]#ar\n", "xx\n".repeat(300)),
    ))
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn fuzzy_search_matches_are_dropped_by_other_changes() -> anyhow::Result<()> {
    // Matches are not recalculated: after another motion the next match
    // action does nothing.
    test_fuzzy((
        indoc! {"\
            #[x|]#x
            ab xx a_b
            "},
        "<A-f>ab<ret>l<A-n>",
        indoc! {"\
            xx
            ab#[ |]#xx a_b
            "},
    ))
    .await?;

    // The same goes for editing the document.
    test_fuzzy((
        indoc! {"\
            #[x|]#x
            ab xx a_b
            "},
        "<A-f>ab<ret>~<A-n>",
        indoc! {"\
            xx
            #[AB|]# xx a_b
            "},
    ))
    .await?;

    Ok(())
}
