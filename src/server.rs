use core::fmt;
use core::mem::uninitialized;
use smoltcp::{
    iface::EthernetInterface,
    socket::{SocketSet, SocketHandle, TcpSocket, TcpSocketBuffer},
    time::Instant,
};


const TCP_PORT: u16 = 23;
const SOCKET_COUNT: usize = 8;

const TCP_RX_BUFFER_SIZE: usize = 2048;
const TCP_TX_BUFFER_SIZE: usize = 2048;

macro_rules! create_socket_storage {
    ($rx_storage:ident, $tx_storage:ident) => (
        let mut $rx_storage = [0; TCP_RX_BUFFER_SIZE];
        let mut $tx_storage = [0; TCP_TX_BUFFER_SIZE];
    )
}

macro_rules! create_socket {
    ($set:ident, $rx_storage:ident, $tx_storage:ident, $target:expr) => (
        let tcp_rx_buffer = TcpSocketBuffer::new(&mut $rx_storage[..]);
        let tcp_tx_buffer = TcpSocketBuffer::new(&mut $tx_storage[..]);
        let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);
        $target = $set.add(tcp_socket);
    )
}

/// Contains a number of server sockets that get all sent the same
/// data (through `fmt::Write`).
pub struct Server<'a, 'b> {
    net: EthernetInterface<'a, 'a, 'a, &'a mut stm32_eth::Eth<'static, 'static>>,
    sockets: SocketSet<'b, 'b, 'static>,
    handles: [SocketHandle; SOCKET_COUNT],
}

impl<'a, 'b> Server<'a, 'b> {
    pub fn run<F>(net: EthernetInterface<'a, 'a, 'a, &'a mut stm32_eth::Eth<'static, 'static>>, f: F)
    where
        F: FnOnce(&mut Server<'a, '_>),
    {
        create_socket_storage!(tcp_rx_storage0, tcp_tx_storage0);
        create_socket_storage!(tcp_rx_storage1, tcp_tx_storage1);
        create_socket_storage!(tcp_rx_storage2, tcp_tx_storage2);
        create_socket_storage!(tcp_rx_storage3, tcp_tx_storage3);
        create_socket_storage!(tcp_rx_storage4, tcp_tx_storage4);
        create_socket_storage!(tcp_rx_storage5, tcp_tx_storage5);
        create_socket_storage!(tcp_rx_storage6, tcp_tx_storage6);
        create_socket_storage!(tcp_rx_storage7, tcp_tx_storage7);

        let mut sockets_storage = [
            None, None, None, None,
            None, None, None, None
        ];
        let mut sockets = SocketSet::new(&mut sockets_storage[..]);
        let mut handles: [SocketHandle; SOCKET_COUNT] = unsafe { uninitialized() };
        create_socket!(sockets, tcp_rx_storage0, tcp_tx_storage0, handles[0]);
        create_socket!(sockets, tcp_rx_storage1, tcp_tx_storage1, handles[1]);
        create_socket!(sockets, tcp_rx_storage2, tcp_tx_storage2, handles[2]);
        create_socket!(sockets, tcp_rx_storage3, tcp_tx_storage3, handles[3]);
        create_socket!(sockets, tcp_rx_storage4, tcp_tx_storage4, handles[4]);
        create_socket!(sockets, tcp_rx_storage5, tcp_tx_storage5, handles[5]);
        create_socket!(sockets, tcp_rx_storage6, tcp_tx_storage6, handles[6]);
        create_socket!(sockets, tcp_rx_storage7, tcp_tx_storage7, handles[7]);

        let mut server = Server {
            handles,
            sockets,
            net,
        };
        f(&mut server);
    }

    pub fn poll(&mut self, now: Instant) {
        let activity = self.net.poll(&mut self.sockets, now)
            .unwrap_or(true);
        if ! activity {
            return;
        }

        for handle in &self.handles {
            let mut socket = self.sockets.get::<TcpSocket>(*handle);
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
            let mut socket = self.sockets.get::<TcpSocket>(*handle);
            if socket.can_send() {
                // Ignore errors, proceed with next client
                let _ = socket.write_str(slice);
            }
        }

        Ok(())
    }
}
