use builtin_modules::BUILTIN_MODULES;

#[test]
fn it_should_get_collectly() {
    assert!(BUILTIN_MODULES.len() == 69);
    assert!(BUILTIN_MODULES.contains("path"));
    assert!(BUILTIN_MODULES.contains("fs"));
}

#[test]
fn it_should_work_is_builtin_module() {
    assert!(builtin_modules::is_builtin_module("path"));
    assert!(builtin_modules::is_builtin_module("fs/promises"));
    assert!(!builtin_modules::is_builtin_module("abc"));
}
