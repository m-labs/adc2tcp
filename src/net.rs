//! As there is only one peripheral, supporting data structures are
//! declared once and globally.

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use bare_metal::CriticalSection;
use stm32f4xx_hal::{
    stm32::{interrupt, Peripherals, NVIC, ETHERNET_MAC, ETHERNET_DMA},
};
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr};
use smoltcp::iface::{NeighborCache, EthernetInterfaceBuilder, EthernetInterface};
use stm32_eth::{Eth, RingEntry, RxDescriptor, TxDescriptor};

/// Not on the stack so that stack can be placed in CCMRAM (which the
/// ethernet peripheral cannot access)
static mut RX_RING: Option<[RingEntry<RxDescriptor>; 8]> = None;
/// Not on the stack so that stack can be placed in CCMRAM (which the
/// ethernet peripheral cannot access)
static mut TX_RING: Option<[RingEntry<TxDescriptor>; 2]> = None;

/// Interrupt pending flag: set by the `ETH` interrupt handler, should
/// be cleared before polling the interface.
static NET_PENDING: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

/// Run callback `f` with ethernet driver and TCP/IP stack
pub fn run<F>(
    nvic: &mut NVIC, ethernet_mac: ETHERNET_MAC, ethernet_dma: ETHERNET_DMA,
    ethernet_addr: EthernetAddress, f: F
) where
    F: FnOnce(EthernetInterface<&mut stm32_eth::Eth<'static, 'static>>),
{
    let rx_ring = unsafe {
        RX_RING.get_or_insert(Default::default())
    };
    let tx_ring = unsafe {
        TX_RING.get_or_insert(Default::default())
    };
    // Ethernet driver
    let mut eth_dev = Eth::new(
        ethernet_mac, ethernet_dma,
        &mut rx_ring[..], &mut tx_ring[..]
    );
    eth_dev.enable_interrupt(nvic);

    // IP stack
    let local_addr = IpAddress::v4(192, 168, 69, 3);
    let mut ip_addrs = [IpCidr::new(local_addr, 24)];
    let mut neighbor_storage = [None; 16];
    let neighbor_cache = NeighborCache::new(&mut neighbor_storage[..]);
    let iface = EthernetInterfaceBuilder::new(&mut eth_dev)
        .ethernet_addr(ethernet_addr)
        .ip_addrs(&mut ip_addrs[..])
        .neighbor_cache(neighbor_cache)
        .finalize();

    f(iface);
}

/// Potentially wake up from `wfi()`, set the interrupt pending flag,
/// clear interrupt flags.
#[interrupt]
fn ETH() {
    cortex_m::interrupt::free(|cs| {
        *NET_PENDING.borrow(cs)
            .borrow_mut() = true;
    });

    let p = unsafe { Peripherals::steal() };
    stm32_eth::eth_interrupt_handler(&p.ETHERNET_DMA);
}

/// Has an interrupt occurred since last call to `clear_pending()`?
pub fn is_pending(cs: &CriticalSection) -> bool {
    *NET_PENDING.borrow(cs)
        .borrow()
}

/// Clear the interrupt pending flag before polling the interface for
/// data.
pub fn clear_pending(cs: &CriticalSection) {
    *NET_PENDING.borrow(cs)
        .borrow_mut() = false;
}
