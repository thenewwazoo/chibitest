
#![feature(used)]
#![no_std]

#[macro_use(exception)]
extern crate cortex_m_rt;

mod chibios;
mod app;

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_intr_handler; 240];

extern "C" fn default_intr_handler() {
    loop {}
}

// These exception handlers are declared in os/common/startup/ARMCMx/compilers/GCC/vectors.c
extern "C" {
    // fn Reset_Handler(); // declared in vectors.c, but obviated by cortex_m_rt
    fn NMI_Handler();
    // fn HardFault_Handler();
    // fn MemManage_Handler();
    // fn BusFault_Handler();
    // fn UsageFault_Handler();
    #[cfg(feature="thumbv7m")]
    fn SVC_Handler();
    // fn DebugMon_Handler();
    #[used]
    fn PendSV_Handler();
// fn SysTick_Handler(); // see e.g. demos/various/NIL-ARMCM0-GENERIC/main.c, CH_IRQ_HANDLER(SysTick_Handler)
}

fn wrap_nmi() {
    unsafe {
        NMI_Handler();
    }
}

exception!(NMI, wrap_nmi);
#[cfg(feature = "thumbv7m")]
exception!(SVCALL, SVC_Handler);
#[cfg(feature = "cortex_alternate_switch")]
exception!(PENDSV, PendSV_Handler);

fn main() {
    app::user_main()
}
