use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;
// use Windows::Foundation::*;
// use Windows::Win32::Foundation::E_BOUNDS;
use std::collections::HashMap;

#[implement(
    Windows::Foundation::Collections::IMap<K,V>,
    Windows::Foundation::Collections::IIterable<Windows::Foundation::Collections::IKeyValuePair<K,V>>,
)]
struct TestMap<K, V>(HashMap<K, V>)
where
    K: ::windows::RuntimeType + 'static,
    V: ::windows::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<K, V> TestMap<K, V>
where
    K: ::windows::RuntimeType + 'static,
    V: ::windows::RuntimeType + 'static,
{
    fn Size(&self) -> Result<i32> {
        Ok(self.0.len() as _)
    }

    fn Clear(&mut self) -> Result<()> {
        Ok(self.0.clear())
    }

    fn GetView(&self) -> Result<() /*IReadOnlyDictionary<K, V>*/> {
        panic!();
    }

    // TODO: support for K deriving Hash and Eq
    fn HasKey(&self, key: &K::DefaultType) -> Result<bool> {
        Ok(self.0.contains_key(key))
    }

    // Returns true if the key existed and the old value was replaced by the value given here
    fn Insert(&self, key: &K::DefaultType, value: &V::DefaultType) -> Result<bool> {
        match self.0.insert(key, value) {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    fn Lookup(&self, key: &K::DefaultType) -> Result<V> {
        match self.0.get(key) {
            None => panic!("expected the map to have the given key"),
            Some(value) => Ok(value),
        }
    }

    fn Remove(&self, key: &K::DefaultType) -> Result<()> {
        self.0.remove(key);
        Ok(())
    }

    fn First(&self) -> Result<IIterator<IKeyValuePair<K, V>>> {
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (IMap<i32, bool>, IMap<i32, i32>) {
        let mut hash_map: HashMap<i32, bool> = HashMap::new();
        let empty_map: HashMap<i32, i32> = HashMap::new();
        hash_map.insert(0, true);
        hash_map.insert(1, false);
        hash_map.insert(2, true);
        hash_map.insert(3, false);
        return (TestMap(hash_map).into(), TestMap(empty_map).into());
    }

    #[test]
    fn test_size() {
        let (map, _) = setup();
        assert_eq!(4, map.Size().unwrap());
    }

    #[test]
    fn test_clear() {
        let (map, _) = setup();
        assert!(map.Clear().is_ok());
        assert_eq!(0, map.Size().unwrap());
    }

    #[test]
    fn test_get_view() {}

    #[test]
    fn test_has_key() {
        let (map, _) = setup();
        assert!(map.HasKey(0).unwrap());
        assert!(map.HasKey(1).unwrap());
        assert!(map.HasKey(2).unwrap());
        assert!(map.HasKey(3).unwrap());
        assert!(!map.HasKey(4).unwrap());
    }

    #[test]
    fn test_insert() {
        let (map, _) = setup();
        // Replaces the value for 0 (true -> false)
        assert!(map.Insert(0, false).unwrap());
        assert!(!map.Insert(4, true).unwrap());
        assert!(map.HasKey(4).unwrap());
    }

    #[test]
    fn test_lookup() {
        let (map, _) = setup();
        assert!(map.Lookup(0).unwrap());
        // Replaces the value for 0 (true -> false)
        assert!(map.Insert(0, false).unwrap());
        assert!(!map.Lookup(0).unwrap());
    }

    #[test]
    fn test_remove() {
        let (map, _) = setup();
    }

    #[test]
    fn test_first() {}

    #[test]
    fn test_map_iter() {}

    #[test]
    fn test_size_empty_map() {
        let (_, map) = setup();
        assert_eq!(0, map.Size().unwrap());
    }

    #[test]
    fn test_clear_empty_map() {
        let (_, map) = setup();
        assert!(map.Clear().is_ok());
        assert_eq!(0, map.Size().unwrap());
    }

    #[test]
    fn test_get_view_empty_map() {}

    #[test]
    fn test_has_key_empty_map() {
        let (_, map) = setup();
        assert!(!map.HasKey(0).unwrap());
    }

    #[test]
    fn test_insert_empty_map() {
        let (_, map) = setup();
        // Insert() returns true iff the insertion was a replacement (same key, new value)
        assert!(!map.Insert(-10, -55).unwrap());
    }

    #[test]
    fn test_empty_map() {
        let (_, map) = setup();
        assert_eq!(0, map.Size().unwrap());
        assert!(!map.HasKey(-10).unwrap());
        assert!(!map.Insert(-10, -55).unwrap());
        assert!(map.HasKey(-10).unwrap());
        assert!(!map.Insert(-11, -89).unwrap());
        assert_eq!(2, map.Size().unwrap());
        assert!(map.Remove(-10).is_ok());
        assert_eq!(1, map.Size().unwrap());
        assert_eq!(-89, map.Lookup(-11).unwrap());
        assert!(map.Remove(-11).is_ok());
        assert_eq!(0, map.Size().unwrap());
        assert!(!map.Insert(-10, -55).unwrap());
        assert!(map.Clear().is_ok());
        assert_eq!(0, map.Size().unwrap());
    }

    #[test]
    fn test_lookup_empty_map() {
        let (_, map) = setup();
        assert!(map.Insert(0, 38).is_ok());
        assert!(map.HasKey(0).unwrap());
        assert!(!map.HasKey(38).unwrap());
    }

    #[test]
    fn test_remove_empty_map() {
        let (_, map) = setup();
        assert!(map.Insert(0, 38).is_ok());
        assert_eq!(1, map.Size().unwrap());
        assert!(map.Remove(0).is_ok());
        assert_eq!(0, map.Size().unwrap());
        // removing again shouldn't be a problem
        assert!(map.Remove(0).is_ok());
    }

    #[test]
    fn test_first_empty_map() {}

    #[test]
    fn test_map_iter_empty_map() {}
}

/*
#[implement(
    Windows::Foundation::Collections::IIterator<IKeyValuePair<K,V>>,
)]
struct Iterator<Windows::Foundation::Collections::IKeyValuePair<K,V>>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
{
    owner: IIterable<IKeyValuePair<K,V>>,
    current: usize,
}

#[allow(non_snake_case)]
impl<K,V> Iterator<IKeyValuePair<K,V>>
where
    K: ::windows::RuntimeType + 'static,
    V: ::windows::RuntimeType + 'static,
{
    fn Current(&self) -> Result<IKeyValuePair<K,V>> {
        let owner = unsafe { TestMap::to_impl(&self.owner) };

        if owner.0.len() > self.current {
            Ok(owner.0[self.current].clone())
        } else {
            Err(Error::new(E_BOUNDS, ""))
        }
    }

    fn HasCurrent(&self) -> Result<bool> {
        let owner = unsafe { TestMap::to_impl(&self.owner) };
        Ok(owner.0.len() > self.current)
    }

    fn MoveNext(&mut self) -> Result<bool> {
        let owner = unsafe { TestMap::to_impl(&self.owner) };
        self.current += 1;
        Ok(owner.0.len() > self.current)
    }

    fn GetMany(&self, _items: &mut [IKeyValuePair<K, V>]) -> Result<u32> {
        panic!(); // TODO: arrays still need some work.
    }
}
*/
