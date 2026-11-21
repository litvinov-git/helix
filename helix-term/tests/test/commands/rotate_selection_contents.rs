use super::*;

// Progression: A -> B -> C -> D
//              as we press `A-)`
const A: &str = indoc! {"
    #(a|)#
    #(b|)#
    #(c|)#
    #[d|]#
    #(e|)#"
};

const B: &str = indoc! {"
    #(e|)#
    #(a|)#
    #(b|)#
    #(c|)#
    #[d|]#"
};

const C: &str = indoc! {"
    #[d|]#
    #(e|)#
    #(a|)#
    #(b|)#
    #(c|)#"
};

const D: &str = indoc! {"
    #(c|)#
    #[d|]#
    #(e|)#
    #(a|)#
    #(b|)#"
};

#[tokio::test(flavor = "multi_thread")]
async fn rotate_selection_contents_forward_repeated() -> anyhow::Result<()> {
    test((A, "mH", B)).await?;
    test((B, "mH", C)).await?;
    test((C, "mH", D)).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn rotate_selection_contents_forward_with_count() -> anyhow::Result<()> {
    test((A, "2mH", C)).await?;
    test((A, "3mH", D)).await?;
    test((B, "2mH", D)).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn rotate_selection_contents_backward_repeated() -> anyhow::Result<()> {
    test((D, "mG", C)).await?;
    test((C, "mG", B)).await?;
    test((B, "mG", A)).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn rotate_selection_contents_backward_with_count() -> anyhow::Result<()> {
    test((D, "2mG", B)).await?;
    test((D, "3mG", A)).await?;
    test((C, "2mG", A)).await?;

    Ok(())
}
