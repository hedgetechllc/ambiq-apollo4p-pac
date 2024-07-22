#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: Mode,
    clkctrl: Clkctrl,
    bgcolor: Bgcolor,
    resxy: Resxy,
    _reserved4: [u8; 0x04],
    frontporchxy: Frontporchxy,
    blankingxy: Blankingxy,
    backporchxy: Backporchxy,
    cursorxy: Cursorxy,
    _reserved8: [u8; 0x04],
    dbicfg: Dbicfg,
    dcgpio: Dcgpio,
    layer0mode: Layer0mode,
    layer0startxy: Layer0startxy,
    layer0sizexy: Layer0sizexy,
    layer0addr: Layer0addr,
    layer0stride: Layer0stride,
    layer0resxy: Layer0resxy,
    layer0scalex: Layer0scalex,
    layer0scaley: Layer0scaley,
    layer1mode: Layer1mode,
    layer1startxy: Layer1startxy,
    layer1sizexy: Layer1sizexy,
    layer1addr: Layer1addr,
    layer1stride: Layer1stride,
    layer1resxy: Layer1resxy,
    layer1scalex: Layer1scalex,
    layer1scaley: Layer1scaley,
    layer2mode: Layer2mode,
    layer2startxy: Layer2startxy,
    layer2sizexy: Layer2sizexy,
    layer2addr: Layer2addr,
    layer2stride: Layer2stride,
    layer2resxy: Layer2resxy,
    layer2scalex: Layer2scalex,
    layer2scaley: Layer2scaley,
    layer3mode: Layer3mode,
    layer3startxy: Layer3startxy,
    layer3sizexy: Layer3sizexy,
    layer3addr: Layer3addr,
    layer3stride: Layer3stride,
    layer3resxy: Layer3resxy,
    layer3scalex: Layer3scalex,
    layer3scaley: Layer3scaley,
    _reserved42: [u8; 0x38],
    dbicmd: Dbicmd,
    dbirdat: Dbirdat,
    confg: Confg,
    idreg: Idreg,
    interrupt: Interrupt,
    status: Status,
    colmod: Colmod,
    _reserved49: [u8; 0x80],
    crc: Crc,
    _reserved50: [u8; 0x0278],
    gllut: Gllut,
    _reserved51: [u8; 0x03fc],
    cursordata: Cursordata,
    _reserved52: [u8; 0x01fc],
    cursorlut: Cursorlut,
    _reserved53: [u8; 0x05fc],
    l0lut: L0lut,
    _reserved54: [u8; 0x03fc],
    l1lut: L1lut,
    _reserved55: [u8; 0x03fc],
    l2lut0: L2lut0,
    _reserved56: [u8; 0x03fc],
    l3lut: L3lut,
}
impl RegisterBlock {
    #[doc = "0x00 - General control register that activates the NEMAp|dc400 controller and various parameters, sets the timing signals' polarity, activates the global look-up table for gamma correction and chooses the output display formats to meet LCD color specifications."]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x04 - Setup proper timing with divisor control bits and specify the number of lines to be prefetched before the start of frame."]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x08 - Specifies the main background color."]
    #[inline(always)]
    pub const fn bgcolor(&self) -> &Bgcolor {
        &self.bgcolor
    }
    #[doc = "0x0c - Specifies the main X and Y resolutions."]
    #[inline(always)]
    pub const fn resxy(&self) -> &Resxy {
        &self.resxy
    }
    #[doc = "0x14 - Specifies the X and Y dimensions for the Front Porch."]
    #[inline(always)]
    pub const fn frontporchxy(&self) -> &Frontporchxy {
        &self.frontporchxy
    }
    #[doc = "0x18 - Specifies the X and Y dimensions for the Blanking Period."]
    #[inline(always)]
    pub const fn blankingxy(&self) -> &Blankingxy {
        &self.blankingxy
    }
    #[doc = "0x1c - Specifies the X and Y dimensions for the Back Porch."]
    #[inline(always)]
    pub const fn backporchxy(&self) -> &Backporchxy {
        &self.backporchxy
    }
    #[doc = "0x20 - Specifies the cursor's start X and Y coordinates."]
    #[inline(always)]
    pub const fn cursorxy(&self) -> &Cursorxy {
        &self.cursorxy
    }
    #[doc = "0x28 - Register for the configuration DBI Type-B interface and the activation of SPI 3-/4-wire interfaces."]
    #[inline(always)]
    pub const fn dbicfg(&self) -> &Dbicfg {
        &self.dbicfg
    }
    #[doc = "0x2c - General Purpose register: read/write GPIO external pins. This is accumulated as- {CGBYPASS_in,13'd0,ADVANCE_ANYWAY_in,5'd0,GPIO_in}"]
    #[inline(always)]
    pub const fn dcgpio(&self) -> &Dcgpio {
        &self.dcgpio
    }
    #[doc = "0x30 - LAYER0_MODE: Activate and set-up layer 0."]
    #[inline(always)]
    pub const fn layer0mode(&self) -> &Layer0mode {
        &self.layer0mode
    }
    #[doc = "0x34 - X and Y start dimensions of layer 0."]
    #[inline(always)]
    pub const fn layer0startxy(&self) -> &Layer0startxy {
        &self.layer0startxy
    }
    #[doc = "0x38 - X and Y size of layer 0."]
    #[inline(always)]
    pub const fn layer0sizexy(&self) -> &Layer0sizexy {
        &self.layer0sizexy
    }
    #[doc = "0x3c - The start address of the framebuffer to be accessed by layer 0."]
    #[inline(always)]
    pub const fn layer0addr(&self) -> &Layer0addr {
        &self.layer0addr
    }
    #[doc = "0x40 - Specify the stride and the AXI bus burst of layer 0."]
    #[inline(always)]
    pub const fn layer0stride(&self) -> &Layer0stride {
        &self.layer0stride
    }
    #[doc = "0x44 - X and Y dimensions for the resolution of layer 0."]
    #[inline(always)]
    pub const fn layer0resxy(&self) -> &Layer0resxy {
        &self.layer0resxy
    }
    #[doc = "0x48 - Scale X factor of layer 0."]
    #[inline(always)]
    pub const fn layer0scalex(&self) -> &Layer0scalex {
        &self.layer0scalex
    }
    #[doc = "0x4c - Scale Y factor of layer 0."]
    #[inline(always)]
    pub const fn layer0scaley(&self) -> &Layer0scaley {
        &self.layer0scaley
    }
    #[doc = "0x50 - Activate and set-up layer 1."]
    #[inline(always)]
    pub const fn layer1mode(&self) -> &Layer1mode {
        &self.layer1mode
    }
    #[doc = "0x54 - X and Y start dimensions of layer 1."]
    #[inline(always)]
    pub const fn layer1startxy(&self) -> &Layer1startxy {
        &self.layer1startxy
    }
    #[doc = "0x58 - X and Y size of layer 1."]
    #[inline(always)]
    pub const fn layer1sizexy(&self) -> &Layer1sizexy {
        &self.layer1sizexy
    }
    #[doc = "0x5c - The start address of the framebuffer to be accessed by layer 1."]
    #[inline(always)]
    pub const fn layer1addr(&self) -> &Layer1addr {
        &self.layer1addr
    }
    #[doc = "0x60 - Specify the stride and the AXI bus burst of layer 1."]
    #[inline(always)]
    pub const fn layer1stride(&self) -> &Layer1stride {
        &self.layer1stride
    }
    #[doc = "0x64 - X and Y dimensions for the resolution of layer 1."]
    #[inline(always)]
    pub const fn layer1resxy(&self) -> &Layer1resxy {
        &self.layer1resxy
    }
    #[doc = "0x68 - Scale X factor of layer 1."]
    #[inline(always)]
    pub const fn layer1scalex(&self) -> &Layer1scalex {
        &self.layer1scalex
    }
    #[doc = "0x6c - Scale Y factor of layer 1."]
    #[inline(always)]
    pub const fn layer1scaley(&self) -> &Layer1scaley {
        &self.layer1scaley
    }
    #[doc = "0x70 - Activate and set-up layer 2."]
    #[inline(always)]
    pub const fn layer2mode(&self) -> &Layer2mode {
        &self.layer2mode
    }
    #[doc = "0x74 - X and Y start dimensions of layer 2."]
    #[inline(always)]
    pub const fn layer2startxy(&self) -> &Layer2startxy {
        &self.layer2startxy
    }
    #[doc = "0x78 - X and Y size of layer 2."]
    #[inline(always)]
    pub const fn layer2sizexy(&self) -> &Layer2sizexy {
        &self.layer2sizexy
    }
    #[doc = "0x7c - The start address of the framebuffer to be accessed by layer 2."]
    #[inline(always)]
    pub const fn layer2addr(&self) -> &Layer2addr {
        &self.layer2addr
    }
    #[doc = "0x80 - Specify the stride and the AXI bus burst of layer 2."]
    #[inline(always)]
    pub const fn layer2stride(&self) -> &Layer2stride {
        &self.layer2stride
    }
    #[doc = "0x84 - X and Y dimensions for the resolution of layer 2."]
    #[inline(always)]
    pub const fn layer2resxy(&self) -> &Layer2resxy {
        &self.layer2resxy
    }
    #[doc = "0x88 - Scale X factor of layer 2."]
    #[inline(always)]
    pub const fn layer2scalex(&self) -> &Layer2scalex {
        &self.layer2scalex
    }
    #[doc = "0x8c - Scale Y factor of layer 2."]
    #[inline(always)]
    pub const fn layer2scaley(&self) -> &Layer2scaley {
        &self.layer2scaley
    }
    #[doc = "0x90 - Activate and set-up layer 3."]
    #[inline(always)]
    pub const fn layer3mode(&self) -> &Layer3mode {
        &self.layer3mode
    }
    #[doc = "0x94 - X and Y start dimensions of layer 3."]
    #[inline(always)]
    pub const fn layer3startxy(&self) -> &Layer3startxy {
        &self.layer3startxy
    }
    #[doc = "0x98 - X and Y size of layer 3."]
    #[inline(always)]
    pub const fn layer3sizexy(&self) -> &Layer3sizexy {
        &self.layer3sizexy
    }
    #[doc = "0x9c - The start address of the framebuffer to be accessed by layer 3."]
    #[inline(always)]
    pub const fn layer3addr(&self) -> &Layer3addr {
        &self.layer3addr
    }
    #[doc = "0xa0 - Specify the stride and the AXI bus burst of layer 3."]
    #[inline(always)]
    pub const fn layer3stride(&self) -> &Layer3stride {
        &self.layer3stride
    }
    #[doc = "0xa4 - X and Y dimensions for the resolution of layer 3."]
    #[inline(always)]
    pub const fn layer3resxy(&self) -> &Layer3resxy {
        &self.layer3resxy
    }
    #[doc = "0xa8 - Scale X factor of layer 3."]
    #[inline(always)]
    pub const fn layer3scalex(&self) -> &Layer3scalex {
        &self.layer3scalex
    }
    #[doc = "0xac - Scale Y factor of layer 3."]
    #[inline(always)]
    pub const fn layer3scaley(&self) -> &Layer3scaley {
        &self.layer3scaley
    }
    #[doc = "0xe8 - Register to read/write commands from/to DBI Type-B interface."]
    #[inline(always)]
    pub const fn dbicmd(&self) -> &Dbicmd {
        &self.dbicmd
    }
    #[doc = "0xec - Data read by DBI Type-B interface are stored in the DBI_RDAT register."]
    #[inline(always)]
    pub const fn dbirdat(&self) -> &Dbirdat {
        &self.dbirdat
    }
    #[doc = "0xf0 - Information of the layers n activation and setup."]
    #[inline(always)]
    pub const fn confg(&self) -> &Confg {
        &self.confg
    }
    #[doc = "0xf4 - Identification Register."]
    #[inline(always)]
    pub const fn idreg(&self) -> &Idreg {
        &self.idreg
    }
    #[doc = "0xf8 - Register interrupts enabled, level or edge enabled."]
    #[inline(always)]
    pub const fn interrupt(&self) -> &Interrupt {
        &self.interrupt
    }
    #[doc = "0xfc - DSI Status register (interrupt and pending activity)"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x100 - Color mode status register indicating formats/back pressure are enabled."]
    #[inline(always)]
    pub const fn colmod(&self) -> &Colmod {
        &self.colmod
    }
    #[doc = "0x184 - if cyclic redundancy errors occur, they are written in the CRC register."]
    #[inline(always)]
    pub const fn crc(&self) -> &Crc {
        &self.crc
    }
    #[doc = "0x400 - R\\[0\\]G\\[0\\]B\\[0\\]
thru R\\[255\\]G\\[255\\]B\\[255\\]
Global palette, gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L0LUT(n) (*((volatile uint32_t*)(&amp;L0LUT + (4*n))))"]
    #[inline(always)]
    pub const fn gllut(&self) -> &Gllut {
        &self.gllut
    }
    #[doc = "0x800 - Color values for the pixel cursor that are used with the Cursor LUT where x starts at 0 thru 127.Access to all 16 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_CURSORDATA(n) (*((volatile uint32_t*)(&amp;CURSORDATA + (4*n))))"]
    #[inline(always)]
    pub const fn cursordata(&self) -> &Cursordata {
        &self.cursordata
    }
    #[doc = "0xa00 - R\\[0\\]G\\[0\\]B\\[0\\]
thru R\\[15\\]G\\[15\\]B\\[15\\]
Cursor Look-up Table where x starts at 0 thru 15.Access to all 16 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_CURSORLUT(n) (*((volatile uint32_t*)(&amp;CURSORLUT + (4*n))))"]
    #[inline(always)]
    pub const fn cursorlut(&self) -> &Cursorlut {
        &self.cursorlut
    }
    #[doc = "0x1000 - A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]. Layer 0 palette,gamma correction memory region where x starts at 0 thru 255."]
    #[inline(always)]
    pub const fn l0lut(&self) -> &L0lut {
        &self.l0lut
    }
    #[doc = "0x1400 - A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 1 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L1LUT(n) (*((volatile uint32_t*)(&amp;L1LUT + (4*n))))"]
    #[inline(always)]
    pub const fn l1lut(&self) -> &L1lut {
        &self.l1lut
    }
    #[doc = "0x1800 - A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 2 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L2LUT(n) (*((volatile uint32_t*)(&amp;L2LUT + (4*n))))"]
    #[inline(always)]
    pub const fn l2lut0(&self) -> &L2lut0 {
        &self.l2lut0
    }
    #[doc = "0x1c00 - A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 3 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L3LUT(n) (*((volatile uint32_t*)(&amp;L3LUT + (4*n))))"]
    #[inline(always)]
    pub const fn l3lut(&self) -> &L3lut {
        &self.l3lut
    }
}
#[doc = "MODE (rw) register accessor: General control register that activates the NEMAp|dc400 controller and various parameters, sets the timing signals' polarity, activates the global look-up table for gamma correction and chooses the output display formats to meet LCD color specifications.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "General control register that activates the NEMAp|dc400 controller and various parameters, sets the timing signals' polarity, activates the global look-up table for gamma correction and chooses the output display formats to meet LCD color specifications."]
pub mod mode;
#[doc = "CLKCTRL (rw) register accessor: Setup proper timing with divisor control bits and specify the number of lines to be prefetched before the start of frame.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Setup proper timing with divisor control bits and specify the number of lines to be prefetched before the start of frame."]
pub mod clkctrl;
#[doc = "BGCOLOR (rw) register accessor: Specifies the main background color.\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcolor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcolor`]
module"]
#[doc(alias = "BGCOLOR")]
pub type Bgcolor = crate::Reg<bgcolor::BgcolorSpec>;
#[doc = "Specifies the main background color."]
pub mod bgcolor;
#[doc = "RESXY (rw) register accessor: Specifies the main X and Y resolutions.\n\nYou can [`read`](crate::Reg::read) this register and get [`resxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resxy`]
module"]
#[doc(alias = "RESXY")]
pub type Resxy = crate::Reg<resxy::ResxySpec>;
#[doc = "Specifies the main X and Y resolutions."]
pub mod resxy;
#[doc = "FRONTPORCHXY (rw) register accessor: Specifies the X and Y dimensions for the Front Porch.\n\nYou can [`read`](crate::Reg::read) this register and get [`frontporchxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frontporchxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frontporchxy`]
module"]
#[doc(alias = "FRONTPORCHXY")]
pub type Frontporchxy = crate::Reg<frontporchxy::FrontporchxySpec>;
#[doc = "Specifies the X and Y dimensions for the Front Porch."]
pub mod frontporchxy;
#[doc = "BLANKINGXY (rw) register accessor: Specifies the X and Y dimensions for the Blanking Period.\n\nYou can [`read`](crate::Reg::read) this register and get [`blankingxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blankingxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blankingxy`]
module"]
#[doc(alias = "BLANKINGXY")]
pub type Blankingxy = crate::Reg<blankingxy::BlankingxySpec>;
#[doc = "Specifies the X and Y dimensions for the Blanking Period."]
pub mod blankingxy;
#[doc = "BACKPORCHXY (rw) register accessor: Specifies the X and Y dimensions for the Back Porch.\n\nYou can [`read`](crate::Reg::read) this register and get [`backporchxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backporchxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backporchxy`]
module"]
#[doc(alias = "BACKPORCHXY")]
pub type Backporchxy = crate::Reg<backporchxy::BackporchxySpec>;
#[doc = "Specifies the X and Y dimensions for the Back Porch."]
pub mod backporchxy;
#[doc = "CURSORXY (rw) register accessor: Specifies the cursor's start X and Y coordinates.\n\nYou can [`read`](crate::Reg::read) this register and get [`cursorxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cursorxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cursorxy`]
module"]
#[doc(alias = "CURSORXY")]
pub type Cursorxy = crate::Reg<cursorxy::CursorxySpec>;
#[doc = "Specifies the cursor's start X and Y coordinates."]
pub mod cursorxy;
#[doc = "DBICFG (rw) register accessor: Register for the configuration DBI Type-B interface and the activation of SPI 3-/4-wire interfaces.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbicfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbicfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbicfg`]
module"]
#[doc(alias = "DBICFG")]
pub type Dbicfg = crate::Reg<dbicfg::DbicfgSpec>;
#[doc = "Register for the configuration DBI Type-B interface and the activation of SPI 3-/4-wire interfaces."]
pub mod dbicfg;
#[doc = "DCGPIO (rw) register accessor: General Purpose register: read/write GPIO external pins. This is accumulated as- {CGBYPASS_in,13'd0,ADVANCE_ANYWAY_in,5'd0,GPIO_in}\n\nYou can [`read`](crate::Reg::read) this register and get [`dcgpio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcgpio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcgpio`]
module"]
#[doc(alias = "DCGPIO")]
pub type Dcgpio = crate::Reg<dcgpio::DcgpioSpec>;
#[doc = "General Purpose register: read/write GPIO external pins. This is accumulated as- {CGBYPASS_in,13'd0,ADVANCE_ANYWAY_in,5'd0,GPIO_in}"]
pub mod dcgpio;
#[doc = "LAYER0MODE (rw) register accessor: LAYER0_MODE: Activate and set-up layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer0mode`]
module"]
#[doc(alias = "LAYER0MODE")]
pub type Layer0mode = crate::Reg<layer0mode::Layer0modeSpec>;
#[doc = "LAYER0_MODE: Activate and set-up layer 0."]
pub mod layer0mode;
#[doc = "LAYER0STARTXY (rw) register accessor: X and Y start dimensions of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0startxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0startxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer0startxy`]
module"]
#[doc(alias = "LAYER0STARTXY")]
pub type Layer0startxy = crate::Reg<layer0startxy::Layer0startxySpec>;
#[doc = "X and Y start dimensions of layer 0."]
pub mod layer0startxy;
#[doc = "LAYER0SIZEXY (rw) register accessor: X and Y size of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0sizexy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0sizexy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer0sizexy`]
module"]
#[doc(alias = "LAYER0SIZEXY")]
pub type Layer0sizexy = crate::Reg<layer0sizexy::Layer0sizexySpec>;
#[doc = "X and Y size of layer 0."]
pub mod layer0sizexy;
#[doc = "LAYER0ADDR (rw) register accessor: The start address of the framebuffer to be accessed by layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer0addr`]
module"]
#[doc(alias = "LAYER0ADDR")]
pub type Layer0addr = crate::Reg<layer0addr::Layer0addrSpec>;
#[doc = "The start address of the framebuffer to be accessed by layer 0."]
pub mod layer0addr;
#[doc = "LAYER0STRIDE (rw) register accessor: Specify the stride and the AXI bus burst of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer0stride`]
module"]
#[doc(alias = "LAYER0STRIDE")]
pub type Layer0stride = crate::Reg<layer0stride::Layer0strideSpec>;
#[doc = "Specify the stride and the AXI bus burst of layer 0."]
pub mod layer0stride;
#[doc = "LAYER0RESXY (rw) register accessor: X and Y dimensions for the resolution of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0resxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0resxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer0resxy`]
module"]
#[doc(alias = "LAYER0RESXY")]
pub type Layer0resxy = crate::Reg<layer0resxy::Layer0resxySpec>;
#[doc = "X and Y dimensions for the resolution of layer 0."]
pub mod layer0resxy;
#[doc = "LAYER0SCALEX (rw) register accessor: Scale X factor of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0scalex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0scalex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer0scalex`]
module"]
#[doc(alias = "LAYER0SCALEX")]
pub type Layer0scalex = crate::Reg<layer0scalex::Layer0scalexSpec>;
#[doc = "Scale X factor of layer 0."]
pub mod layer0scalex;
#[doc = "LAYER0SCALEY (rw) register accessor: Scale Y factor of layer 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer0scaley::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0scaley::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer0scaley`]
module"]
#[doc(alias = "LAYER0SCALEY")]
pub type Layer0scaley = crate::Reg<layer0scaley::Layer0scaleySpec>;
#[doc = "Scale Y factor of layer 0."]
pub mod layer0scaley;
#[doc = "LAYER1MODE (rw) register accessor: Activate and set-up layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer1mode`]
module"]
#[doc(alias = "LAYER1MODE")]
pub type Layer1mode = crate::Reg<layer1mode::Layer1modeSpec>;
#[doc = "Activate and set-up layer 1."]
pub mod layer1mode;
#[doc = "LAYER1STARTXY (rw) register accessor: X and Y start dimensions of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1startxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1startxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer1startxy`]
module"]
#[doc(alias = "LAYER1STARTXY")]
pub type Layer1startxy = crate::Reg<layer1startxy::Layer1startxySpec>;
#[doc = "X and Y start dimensions of layer 1."]
pub mod layer1startxy;
#[doc = "LAYER1SIZEXY (rw) register accessor: X and Y size of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1sizexy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1sizexy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer1sizexy`]
module"]
#[doc(alias = "LAYER1SIZEXY")]
pub type Layer1sizexy = crate::Reg<layer1sizexy::Layer1sizexySpec>;
#[doc = "X and Y size of layer 1."]
pub mod layer1sizexy;
#[doc = "LAYER1ADDR (rw) register accessor: The start address of the framebuffer to be accessed by layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer1addr`]
module"]
#[doc(alias = "LAYER1ADDR")]
pub type Layer1addr = crate::Reg<layer1addr::Layer1addrSpec>;
#[doc = "The start address of the framebuffer to be accessed by layer 1."]
pub mod layer1addr;
#[doc = "LAYER1STRIDE (rw) register accessor: Specify the stride and the AXI bus burst of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer1stride`]
module"]
#[doc(alias = "LAYER1STRIDE")]
pub type Layer1stride = crate::Reg<layer1stride::Layer1strideSpec>;
#[doc = "Specify the stride and the AXI bus burst of layer 1."]
pub mod layer1stride;
#[doc = "LAYER1RESXY (rw) register accessor: X and Y dimensions for the resolution of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1resxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1resxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer1resxy`]
module"]
#[doc(alias = "LAYER1RESXY")]
pub type Layer1resxy = crate::Reg<layer1resxy::Layer1resxySpec>;
#[doc = "X and Y dimensions for the resolution of layer 1."]
pub mod layer1resxy;
#[doc = "LAYER1SCALEX (rw) register accessor: Scale X factor of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1scalex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1scalex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer1scalex`]
module"]
#[doc(alias = "LAYER1SCALEX")]
pub type Layer1scalex = crate::Reg<layer1scalex::Layer1scalexSpec>;
#[doc = "Scale X factor of layer 1."]
pub mod layer1scalex;
#[doc = "LAYER1SCALEY (rw) register accessor: Scale Y factor of layer 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer1scaley::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer1scaley::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer1scaley`]
module"]
#[doc(alias = "LAYER1SCALEY")]
pub type Layer1scaley = crate::Reg<layer1scaley::Layer1scaleySpec>;
#[doc = "Scale Y factor of layer 1."]
pub mod layer1scaley;
#[doc = "LAYER2MODE (rw) register accessor: Activate and set-up layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer2mode`]
module"]
#[doc(alias = "LAYER2MODE")]
pub type Layer2mode = crate::Reg<layer2mode::Layer2modeSpec>;
#[doc = "Activate and set-up layer 2."]
pub mod layer2mode;
#[doc = "LAYER2STARTXY (rw) register accessor: X and Y start dimensions of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2startxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2startxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer2startxy`]
module"]
#[doc(alias = "LAYER2STARTXY")]
pub type Layer2startxy = crate::Reg<layer2startxy::Layer2startxySpec>;
#[doc = "X and Y start dimensions of layer 2."]
pub mod layer2startxy;
#[doc = "LAYER2SIZEXY (rw) register accessor: X and Y size of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2sizexy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2sizexy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer2sizexy`]
module"]
#[doc(alias = "LAYER2SIZEXY")]
pub type Layer2sizexy = crate::Reg<layer2sizexy::Layer2sizexySpec>;
#[doc = "X and Y size of layer 2."]
pub mod layer2sizexy;
#[doc = "LAYER2ADDR (rw) register accessor: The start address of the framebuffer to be accessed by layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer2addr`]
module"]
#[doc(alias = "LAYER2ADDR")]
pub type Layer2addr = crate::Reg<layer2addr::Layer2addrSpec>;
#[doc = "The start address of the framebuffer to be accessed by layer 2."]
pub mod layer2addr;
#[doc = "LAYER2STRIDE (rw) register accessor: Specify the stride and the AXI bus burst of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer2stride`]
module"]
#[doc(alias = "LAYER2STRIDE")]
pub type Layer2stride = crate::Reg<layer2stride::Layer2strideSpec>;
#[doc = "Specify the stride and the AXI bus burst of layer 2."]
pub mod layer2stride;
#[doc = "LAYER2RESXY (rw) register accessor: X and Y dimensions for the resolution of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2resxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2resxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer2resxy`]
module"]
#[doc(alias = "LAYER2RESXY")]
pub type Layer2resxy = crate::Reg<layer2resxy::Layer2resxySpec>;
#[doc = "X and Y dimensions for the resolution of layer 2."]
pub mod layer2resxy;
#[doc = "LAYER2SCALEX (rw) register accessor: Scale X factor of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2scalex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2scalex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer2scalex`]
module"]
#[doc(alias = "LAYER2SCALEX")]
pub type Layer2scalex = crate::Reg<layer2scalex::Layer2scalexSpec>;
#[doc = "Scale X factor of layer 2."]
pub mod layer2scalex;
#[doc = "LAYER2SCALEY (rw) register accessor: Scale Y factor of layer 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer2scaley::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer2scaley::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer2scaley`]
module"]
#[doc(alias = "LAYER2SCALEY")]
pub type Layer2scaley = crate::Reg<layer2scaley::Layer2scaleySpec>;
#[doc = "Scale Y factor of layer 2."]
pub mod layer2scaley;
#[doc = "LAYER3MODE (rw) register accessor: Activate and set-up layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer3mode`]
module"]
#[doc(alias = "LAYER3MODE")]
pub type Layer3mode = crate::Reg<layer3mode::Layer3modeSpec>;
#[doc = "Activate and set-up layer 3."]
pub mod layer3mode;
#[doc = "LAYER3STARTXY (rw) register accessor: X and Y start dimensions of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3startxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3startxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer3startxy`]
module"]
#[doc(alias = "LAYER3STARTXY")]
pub type Layer3startxy = crate::Reg<layer3startxy::Layer3startxySpec>;
#[doc = "X and Y start dimensions of layer 3."]
pub mod layer3startxy;
#[doc = "LAYER3SIZEXY (rw) register accessor: X and Y size of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3sizexy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3sizexy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer3sizexy`]
module"]
#[doc(alias = "LAYER3SIZEXY")]
pub type Layer3sizexy = crate::Reg<layer3sizexy::Layer3sizexySpec>;
#[doc = "X and Y size of layer 3."]
pub mod layer3sizexy;
#[doc = "LAYER3ADDR (rw) register accessor: The start address of the framebuffer to be accessed by layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer3addr`]
module"]
#[doc(alias = "LAYER3ADDR")]
pub type Layer3addr = crate::Reg<layer3addr::Layer3addrSpec>;
#[doc = "The start address of the framebuffer to be accessed by layer 3."]
pub mod layer3addr;
#[doc = "LAYER3STRIDE (rw) register accessor: Specify the stride and the AXI bus burst of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer3stride`]
module"]
#[doc(alias = "LAYER3STRIDE")]
pub type Layer3stride = crate::Reg<layer3stride::Layer3strideSpec>;
#[doc = "Specify the stride and the AXI bus burst of layer 3."]
pub mod layer3stride;
#[doc = "LAYER3RESXY (rw) register accessor: X and Y dimensions for the resolution of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3resxy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3resxy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer3resxy`]
module"]
#[doc(alias = "LAYER3RESXY")]
pub type Layer3resxy = crate::Reg<layer3resxy::Layer3resxySpec>;
#[doc = "X and Y dimensions for the resolution of layer 3."]
pub mod layer3resxy;
#[doc = "LAYER3SCALEX (rw) register accessor: Scale X factor of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3scalex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3scalex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer3scalex`]
module"]
#[doc(alias = "LAYER3SCALEX")]
pub type Layer3scalex = crate::Reg<layer3scalex::Layer3scalexSpec>;
#[doc = "Scale X factor of layer 3."]
pub mod layer3scalex;
#[doc = "LAYER3SCALEY (rw) register accessor: Scale Y factor of layer 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`layer3scaley::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer3scaley::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@layer3scaley`]
module"]
#[doc(alias = "LAYER3SCALEY")]
pub type Layer3scaley = crate::Reg<layer3scaley::Layer3scaleySpec>;
#[doc = "Scale Y factor of layer 3."]
pub mod layer3scaley;
#[doc = "DBICMD (rw) register accessor: Register to read/write commands from/to DBI Type-B interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbicmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbicmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbicmd`]
module"]
#[doc(alias = "DBICMD")]
pub type Dbicmd = crate::Reg<dbicmd::DbicmdSpec>;
#[doc = "Register to read/write commands from/to DBI Type-B interface."]
pub mod dbicmd;
#[doc = "DBIRDAT (rw) register accessor: Data read by DBI Type-B interface are stored in the DBI_RDAT register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbirdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbirdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbirdat`]
module"]
#[doc(alias = "DBIRDAT")]
pub type Dbirdat = crate::Reg<dbirdat::DbirdatSpec>;
#[doc = "Data read by DBI Type-B interface are stored in the DBI_RDAT register."]
pub mod dbirdat;
#[doc = "CONFG (rw) register accessor: Information of the layers n activation and setup.\n\nYou can [`read`](crate::Reg::read) this register and get [`confg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confg`]
module"]
#[doc(alias = "CONFG")]
pub type Confg = crate::Reg<confg::ConfgSpec>;
#[doc = "Information of the layers n activation and setup."]
pub mod confg;
#[doc = "IDREG (rw) register accessor: Identification Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`idreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idreg`]
module"]
#[doc(alias = "IDREG")]
pub type Idreg = crate::Reg<idreg::IdregSpec>;
#[doc = "Identification Register."]
pub mod idreg;
#[doc = "INTERRUPT (rw) register accessor: Register interrupts enabled, level or edge enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`]
module"]
#[doc(alias = "INTERRUPT")]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = "Register interrupts enabled, level or edge enabled."]
pub mod interrupt;
#[doc = "STATUS (rw) register accessor: DSI Status register (interrupt and pending activity)\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DSI Status register (interrupt and pending activity)"]
pub mod status;
#[doc = "COLMOD (rw) register accessor: Color mode status register indicating formats/back pressure are enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`colmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`colmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@colmod`]
module"]
#[doc(alias = "COLMOD")]
pub type Colmod = crate::Reg<colmod::ColmodSpec>;
#[doc = "Color mode status register indicating formats/back pressure are enabled."]
pub mod colmod;
#[doc = "CRC (rw) register accessor: if cyclic redundancy errors occur, they are written in the CRC register.\n\nYou can [`read`](crate::Reg::read) this register and get [`crc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc`]
module"]
#[doc(alias = "CRC")]
pub type Crc = crate::Reg<crc::CrcSpec>;
#[doc = "if cyclic redundancy errors occur, they are written in the CRC register."]
pub mod crc;
#[doc = "GLLUT (rw) register accessor: R\\[0\\]G\\[0\\]B\\[0\\]
thru R\\[255\\]G\\[255\\]B\\[255\\]
Global palette, gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L0LUT(n) (*((volatile uint32_t*)(&amp;L0LUT + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`gllut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gllut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gllut`]
module"]
#[doc(alias = "GLLUT")]
pub type Gllut = crate::Reg<gllut::GllutSpec>;
#[doc = "R\\[0\\]G\\[0\\]B\\[0\\]
thru R\\[255\\]G\\[255\\]B\\[255\\]
Global palette, gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L0LUT(n) (*((volatile uint32_t*)(&amp;L0LUT + (4*n))))"]
pub mod gllut;
#[doc = "CURSORDATA (rw) register accessor: Color values for the pixel cursor that are used with the Cursor LUT where x starts at 0 thru 127.Access to all 16 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_CURSORDATA(n) (*((volatile uint32_t*)(&amp;CURSORDATA + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`cursordata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cursordata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cursordata`]
module"]
#[doc(alias = "CURSORDATA")]
pub type Cursordata = crate::Reg<cursordata::CursordataSpec>;
#[doc = "Color values for the pixel cursor that are used with the Cursor LUT where x starts at 0 thru 127.Access to all 16 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_CURSORDATA(n) (*((volatile uint32_t*)(&amp;CURSORDATA + (4*n))))"]
pub mod cursordata;
#[doc = "CURSORLUT (rw) register accessor: R\\[0\\]G\\[0\\]B\\[0\\]
thru R\\[15\\]G\\[15\\]B\\[15\\]
Cursor Look-up Table where x starts at 0 thru 15.Access to all 16 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_CURSORLUT(n) (*((volatile uint32_t*)(&amp;CURSORLUT + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`cursorlut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cursorlut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cursorlut`]
module"]
#[doc(alias = "CURSORLUT")]
pub type Cursorlut = crate::Reg<cursorlut::CursorlutSpec>;
#[doc = "R\\[0\\]G\\[0\\]B\\[0\\]
thru R\\[15\\]G\\[15\\]B\\[15\\]
Cursor Look-up Table where x starts at 0 thru 15.Access to all 16 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_CURSORLUT(n) (*((volatile uint32_t*)(&amp;CURSORLUT + (4*n))))"]
pub mod cursorlut;
#[doc = "L0LUT (rw) register accessor: A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]. Layer 0 palette,gamma correction memory region where x starts at 0 thru 255.\n\nYou can [`read`](crate::Reg::read) this register and get [`l0lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l0lut`]
module"]
#[doc(alias = "L0LUT")]
pub type L0lut = crate::Reg<l0lut::L0lutSpec>;
#[doc = "A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]. Layer 0 palette,gamma correction memory region where x starts at 0 thru 255."]
pub mod l0lut;
#[doc = "L1LUT (rw) register accessor: A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 1 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L1LUT(n) (*((volatile uint32_t*)(&amp;L1LUT + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`l1lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l1lut`]
module"]
#[doc(alias = "L1LUT")]
pub type L1lut = crate::Reg<l1lut::L1lutSpec>;
#[doc = "A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 1 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L1LUT(n) (*((volatile uint32_t*)(&amp;L1LUT + (4*n))))"]
pub mod l1lut;
#[doc = "L2LUT0 (rw) register accessor: A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 2 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L2LUT(n) (*((volatile uint32_t*)(&amp;L2LUT + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`l2lut0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2lut0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l2lut0`]
module"]
#[doc(alias = "L2LUT0")]
pub type L2lut0 = crate::Reg<l2lut0::L2lut0Spec>;
#[doc = "A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 2 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L2LUT(n) (*((volatile uint32_t*)(&amp;L2LUT + (4*n))))"]
pub mod l2lut0;
#[doc = "L3LUT (rw) register accessor: A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 3 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L3LUT(n) (*((volatile uint32_t*)(&amp;L3LUT + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`l3lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l3lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l3lut`]
module"]
#[doc(alias = "L3LUT")]
pub type L3lut = crate::Reg<l3lut::L3lutSpec>;
#[doc = "A\\[0\\]R\\[0\\]G\\[0\\]B\\[0\\]
thru A\\[255\\]R\\[255\\]G\\[255\\]B\\[255\\]
Layer 3 palette,gamma correction memory region where x starts at 0 thru 255.Access to all 256 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_L3LUT(n) (*((volatile uint32_t*)(&amp;L3LUT + (4*n))))"]
pub mod l3lut;
