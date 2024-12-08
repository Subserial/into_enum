#![allow(dead_code)]

use derive_into_enum::IntoEnum;

#[test]
fn compile_from() -> Result<(), String> {
    #[derive(Debug, IntoEnum, PartialEq)]
    enum TestEnum {
        NoValue,
        OneValue(&'static str),
        TwoValues(&'static str, &'static str),
        IgnoredNamed {
            one: &'static str,
            two: u8,
        },
        #[into_enum(skip)]
        SkippedDuplicate(&'static str),
    }
    assert_eq!(
        TestEnum::from(()),
        TestEnum::NoValue,
        "wrong conversion result for ()"
    );
    assert_eq!(
        TestEnum::from("1"),
        TestEnum::OneValue("1"),
        "wrong conversion result for OneValue"
    );
    assert_eq!(
        TestEnum::from(("1", "2")),
        TestEnum::TwoValues("1", "2"),
        "wrong conversion result for TwoValues"
    );
    Ok(())
}

#[test]
fn generic() -> Result<(), String> {
    struct NoTraits;
    struct ImplOne;
    struct ImplTwo;

    trait ExclusiveTrait {}
    impl ExclusiveTrait for ImplOne {}
    impl ExclusiveTrait for ImplTwo {}

    #[derive(IntoEnum)]
    enum TestGeneric<T: ExclusiveTrait> {
        NoValue,
        Generic(T),
        NonGeneric(NoTraits),
    }
    let _ = TestGeneric::from(ImplOne);
    let _ = TestGeneric::from(ImplTwo);
    let _ = TestGeneric::<ImplOne>::from(NoTraits);
    let _ = TestGeneric::<ImplTwo>::from(NoTraits);
    let _ = TestGeneric::<ImplOne>::from(());
    let _ = TestGeneric::<ImplTwo>::from(());
    Ok(())
}
