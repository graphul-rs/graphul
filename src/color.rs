// Colors is a struct to define custom colors for Graphul app and middlewares.
pub struct Colors {
    // Black color.
    //
    // Optional. Default: "\u001b[90m"
    pub black: &'static str,

    // Red color.
    //
    // Optional. Default: "\u001b[91m"
    pub red: &'static str,

    // Green color.
    //
    // Optional. Default: "\u001b[92m"
    pub green: &'static str,

    // Yellow color.
    //
    // Optional. Default: "\u001b[93m"
    pub yellow: &'static str,

    // Blue color.
    //
    // Optional. Default: "\u001b[94m"
    pub blue: &'static str,

    // Magenta color.
    //
    // Optional. Default: "\u001b[95m"
    pub magenta: &'static str,

    // Cyan color.
    //
    // Optional. Default: "\u001b[96m"
    pub cyan: &'static str,

    // White color.
    //
    // Optional. Default: "\u001b[97m"
    pub white: &'static str,

    // Reset color.
    //
    // Optional. Default: "\u001b[0m"
    pub reset: &'static str,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            black: Default::default(),
            red: Default::default(),
            green: Default::default(),
            yellow: Default::default(),
            blue: Default::default(),
            magenta: Default::default(),
            cyan: Default::default(),
            white: Default::default(),
            reset: Default::default(),
        }
    }
}

// Default color codes
pub const DEFAULT_COLORS: Colors = Colors {
    black: "\x1b[90m",
    red: "\x1b[91m",
    green: "\x1b[92m",
    yellow: "\x1b[93m",
    blue: "\x1b[94m",
    magenta: "\x1b[95m",
    cyan: "\x1b[96m",
    white: "\x1b[97m",
    reset: "\x1b[0m",
};

// defaultColors is a function to override default colors to config
#[allow(dead_code)]
pub fn replace_default_colors(colors: &mut Colors) {
    if colors.black == "" {
        colors.black = DEFAULT_COLORS.black;
    }

    if colors.red == "" {
        colors.red = DEFAULT_COLORS.red;
    }

    if colors.green == "" {
        colors.green = DEFAULT_COLORS.green;
    }

    if colors.yellow == "" {
        colors.yellow = DEFAULT_COLORS.yellow;
    }

    if colors.blue == "" {
        colors.blue = DEFAULT_COLORS.blue;
    }

    if colors.magenta == "" {
        colors.magenta = DEFAULT_COLORS.magenta;
    }

    if colors.cyan == "" {
        colors.cyan = DEFAULT_COLORS.cyan;
    }

    if colors.white == "" {
        colors.white = DEFAULT_COLORS.white;
    }

    if colors.reset == "" {
        colors.reset = DEFAULT_COLORS.reset;
    }
}
