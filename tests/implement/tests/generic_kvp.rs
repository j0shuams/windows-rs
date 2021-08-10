use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;

#[implement(
    Windows::Foundation::Collections::IKeyValuePair<K,V>,
)]

// declare in Build so its binding is available
struct TestKVPair<K, V>((K, V))
where
    K: ::windows::RuntimeType + 'static,
    V: ::windows::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<K: ::windows::RuntimeType + 'static, V: ::windows::RuntimeType + 'static> TestKVPair<K, V> {
    fn Key(&self) -> Result<K> {
        let (key, _) = &self.0;
        Ok(key.clone())
    }

    fn Value(&self) -> Result<V> {
        let (_, value) = &self.0;
        Ok(value.clone())
    }
}

#[test]
fn test_implement() -> Result<()> {
    let kvp: IKeyValuePair<i32, i32> = TestKVPair((5, 120)).into();
    assert_eq!(kvp.Key()?, 5);
    assert_eq!(kvp.Value()?, 120);

    Ok(())
}
