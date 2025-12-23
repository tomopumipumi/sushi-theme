use zed_extension_api as zed;

struct SushiTheme;
impl zed::Extension for SushiTheme {
    fn new() -> Self {
        SushiTheme
    }
}

zed::register_extension!(SushiTheme);
