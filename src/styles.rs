pub mod color {
    // The color palette that I used as a starting point
    // https://coolors.co/palette/001219-005f73-0a9396-94d2bd-e9d8a6-ee9b00-ca6702-bb3e03-ae2012-9b2226

    use bevy::prelude::Color;

    pub const PRIMARY: Color = Color::Rgba {
        red: 0.0,
        green: 0.37254901960784315,
        blue: 0.45098039215686275,
        alpha: 1.0,
    };

    pub const SECONDARY: Color = Color::Rgba {
        red: 0.03921568627,
        green: 0.5764705882352941,
        blue: 0.5882352941176471,
        alpha: 1.0,
    };

    pub const ACCENT: Color = Color::Rgba {
        red: 0.9333333333333333,
        green: 0.6078431372549019,
        blue: 0.0,
        alpha: 1.0,
    };

    pub const BACKGROUND: Color = Color::Rgba {
        red: 0.0,
        green: 0.07058823529411765,
        blue: 0.09803921568627451,
        alpha: 1.0,
    };

    pub const TEXT: Color = Color::Rgba {
        red: 0.9137254901960784,
        green: 0.8470588235294118,
        blue: 0.6509803921568628,
        alpha: 1.0,
    };

    pub const HOVER: Color = Color::Rgba {
        red: 0.5803921568627451,
        green: 0.8235294117647058,
        blue: 0.7411764705882353,
        alpha: 1.0,
    };

    pub const FOCUS: Color = Color::Rgba {
        red: 0.6078431372549019,
        green: 0.13333333333333333,
        blue: 0.14901960784313725,
        alpha: 1.0,
    };

    pub const ERROR: Color = Color::Rgba {
        red: 0.6823529411764706,
        green: 0.12549019607843137,
        blue: 0.07058823529411765,
        alpha: 1.0,
    };

    pub const WARNING: Color = Color::Rgba {
        red: 0.792156862745098,
        green: 0.403921568627451,
        blue: 0.00784313725490196,
        alpha: 1.0,
    };

    pub const DISABLED: Color = Color::Rgba {
        red: 0.7333333333333333,
        green: 0.24313725490196078,
        blue: 0.011764705882352941,
        alpha: 1.0,
    };
}
