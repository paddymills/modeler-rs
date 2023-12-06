
use crate::prelude::*;


pub fn empty_buffer(display: &Display) -> VertexBuffer {
    VertexBuffer::new(display, &[]).unwrap()
}
