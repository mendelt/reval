use crate::common::eval_expr;

#[tokio::test]
async fn should_evaluate_for_map() {
    assert_eq!(
        eval_expr("for item in [i1, i2, i3, i4] map item + i3", ()).await,
        vec![4, 5, 6, 7].into()
    );
}

#[tokio::test]
async fn should_evaluate_nested_for_map() {
    assert_eq!(
        eval_expr(
            "for item in for item in [i1, i2, i3, i4] map item + i2 map item + i3",
            ()
        )
        .await,
        vec![6, 7, 8, 9].into()
    );
}

#[tokio::test]
async fn should_evaluate_for_filter() {
    assert_eq!(
        eval_expr("for item in [i1, i2, i3, i4] filter item > i2", ()).await,
        vec![3, 4].into()
    );
}

#[tokio::test]
async fn should_evaluate_nested_for_filter() {
    assert_eq!(
        eval_expr(
            "for item in for item in [i1, i2, i3, i4] filter item > i2 filter item < i4",
            ()
        )
        .await,
        vec![3].into()
    );
}

#[tokio::test]
async fn should_evaluate_any() {
    assert_eq!(
        eval_expr("any([false, false, true, false])", ()).await,
        true.into()
    );
    assert_eq!(
        eval_expr("any([false, false, false])", ()).await,
        false.into()
    );
}

#[tokio::test]
async fn should_evaluate_all() {
    assert_eq!(eval_expr("all([true, true, true])", ()).await, true.into());
    assert_eq!(
        eval_expr("all([true, false, true])", ()).await,
        false.into()
    );
}

#[tokio::test]
async fn should_evaluate_all_with_for_map() {
    assert_eq!(
        eval_expr("all(for item in [i1, i2, i3, i4] map item > i0)", ()).await,
        true.into()
    );
    assert_eq!(
        eval_expr("all(for item in [i1, i2, i3, i4] map item < i4)", ()).await,
        false.into()
    );
}

#[tokio::test]
async fn should_evaluate_any_with_for_map() {
    assert_eq!(
        eval_expr("any(for item in [i1, i2, i3, i4] map item == i2)", ()).await,
        true.into()
    );
    assert_eq!(
        eval_expr("any(for item in [i1, i2, i3, i4] map item > i4)", ()).await,
        false.into()
    );
}
