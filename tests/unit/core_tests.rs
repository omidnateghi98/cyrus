//! Unit tests for core functionality

use cyrus::core::CyrusCore;

#[test]
fn test_cyrus_core_initialization() {
    let core = CyrusCore::new().unwrap();
    assert!(core.cyrus_dir.exists());
    assert!(core.config_dir.exists());
    assert!(core.languages_dir.exists());
}

#[test]
fn test_language_path_generation() {
    let core = CyrusCore::new().unwrap();
    let path = core.language_path("python", "3.11");
    assert!(path.ends_with(".cyrus/languages/python/3.11"));
}
