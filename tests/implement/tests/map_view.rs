use std::collections::BTreeMap;
use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;
use Windows::Foundation::*;
use Windows::Win32::Foundation::E_BOUNDS;

#[implement(
    Windows::Foundation::Collections::IMapView<K,V>,
)]
struct TestMapView<K, V>(BTreeMap<K::DefaultType, V::DefaultType>)
where
    K: ::windows::RuntimeType + 'static,
    V: ::windows::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<K, V> TestMapView<K, V>
where
    K: ::windows::RuntimeType + 'static,
    V: ::windows::RuntimeType + 'static,
{
    /// Size returns the number of elements in the map
    fn Size(&self) -> Result<u32> {
        Ok(self.0.len() as _)
    }

    /// HasKey returns true if the map has the given key, and false if the key is not found
    fn HasKey(&self, key: &K::DefaultType) -> Result<bool> {
        Ok(self.0.contains_key(key))
    }

    /// Lookup returns the item at the given key; if the key is not found an error is emitted to use HasKey
    fn Lookup(&self, _key: &K::DefaultType) -> Result<V::DefaultType> {
        match self.0.get(_key) {
            Some(value) => <V as Abi>::ok(value),
            None => Err(Error::new(E_BOUNDS, "IMapView.Lookup: Given key was not found, use IMapView.HasKey before calling IMapView.Lookup"))
        }
    }

    /// Split splits the map view into two views, storing them in the given mutable parameters
    fn Split(
        &self,
        _left_view: &mut IMapView<K, V>,
        _right_view: &mut IMapView<K, V>,
    ) -> Result<()> {
        let mid = (self.0.len() / 2).floor();
        let left_view = self.0.range_mut(range(min, mid));
        let right_view = self.0.range_mut(range(mid, max));
        _left_view.clone_from(left_view).into();
        _right_view.clone_from(right_view).into();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup() -> (
        IMapView<bool, bool>,
        IMapView<i32, bool>,
        IMapView<IStringable, HSTRING>,
    ) {
        let empty_view: IMapView<bool, bool> = BTreeMap::new();
        let mut three_view: IMapView<i32, bool> = BTreeMap::new();
        let mut uri_view: IMapView<IStringable, HSTRING> = BTreeMap::new();

        three_view.insert(0, true);
        three_view.insert(1, false);
        three_view.insert(2, true);

        uri_view.insert(Uri::CreateUri("https://one")?.into(), "Ana".into());
        uri_view.insert(Uri::CreateUri("https://two")?.into(), "Bob".into());
        uri_view.insert(Uri::CreateUri("https://three")?.into(), "Cai".into());
        uri_view.insert(Uri::CreateUri("https://four")?.into(), "Dee".into());

        return (
            TestMapView(empty_view).into(),
            TestMapView(three_view).into(),
            TestMapView(uri_view).into(),
        );
    }

    #[test]
    fn test_size() {
        let (empty_view, three_view, uri_view) = setup();
        assert_eq!(0, empty_view.Size().unwrap());
        assert_eq!(3, three_view.Size().unwrap());
        assert_eq!(4, uri_view.Size().unwrap());
    }

    #[test]
    fn test_has_key() {
        let (empty_view, three_view, uri_view) = setup();

        // Check for keys that DONT exist in the map
        assert!(!empty_view.HasKey(true).unwrap(), "empty_view has no keys");
        assert!(
            !three_view.HasKey(4).unwrap(),
            "three_view doesn't have key = 4"
        );
        assert!(
            !uri_view
                .HasKey(Uri::CreateUri("https://five")?.into())
                .unwrap(),
            "uri_view doesn't have key = https://five"
        );

        // Check for keys that DO exist in the map
        assert!(three_view.HasKey(0).unwrap(), "three_view has key = 0");
        assert!(three_view.HasKey(1).unwrap(), "three_view has key = 1");
        assert!(three_view.HasKey(2).unwrap(), "three_view has key = 2");

        assert!(
            uri_view
                .HasKey(Uri::CreateUri("https://one")?.into())
                .unwrap(),
            "uri_view has key = https://one"
        );
        assert!(
            uri_view
                .HasKey(Uri::CreateUri("https://two")?.into())
                .unwrap(),
            "uri_view has key = https://two"
        );
        assert!(
            uri_view
                .HasKey(Uri::CreateUri("https://three")?.into())
                .unwrap(),
            "uri_view has key = https://three"
        );
        assert!(
            uri_view
                .HasKey(Uri::CreateUri("https://four")?.into())
                .unwrap(),
            "uri_view has key = https://four"
        );
    }

    #[test]
    fn test_lookup() {
        let (empty_view, three_view, uri_view) = setup();
        assert_eq!(
            true,
            three_view.Lookup(0).unwrap(),
            "three_view has 0 -> true"
        );
        assert_eq!(
            false,
            three_view.Lookup(1).unwrap(),
            "three_view has 1 -> false"
        );
        assert_eq!(
            true,
            three_view.Lookup(2).unwrap(),
            "three_view has 2 -> true"
        );
        assert_eq!(
            "Ana".into(),
            uri_view
                .Lookup(Uri::CreateUri("https://one")?.into())
                .unwrap(),
            "uri_view has key = https://one"
        );
        assert_eq!(
            "Bob".into(),
            uri_view
                .Lookup(Uri::CreateUri("https://two")?.into())
                .unwrap(),
            "uri_view has key = https://two"
        );
        assert_eq!(
            "Cai".into(),
            uri_view
                .Lookup(Uri::CreateUri("https://three")?.into())
                .unwrap(),
            "uri_view has key = https://three"
        );
        assert_eq!(
            "Dee".into(),
            uri_view
                .Lookup(Uri::CreateUri("https://four")?.into())
                .unwrap(),
            "uri_view has key = https://four"
        );
    }

    #[test]
    #[ignore = "unimplemented"]
    fn test_split() {
        let (empty_view, three_view, uri_view) = setup();
    }
}
