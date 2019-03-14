//! As there is only one peripheral, supporting data structures are
//! declared once and globally.

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use bare_metal::CriticalSection;
use stm32f4xx_hal::{
    stm32::{interrupt, Peripherals, NVIC, ETHERNET_MAC, ETHERNET_DMA},
};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr};
use smoltcp::iface::{NeighborCache, EthernetInterfaceBuilder, EthernetInterface};
use smoltcp::socket::SocketSet;
use stm32_eth::{Eth, RingEntry, RxDescriptor, TxDescriptor};

// TODO: ram regions
static mut RX_RING: Option<[RingEntry<RxDescriptor>; 8]> = None;
static mut TX_RING: Option<[RingEntry<TxDescriptor>; 2]> = None;

// TODO: generate one from device id
const SRC_MAC: [u8; 6] = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];

static NET_PENDING: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

pub fn run<F>(nvic: &mut NVIC, ethernet_mac: ETHERNET_MAC, ethernet_dma: ETHERNET_DMA, f: F) -> !
where
    F: FnOnce(&mut NetInterface) -> !
{
    let rx_ring = unsafe {
        RX_RING.get_or_insert(Default::default())
    };
    let tx_ring = unsafe {
        TX_RING.get_or_insert(Default::default())
    };
    let mut eth_dev = Eth::new(
        ethernet_mac, ethernet_dma,
        &mut rx_ring[..], &mut tx_ring[..]
    );
    eth_dev.enable_interrupt(nvic);

    let local_addr = IpAddress::v4(192, 168, 69, 3);
    let mut ip_addrs = [IpCidr::new(local_addr, 24)];
    let mut neighbor_storage = [None; 16];
    let neighbor_cache = NeighborCache::new(&mut neighbor_storage[..]);
    let ethernet_addr = EthernetAddress(SRC_MAC);
    let iface = EthernetInterfaceBuilder::new(&mut eth_dev)
        .ethernet_addr(ethernet_addr)
        .ip_addrs(&mut ip_addrs[..])
        .neighbor_cache(neighbor_cache)
        .finalize();

    let mut sockets_storage = [None, None, None, None];
    let sockets = SocketSet::new(&mut sockets_storage[..]);

    let mut net_iface = NetInterface {
        iface,
        sockets,
    };
    f(&mut net_iface);
}

pub struct NetInterface<'a> {
    iface: EthernetInterface<'a, 'a, 'a, &'a mut stm32_eth::Eth<'static, 'static>>,
    sockets: SocketSet<'a, 'static, 'static>,
}

impl<'a> NetInterface<'a> {
    /// Passes the boolean that indicates any sockets change.
    pub fn poll(&mut self, now: Instant) -> bool {
        // TODO: clear pending flag

        self.iface.poll(&mut self.sockets, now)
            .ok()
            .unwrap_or(false)
    }

    pub fn sockets(&mut self) -> &mut SocketSet<'a, 'static, 'static> {
        &mut self.sockets
    }
}

/// Wwake up from `wfi()`, clear interrupt flags,
/// and TODO: set pending flag
#[interrupt]
fn ETH() {
    cortex_m::interrupt::free(|cs| {
        *NET_PENDING.borrow(cs)
            .borrow_mut() = true;
    });

    let p = unsafe { Peripherals::steal() };
    stm32_eth::eth_interrupt_handler(&p.ETHERNET_DMA);
}

pub fn is_pending(cs: &CriticalSection) -> bool {
    *NET_PENDING.borrow(cs)
        .borrow()
}

pub fn clear_pending(cs: &CriticalSection) {
    *NET_PENDING.borrow(cs)
        .borrow_mut() = false;
}
