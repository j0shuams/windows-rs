use std::vec;
use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;
use Windows::Win32::Foundation::E_BOUNDS;

#[implement(
    Windows::Foundation::Collections::IIterator<T>,
)]
struct Iterator<T>
where
    T: RuntimeType + 'static,
{
    owner: IIterable<T>,
    current: usize,
}

#[allow(non_snake_case)]
impl<T> Iterator<T>
where
    T: ::windows::RuntimeType + 'static,
{
    fn Current(&self) -> Result<T> {
        let owner = unsafe { TestView::to_impl(&self.owner) };

        if owner.0.len() > self.current {
            Ok(owner.0[self.current].clone())
        } else {
            Err(Error::new(E_BOUNDS, ""))
        }
    }

    fn HasCurrent(&self) -> Result<bool> {
        let owner = unsafe { TestView::to_impl(&self.owner) };
        Ok(owner.0.len() > self.current)
    }

    fn MoveNext(&mut self) -> Result<bool> {
        let owner = unsafe { TestView::to_impl(&self.owner) };
        self.current += 1;
        Ok(owner.0.len() > self.current)
    }

    fn GetMany(&self, _items: &mut [<T as Abi>::DefaultType]) -> Result<u32> {
        panic!(); // TODO: arrays still need some work.
    }
}

#[implement(
    Windows::Foundation::Collections::IVectorView<T>,
    Windows::Foundation::Collections::IIterable<T>,
)]
struct TestView<T>(Vec<T>)
where
    T: ::windows::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<T> TestView<T>
