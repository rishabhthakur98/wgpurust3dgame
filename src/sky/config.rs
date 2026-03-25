// Angles in degrees
pub const SUN_AZIMUTH: f32 = 135.0;   // Left/Right rotation (Compass direction)
pub const SUN_ELEVATION: f32 = 45.0;  // Up/Down angle (0 = Horizon, 90 = High Noon)

// Sun Light configuration
pub const SUN_COLOR: [f32; 3] = [1.0, 0.95, 0.9];
pub const SUN_INTENSITY_DAY: f32 = 1.5;

// Skybox visual colors
pub const SKY_ZENITH_DAY: [f32; 3] = [0.15, 0.35, 0.75]; // Deep blue top
pub const SKY_HORIZON_DAY: [f32; 3] = [0.6, 0.8, 0.9];   // Light blue horizon
pub const SKY_NIGHT: [f32; 3] = [0.02, 0.02, 0.05];      // Dark night sky