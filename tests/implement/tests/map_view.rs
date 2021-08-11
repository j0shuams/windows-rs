use std::{collections::HashMap, fmt::Result};
use test_implement::{self::*, Windows::Foundation::Collections::IMapView};
use windows::*;

#[implement(
    Windows::Foundation::Collections::IMapView<K,V>,
)]
struct TestMapView<K,V>(HashMap<K::DefaultType, V::DefaultType>)
where 
    K: ::windows::RuntimeType + 'static,
    V: ::windows::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<K,V> TestMapView<K,V>
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
        Ok(true)
    }

    /// Lookup returns the item at the given key 
    fn Lookup(&self, _key: &K::DefaultType) -> Result<V::DefaultType> {
        panic!();
    }

    /// Split splits the map view into two views, storing them in the given mutable parameters
    fn Split(&self, _left_view: &mut IMapView<K,V>, _right_view: &mut IMapView<K,V>) -> Result<()> {
        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use test_implement::Windows::Foundation::Collections::IMapView;

    use super::*;

    // fn setup() -> (IMapView<i32,bool>, IMapView<bool,bool>) { }
}