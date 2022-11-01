use iced::Font;

pub const JETBRAINS_MONO: Font = Font::External {
    name: "Jetbrains Mono",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Regular.ttf"),
};

pub const JETBRAINS_MONO_BOLD: Font = Font::External {
    name: "Jetbrains Mono Bold",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-Bold.ttf"),
};

pub const JETBRAINS_MONO_NL_EXTRA_BOLD_ITALIC: Font = Font::External {
    name: "Jetbrains Mono NL Extra Bold Italic",
    bytes: include_bytes!("../../resources/font/JetBrainsMonoNL-ExtraBoldItalic.ttf"),
};

pub const JETBRAINS_MONO_LIGHT_ITALIC: Font = Font::External {
    name: "Jetbrains Mono Light Italic",
    bytes: include_bytes!("../../resources/font/JetBrainsMono-LightItalic.ttf"),
};

pub const SOURCE_HAN_SANS: Font = Font::External {
    name: "Source Han Sans",
    bytes: include_bytes!("../../resources/font/sarasa-fixed-sc-regular.ttf"),
};

pub const SOURCE_HAN_SANS_BOLD: Font = Font::External {
    name: "Source Han Sans Bold",
    bytes: include_bytes!("../../resources/font/sarasa-fixed-sc-bold.ttf"),
};

pub const SOURCE_HAN_SANS_HEAVY_ITALIC: Font = Font::External {
    name: "Source Han Sans Heavy Italic",
    bytes: include_bytes!("../../resources/font/sarasa-fixed-sc-bold.ttf"),
};

pub const SOURCE_HAN_SANS_LIGHT_ITALIC: Font = Font::External {
    name: "Source Han Sans Light Italic",
    bytes: include_bytes!("../../resources/font/sarasa-fixed-sc-regular.ttf"),
};
