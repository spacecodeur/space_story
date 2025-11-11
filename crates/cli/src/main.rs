use cursive::Cursive;
use cursive::align::{HAlign, VAlign};
use cursive::theme::{BaseColor, Color, PaletteColor};
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::{
    Dialog, LinearLayout, ResizedView, SelectView, TextArea, TextView, ThemedView,
};
use orchestrator::get_next_prompt;

use crate::theme::panel_choices;

mod theme;

fn main() {
    let mut siv = cursive::default();
    siv.set_theme(theme::global_dark());

    let mut select = SelectView::new().h_align(HAlign::Center).autojump();

    select.add_all_str(orchestrator::get_lore_titles());

    select.set_on_submit(show_next_window);

    siv.add_layer(Dialog::around(select.scrollable().fixed_size((20, 10))).title("Which lore ?"));

    siv.run();
}

fn show_next_window(siv: &mut Cursive, lore: &str) {
    siv.pop_layer();

    // println!("{}", lore);

    let prompt = get_next_prompt(None);

    let screen_size = siv.screen_size();

    let main_layout = ResizedView::new(
        SizeConstraint::Full,
        SizeConstraint::Fixed(screen_size.y * 3 / 4),
        TextView::new(prompt.text),
    );

    let mut select = SelectView::new().v_align(VAlign::Center).autojump();

    for choice in prompt.action.choices {
        select.add_item(choice, 1);
    }

    let second_layout_inner =
        LinearLayout::vertical().child(Dialog::around(select).title(prompt.action.question));

    let second_layout = ThemedView::new(
        panel_choices(),
        ResizedView::new(
            SizeConstraint::Full,
            SizeConstraint::Fixed(screen_size.y * 1 / 4),
            second_layout_inner,
        ),
    );

    let layouts = LinearLayout::vertical()
        .child(main_layout)
        .child(second_layout);

    siv.add_layer(layouts);
}
