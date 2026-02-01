//! Buffer wrappers for dataframe interchange.

use std::any::Any;
use std::mem::size_of;
use std::sync::Arc;

use arrow::bitmap::Bitmap;
use arrow::buffer::Buffer as ArrowBuffer;
use arrow::offset::OffsetsBuffer;
use arrow::types::NativeType;

use crate::collections::dataframe::interchange::protocol::{
    Buffer, DlpackDeviceType, InterchangeError,
};

#[derive(Clone)]
pub struct PolarsInterchangeBuffer {
    owner: Arc<dyn Any + Send + Sync>,
    ptr: *const u8,
    bufsize: usize,
    device: DlpackDeviceType,
}

// Safety: pointer is derived from `owner` and remains valid while `owner` is alive.
unsafe impl Send for PolarsInterchangeBuffer {}
unsafe impl Sync for PolarsInterchangeBuffer {}

impl PolarsInterchangeBuffer {
    pub fn from_buffer<T>(buffer: ArrowBuffer<T>) -> Result<Self, InterchangeError>
    where
        T: NativeType + 'static,
    {
        let owner = Arc::new(buffer);
        let slice = owner.as_slice();
        let ptr = slice.as_ptr() as *const u8;
        let bufsize = slice.len() * size_of::<T>();
        Ok(Self {
            owner,
            ptr,
            bufsize,
            device: DlpackDeviceType::CPU,
        })
    }

    pub fn from_offsets<O>(offsets: OffsetsBuffer<O>) -> Result<Self, InterchangeError>
    where
        O: arrow::offset::Offset + 'static,
    {
        Self::from_buffer(offsets.buffer().clone())
    }

    pub fn from_bitmap(bitmap: Bitmap) -> Result<Self, InterchangeError> {
        let owner = Arc::new(bitmap);
        let (bytes, _offset_bits, _len_bits) = owner.as_slice();
        let ptr = bytes.as_ptr();
        let bufsize = bytes.len();
        Ok(Self {
            owner,
            ptr,
            bufsize,
            device: DlpackDeviceType::CPU,
        })
    }

    pub fn as_buffer(&self) -> &Arc<dyn Any + Send + Sync> {
        &self.owner
    }
}

impl Buffer for PolarsInterchangeBuffer {
    fn bufsize(&self) -> usize {
        self.bufsize
    }

    fn ptr(&self) -> *const u8 {
        self.ptr
    }

    fn dlpack_device(&self) -> (DlpackDeviceType, Option<i32>) {
        (self.device, None)
    }
}
