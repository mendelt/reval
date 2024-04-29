use crate::common::eval_expr;

#[tokio::test]
async fn should_evaluate_nested_if() {
    assert_eq!(
        eval_expr("if if true then true else false then i1 else i2", ()).await,
        1.into()
    );
}

#[tokio::test]
async fn should_evaluate_nested_then() {
    assert_eq!(
        eval_expr("if true then if false then i1 else i2 else i3", ()).await,
        2.into()
    );
}

#[tokio::test]
async fn should_evaluate_nested_else() {
    assert_eq!(
        eval_expr("if false then i1 else if false then i2 else i3", ()).await,
        3.into()
    );
}
