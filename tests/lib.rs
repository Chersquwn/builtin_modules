use builtin_modules::BUILTIN_MODULES;

#[test]
fn it_should_get_collectly() {
    assert!(BUILTIN_MODULES.len() == 69);
    assert!(BUILTIN_MODULES.contains("path"));
    assert!(BUILTIN_MODULES.contains("fs"));
}
