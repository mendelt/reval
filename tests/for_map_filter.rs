use crate::common::eval_expr;

#[tokio::test]
async fn should_evaluate_for_map() {
    assert_eq!(
        eval_expr("for item in [i1, i2, i3, i4] map item + i3", ()).await,
        vec![4, 5, 6, 7].into()
    );
}

#[tokio::test]
async fn should_evaluate_for_filter() {
    assert_eq!(
        eval_expr("for item in [i1, i2, i3, i4] filter item > i2", ()).await,
        vec![3, 4].into()
    );
}
