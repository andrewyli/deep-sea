pub trait DataType: Clone + num::Zero + num::One + From<u16> + From<bool> {}
impl<T> DataType for T where T: Clone + num::Zero + num::One + From<u16> + From<bool> {}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnifiedIterator<T, U> {
    Opt1(T),
    Opt2(U),
}


impl<T, U, R> Iterator for UnifiedIterator<T, U>
where
    T: Iterator<Item = R>,
    U: Iterator<Item = R>,
{
    type Item = R;

    fn next(&mut self) -> Option<R> {
        match self {
            Self::Opt1(t) => t.next(),
            Self::Opt2(u) => u.next(),
        }
    }
}


pub trait Into1DArray {
    fn into_ndarray<T: DataType>(self) -> ndarray::Array1<T>;
}


pub trait Unpackable {
    fn unpack<T: DataType>(&self) -> impl Iterator<Item = T>;

    fn unpacked_size(&self) -> usize;
}


impl<U: Unpackable> Into1DArray for U {
    fn into_ndarray<T: DataType>(self) -> ndarray::Array1<T> {
        ndarray::Array1::<T>::from(self.unpack::<T>().collect::<Vec<T>>())
    }
}


impl<U: Unpackable> Unpackable for Vec<U> {
    fn unpack<T: DataType>(&self) -> impl Iterator<Item = T> {
        self.iter().flat_map(|x| x.unpack::<T>())
    }

    fn unpacked_size(&self) -> usize {
        self.iter().fold(0, |a, b| a + b.unpacked_size())
    }
}


impl<U: Unpackable> Unpackable for &[U] {
    fn unpack<T: DataType>(&self) -> impl Iterator<Item = T> {
        self.iter().flat_map(|x| x.unpack::<T>())
    }

    fn unpacked_size(&self) -> usize {
        self.iter().fold(0, |a, b| a + b.unpacked_size())
    }
}
