use core::fmt;
use core::mem::uninitialized;
use smoltcp::{
    socket::{SocketHandle, TcpSocket, TcpSocketBuffer},
    time::Instant,
};

use crate::net::NetInterface;

const TCP_PORT: u16 = 23;
const SOCKET_COUNT: usize = 4;
const SOCKET_BUFFER_SIZE: usize = 2048;
const SOCKET_BUFFERS_LENGTH: usize = 2 * SOCKET_COUNT * SOCKET_BUFFER_SIZE;

static mut SOCKET_BUFFERS: [u8; SOCKET_BUFFERS_LENGTH] = [0u8; SOCKET_BUFFERS_LENGTH];

fn get_socket_buffers(i: usize) -> (&'static mut [u8], &'static mut [u8]) {
    let offset1 = 2 * i * SOCKET_BUFFER_SIZE;
    let offset2 = offset1 + SOCKET_BUFFER_SIZE;
    let offset3 = offset2 + SOCKET_BUFFER_SIZE;
    unsafe {
        (&mut SOCKET_BUFFERS[offset1..offset2],
         &mut SOCKET_BUFFERS[offset2..offset3])
    }
}

/// Contains a number of server sockets that get all sent the same
/// data (through `fmt::Write`).
pub struct Server<'a, 's> {
    handles: [SocketHandle; SOCKET_COUNT],
    net: &'s mut NetInterface<'a>,
}

impl<'a, 's> Server<'a, 's> {
    pub fn new(net: &'s mut NetInterface<'a>) -> Self {
        let mut server = Server {
            handles: unsafe { uninitialized() },
            net,
        };

        for i in 0..SOCKET_COUNT {
            let buffers = get_socket_buffers(i);
            let server_socket = TcpSocket::new(
                TcpSocketBuffer::new(&mut buffers.0[..]),
                TcpSocketBuffer::new(&mut buffers.1[..])
            );
            server.handles[i] = server.net.sockets().add(server_socket);
        }

        server
    }

    pub fn poll(&mut self, now: Instant) {
        let activity = self.net.poll(now);
        if ! activity {
            return;
        }

        for handle in &self.handles {
            let mut socket = self.net.sockets().get::<TcpSocket>(*handle);
            if ! socket.is_open() {
                socket.listen(TCP_PORT)
                    .unwrap();
            }
        }
    }
}

impl<'a, 's> fmt::Write for Server<'a, 's> {
    /// Write to all connected clients
    fn write_str(&mut self, slice: &str) -> fmt::Result {
        for handle in &self.handles {
            let mut socket = self.net.sockets().get::<TcpSocket>(*handle);
            if socket.can_send() {
                // Ignore errors, proceed with next client
                let _ = socket.write_str(slice);
            }
        }

        Ok(())
    }
}
