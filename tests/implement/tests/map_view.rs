use std::collections::HashMap;
use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;

#[implement(
    Windows::Foundation::Collections::IMapView<K,V>,
)]
struct TestMapView<K, V>(HashMap<K::DefaultType, V::DefaultType>)
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

    /// HasKey returns true iff the map has the given key
    fn HasKey(&self, _key: &K::DefaultType) -> Result<bool> {
        Ok(self.0.contains_key(_key))
    }

    /// Lookup returns the item at the given key
    fn Lookup(&self, _key: &K::DefaultType) -> Result<V> {
        match self.0.get(_key) {
            None => panic!(),
            Some(value) => panic!(),
        }
    }

    /// Split splits the map view into two views, storing them in the given mutable parameters
    fn Split(
        &self,
        _left_view: &mut IMapView<K, V>,
        _right_view: &mut IMapView<K, V>,
    ) -> Result<()> {
        Ok(())
    }

    fn Idk() -> Result<()> {
        let mut alpha = HashMap::new();
        alpha.insert("a", 1);
        alpha.insert("b", 2);
        alpha.insert("c", 3);
        alpha.insert("d", 4);
        let iter = alpha.iter();
        let bound = iter.len();
        let mut flag = 0;
        let (left, right) = iter.partition(|x| {
            let hmm = !(flag >= (bound / 2));
            flag += 1;
            return hmm;
        });
        println!(left);
        println!(right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn setup() -> (IMapView<i32,bool>, IMapView<bool,bool>) { }
}
