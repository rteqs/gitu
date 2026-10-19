use super::*;

fn setup(ctx: TestContext) -> TestContext {
    commit(&ctx.dir, "unwanted-file", "");
    ctx
}

#[test]
pub(crate) fn reset_menu() {
    snapshot!(setup(setup_clone!()), "LljX");
}

#[test]
pub(crate) fn reset_soft_prompt() {
    snapshot!(setup(setup_clone!()), "LljXsq");
}

#[test]
pub(crate) fn reset_soft() {
    snapshot!(setup(setup_clone!()), "LljXs<enter>q");
}

#[test]
pub(crate) fn reset_mixed() {
    snapshot!(setup(setup_clone!()), "LljXm<enter>q");
}

#[test]
fn reset_hard() {
    snapshot!(setup(setup_clone!()), "LljXh<enter>q");
}
