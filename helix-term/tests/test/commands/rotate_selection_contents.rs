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
    test((A, "mO", B)).await?;
    test((B, "mO", C)).await?;
    test((C, "mO", D)).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn rotate_selection_contents_forward_with_count() -> anyhow::Result<()> {
    test((A, "2mO", C)).await?;
    test((A, "3mO", D)).await?;
    test((B, "2mO", D)).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn rotate_selection_contents_backward_repeated() -> anyhow::Result<()> {
    test((D, "mU", C)).await?;
    test((C, "mU", B)).await?;
    test((B, "mU", A)).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn rotate_selection_contents_backward_with_count() -> anyhow::Result<()> {
    test((D, "2mU", B)).await?;
    test((D, "3mU", A)).await?;
    test((C, "2mU", A)).await?;

    Ok(())
}