where
    T: ::windows::RuntimeType + 'static,
{
    /* GetAt returns the value at the given index, returning error E_BOUNDS if the given index is out of bounds */
    fn GetAt(&self, index: u32) -> Result<T> {
        let vec_size = self.0.len() as u32;
        if index < vec_size {
            self.0
                .get(index as usize)
                .cloned()
                .ok_or_else(|| Error::new(E_BOUNDS, ""))
        } else {
            Err(Error::new(
                E_BOUNDS,
                format!(
                    "Given index ({}) was out of bounds for the IVectorView (length {})",
                    index, vec_size
                )
                .as_str(),
            ))
        }
    }

    /* Size returns the length of the underlying vector */
    fn Size(&self) -> Result<u32> {
        Ok(self.0.len() as _)
    }

    /* IndexOf returns true if the given value is in the IVectorView, and false otherwise.
    If the given value is found, then its index is written to the `index` parameter */
    fn IndexOf(&self, _value: &T::DefaultType, index: &mut u32) -> Result<bool> {
        // note: DefaultType covers nullable types; expand impl block to have constraints on when T is an Option and not
        let vec_size = self.0.len();
        for i in 0..vec_size {
            let _vec_element = self.0.get(i).unwrap();

            if false
            /* *vec_element == *value */
            {
                *index = i as _;
                return Ok(true);
            }
        }
        return Ok(false);
    }

    /* GetMany retrieves multiple items, storing them in `items`, and returns the amount of items retrieved  */
    fn GetMany(&self, start_index: u32, items: &mut [T]) -> Result<u32> {
        let vec_size: u32 = self.0.len() as u32;

        // (?) assert that the incoming `items` array is sufficiently long enough (?)

        if start_index < vec_size {
            let mut get_many_size: u32 = 0;
            for n in start_index..vec_size {
                let idx = n as usize;
                items[idx] = self.0.get(idx).unwrap().clone();
                get_many_size += 1;
            }
            Ok(get_many_size)
        } else {
            Err(Error::new(
                E_BOUNDS,
                format!(
                    "Start index ({}) was out of bounds for the IVectorView (length {})",
                    start_index, vec_size
                )
                .as_str(),
            ))
        }
    }

    /* Returns an IIterator<T> */
    fn First(&mut self) -> Result<IIterator<T>> {
        Ok(Iterator::<T> {
            owner: self.into(),
            current: 0,
        }
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
        Tests using an int vector (size 3)
    */

    #[test]
    fn test_current_iter_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();
        let iter: IIterator<i32> = three_vec.First().unwrap();
        assert_eq!(5, iter.Current().unwrap());
    }

    #[test]
    fn test_has_current_iter_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();
        let iter: IIterator<i32> = three_vec.First().unwrap();
        assert!(iter.HasCurrent().unwrap());
    }

    #[test]
    fn test_iter_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 6, 7]).into();
        let iter: IIterator<i32> = three_vec.First().unwrap();
        let mut i = 0;

        while iter.HasCurrent().unwrap() {
            let curr = iter.Current().unwrap();
            assert_eq!(curr - i, 5, "We have the correct element {}", curr);
            assert!(iter.MoveNext().is_ok());
            i += 1;
        }
        assert_eq!(i, 3, "We got through the entire vector {}", i);
    }

    #[test]
    fn test_get_at_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();
        assert_eq!(5, three_vec.GetAt(0).unwrap());
        assert_eq!(120, three_vec.GetAt(1).unwrap());
        assert_eq!(625, three_vec.GetAt(2).unwrap());
    }

    #[test]
    fn test_size_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();
        assert_eq!(3, three_vec.Size().unwrap());
    }

    #[test]
    #[ignore = "Waiting on helper trait for DefaultType"]
    fn test_index_of_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();

        let mut idx = 0;

        let index_of_five = three_vec.IndexOf(5, &mut idx).unwrap();
        assert!(index_of_five);
        assert_eq!(0, idx);
    }

    #[test]
    #[ignore = "Waiting on array support"]
    fn test_get_many_three_vec() {
        let three_vec: IVectorView<i32> = TestView(vec![5, 120, 625]).into();

        // add a test where the items array (here, `arr`) has size 0
        let mut arr = [0; 3];
        three_vec.GetMany(1, &mut arr).unwrap();
        assert_eq!(120, arr[0]);
        assert_eq!(625, arr[1]);
    }

    /*
        Tests using an empty vector
    */

    #[test]
    fn test_get_at_empty_vec() {
        let empty_vec: IVectorView<bool> = TestView(vec![]).into();
        empty_vec
            .GetAt(0)
            .expect_err("Given index (0) was out of bounds for the IVectorView (length 0)");
    }

    #[test]
    fn test_size_empty_vec() {
        let empty_vec: IVectorView<bool> = TestView(vec![]).into();
        assert_eq!(0, empty_vec.Size().unwrap());
    }

    #[test]
    fn test_index_of_empty_vec() {
        let empty_vec: IVectorView<bool> = TestView(vec![]).into();

        let mut idx = 0;

        let index_of_anything = empty_vec.IndexOf(true, &mut idx).unwrap();
        assert!(!index_of_anything);
    }

    #[test]
    fn test_current_iter_empty_vec() {
        let empty_vec: IVectorView<bool> = TestView(vec![]).into();
        let iter: IIterator<bool> = empty_vec.First().unwrap();
        iter.Current().expect_err("");
    }

    #[test]
    fn test_has_current_iter_empty_vec() {
        let empty_vec: IVectorView<bool> = TestView(vec![]).into();
        let iter: IIterator<bool> = empty_vec.First().unwrap();
        assert!(!iter.HasCurrent().unwrap());
    }

    #[test]
    fn test_iter_empty_vec() {
        let empty_vec: IVectorView<bool> = TestView(vec![]).into();
        let iter: IIterator<bool> = empty_vec.First().unwrap();
        let mut i = 0;

        while iter.HasCurrent().unwrap() {
            let _curr = iter.Current().unwrap();
            assert!(iter.MoveNext().is_ok());
            i += 1;
        }
        assert_eq!(i, 0, "We got through the entire vector {}", i);
    }

    /* todo: test using Vectors of different types (e.g. HSTRING, Uri, ...) */
}
