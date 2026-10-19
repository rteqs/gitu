use super::*;
use crate::item_data::{ItemData, SectionHeader};

fn setup_hunk(mut ctx: TestContext) -> (TestContext, crate::app::App) {
    commit(&ctx.dir, "file", "old-a\nold-b\n");
    fs::write(ctx.dir.join("file"), "new-a\nnew-b\n").unwrap();

    let mut app = ctx.init_app();
    ctx.update(&mut app, keys("jj<tab>j"));
    assert!(matches!(
        app.screen().get_selected_item().data,
        ItemData::Hunk { .. }
    ));

    (ctx, app)
}

#[test]
fn vim_motion_walks_hunk_lines() {
    let (mut ctx, mut app) = setup_hunk(setup_clone!());
    let line_count = 4;

    for (down, up) in [("j", "k"), ("<down>", "<up>")] {
        for expected_line in 0..line_count {
            ctx.update(&mut app, keys(down));
            assert!(matches!(
                app.screen().get_selected_item().data,
                ItemData::HunkLine { line_i, .. } if line_i == expected_line
            ));
        }

        for expected_line in (0..line_count - 1).rev() {
            ctx.update(&mut app, keys(up));
            assert!(matches!(
                app.screen().get_selected_item().data,
                ItemData::HunkLine { line_i, .. } if line_i == expected_line
            ));
        }

        ctx.update(&mut app, keys(up));
        assert!(matches!(
            app.screen().get_selected_item().data,
            ItemData::Hunk { .. }
        ));
    }
}

#[test]
fn ctrl_motion_skips_hunk_lines() {
    let (mut ctx, mut app) = setup_hunk(setup_clone!());

    for (down, up) in [("<ctrl+j>", "<ctrl+k>"), ("<ctrl+down>", "<ctrl+up>")] {
        ctx.update(&mut app, keys("j"));
        ctx.update(&mut app, keys(down));
        assert!(matches!(
            app.screen().get_selected_item().data,
            ItemData::Header(SectionHeader::RecentCommits)
        ));

        ctx.update(&mut app, keys(up));
        assert!(matches!(
            app.screen().get_selected_item().data,
            ItemData::Hunk { .. }
        ));
    }
}

#[test]
fn vim_motion_skips_collapsed_hunk() {
    let (mut ctx, mut app) = setup_hunk(setup_clone!());
    ctx.update(&mut app, keys("<tab>"));

    for (down, up) in [("j", "k"), ("<down>", "<up>")] {
        ctx.update(&mut app, keys(down));
        assert!(matches!(
            app.screen().get_selected_item().data,
            ItemData::Header(SectionHeader::RecentCommits)
        ));

        ctx.update(&mut app, keys(up));
        assert!(matches!(
            app.screen().get_selected_item().data,
            ItemData::Hunk { .. }
        ));
        assert!(app.screen().is_collapsed(app.screen().get_selected_item()));
    }
}

fn setup_scroll(mut ctx: TestContext) -> (TestContext, crate::app::App) {
    for file in ["file-1", "file-2", "file-3"] {
        commit(&ctx.dir, file, "");
        fs::write(
            ctx.dir.join(file),
            (1..=20).fold(String::new(), |mut acc, i| {
                use std::fmt::Write as _;

                writeln!(acc, "line {} ({})", i, file).unwrap();
                acc
            }),
        )
        .unwrap();
    }

    let mut app = ctx.init_app();
    ctx.update(&mut app, keys("jjjj<tab>k<tab>k<tab>"));
    (ctx, app)
}

#[test]
fn scroll_down() {
    let (mut ctx, mut app) = setup_scroll(setup_clone!());
    ctx.update(&mut app, keys(HALF_PAGE_DOWN));
    insta::assert_snapshot!(ctx.redact_buffer());
}

#[test]
fn scroll_past_selection() {
    let (mut ctx, mut app) = setup_scroll(setup_clone!());
    ctx.update(
        &mut app,
        keys(&format!("{HALF_PAGE_DOWN}{HALF_PAGE_DOWN}{HALF_PAGE_DOWN}")),
    );
    insta::assert_snapshot!(ctx.redact_buffer());
}

#[test]
fn move_prev_sibling() {
    let (mut ctx, mut app) = setup_scroll(setup_clone!());
    ctx.update(&mut app, keys("<alt+k><alt+k>"));
    insta::assert_snapshot!(ctx.redact_buffer());
}

#[test]
fn move_next_sibling() {
    let (mut ctx, mut app) = setup_scroll(setup_clone!());
    ctx.update(&mut app, keys("<alt+j>"));
    insta::assert_snapshot!(ctx.redact_buffer());
}

#[test]
fn move_next_then_parent_section() {
    let (mut ctx, mut app) = setup_scroll(setup_clone!());
    ctx.update(&mut app, keys("<alt+j><alt+h>"));
    insta::assert_snapshot!(ctx.redact_buffer());
}

#[test]
fn exit_from_picker_exits_menu() {
    snapshot!(setup_clone!(), "bb<esc>");
}

#[test]
fn re_enter_picker_from_menu() {
    snapshot!(setup_clone!(), "bb<esc>bb");
}
