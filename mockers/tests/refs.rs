#![feature(plugin, custom_derive)]
#![plugin(mockers_macros)]

///! Test that mockers can mock methods with reference parameters.

extern crate mockers;

use mockers::Scenario;
use mockers::matchers::ANY;

#[derive(Mock)]
pub trait A {
    fn foo(&self, a: &u32);
}

#[test]
fn test_any_works_for_refs() {
    let scenario = Scenario::new();
    let mock = scenario.create_mock::<AMock>();

    scenario.expect(mock.foo_call(ANY).and_return_default().times(1));

    mock.foo(&3);
}

#[test]
fn test_refs_comparison() {
    let scenario = Scenario::new();
    let mock = scenario.create_mock::<AMock>();

    scenario.expect(mock.foo_call(&2).and_return_default().times(1));

    mock.foo(&2);
}
