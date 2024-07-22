#[doc = "Register `DBGCTRL` reader"]
pub type R = crate::R<DbgctrlSpec>;
#[doc = "Register `DBGCTRL` writer"]
pub type W = crate::W<DbgctrlSpec>;
#[doc = "TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out trace data from ARM ITM and ETM modules through either SWO or TRACEDATA ports\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cm4tpiuenable {
    #[doc = "0: Disable the TPIU."]
    Dis = 0,
    #[doc = "1: Enable the TPIU."]
    En = 1,
}
impl From<Cm4tpiuenable> for bool {
    #[inline(always)]
    fn from(variant: Cm4tpiuenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CM4TPIUENABLE` reader - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out trace data from ARM ITM and ETM modules through either SWO or TRACEDATA ports"]
pub type Cm4tpiuenableR = crate::BitReader<Cm4tpiuenable>;
impl Cm4tpiuenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm4tpiuenable {
        match self.bits {
            false => Cm4tpiuenable::Dis,
            true => Cm4tpiuenable::En,
        }
    }
    #[doc = "Disable the TPIU."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cm4tpiuenable::Dis
    }
    #[doc = "Enable the TPIU."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cm4tpiuenable::En
    }
}
#[doc = "Field `CM4TPIUENABLE` writer - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out trace data from ARM ITM and ETM modules through either SWO or TRACEDATA ports"]
pub type Cm4tpiuenableW<'a, REG> = crate::BitWriter<'a, REG, Cm4tpiuenable>;
impl<'a, REG> Cm4tpiuenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the TPIU."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4tpiuenable::Dis)
    }
    #[doc = "Enable the TPIU."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4tpiuenable::En)
    }
}
#[doc = "This field selects the frequency of the ARM M4 TPIU port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm4clksel {
    #[doc = "0: Low power state."]
    Lowpwr = 0,
    #[doc = "1: Selects HFRC 96Mhz as the source TPIU clk"]
    Hfrc96 = 1,
    #[doc = "2: Selects HFRC 48Mhz as the source TPIU clk"]
    Hfrc48 = 2,
    #[doc = "3: Selects HFRC 24Mhz as the source TPIU clk"]
    Hfrc24 = 3,
    #[doc = "4: Selects HFRC 6Mhz as the source TPIU clk"]
    Hfrc6 = 4,
    #[doc = "5: Selects HFRC 3Mhz as the source TPIU clk"]
    Hfrc3 = 5,
    #[doc = "6: Selects HFRC 1.5Mhz as the source TPIU clk"]
    Hfrc1p5 = 6,
    #[doc = "7: Selects HFRC2 192Mhz as the source TPIU clk. Note that this setting requires CLKGEN.MISC.HFRC2 be enabled."]
    Hfrc2_192 = 7,
}
impl From<Cm4clksel> for u8 {
    #[inline(always)]
    fn from(variant: Cm4clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm4clksel {
    type Ux = u8;
}
impl crate::IsEnum for Cm4clksel {}
#[doc = "Field `CM4CLKSEL` reader - This field selects the frequency of the ARM M4 TPIU port."]
pub type Cm4clkselR = crate::FieldReader<Cm4clksel>;
impl Cm4clkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm4clksel {
        match self.bits {
            0 => Cm4clksel::Lowpwr,
            1 => Cm4clksel::Hfrc96,
            2 => Cm4clksel::Hfrc48,
            3 => Cm4clksel::Hfrc24,
            4 => Cm4clksel::Hfrc6,
            5 => Cm4clksel::Hfrc3,
            6 => Cm4clksel::Hfrc1p5,
            7 => Cm4clksel::Hfrc2_192,
            _ => unreachable!(),
        }
    }
    #[doc = "Low power state."]
    #[inline(always)]
    pub fn is_lowpwr(&self) -> bool {
        *self == Cm4clksel::Lowpwr
    }
    #[doc = "Selects HFRC 96Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn is_hfrc96(&self) -> bool {
        *self == Cm4clksel::Hfrc96
    }
    #[doc = "Selects HFRC 48Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn is_hfrc48(&self) -> bool {
        *self == Cm4clksel::Hfrc48
    }
    #[doc = "Selects HFRC 24Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn is_hfrc24(&self) -> bool {
        *self == Cm4clksel::Hfrc24
    }
    #[doc = "Selects HFRC 6Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn is_hfrc6(&self) -> bool {
        *self == Cm4clksel::Hfrc6
    }
    #[doc = "Selects HFRC 3Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn is_hfrc3(&self) -> bool {
        *self == Cm4clksel::Hfrc3
    }
    #[doc = "Selects HFRC 1.5Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn is_hfrc1p5(&self) -> bool {
        *self == Cm4clksel::Hfrc1p5
    }
    #[doc = "Selects HFRC2 192Mhz as the source TPIU clk. Note that this setting requires CLKGEN.MISC.HFRC2 be enabled."]
    #[inline(always)]
    pub fn is_hfrc2_192(&self) -> bool {
        *self == Cm4clksel::Hfrc2_192
    }
}
#[doc = "Field `CM4CLKSEL` writer - This field selects the frequency of the ARM M4 TPIU port."]
pub type Cm4clkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cm4clksel, crate::Safe>;
impl<'a, REG> Cm4clkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low power state."]
    #[inline(always)]
    pub fn lowpwr(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4clksel::Lowpwr)
    }
    #[doc = "Selects HFRC 96Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn hfrc96(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4clksel::Hfrc96)
    }
    #[doc = "Selects HFRC 48Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn hfrc48(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4clksel::Hfrc48)
    }
    #[doc = "Selects HFRC 24Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn hfrc24(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4clksel::Hfrc24)
    }
    #[doc = "Selects HFRC 6Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn hfrc6(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4clksel::Hfrc6)
    }
    #[doc = "Selects HFRC 3Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn hfrc3(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4clksel::Hfrc3)
    }
    #[doc = "Selects HFRC 1.5Mhz as the source TPIU clk"]
    #[inline(always)]
    pub fn hfrc1p5(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4clksel::Hfrc1p5)
    }
    #[doc = "Selects HFRC2 192Mhz as the source TPIU clk. Note that this setting requires CLKGEN.MISC.HFRC2 be enabled."]
    #[inline(always)]
    pub fn hfrc2_192(self) -> &'a mut crate::W<REG> {
        self.variant(Cm4clksel::Hfrc2_192)
    }
}
#[doc = "Debug subsystem ETB enable to store the trace data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgetbenable {
    #[doc = "0: Disable ETB."]
    Dis = 0,
    #[doc = "1: Enable ETB."]
    En = 1,
}
impl From<Dbgetbenable> for bool {
    #[inline(always)]
    fn from(variant: Dbgetbenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGETBENABLE` reader - Debug subsystem ETB enable to store the trace data."]
pub type DbgetbenableR = crate::BitReader<Dbgetbenable>;
impl DbgetbenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgetbenable {
        match self.bits {
            false => Dbgetbenable::Dis,
            true => Dbgetbenable::En,
        }
    }
    #[doc = "Disable ETB."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dbgetbenable::Dis
    }
    #[doc = "Enable ETB."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dbgetbenable::En
    }
}
#[doc = "Field `DBGETBENABLE` writer - Debug subsystem ETB enable to store the trace data."]
pub type DbgetbenableW<'a, REG> = crate::BitWriter<'a, REG, Dbgetbenable>;
impl<'a, REG> DbgetbenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ETB."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgetbenable::Dis)
    }
    #[doc = "Enable ETB."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgetbenable::En)
    }
}
#[doc = "Debug subsystem ETM trace enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgetmtraceen {
    #[doc = "0: Disable ETM trace."]
    Dis = 0,
    #[doc = "1: Enable ETM trace."]
    En = 1,
}
impl From<Dbgetmtraceen> for bool {
    #[inline(always)]
    fn from(variant: Dbgetmtraceen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGETMTRACEEN` reader - Debug subsystem ETM trace enable"]
pub type DbgetmtraceenR = crate::BitReader<Dbgetmtraceen>;
impl DbgetmtraceenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgetmtraceen {
        match self.bits {
            false => Dbgetmtraceen::Dis,
            true => Dbgetmtraceen::En,
        }
    }
    #[doc = "Disable ETM trace."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dbgetmtraceen::Dis
    }
    #[doc = "Enable ETM trace."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dbgetmtraceen::En
    }
}
#[doc = "Field `DBGETMTRACEEN` writer - Debug subsystem ETM trace enable"]
pub type DbgetmtraceenW<'a, REG> = crate::BitWriter<'a, REG, Dbgetmtraceen>;
impl<'a, REG> DbgetmtraceenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ETM trace."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgetmtraceen::Dis)
    }
    #[doc = "Enable ETM trace."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgetmtraceen::En)
    }
}
#[doc = "Debug subsystem DSP0 trace enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgdsp0traceen {
    #[doc = "0: Disable DSP0 trace."]
    Dis = 0,
    #[doc = "1: Enable DSP0 trace."]
    En = 1,
}
impl From<Dbgdsp0traceen> for bool {
    #[inline(always)]
    fn from(variant: Dbgdsp0traceen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGDSP0TRACEEN` reader - Debug subsystem DSP0 trace enable"]
pub type Dbgdsp0traceenR = crate::BitReader<Dbgdsp0traceen>;
impl Dbgdsp0traceenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgdsp0traceen {
        match self.bits {
            false => Dbgdsp0traceen::Dis,
            true => Dbgdsp0traceen::En,
        }
    }
    #[doc = "Disable DSP0 trace."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dbgdsp0traceen::Dis
    }
    #[doc = "Enable DSP0 trace."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dbgdsp0traceen::En
    }
}
#[doc = "Field `DBGDSP0TRACEEN` writer - Debug subsystem DSP0 trace enable"]
pub type Dbgdsp0traceenW<'a, REG> = crate::BitWriter<'a, REG, Dbgdsp0traceen>;
impl<'a, REG> Dbgdsp0traceenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DSP0 trace."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdsp0traceen::Dis)
    }
    #[doc = "Enable DSP0 trace."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdsp0traceen::En)
    }
}
#[doc = "Debug subsystem DSP1 trace enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgdsp1traceen {
    #[doc = "0: Disable DSP1 trace."]
    Dis = 0,
    #[doc = "1: Enable DSP1 trace."]
    En = 1,
}
impl From<Dbgdsp1traceen> for bool {
    #[inline(always)]
    fn from(variant: Dbgdsp1traceen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGDSP1TRACEEN` reader - Debug subsystem DSP1 trace enable"]
pub type Dbgdsp1traceenR = crate::BitReader<Dbgdsp1traceen>;
impl Dbgdsp1traceenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgdsp1traceen {
        match self.bits {
            false => Dbgdsp1traceen::Dis,
            true => Dbgdsp1traceen::En,
        }
    }
    #[doc = "Disable DSP1 trace."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dbgdsp1traceen::Dis
    }
    #[doc = "Enable DSP1 trace."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dbgdsp1traceen::En
    }
}
#[doc = "Field `DBGDSP1TRACEEN` writer - Debug subsystem DSP1 trace enable"]
pub type Dbgdsp1traceenW<'a, REG> = crate::BitWriter<'a, REG, Dbgdsp1traceen>;
impl<'a, REG> Dbgdsp1traceenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DSP1 trace."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdsp1traceen::Dis)
    }
    #[doc = "Enable DSP1 trace."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdsp1traceen::En)
    }
}
#[doc = "This field selects the frequency of the ARM M4 dbg ts port.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dbgtsclksel {
    #[doc = "0: Low power state."]
    Lowpwr = 0,
    #[doc = "1: Selects HFRC divided by 2 as the source dbg ts clk"]
    Hfrcdiv2 = 1,
    #[doc = "2: Selects HFRC divided by 8 as the source dbg ts clk"]
    Hfrcdiv8 = 2,
    #[doc = "3: Selects HFRC divided by 16 as the source dbg ts clk"]
    Hfrcdiv16 = 3,
    #[doc = "4: Selects HFRC divided by 32 as the source dbg ts clk"]
    Hfrcdiv32 = 4,
}
impl From<Dbgtsclksel> for u8 {
    #[inline(always)]
    fn from(variant: Dbgtsclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dbgtsclksel {
    type Ux = u8;
}
impl crate::IsEnum for Dbgtsclksel {}
#[doc = "Field `DBGTSCLKSEL` reader - This field selects the frequency of the ARM M4 dbg ts port."]
pub type DbgtsclkselR = crate::FieldReader<Dbgtsclksel>;
impl DbgtsclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dbgtsclksel> {
        match self.bits {
            0 => Some(Dbgtsclksel::Lowpwr),
            1 => Some(Dbgtsclksel::Hfrcdiv2),
            2 => Some(Dbgtsclksel::Hfrcdiv8),
            3 => Some(Dbgtsclksel::Hfrcdiv16),
            4 => Some(Dbgtsclksel::Hfrcdiv32),
            _ => None,
        }
    }
    #[doc = "Low power state."]
    #[inline(always)]
    pub fn is_lowpwr(&self) -> bool {
        *self == Dbgtsclksel::Lowpwr
    }
    #[doc = "Selects HFRC divided by 2 as the source dbg ts clk"]
    #[inline(always)]
    pub fn is_hfrcdiv2(&self) -> bool {
        *self == Dbgtsclksel::Hfrcdiv2
    }
    #[doc = "Selects HFRC divided by 8 as the source dbg ts clk"]
    #[inline(always)]
    pub fn is_hfrcdiv8(&self) -> bool {
        *self == Dbgtsclksel::Hfrcdiv8
    }
    #[doc = "Selects HFRC divided by 16 as the source dbg ts clk"]
    #[inline(always)]
    pub fn is_hfrcdiv16(&self) -> bool {
        *self == Dbgtsclksel::Hfrcdiv16
    }
    #[doc = "Selects HFRC divided by 32 as the source dbg ts clk"]
    #[inline(always)]
    pub fn is_hfrcdiv32(&self) -> bool {
        *self == Dbgtsclksel::Hfrcdiv32
    }
}
#[doc = "Field `DBGTSCLKSEL` writer - This field selects the frequency of the ARM M4 dbg ts port."]
pub type DbgtsclkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dbgtsclksel>;
impl<'a, REG> DbgtsclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low power state."]
    #[inline(always)]
    pub fn lowpwr(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgtsclksel::Lowpwr)
    }
    #[doc = "Selects HFRC divided by 2 as the source dbg ts clk"]
    #[inline(always)]
    pub fn hfrcdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgtsclksel::Hfrcdiv2)
    }
    #[doc = "Selects HFRC divided by 8 as the source dbg ts clk"]
    #[inline(always)]
    pub fn hfrcdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgtsclksel::Hfrcdiv8)
    }
    #[doc = "Selects HFRC divided by 16 as the source dbg ts clk"]
    #[inline(always)]
    pub fn hfrcdiv16(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgtsclksel::Hfrcdiv16)
    }
    #[doc = "Selects HFRC divided by 32 as the source dbg ts clk"]
    #[inline(always)]
    pub fn hfrcdiv32(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgtsclksel::Hfrcdiv32)
    }
}
#[doc = "Debug subsystem DSP0 OCD Halt on Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgdsp0ocdhaltonrst {
    #[doc = "0: Disable DSP0 OCD Halt on Reset."]
    Dis = 0,
    #[doc = "1: Enable DSP0 OCD Halt on Reset."]
    En = 1,
}
impl From<Dbgdsp0ocdhaltonrst> for bool {
    #[inline(always)]
    fn from(variant: Dbgdsp0ocdhaltonrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGDSP0OCDHALTONRST` reader - Debug subsystem DSP0 OCD Halt on Reset"]
pub type Dbgdsp0ocdhaltonrstR = crate::BitReader<Dbgdsp0ocdhaltonrst>;
impl Dbgdsp0ocdhaltonrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgdsp0ocdhaltonrst {
        match self.bits {
            false => Dbgdsp0ocdhaltonrst::Dis,
            true => Dbgdsp0ocdhaltonrst::En,
        }
    }
    #[doc = "Disable DSP0 OCD Halt on Reset."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dbgdsp0ocdhaltonrst::Dis
    }
    #[doc = "Enable DSP0 OCD Halt on Reset."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dbgdsp0ocdhaltonrst::En
    }
}
#[doc = "Field `DBGDSP0OCDHALTONRST` writer - Debug subsystem DSP0 OCD Halt on Reset"]
pub type Dbgdsp0ocdhaltonrstW<'a, REG> = crate::BitWriter<'a, REG, Dbgdsp0ocdhaltonrst>;
impl<'a, REG> Dbgdsp0ocdhaltonrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DSP0 OCD Halt on Reset."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdsp0ocdhaltonrst::Dis)
    }
    #[doc = "Enable DSP0 OCD Halt on Reset."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdsp0ocdhaltonrst::En)
    }
}
#[doc = "Debug subsystem DSP1 OCD Halt on Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgdsp1ocdhaltonrst {
    #[doc = "0: Disable DSP1 OCD Halt on Reset."]
    Dis = 0,
    #[doc = "1: Enable DSP1 OCD Halt on Reset."]
    En = 1,
}
impl From<Dbgdsp1ocdhaltonrst> for bool {
    #[inline(always)]
    fn from(variant: Dbgdsp1ocdhaltonrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGDSP1OCDHALTONRST` reader - Debug subsystem DSP1 OCD Halt on Reset"]
pub type Dbgdsp1ocdhaltonrstR = crate::BitReader<Dbgdsp1ocdhaltonrst>;
impl Dbgdsp1ocdhaltonrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgdsp1ocdhaltonrst {
        match self.bits {
            false => Dbgdsp1ocdhaltonrst::Dis,
            true => Dbgdsp1ocdhaltonrst::En,
        }
    }
    #[doc = "Disable DSP1 OCD Halt on Reset."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dbgdsp1ocdhaltonrst::Dis
    }
    #[doc = "Enable DSP1 OCD Halt on Reset."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dbgdsp1ocdhaltonrst::En
    }
}
#[doc = "Field `DBGDSP1OCDHALTONRST` writer - Debug subsystem DSP1 OCD Halt on Reset"]
pub type Dbgdsp1ocdhaltonrstW<'a, REG> = crate::BitWriter<'a, REG, Dbgdsp1ocdhaltonrst>;
impl<'a, REG> Dbgdsp1ocdhaltonrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DSP1 OCD Halt on Reset."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdsp1ocdhaltonrst::Dis)
    }
    #[doc = "Enable DSP1 OCD Halt on Reset."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdsp1ocdhaltonrst::En)
    }
}
impl R {
    #[doc = "Bit 0 - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out trace data from ARM ITM and ETM modules through either SWO or TRACEDATA ports"]
    #[inline(always)]
    pub fn cm4tpiuenable(&self) -> Cm4tpiuenableR {
        Cm4tpiuenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - This field selects the frequency of the ARM M4 TPIU port."]
    #[inline(always)]
    pub fn cm4clksel(&self) -> Cm4clkselR {
        Cm4clkselR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - Debug subsystem ETB enable to store the trace data."]
    #[inline(always)]
    pub fn dbgetbenable(&self) -> DbgetbenableR {
        DbgetbenableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug subsystem ETM trace enable"]
    #[inline(always)]
    pub fn dbgetmtraceen(&self) -> DbgetmtraceenR {
        DbgetmtraceenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug subsystem DSP0 trace enable"]
    #[inline(always)]
    pub fn dbgdsp0traceen(&self) -> Dbgdsp0traceenR {
        Dbgdsp0traceenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug subsystem DSP1 trace enable"]
    #[inline(always)]
    pub fn dbgdsp1traceen(&self) -> Dbgdsp1traceenR {
        Dbgdsp1traceenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - This field selects the frequency of the ARM M4 dbg ts port."]
    #[inline(always)]
    pub fn dbgtsclksel(&self) -> DbgtsclkselR {
        DbgtsclkselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Debug subsystem DSP0 OCD Halt on Reset"]
    #[inline(always)]
    pub fn dbgdsp0ocdhaltonrst(&self) -> Dbgdsp0ocdhaltonrstR {
        Dbgdsp0ocdhaltonrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Debug subsystem DSP1 OCD Halt on Reset"]
    #[inline(always)]
    pub fn dbgdsp1ocdhaltonrst(&self) -> Dbgdsp1ocdhaltonrstR {
        Dbgdsp1ocdhaltonrstR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TPIU Enable field. When set, the ARM M4 TPIU is enabled and data can be streamed out trace data from ARM ITM and ETM modules through either SWO or TRACEDATA ports"]
    #[inline(always)]
    #[must_use]
    pub fn cm4tpiuenable(&mut self) -> Cm4tpiuenableW<DbgctrlSpec> {
        Cm4tpiuenableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - This field selects the frequency of the ARM M4 TPIU port."]
    #[inline(always)]
    #[must_use]
    pub fn cm4clksel(&mut self) -> Cm4clkselW<DbgctrlSpec> {
        Cm4clkselW::new(self, 1)
    }
    #[doc = "Bit 8 - Debug subsystem ETB enable to store the trace data."]
    #[inline(always)]
    #[must_use]
    pub fn dbgetbenable(&mut self) -> DbgetbenableW<DbgctrlSpec> {
        DbgetbenableW::new(self, 8)
    }
    #[doc = "Bit 9 - Debug subsystem ETM trace enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgetmtraceen(&mut self) -> DbgetmtraceenW<DbgctrlSpec> {
        DbgetmtraceenW::new(self, 9)
    }
    #[doc = "Bit 10 - Debug subsystem DSP0 trace enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgdsp0traceen(&mut self) -> Dbgdsp0traceenW<DbgctrlSpec> {
        Dbgdsp0traceenW::new(self, 10)
    }
    #[doc = "Bit 11 - Debug subsystem DSP1 trace enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgdsp1traceen(&mut self) -> Dbgdsp1traceenW<DbgctrlSpec> {
        Dbgdsp1traceenW::new(self, 11)
    }
    #[doc = "Bits 12:14 - This field selects the frequency of the ARM M4 dbg ts port."]
    #[inline(always)]
    #[must_use]
    pub fn dbgtsclksel(&mut self) -> DbgtsclkselW<DbgctrlSpec> {
        DbgtsclkselW::new(self, 12)
    }
    #[doc = "Bit 16 - Debug subsystem DSP0 OCD Halt on Reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgdsp0ocdhaltonrst(&mut self) -> Dbgdsp0ocdhaltonrstW<DbgctrlSpec> {
        Dbgdsp0ocdhaltonrstW::new(self, 16)
    }
    #[doc = "Bit 17 - Debug subsystem DSP1 OCD Halt on Reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgdsp1ocdhaltonrst(&mut self) -> Dbgdsp1ocdhaltonrstW<DbgctrlSpec> {
        Dbgdsp1ocdhaltonrstW::new(self, 17)
    }
}
#[doc = "Debug subsystem Control. Determines the debug components enable and clk frequency.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgctrlSpec;
impl crate::RegisterSpec for DbgctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgctrl::R`](R) reader structure"]
impl crate::Readable for DbgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgctrl::W`](W) writer structure"]
impl crate::Writable for DbgctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGCTRL to value 0x4000"]
impl crate::Resettable for DbgctrlSpec {
    const RESET_VALUE: u32 = 0x4000;
}
