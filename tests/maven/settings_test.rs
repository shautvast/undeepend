use undeepend::maven::settings::get_settings_from_string;

#[test]
fn test() {
    let settings = include_str!("../maven/resources/settings.xml").to_string();
    let settings = get_settings_from_string(settings).expect("no fail");
    assert!(!settings.profiles.is_empty());
}