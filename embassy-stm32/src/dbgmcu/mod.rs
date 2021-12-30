pub struct Dbgmcu {}

impl Dbgmcu {
    pub unsafe fn enable_all() {
        #[cfg(rcc_g0)]
        crate::pac::RCC.apbenr1().modify(|w| w.set_dbgen(true));

        crate::pac::DBGMCU.cr().modify(|cr| {
            crate::pac::dbgmcu! {
                (cr, $fn_name:ident) => {
                    cr.$fn_name(true);
                };
            }
        });
    }
}
