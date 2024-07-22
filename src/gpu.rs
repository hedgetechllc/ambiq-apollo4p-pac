#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tex0base: Tex0base,
    tex0stride: Tex0stride,
    tex0res: Tex0res,
    _reserved3: [u8; 0x04],
    tex1base: Tex1base,
    tex1stride: Tex1stride,
    tex1res: Tex1res,
    tex1color: Tex1color,
    tex2base: Tex2base,
    tex2stride: Tex2stride,
    tex2res: Tex2res,
    _reserved10: [u8; 0x04],
    tex3base: Tex3base,
    tex3stride: Tex3stride,
    tex3res: Tex3res,
    _reserved13: [u8; 0x54],
    cgcmd: Cgcmd,
    cgctrl: Cgctrl,
    dirtytrigmin: Dirtytrigmin,
    dirtytrigmax: Dirtytrigmax,
    _reserved17: [u8; 0x10],
    status: Status,
    _reserved18: [u8; 0x0c],
    busctrl: Busctrl,
    imemldiaddr: Imemldiaddr,
    imemldidatahl: Imemldidatahl,
    imemldidatahh: Imemldidatahh,
    _reserved22: [u8; 0x18],
    cmdliststatus: Cmdliststatus,
    cmdlistringstop: Cmdlistringstop,
    cmdlistaddr: Cmdlistaddr,
    cmdlistsize: Cmdlistsize,
    interruptctrl: Interruptctrl,
    sysclear: Sysclear,
    drawcmd: Drawcmd,
    drawpt0: Drawpt0,
    drawpt1: Drawpt1,
    _reserved31: [u8; 0x04],
    clipmin: Clipmin,
    clipmax: Clipmax,
    rastctrl: Rastctrl,
    drawcodeptr: Drawcodeptr,
    drawpt0x: Drawpt0x,
    drawpt0y: Drawpt0y,
    drawpt0z: Drawpt0z,
    drawcolor: Drawcolor,
    drawpt1x: Drawpt1x,
    drawpt1y: Drawpt1y,
    drawpt1z: Drawpt1z,
    _reserved42: [u8; 0x04],
    drawpt2x: Drawpt2x,
    drawpt2y: Drawpt2y,
    drawpt2z: Drawpt2z,
    _reserved45: [u8; 0x04],
    drawpt3x: Drawpt3x,
    drawpt3y: Drawpt3y,
    drawpt3z: Drawpt3z,
    _reserved48: [u8; 0x04],
    mm00: Mm00,
    mm01: Mm01,
    mm02: Mm02,
    mm10: Mm10,
    mm11: Mm11,
    mm12: Mm12,
    mm20: Mm20,
    mm21: Mm21,
    mm22: Mm22,
    depthstartl: Depthstartl,
    depthstarth: Depthstarth,
    depthdxl: Depthdxl,
    depthdxh: Depthdxh,
    depthdyl: Depthdyl,
    depthdyh: Depthdyh,
    _reserved63: [u8; 0x04],
    redx: Redx,
    redy: Redy,
    greenx: Greenx,
    greeny: Greeny,
    bluex: Bluex,
    bluey: Bluey,
    alfx: Alfx,
    alfy: Alfy,
    redinit: Redinit,
    greinit: Greinit,
    bluinit: Bluinit,
    alfinit: Alfinit,
    _reserved75: [u8; 0x1c],
    idreg: Idreg,
    loadctrl: Loadctrl,
    _reserved77: [u8; 0x0c],
    c0reg: C0reg,
    c1reg: C1reg,
    c2reg: C2reg,
    c3reg: C3reg,
    _reserved81: [u8; 0x0de0],
    irqid: Irqid,
}
impl RegisterBlock {
    #[doc = "0x00 - Base address of the drawing surface 0 (must be word aligned)."]
    #[inline(always)]
    pub const fn tex0base(&self) -> &Tex0base {
        &self.tex0base
    }
    #[doc = "0x04 - Image 0 mode and stride."]
    #[inline(always)]
    pub const fn tex0stride(&self) -> &Tex0stride {
        &self.tex0stride
    }
    #[doc = "0x08 - Image 0 resolution."]
    #[inline(always)]
    pub const fn tex0res(&self) -> &Tex0res {
        &self.tex0res
    }
    #[doc = "0x10 - Base address of the drawing surface 1 (must be word aligned)."]
    #[inline(always)]
    pub const fn tex1base(&self) -> &Tex1base {
        &self.tex1base
    }
    #[doc = "0x14 - Image 1 mode and stride."]
    #[inline(always)]
    pub const fn tex1stride(&self) -> &Tex1stride {
        &self.tex1stride
    }
    #[doc = "0x18 - Image 1 resolution."]
    #[inline(always)]
    pub const fn tex1res(&self) -> &Tex1res {
        &self.tex1res
    }
    #[doc = "0x1c - Texture maps default color.Used with luminance and alpha-only color formats."]
    #[inline(always)]
    pub const fn tex1color(&self) -> &Tex1color {
        &self.tex1color
    }
    #[doc = "0x20 - Base address of the drawing surface 2 (must be word aligned)."]
    #[inline(always)]
    pub const fn tex2base(&self) -> &Tex2base {
        &self.tex2base
    }
    #[doc = "0x24 - Image 2 mode and stride."]
    #[inline(always)]
    pub const fn tex2stride(&self) -> &Tex2stride {
        &self.tex2stride
    }
    #[doc = "0x28 - Image 2 resolution."]
    #[inline(always)]
    pub const fn tex2res(&self) -> &Tex2res {
        &self.tex2res
    }
    #[doc = "0x30 - Base address of the drawing surface 3 (must be word aligned)."]
    #[inline(always)]
    pub const fn tex3base(&self) -> &Tex3base {
        &self.tex3base
    }
    #[doc = "0x34 - mode and stride."]
    #[inline(always)]
    pub const fn tex3stride(&self) -> &Tex3stride {
        &self.tex3stride
    }
    #[doc = "0x38 - Image 3 resolution."]
    #[inline(always)]
    pub const fn tex3res(&self) -> &Tex3res {
        &self.tex3res
    }
    #[doc = "0x90 - Clock gating enable"]
    #[inline(always)]
    pub const fn cgcmd(&self) -> &Cgcmd {
        &self.cgcmd
    }
    #[doc = "0x94 - CGCTRL register description needed here."]
    #[inline(always)]
    pub const fn cgctrl(&self) -> &Cgctrl {
        &self.cgctrl
    }
    #[doc = "0x98 - Resets dirty region to resolution size when written."]
    #[inline(always)]
    pub const fn dirtytrigmin(&self) -> &Dirtytrigmin {
        &self.dirtytrigmin
    }
    #[doc = "0x9c - Resets dirty region to resolution size when written."]
    #[inline(always)]
    pub const fn dirtytrigmax(&self) -> &Dirtytrigmax {
        &self.dirtytrigmax
    }
    #[doc = "0xb0 - On read, returns GPU status (CHECK address!!)."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0xc0 - Bus Control"]
    #[inline(always)]
    pub const fn busctrl(&self) -> &Busctrl {
        &self.busctrl
    }
    #[doc = "0xc4 - Load shader instruction memory address."]
    #[inline(always)]
    pub const fn imemldiaddr(&self) -> &Imemldiaddr {
        &self.imemldiaddr
    }
    #[doc = "0xc8 - Load shader instruction Memory data (31:0)."]
    #[inline(always)]
    pub const fn imemldidatahl(&self) -> &Imemldidatahl {
        &self.imemldidatahl
    }
    #[doc = "0xcc - Load shader instruction Memory data (63:32)."]
    #[inline(always)]
    pub const fn imemldidatahh(&self) -> &Imemldidatahh {
        &self.imemldidatahh
    }
    #[doc = "0xe8 - On read, returns command list processor status; On write, resets command list processor."]
    #[inline(always)]
    pub const fn cmdliststatus(&self) -> &Cmdliststatus {
        &self.cmdliststatus
    }
    #[doc = "0xec - Updates GPU command list pointer to stop executing."]
    #[inline(always)]
    pub const fn cmdlistringstop(&self) -> &Cmdlistringstop {
        &self.cmdlistringstop
    }
    #[doc = "0xf0 - Command list base pointer."]
    #[inline(always)]
    pub const fn cmdlistaddr(&self) -> &Cmdlistaddr {
        &self.cmdlistaddr
    }
    #[doc = "0xf4 - Command list length in words."]
    #[inline(always)]
    pub const fn cmdlistsize(&self) -> &Cmdlistsize {
        &self.cmdlistsize
    }
    #[doc = "0xf8 - On write, clears the IRQ (CHECK address!)."]
    #[inline(always)]
    pub const fn interruptctrl(&self) -> &Interruptctrl {
        &self.interruptctrl
    }
    #[doc = "0xfc - On write, resets the GPU (CHECK address!)."]
    #[inline(always)]
    pub const fn sysclear(&self) -> &Sysclear {
        &self.sysclear
    }
    #[doc = "0x100 - Rasterizer drawing command."]
    #[inline(always)]
    pub const fn drawcmd(&self) -> &Drawcmd {
        &self.drawcmd
    }
    #[doc = "0x104 - Stores only integer values. For greater accurancy DRAWPT0X and DRAWPT0Y registers are used which are 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt0(&self) -> &Drawpt0 {
        &self.drawpt0
    }
    #[doc = "0x108 - Stores only integer values. Vertex 1 drawing primitive. Stores only integer values. For greater accurancy DRAWPT1X and DRAWPT1Y registers are used which are 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt1(&self) -> &Drawpt1 {
        &self.drawpt1
    }
    #[doc = "0x110 - Clipping rectangle upper left vertex."]
    #[inline(always)]
    pub const fn clipmin(&self) -> &Clipmin {
        &self.clipmin
    }
    #[doc = "0x114 - Clipping rectangle bottom right vertex."]
    #[inline(always)]
    pub const fn clipmax(&self) -> &Clipmax {
        &self.clipmax
    }
    #[doc = "0x118 - Rasterizer matrix multiplication control"]
    #[inline(always)]
    pub const fn rastctrl(&self) -> &Rastctrl {
        &self.rastctrl
    }
    #[doc = "0x11c - DRAWCODEPTR register description needed here."]
    #[inline(always)]
    pub const fn drawcodeptr(&self) -> &Drawcodeptr {
        &self.drawcodeptr
    }
    #[doc = "0x120 - X coordinate of Vertex 0 drawing primitive 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt0x(&self) -> &Drawpt0x {
        &self.drawpt0x
    }
    #[doc = "0x124 - Y coordinate of Vertex 0 drawing primitive 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt0y(&self) -> &Drawpt0y {
        &self.drawpt0y
    }
    #[doc = "0x128 - DRAWPTOX register description needed here."]
    #[inline(always)]
    pub const fn drawpt0z(&self) -> &Drawpt0z {
        &self.drawpt0z
    }
    #[doc = "0x12c - DRAWCOLOR register description needed here."]
    #[inline(always)]
    pub const fn drawcolor(&self) -> &Drawcolor {
        &self.drawcolor
    }
    #[doc = "0x130 - X coordinate of Vertex 1 drawing primitive 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt1x(&self) -> &Drawpt1x {
        &self.drawpt1x
    }
    #[doc = "0x134 - Y coordinate of Vertex 1 drawing primitive 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt1y(&self) -> &Drawpt1y {
        &self.drawpt1y
    }
    #[doc = "0x138 - DRAWPT1Z register description needed here."]
    #[inline(always)]
    pub const fn drawpt1z(&self) -> &Drawpt1z {
        &self.drawpt1z
    }
    #[doc = "0x140 - X coordinate of Vertex 2 drawing primitive 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt2x(&self) -> &Drawpt2x {
        &self.drawpt2x
    }
    #[doc = "0x144 - Y coordinate of Vertex 2 drawing primitive 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt2y(&self) -> &Drawpt2y {
        &self.drawpt2y
    }
    #[doc = "0x148 - DRAWPT2Z register description needed here."]
    #[inline(always)]
    pub const fn drawpt2z(&self) -> &Drawpt2z {
        &self.drawpt2z
    }
    #[doc = "0x150 - X coordinate of Vertex 3 drawing primitive 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt3x(&self) -> &Drawpt3x {
        &self.drawpt3x
    }
    #[doc = "0x154 - Y coordinate of Vertex 3 drawing primitive 16, 16 fixed point."]
    #[inline(always)]
    pub const fn drawpt3y(&self) -> &Drawpt3y {
        &self.drawpt3y
    }
    #[doc = "0x158 - Fixed value (not accessible). Registers 0x160-0x180 are the elements of the 3x3 transformation matrix used for homogeneous conversion from screen coordinates to texture coordinates; the elements are floating points"]
    #[inline(always)]
    pub const fn drawpt3z(&self) -> &Drawpt3z {
        &self.drawpt3z
    }
    #[doc = "0x160 - matrix floating point element."]
    #[inline(always)]
    pub const fn mm00(&self) -> &Mm00 {
        &self.mm00
    }
    #[doc = "0x164 - matrix floating point element."]
    #[inline(always)]
    pub const fn mm01(&self) -> &Mm01 {
        &self.mm01
    }
    #[doc = "0x168 - matrix floating point element; sets to unit matrix if previously written element is MM12."]
    #[inline(always)]
    pub const fn mm02(&self) -> &Mm02 {
        &self.mm02
    }
    #[doc = "0x16c - matrix floating point element."]
    #[inline(always)]
    pub const fn mm10(&self) -> &Mm10 {
        &self.mm10
    }
    #[doc = "0x170 - matrix floating point element."]
    #[inline(always)]
    pub const fn mm11(&self) -> &Mm11 {
        &self.mm11
    }
    #[doc = "0x174 - matrix floating point element."]
    #[inline(always)]
    pub const fn mm12(&self) -> &Mm12 {
        &self.mm12
    }
    #[doc = "0x178 - matrix floating point element."]
    #[inline(always)]
    pub const fn mm20(&self) -> &Mm20 {
        &self.mm20
    }
    #[doc = "0x17c - matrix floating point element."]
    #[inline(always)]
    pub const fn mm21(&self) -> &Mm21 {
        &self.mm21
    }
    #[doc = "0x180 - matrix floating point element."]
    #[inline(always)]
    pub const fn mm22(&self) -> &Mm22 {
        &self.mm22
    }
    #[doc = "0x184 - Depth value of START pixel, (32 low bits fractional.)"]
    #[inline(always)]
    pub const fn depthstartl(&self) -> &Depthstartl {
        &self.depthstartl
    }
    #[doc = "0x188 - Depth value of START pixel, (32 high bits integral.)"]
    #[inline(always)]
    pub const fn depthstarth(&self) -> &Depthstarth {
        &self.depthstarth
    }
    #[doc = "0x18c - Added depth value for each step at x-axis (32 low bits fractional.)"]
    #[inline(always)]
    pub const fn depthdxl(&self) -> &Depthdxl {
        &self.depthdxl
    }
    #[doc = "0x190 - Added depth value for each step at x-axis (32 high bits integral.)"]
    #[inline(always)]
    pub const fn depthdxh(&self) -> &Depthdxh {
        &self.depthdxh
    }
    #[doc = "0x194 - Added depth value for each step at y-axis (32 low bits fractional.)"]
    #[inline(always)]
    pub const fn depthdyl(&self) -> &Depthdyl {
        &self.depthdyl
    }
    #[doc = "0x198 - Added depth value for each step at y-axis (32 high bits integral.)"]
    #[inline(always)]
    pub const fn depthdyh(&self) -> &Depthdyh {
        &self.depthdyh
    }
    #[doc = "0x1a0 - Added red value for each step at x-axis, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn redx(&self) -> &Redx {
        &self.redx
    }
    #[doc = "0x1a4 - Added red value for each step at y-axis, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn redy(&self) -> &Redy {
        &self.redy
    }
    #[doc = "0x1a8 - Added green value for each step at x-axis, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn greenx(&self) -> &Greenx {
        &self.greenx
    }
    #[doc = "0x1ac - Added green value for each step at y-axis, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn greeny(&self) -> &Greeny {
        &self.greeny
    }
    #[doc = "0x1b0 - Added blue value for each step at x-axis, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn bluex(&self) -> &Bluex {
        &self.bluex
    }
    #[doc = "0x1b4 - Added blue value for each step at y-axis, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn bluey(&self) -> &Bluey {
        &self.bluey
    }
    #[doc = "0x1b8 - Added alfa value for each step at x-axis, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn alfx(&self) -> &Alfx {
        &self.alfx
    }
    #[doc = "0x1bc - Added alfa value for each step at y-axis, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn alfy(&self) -> &Alfy {
        &self.alfy
    }
    #[doc = "0x1c0 - Red value of STARTXY pixel, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn redinit(&self) -> &Redinit {
        &self.redinit
    }
    #[doc = "0x1c4 - Green value of STARTXY pixel, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn greinit(&self) -> &Greinit {
        &self.greinit
    }
    #[doc = "0x1c8 - Blue value of STARTXY pixel, (16, 16 fixed point)"]
    #[inline(always)]
    pub const fn bluinit(&self) -> &Bluinit {
        &self.bluinit
    }
    #[doc = "0x1cc - Alfa value of STARTXY pixel, (16, 16 fixed point) Shader Registers"]
    #[inline(always)]
    pub const fn alfinit(&self) -> &Alfinit {
        &self.alfinit
    }
    #[doc = "0x1ec - Fixed value"]
    #[inline(always)]
    pub const fn idreg(&self) -> &Idreg {
        &self.idreg
    }
    #[doc = "0x1f0 - Load Control"]
    #[inline(always)]
    pub const fn loadctrl(&self) -> &Loadctrl {
        &self.loadctrl
    }
    #[doc = "0x200 - Shader constant register 0."]
    #[inline(always)]
    pub const fn c0reg(&self) -> &C0reg {
        &self.c0reg
    }
    #[doc = "0x204 - Shader constant register 1."]
    #[inline(always)]
    pub const fn c1reg(&self) -> &C1reg {
        &self.c1reg
    }
    #[doc = "0x208 - Shader constant register 2."]
    #[inline(always)]
    pub const fn c2reg(&self) -> &C2reg {
        &self.c2reg
    }
    #[doc = "0x20c - Shader constant register 3, the dirty Region Register"]
    #[inline(always)]
    pub const fn c3reg(&self) -> &C3reg {
        &self.c3reg
    }
    #[doc = "0xff0 - Signals interrupt when set (CHECK address!)."]
    #[inline(always)]
    pub const fn irqid(&self) -> &Irqid {
        &self.irqid
    }
}
#[doc = "TEX0BASE (rw) register accessor: Base address of the drawing surface 0 (must be word aligned).\n\nYou can [`read`](crate::Reg::read) this register and get [`tex0base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex0base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex0base`]
module"]
#[doc(alias = "TEX0BASE")]
pub type Tex0base = crate::Reg<tex0base::Tex0baseSpec>;
#[doc = "Base address of the drawing surface 0 (must be word aligned)."]
pub mod tex0base;
#[doc = "TEX0STRIDE (rw) register accessor: Image 0 mode and stride.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex0stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex0stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex0stride`]
module"]
#[doc(alias = "TEX0STRIDE")]
pub type Tex0stride = crate::Reg<tex0stride::Tex0strideSpec>;
#[doc = "Image 0 mode and stride."]
pub mod tex0stride;
#[doc = "TEX0RES (rw) register accessor: Image 0 resolution.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex0res::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex0res::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex0res`]
module"]
#[doc(alias = "TEX0RES")]
pub type Tex0res = crate::Reg<tex0res::Tex0resSpec>;
#[doc = "Image 0 resolution."]
pub mod tex0res;
#[doc = "TEX1BASE (rw) register accessor: Base address of the drawing surface 1 (must be word aligned).\n\nYou can [`read`](crate::Reg::read) this register and get [`tex1base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex1base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex1base`]
module"]
#[doc(alias = "TEX1BASE")]
pub type Tex1base = crate::Reg<tex1base::Tex1baseSpec>;
#[doc = "Base address of the drawing surface 1 (must be word aligned)."]
pub mod tex1base;
#[doc = "TEX1STRIDE (rw) register accessor: Image 1 mode and stride.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex1stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex1stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex1stride`]
module"]
#[doc(alias = "TEX1STRIDE")]
pub type Tex1stride = crate::Reg<tex1stride::Tex1strideSpec>;
#[doc = "Image 1 mode and stride."]
pub mod tex1stride;
#[doc = "TEX1RES (rw) register accessor: Image 1 resolution.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex1res::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex1res::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex1res`]
module"]
#[doc(alias = "TEX1RES")]
pub type Tex1res = crate::Reg<tex1res::Tex1resSpec>;
#[doc = "Image 1 resolution."]
pub mod tex1res;
#[doc = "TEX1COLOR (rw) register accessor: Texture maps default color.Used with luminance and alpha-only color formats.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex1color::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex1color::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex1color`]
module"]
#[doc(alias = "TEX1COLOR")]
pub type Tex1color = crate::Reg<tex1color::Tex1colorSpec>;
#[doc = "Texture maps default color.Used with luminance and alpha-only color formats."]
pub mod tex1color;
#[doc = "TEX2BASE (rw) register accessor: Base address of the drawing surface 2 (must be word aligned).\n\nYou can [`read`](crate::Reg::read) this register and get [`tex2base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex2base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex2base`]
module"]
#[doc(alias = "TEX2BASE")]
pub type Tex2base = crate::Reg<tex2base::Tex2baseSpec>;
#[doc = "Base address of the drawing surface 2 (must be word aligned)."]
pub mod tex2base;
#[doc = "TEX2STRIDE (rw) register accessor: Image 2 mode and stride.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex2stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex2stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex2stride`]
module"]
#[doc(alias = "TEX2STRIDE")]
pub type Tex2stride = crate::Reg<tex2stride::Tex2strideSpec>;
#[doc = "Image 2 mode and stride."]
pub mod tex2stride;
#[doc = "TEX2RES (rw) register accessor: Image 2 resolution.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex2res::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex2res::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex2res`]
module"]
#[doc(alias = "TEX2RES")]
pub type Tex2res = crate::Reg<tex2res::Tex2resSpec>;
#[doc = "Image 2 resolution."]
pub mod tex2res;
#[doc = "TEX3BASE (rw) register accessor: Base address of the drawing surface 3 (must be word aligned).\n\nYou can [`read`](crate::Reg::read) this register and get [`tex3base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex3base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex3base`]
module"]
#[doc(alias = "TEX3BASE")]
pub type Tex3base = crate::Reg<tex3base::Tex3baseSpec>;
#[doc = "Base address of the drawing surface 3 (must be word aligned)."]
pub mod tex3base;
#[doc = "TEX3STRIDE (rw) register accessor: mode and stride.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex3stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex3stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex3stride`]
module"]
#[doc(alias = "TEX3STRIDE")]
pub type Tex3stride = crate::Reg<tex3stride::Tex3strideSpec>;
#[doc = "mode and stride."]
pub mod tex3stride;
#[doc = "TEX3RES (rw) register accessor: Image 3 resolution.\n\nYou can [`read`](crate::Reg::read) this register and get [`tex3res::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tex3res::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tex3res`]
module"]
#[doc(alias = "TEX3RES")]
pub type Tex3res = crate::Reg<tex3res::Tex3resSpec>;
#[doc = "Image 3 resolution."]
pub mod tex3res;
#[doc = "CGCMD (rw) register accessor: Clock gating enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgcmd`]
module"]
#[doc(alias = "CGCMD")]
pub type Cgcmd = crate::Reg<cgcmd::CgcmdSpec>;
#[doc = "Clock gating enable"]
pub mod cgcmd;
#[doc = "CGCTRL (rw) register accessor: CGCTRL register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`cgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgctrl`]
module"]
#[doc(alias = "CGCTRL")]
pub type Cgctrl = crate::Reg<cgctrl::CgctrlSpec>;
#[doc = "CGCTRL register description needed here."]
pub mod cgctrl;
#[doc = "DIRTYTRIGMIN (rw) register accessor: Resets dirty region to resolution size when written.\n\nYou can [`read`](crate::Reg::read) this register and get [`dirtytrigmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirtytrigmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirtytrigmin`]
module"]
#[doc(alias = "DIRTYTRIGMIN")]
pub type Dirtytrigmin = crate::Reg<dirtytrigmin::DirtytrigminSpec>;
#[doc = "Resets dirty region to resolution size when written."]
pub mod dirtytrigmin;
#[doc = "DIRTYTRIGMAX (rw) register accessor: Resets dirty region to resolution size when written.\n\nYou can [`read`](crate::Reg::read) this register and get [`dirtytrigmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirtytrigmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirtytrigmax`]
module"]
#[doc(alias = "DIRTYTRIGMAX")]
pub type Dirtytrigmax = crate::Reg<dirtytrigmax::DirtytrigmaxSpec>;
#[doc = "Resets dirty region to resolution size when written."]
pub mod dirtytrigmax;
#[doc = "STATUS (rw) register accessor: On read, returns GPU status (CHECK address!!).\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "On read, returns GPU status (CHECK address!!)."]
pub mod status;
#[doc = "BUSCTRL (rw) register accessor: Bus Control\n\nYou can [`read`](crate::Reg::read) this register and get [`busctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busctrl`]
module"]
#[doc(alias = "BUSCTRL")]
pub type Busctrl = crate::Reg<busctrl::BusctrlSpec>;
#[doc = "Bus Control"]
pub mod busctrl;
#[doc = "IMEMLDIADDR (rw) register accessor: Load shader instruction memory address.\n\nYou can [`read`](crate::Reg::read) this register and get [`imemldiaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imemldiaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imemldiaddr`]
module"]
#[doc(alias = "IMEMLDIADDR")]
pub type Imemldiaddr = crate::Reg<imemldiaddr::ImemldiaddrSpec>;
#[doc = "Load shader instruction memory address."]
pub mod imemldiaddr;
#[doc = "IMEMLDIDATAHL (rw) register accessor: Load shader instruction Memory data (31:0).\n\nYou can [`read`](crate::Reg::read) this register and get [`imemldidatahl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imemldidatahl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imemldidatahl`]
module"]
#[doc(alias = "IMEMLDIDATAHL")]
pub type Imemldidatahl = crate::Reg<imemldidatahl::ImemldidatahlSpec>;
#[doc = "Load shader instruction Memory data (31:0)."]
pub mod imemldidatahl;
#[doc = "IMEMLDIDATAHH (rw) register accessor: Load shader instruction Memory data (63:32).\n\nYou can [`read`](crate::Reg::read) this register and get [`imemldidatahh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imemldidatahh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imemldidatahh`]
module"]
#[doc(alias = "IMEMLDIDATAHH")]
pub type Imemldidatahh = crate::Reg<imemldidatahh::ImemldidatahhSpec>;
#[doc = "Load shader instruction Memory data (63:32)."]
pub mod imemldidatahh;
#[doc = "CMDLISTSTATUS (rw) register accessor: On read, returns command list processor status; On write, resets command list processor.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdliststatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdliststatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdliststatus`]
module"]
#[doc(alias = "CMDLISTSTATUS")]
pub type Cmdliststatus = crate::Reg<cmdliststatus::CmdliststatusSpec>;
#[doc = "On read, returns command list processor status; On write, resets command list processor."]
pub mod cmdliststatus;
#[doc = "CMDLISTRINGSTOP (rw) register accessor: Updates GPU command list pointer to stop executing.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdlistringstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdlistringstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdlistringstop`]
module"]
#[doc(alias = "CMDLISTRINGSTOP")]
pub type Cmdlistringstop = crate::Reg<cmdlistringstop::CmdlistringstopSpec>;
#[doc = "Updates GPU command list pointer to stop executing."]
pub mod cmdlistringstop;
#[doc = "CMDLISTADDR (rw) register accessor: Command list base pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdlistaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdlistaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdlistaddr`]
module"]
#[doc(alias = "CMDLISTADDR")]
pub type Cmdlistaddr = crate::Reg<cmdlistaddr::CmdlistaddrSpec>;
#[doc = "Command list base pointer."]
pub mod cmdlistaddr;
#[doc = "CMDLISTSIZE (rw) register accessor: Command list length in words.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdlistsize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdlistsize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdlistsize`]
module"]
#[doc(alias = "CMDLISTSIZE")]
pub type Cmdlistsize = crate::Reg<cmdlistsize::CmdlistsizeSpec>;
#[doc = "Command list length in words."]
pub mod cmdlistsize;
#[doc = "INTERRUPTCTRL (rw) register accessor: On write, clears the IRQ (CHECK address!).\n\nYou can [`read`](crate::Reg::read) this register and get [`interruptctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interruptctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interruptctrl`]
module"]
#[doc(alias = "INTERRUPTCTRL")]
pub type Interruptctrl = crate::Reg<interruptctrl::InterruptctrlSpec>;
#[doc = "On write, clears the IRQ (CHECK address!)."]
pub mod interruptctrl;
#[doc = "SYSCLEAR (rw) register accessor: On write, resets the GPU (CHECK address!).\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclear`]
module"]
#[doc(alias = "SYSCLEAR")]
pub type Sysclear = crate::Reg<sysclear::SysclearSpec>;
#[doc = "On write, resets the GPU (CHECK address!)."]
pub mod sysclear;
#[doc = "DRAWCMD (rw) register accessor: Rasterizer drawing command.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawcmd`]
module"]
#[doc(alias = "DRAWCMD")]
pub type Drawcmd = crate::Reg<drawcmd::DrawcmdSpec>;
#[doc = "Rasterizer drawing command."]
pub mod drawcmd;
#[doc = "DRAWPT0 (rw) register accessor: Stores only integer values. For greater accurancy DRAWPT0X and DRAWPT0Y registers are used which are 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt0`]
module"]
#[doc(alias = "DRAWPT0")]
pub type Drawpt0 = crate::Reg<drawpt0::Drawpt0Spec>;
#[doc = "Stores only integer values. For greater accurancy DRAWPT0X and DRAWPT0Y registers are used which are 16, 16 fixed point."]
pub mod drawpt0;
#[doc = "DRAWPT1 (rw) register accessor: Stores only integer values. Vertex 1 drawing primitive. Stores only integer values. For greater accurancy DRAWPT1X and DRAWPT1Y registers are used which are 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt1`]
module"]
#[doc(alias = "DRAWPT1")]
pub type Drawpt1 = crate::Reg<drawpt1::Drawpt1Spec>;
#[doc = "Stores only integer values. Vertex 1 drawing primitive. Stores only integer values. For greater accurancy DRAWPT1X and DRAWPT1Y registers are used which are 16, 16 fixed point."]
pub mod drawpt1;
#[doc = "CLIPMIN (rw) register accessor: Clipping rectangle upper left vertex.\n\nYou can [`read`](crate::Reg::read) this register and get [`clipmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clipmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clipmin`]
module"]
#[doc(alias = "CLIPMIN")]
pub type Clipmin = crate::Reg<clipmin::ClipminSpec>;
#[doc = "Clipping rectangle upper left vertex."]
pub mod clipmin;
#[doc = "CLIPMAX (rw) register accessor: Clipping rectangle bottom right vertex.\n\nYou can [`read`](crate::Reg::read) this register and get [`clipmax::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clipmax::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clipmax`]
module"]
#[doc(alias = "CLIPMAX")]
pub type Clipmax = crate::Reg<clipmax::ClipmaxSpec>;
#[doc = "Clipping rectangle bottom right vertex."]
pub mod clipmax;
#[doc = "RASTCTRL (rw) register accessor: Rasterizer matrix multiplication control\n\nYou can [`read`](crate::Reg::read) this register and get [`rastctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rastctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rastctrl`]
module"]
#[doc(alias = "RASTCTRL")]
pub type Rastctrl = crate::Reg<rastctrl::RastctrlSpec>;
#[doc = "Rasterizer matrix multiplication control"]
pub mod rastctrl;
#[doc = "DRAWCODEPTR (rw) register accessor: DRAWCODEPTR register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawcodeptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawcodeptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawcodeptr`]
module"]
#[doc(alias = "DRAWCODEPTR")]
pub type Drawcodeptr = crate::Reg<drawcodeptr::DrawcodeptrSpec>;
#[doc = "DRAWCODEPTR register description needed here."]
pub mod drawcodeptr;
#[doc = "DRAWPT0X (rw) register accessor: X coordinate of Vertex 0 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt0x::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt0x::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt0x`]
module"]
#[doc(alias = "DRAWPT0X")]
pub type Drawpt0x = crate::Reg<drawpt0x::Drawpt0xSpec>;
#[doc = "X coordinate of Vertex 0 drawing primitive 16, 16 fixed point."]
pub mod drawpt0x;
#[doc = "DRAWPT0Y (rw) register accessor: Y coordinate of Vertex 0 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt0y::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt0y::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt0y`]
module"]
#[doc(alias = "DRAWPT0Y")]
pub type Drawpt0y = crate::Reg<drawpt0y::Drawpt0ySpec>;
#[doc = "Y coordinate of Vertex 0 drawing primitive 16, 16 fixed point."]
pub mod drawpt0y;
#[doc = "DRAWPT0Z (rw) register accessor: DRAWPTOX register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt0z::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt0z::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt0z`]
module"]
#[doc(alias = "DRAWPT0Z")]
pub type Drawpt0z = crate::Reg<drawpt0z::Drawpt0zSpec>;
#[doc = "DRAWPTOX register description needed here."]
pub mod drawpt0z;
#[doc = "DRAWCOLOR (rw) register accessor: DRAWCOLOR register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawcolor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawcolor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawcolor`]
module"]
#[doc(alias = "DRAWCOLOR")]
pub type Drawcolor = crate::Reg<drawcolor::DrawcolorSpec>;
#[doc = "DRAWCOLOR register description needed here."]
pub mod drawcolor;
#[doc = "DRAWPT1X (rw) register accessor: X coordinate of Vertex 1 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt1x::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt1x::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt1x`]
module"]
#[doc(alias = "DRAWPT1X")]
pub type Drawpt1x = crate::Reg<drawpt1x::Drawpt1xSpec>;
#[doc = "X coordinate of Vertex 1 drawing primitive 16, 16 fixed point."]
pub mod drawpt1x;
#[doc = "DRAWPT1Y (rw) register accessor: Y coordinate of Vertex 1 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt1y::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt1y::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt1y`]
module"]
#[doc(alias = "DRAWPT1Y")]
pub type Drawpt1y = crate::Reg<drawpt1y::Drawpt1ySpec>;
#[doc = "Y coordinate of Vertex 1 drawing primitive 16, 16 fixed point."]
pub mod drawpt1y;
#[doc = "DRAWPT1Z (rw) register accessor: DRAWPT1Z register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt1z::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt1z::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt1z`]
module"]
#[doc(alias = "DRAWPT1Z")]
pub type Drawpt1z = crate::Reg<drawpt1z::Drawpt1zSpec>;
#[doc = "DRAWPT1Z register description needed here."]
pub mod drawpt1z;
#[doc = "DRAWPT2X (rw) register accessor: X coordinate of Vertex 2 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt2x::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt2x::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt2x`]
module"]
#[doc(alias = "DRAWPT2X")]
pub type Drawpt2x = crate::Reg<drawpt2x::Drawpt2xSpec>;
#[doc = "X coordinate of Vertex 2 drawing primitive 16, 16 fixed point."]
pub mod drawpt2x;
#[doc = "DRAWPT2Y (rw) register accessor: Y coordinate of Vertex 2 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt2y::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt2y::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt2y`]
module"]
#[doc(alias = "DRAWPT2Y")]
pub type Drawpt2y = crate::Reg<drawpt2y::Drawpt2ySpec>;
#[doc = "Y coordinate of Vertex 2 drawing primitive 16, 16 fixed point."]
pub mod drawpt2y;
#[doc = "DRAWPT2Z (rw) register accessor: DRAWPT2Z register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt2z::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt2z::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt2z`]
module"]
#[doc(alias = "DRAWPT2Z")]
pub type Drawpt2z = crate::Reg<drawpt2z::Drawpt2zSpec>;
#[doc = "DRAWPT2Z register description needed here."]
pub mod drawpt2z;
#[doc = "DRAWPT3X (rw) register accessor: X coordinate of Vertex 3 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt3x::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt3x::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt3x`]
module"]
#[doc(alias = "DRAWPT3X")]
pub type Drawpt3x = crate::Reg<drawpt3x::Drawpt3xSpec>;
#[doc = "X coordinate of Vertex 3 drawing primitive 16, 16 fixed point."]
pub mod drawpt3x;
#[doc = "DRAWPT3Y (rw) register accessor: Y coordinate of Vertex 3 drawing primitive 16, 16 fixed point.\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt3y::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt3y::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt3y`]
module"]
#[doc(alias = "DRAWPT3Y")]
pub type Drawpt3y = crate::Reg<drawpt3y::Drawpt3ySpec>;
#[doc = "Y coordinate of Vertex 3 drawing primitive 16, 16 fixed point."]
pub mod drawpt3y;
#[doc = "DRAWPT3Z (rw) register accessor: Fixed value (not accessible). Registers 0x160-0x180 are the elements of the 3x3 transformation matrix used for homogeneous conversion from screen coordinates to texture coordinates; the elements are floating points\n\nYou can [`read`](crate::Reg::read) this register and get [`drawpt3z::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drawpt3z::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drawpt3z`]
module"]
#[doc(alias = "DRAWPT3Z")]
pub type Drawpt3z = crate::Reg<drawpt3z::Drawpt3zSpec>;
#[doc = "Fixed value (not accessible). Registers 0x160-0x180 are the elements of the 3x3 transformation matrix used for homogeneous conversion from screen coordinates to texture coordinates; the elements are floating points"]
pub mod drawpt3z;
#[doc = "MM00 (rw) register accessor: matrix floating point element.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm00`]
module"]
#[doc(alias = "MM00")]
pub type Mm00 = crate::Reg<mm00::Mm00Spec>;
#[doc = "matrix floating point element."]
pub mod mm00;
#[doc = "MM01 (rw) register accessor: matrix floating point element.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm01`]
module"]
#[doc(alias = "MM01")]
pub type Mm01 = crate::Reg<mm01::Mm01Spec>;
#[doc = "matrix floating point element."]
pub mod mm01;
#[doc = "MM02 (rw) register accessor: matrix floating point element; sets to unit matrix if previously written element is MM12.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm02`]
module"]
#[doc(alias = "MM02")]
pub type Mm02 = crate::Reg<mm02::Mm02Spec>;
#[doc = "matrix floating point element; sets to unit matrix if previously written element is MM12."]
pub mod mm02;
#[doc = "MM10 (rw) register accessor: matrix floating point element.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm10`]
module"]
#[doc(alias = "MM10")]
pub type Mm10 = crate::Reg<mm10::Mm10Spec>;
#[doc = "matrix floating point element."]
pub mod mm10;
#[doc = "MM11 (rw) register accessor: matrix floating point element.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm11`]
module"]
#[doc(alias = "MM11")]
pub type Mm11 = crate::Reg<mm11::Mm11Spec>;
#[doc = "matrix floating point element."]
pub mod mm11;
#[doc = "MM12 (rw) register accessor: matrix floating point element.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm12`]
module"]
#[doc(alias = "MM12")]
pub type Mm12 = crate::Reg<mm12::Mm12Spec>;
#[doc = "matrix floating point element."]
pub mod mm12;
#[doc = "MM20 (rw) register accessor: matrix floating point element.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm20`]
module"]
#[doc(alias = "MM20")]
pub type Mm20 = crate::Reg<mm20::Mm20Spec>;
#[doc = "matrix floating point element."]
pub mod mm20;
#[doc = "MM21 (rw) register accessor: matrix floating point element.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm21`]
module"]
#[doc(alias = "MM21")]
pub type Mm21 = crate::Reg<mm21::Mm21Spec>;
#[doc = "matrix floating point element."]
pub mod mm21;
#[doc = "MM22 (rw) register accessor: matrix floating point element.\n\nYou can [`read`](crate::Reg::read) this register and get [`mm22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm22`]
module"]
#[doc(alias = "MM22")]
pub type Mm22 = crate::Reg<mm22::Mm22Spec>;
#[doc = "matrix floating point element."]
pub mod mm22;
#[doc = "DEPTHSTARTL (rw) register accessor: Depth value of START pixel, (32 low bits fractional.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthstartl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthstartl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depthstartl`]
module"]
#[doc(alias = "DEPTHSTARTL")]
pub type Depthstartl = crate::Reg<depthstartl::DepthstartlSpec>;
#[doc = "Depth value of START pixel, (32 low bits fractional.)"]
pub mod depthstartl;
#[doc = "DEPTHSTARTH (rw) register accessor: Depth value of START pixel, (32 high bits integral.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthstarth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthstarth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depthstarth`]
module"]
#[doc(alias = "DEPTHSTARTH")]
pub type Depthstarth = crate::Reg<depthstarth::DepthstarthSpec>;
#[doc = "Depth value of START pixel, (32 high bits integral.)"]
pub mod depthstarth;
#[doc = "DEPTHDXL (rw) register accessor: Added depth value for each step at x-axis (32 low bits fractional.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthdxl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthdxl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depthdxl`]
module"]
#[doc(alias = "DEPTHDXL")]
pub type Depthdxl = crate::Reg<depthdxl::DepthdxlSpec>;
#[doc = "Added depth value for each step at x-axis (32 low bits fractional.)"]
pub mod depthdxl;
#[doc = "DEPTHDXH (rw) register accessor: Added depth value for each step at x-axis (32 high bits integral.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthdxh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthdxh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depthdxh`]
module"]
#[doc(alias = "DEPTHDXH")]
pub type Depthdxh = crate::Reg<depthdxh::DepthdxhSpec>;
#[doc = "Added depth value for each step at x-axis (32 high bits integral.)"]
pub mod depthdxh;
#[doc = "DEPTHDYL (rw) register accessor: Added depth value for each step at y-axis (32 low bits fractional.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthdyl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthdyl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depthdyl`]
module"]
#[doc(alias = "DEPTHDYL")]
pub type Depthdyl = crate::Reg<depthdyl::DepthdylSpec>;
#[doc = "Added depth value for each step at y-axis (32 low bits fractional.)"]
pub mod depthdyl;
#[doc = "DEPTHDYH (rw) register accessor: Added depth value for each step at y-axis (32 high bits integral.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthdyh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthdyh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@depthdyh`]
module"]
#[doc(alias = "DEPTHDYH")]
pub type Depthdyh = crate::Reg<depthdyh::DepthdyhSpec>;
#[doc = "Added depth value for each step at y-axis (32 high bits integral.)"]
pub mod depthdyh;
#[doc = "REDX (rw) register accessor: Added red value for each step at x-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`redx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`redx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@redx`]
module"]
#[doc(alias = "REDX")]
pub type Redx = crate::Reg<redx::RedxSpec>;
#[doc = "Added red value for each step at x-axis, (16, 16 fixed point)"]
pub mod redx;
#[doc = "REDY (rw) register accessor: Added red value for each step at y-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`redy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`redy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@redy`]
module"]
#[doc(alias = "REDY")]
pub type Redy = crate::Reg<redy::RedySpec>;
#[doc = "Added red value for each step at y-axis, (16, 16 fixed point)"]
pub mod redy;
#[doc = "GREENX (rw) register accessor: Added green value for each step at x-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`greenx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`greenx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@greenx`]
module"]
#[doc(alias = "GREENX")]
pub type Greenx = crate::Reg<greenx::GreenxSpec>;
#[doc = "Added green value for each step at x-axis, (16, 16 fixed point)"]
pub mod greenx;
#[doc = "GREENY (rw) register accessor: Added green value for each step at y-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`greeny::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`greeny::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@greeny`]
module"]
#[doc(alias = "GREENY")]
pub type Greeny = crate::Reg<greeny::GreenySpec>;
#[doc = "Added green value for each step at y-axis, (16, 16 fixed point)"]
pub mod greeny;
#[doc = "BLUEX (rw) register accessor: Added blue value for each step at x-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`bluex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bluex`]
module"]
#[doc(alias = "BLUEX")]
pub type Bluex = crate::Reg<bluex::BluexSpec>;
#[doc = "Added blue value for each step at x-axis, (16, 16 fixed point)"]
pub mod bluex;
#[doc = "BLUEY (rw) register accessor: Added blue value for each step at y-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`bluey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bluey`]
module"]
#[doc(alias = "BLUEY")]
pub type Bluey = crate::Reg<bluey::BlueySpec>;
#[doc = "Added blue value for each step at y-axis, (16, 16 fixed point)"]
pub mod bluey;
#[doc = "ALFX (rw) register accessor: Added alfa value for each step at x-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`alfx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alfx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alfx`]
module"]
#[doc(alias = "ALFX")]
pub type Alfx = crate::Reg<alfx::AlfxSpec>;
#[doc = "Added alfa value for each step at x-axis, (16, 16 fixed point)"]
pub mod alfx;
#[doc = "ALFY (rw) register accessor: Added alfa value for each step at y-axis, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`alfy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alfy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alfy`]
module"]
#[doc(alias = "ALFY")]
pub type Alfy = crate::Reg<alfy::AlfySpec>;
#[doc = "Added alfa value for each step at y-axis, (16, 16 fixed point)"]
pub mod alfy;
#[doc = "REDINIT (rw) register accessor: Red value of STARTXY pixel, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`redinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`redinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@redinit`]
module"]
#[doc(alias = "REDINIT")]
pub type Redinit = crate::Reg<redinit::RedinitSpec>;
#[doc = "Red value of STARTXY pixel, (16, 16 fixed point)"]
pub mod redinit;
#[doc = "GREINIT (rw) register accessor: Green value of STARTXY pixel, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`greinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`greinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@greinit`]
module"]
#[doc(alias = "GREINIT")]
pub type Greinit = crate::Reg<greinit::GreinitSpec>;
#[doc = "Green value of STARTXY pixel, (16, 16 fixed point)"]
pub mod greinit;
#[doc = "BLUINIT (rw) register accessor: Blue value of STARTXY pixel, (16, 16 fixed point)\n\nYou can [`read`](crate::Reg::read) this register and get [`bluinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bluinit`]
module"]
#[doc(alias = "BLUINIT")]
pub type Bluinit = crate::Reg<bluinit::BluinitSpec>;
#[doc = "Blue value of STARTXY pixel, (16, 16 fixed point)"]
pub mod bluinit;
#[doc = "ALFINIT (rw) register accessor: Alfa value of STARTXY pixel, (16, 16 fixed point) Shader Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`alfinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alfinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alfinit`]
module"]
#[doc(alias = "ALFINIT")]
pub type Alfinit = crate::Reg<alfinit::AlfinitSpec>;
#[doc = "Alfa value of STARTXY pixel, (16, 16 fixed point) Shader Registers"]
pub mod alfinit;
#[doc = "IDREG (rw) register accessor: Fixed value\n\nYou can [`read`](crate::Reg::read) this register and get [`idreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idreg`]
module"]
#[doc(alias = "IDREG")]
pub type Idreg = crate::Reg<idreg::IdregSpec>;
#[doc = "Fixed value"]
pub mod idreg;
#[doc = "LOADCTRL (rw) register accessor: Load Control\n\nYou can [`read`](crate::Reg::read) this register and get [`loadctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loadctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loadctrl`]
module"]
#[doc(alias = "LOADCTRL")]
pub type Loadctrl = crate::Reg<loadctrl::LoadctrlSpec>;
#[doc = "Load Control"]
pub mod loadctrl;
#[doc = "C0REG (rw) register accessor: Shader constant register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`c0reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0reg`]
module"]
#[doc(alias = "C0REG")]
pub type C0reg = crate::Reg<c0reg::C0regSpec>;
#[doc = "Shader constant register 0."]
pub mod c0reg;
#[doc = "C1REG (rw) register accessor: Shader constant register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`c1reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1reg`]
module"]
#[doc(alias = "C1REG")]
pub type C1reg = crate::Reg<c1reg::C1regSpec>;
#[doc = "Shader constant register 1."]
pub mod c1reg;
#[doc = "C2REG (rw) register accessor: Shader constant register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`c2reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2reg`]
module"]
#[doc(alias = "C2REG")]
pub type C2reg = crate::Reg<c2reg::C2regSpec>;
#[doc = "Shader constant register 2."]
pub mod c2reg;
#[doc = "C3REG (rw) register accessor: Shader constant register 3, the dirty Region Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3reg`]
module"]
#[doc(alias = "C3REG")]
pub type C3reg = crate::Reg<c3reg::C3regSpec>;
#[doc = "Shader constant register 3, the dirty Region Register"]
pub mod c3reg;
#[doc = "IRQID (rw) register accessor: Signals interrupt when set (CHECK address!).\n\nYou can [`read`](crate::Reg::read) this register and get [`irqid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqid`]
module"]
#[doc(alias = "IRQID")]
pub type Irqid = crate::Reg<irqid::IrqidSpec>;
#[doc = "Signals interrupt when set (CHECK address!)."]
pub mod irqid;
