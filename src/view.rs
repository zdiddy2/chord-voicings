use druid::widget::{Button, Flex, Label, List};
use druid::{LensExt, Widget, WidgetExt};

use crate::data::*;

pub fn build_ui() -> impl Widget<AppState> {
    let add_chord_button = Button::new("Add Chord").on_click(AppState::click_add_chord);

    let chord_list = List::new(chord).with_spacing(5.).lens(AppState::bars);

    Flex::column()
        .with_child(Flex::row().with_child(chord_list))
        .with_child(Flex::row().with_child(add_chord_button))
}

pub fn chord() -> impl Widget<Bar> {
    Label::raw().lens(Bar::chord.map(|chord| chord.get_name(), |_, _| ()))
}
