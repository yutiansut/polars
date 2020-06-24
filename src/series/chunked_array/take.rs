use crate::prelude::*;
use crate::series::chunked_array::builder::{PrimitiveChunkedBuilder, Utf8ChunkedBuilder};
use arrow::{array::PrimitiveArrayOps, compute::TakeOptions, datatypes::ArrowPrimitiveType};

pub trait Take {
    fn take(
        &self,
        indices: impl Iterator<Item = Option<usize>>,
        options: Option<TakeOptions>,
        capacity: Option<usize>,
    ) -> Result<Self>
    where
        Self: std::marker::Sized;
}

macro_rules! impl_take_builder {
    ($self:ident, $indices:ident, $builder:ident, $chunks:ident) => {{
        for opt_idx in $indices {
            match opt_idx {
                Some(idx) => {
                    let (chunk_idx, i) = $self.index_to_chunked_index(idx);
                    let arr = $chunks[chunk_idx];
                    $builder.append_value(arr.value(i))?
                }
                None => $builder.append_null()?,
            }
        }
        Ok($builder.finish())
    }};
}

impl<T> Take for ChunkedArray<T>
where
    T: ArrowPrimitiveType,
{
    fn take(
        &self,
        indices: impl Iterator<Item = Option<usize>>,
        _options: Option<TakeOptions>,
        capacity: Option<usize>,
    ) -> Result<Self> {
        // TODO: implement takeoptions
        let capacity = capacity.unwrap_or(1024);
        let mut builder = PrimitiveChunkedBuilder::new(self.name(), capacity);
        let chunks = self.downcast_chunks();
        impl_take_builder!(self, indices, builder, chunks)
    }
}

impl Take for Utf8Chunked {
    fn take(
        &self,
        indices: impl Iterator<Item = Option<usize>>,
        _options: Option<TakeOptions>,
        capacity: Option<usize>,
    ) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        // TODO: implement takeoptions
        let capacity = capacity.unwrap_or(1024);
        let mut builder = Utf8ChunkedBuilder::new(self.name(), capacity);
        let chunks = self.downcast_chunks();
        impl_take_builder!(self, indices, builder, chunks)
    }
}

pub trait TakeIndex {
    fn as_take_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Option<usize>> + 'a>;

    fn take_index_len(&self) -> usize;
}

impl TakeIndex for UInt32Chunked {
    fn as_take_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Option<usize>> + 'a> {
        Box::new(self.iter().map(|opt_val| opt_val.map(|val| val as usize)))
    }
    fn take_index_len(&self) -> usize {
        self.len()
    }
}

impl TakeIndex for [usize] {
    fn as_take_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Option<usize>> + 'a> {
        Box::new(self.iter().map(|&v| Some(v)))
    }
    fn take_index_len(&self) -> usize {
        self.len()
    }
}

impl TakeIndex for Vec<usize> {
    fn as_take_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Option<usize>> + 'a> {
        Box::new(self.iter().map(|&v| Some(v)))
    }
    fn take_index_len(&self) -> usize {
        self.len()
    }
}

impl TakeIndex for [u32] {
    fn as_take_iter<'a>(&'a self) -> Box<dyn Iterator<Item = Option<usize>> + 'a> {
        Box::new(self.iter().map(|&v| Some(v as usize)))
    }
    fn take_index_len(&self) -> usize {
        self.len()
    }
}