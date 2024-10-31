use engine::executor::Executor;

#[test]
fn test_feature() {
    let executor = Executor::new("tests/_test_data/test.feature");
    executor.run()
}
