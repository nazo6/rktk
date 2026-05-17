use smart_default::SmartDefault;

#[macro_rules_attribute::apply(crate::schema::common_derive)]
#[derive(SmartDefault)]
#[serde(default)]
pub struct MagneticConfig {
    /// Default press distance (normalized 0-65535)
    #[default(1000)]
    pub press_dist: u16,
    /// Default release distance (normalized 0-65535)
    #[default(1000)]
    pub release_dist: u16,
}
