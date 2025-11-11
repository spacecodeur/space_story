use cursive::theme::{
    BaseColor, BorderStyle, Color, Palette,
    PaletteColor::{self, *},
    Theme,
};

pub fn global_dark() -> Theme {
    let mut palette = Palette::default();
    palette[Background] = Color::Dark(BaseColor::Black);
    palette[View] = Color::Dark(BaseColor::Black);
    palette[Primary] = Color::Dark(BaseColor::White);
    palette[TitlePrimary] = Color::Dark(BaseColor::White);
    palette[Highlight] = Color::Dark(BaseColor::Blue);
    palette[HighlightInactive] = Color::Dark(BaseColor::Blue);

    Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette,
    }
}

pub fn panel_choices() -> Theme {
    let mut theme = global_dark();

    theme.palette[PaletteColor::View] = Color::Rgb(50, 120, 120);

    theme
}
