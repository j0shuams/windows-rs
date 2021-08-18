use std::vec;
use test_implement::*;
use windows::*;
use Windows::Foundation::Collections::*;
use Windows::Win32::Foundation::E_BOUNDS;

#[implement(
    Windows::Foundation::Collections::IVector<A>,
)]
struct IList<A>(Vec<A::DefaultType>)
where
    A: ::windows::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<A> IList<A>
where
    A: ::windows::RuntimeType + 'static,
{
    /// Gets the number of items in the vector
    fn Size(&self) -> Result<u32> {
        Ok(self.0.len() as _)
    }

    /// Adds an item to the end of the vector
    fn Append(&self, value: A::DefaultType) -> Result<()> {
        Ok(self.0.push(value))
    }

    /// Removes all items from the vector
    fn Clear(&self) -> Result<()> {
        Ok(self.0.clear())
    }

    /// Returns the item at the specified index in the vector
    fn GetAt(&self, index: u32) -> Result<A> {
        match self.0.get(index as usize) {
            Some(value) => <A as Abi>::ok(value),
            None => Err(Error::new(
                E_BOUNDS,
                format!(
                    "GetAt: Given index ({}) was out of bounds for type IVector",
                    index
                )
                .as_str(),
            )),
        }
    }

    /// Retrieves multiple items from the vector beginning at the given index
    fn GetMany(&self, start_index: u32, items: &mut [A]) -> Result<u32> {
        let vec_size: u32 = self.0.len() as u32;
        // let iter = self.0.iter();

        // (?) assert that the incoming `items` array is sufficiently long enough (?)
        if start_index < vec_size {
            let mut get_many_size: u32 = 0;
            for n in start_index..vec_size {
                items[get_many_size as usize] = self.GetAt(n).unwrap(); // iter.nth(n).unwrap();
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

    /// Returns an immutable view of the vector
    fn GetView(&self) -> Result<IVectorView<A>> {
        Ok(IVectorView(self.clone()).into())
    }

    /// Retrieves the index of a specified item in the vector
    fn IndexOf(&self, value: &A::DefaultType, result: &mut u32) -> Result<bool> {
        match self.0.iter().position(|element| element == value) {
            Some(index) => {
                *result = index as _;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Inserts an item at a specified index in the vector
    fn InsertAt(&self, index: u32, value: A::DefaultType) -> Result<()> {
        Ok(self.0.insert(index as usize, value))
    }

    /// Removes the item at the specified index in the vector
    fn RemoveAt(&self, position: u32) -> Result<()> {
        self.0.remove(position as usize);
        Ok(())
    }

    /// Removes the last item from the vector
    fn RemoveAtEnd(&self) -> Result<()> {
        self.0.pop();
        Ok(())
    }

    /// Replaces all the items in the vector with the specified items
    fn ReplaceAll(&self, items: &[A::DefaultType]) -> Result<()> {

        if items.len() < self.0.len() {
            Err(Error::new(
                E_BOUNDS,
                "Not enough given items to replace all the vector's items",
            ));
        }

        for n in 0..self.0.len() {
            self.SetAt(n as u32, items[n]);
        }

        for n in self.0.len()..items.len() {
            self.0.insert(n as _, items[n]);
        }

        Ok(())
    }

    /// Sets the value at the specified index in the vector
    fn SetAt(&self, index: u32, value: A::DefaultType) -> Result<()> {
        self.0.remove(index as _);
        self.0.insert(index as _, value);
        Ok(())
    }
}
