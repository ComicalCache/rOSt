use x86_64::{
    instructions::port::{PortGeneric, PortReadAccess, PortWriteAccess},
    structures::port::{PortRead, PortWrite},
};

pub trait PortExtRead<T: PortRead> {
    unsafe fn read_to_buffer(&mut self, buffer: &mut [T]);
}

pub trait PortExtWrite<T: PortWrite> {
    unsafe fn write_from_buffer(&mut self, buffer: &[T]);
}

impl<T: PortRead, A: PortReadAccess> PortExtRead<T> for PortGeneric<T, A> {
    /// Reads a given number of values from the port and into a buffer.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    #[inline]
    unsafe fn read_to_buffer(self: &mut PortGeneric<T, A>, buffer: &mut [T]) {
        for data in buffer {
            *data = self.read();
        }
    }
}

impl<T: PortWrite + Copy, A: PortWriteAccess> PortExtWrite<T> for PortGeneric<T, A> {
    /// Writes a buffer to the port.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the I/O port could have side effects that violate memory
    /// safety.
    #[inline]
    unsafe fn write_from_buffer(self: &mut PortGeneric<T, A>, buffer: &[T]) {
        for data in buffer {
            self.write(data.clone());
        }
    }
}
