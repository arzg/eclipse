use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["foreground", "editor.foreground"],
        palette.base(BaseScale::Fg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.brown());

    builder.add_rule(Semantic("variable"), palette.light_yellow());
    builder.add_rule(Semantic("variable.declaration"), palette.yellow());
    builder.add_rule(Semantic("parameter"), palette.bright_blue());

    builder.add_rule(Semantic("string"), palette.turquoise());
    builder.add_rule(Semantic("number"), palette.blue());

    builder.add_rules(
        &[Semantic("function"), Semantic("method")],
        palette.light_lime_green(),
    );
    builder.add_rule(
        Semantic("method.static"),
        (palette.lime_green(), FontStyle::Italic),
    );
    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.green(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("builtinType"),
            Semantic("typeAlias"),
        ],
        palette.dark_blue(),
    );

    builder.add_rule(Semantic("interface"), palette.light_cyan());

    builder.add_rule(Semantic("enum"), (palette.pink(), FontStyle::Italic));
    builder.add_rule(
        Semantic("enumMember"),
        (palette.light_blue(), FontStyle::Italic),
    );

    builder.add_rule(Semantic("typeParameter"), palette.salmon());

    builder.add_rule(Semantic("property"), palette.cyan());
}
