#[doc = "Register `DSIFUNCPRG` reader"]
pub type R = crate::R<DsifuncprgSpec>;
#[doc = "Register `DSIFUNCPRG` writer"]
pub type W = crate::W<DsifuncprgSpec>;
#[doc = "The number Data lanes to be supported is programmed by the processor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datalanes {
    #[doc = "0: Zero data lane"]
    Datal0 = 0,
    #[doc = "1: One data lane"]
    Datal1 = 1,
    #[doc = "2: Two data lane"]
    Datal2 = 2,
    #[doc = "3: Three data lane"]
    Datal3 = 3,
    #[doc = "4: Four data lane"]
    Datal4 = 4,
}
impl From<Datalanes> for u8 {
    #[inline(always)]
    fn from(variant: Datalanes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datalanes {
    type Ux = u8;
}
impl crate::IsEnum for Datalanes {}
#[doc = "Field `DATALANES` reader - The number Data lanes to be supported is programmed by the processor"]
pub type DatalanesR = crate::FieldReader<Datalanes>;
impl DatalanesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datalanes> {
        match self.bits {
            0 => Some(Datalanes::Datal0),
            1 => Some(Datalanes::Datal1),
            2 => Some(Datalanes::Datal2),
            3 => Some(Datalanes::Datal3),
            4 => Some(Datalanes::Datal4),
            _ => None,
        }
    }
    #[doc = "Zero data lane"]
    #[inline(always)]
    pub fn is_datal0(&self) -> bool {
        *self == Datalanes::Datal0
    }
    #[doc = "One data lane"]
    #[inline(always)]
    pub fn is_datal1(&self) -> bool {
        *self == Datalanes::Datal1
    }
    #[doc = "Two data lane"]
    #[inline(always)]
    pub fn is_datal2(&self) -> bool {
        *self == Datalanes::Datal2
    }
    #[doc = "Three data lane"]
    #[inline(always)]
    pub fn is_datal3(&self) -> bool {
        *self == Datalanes::Datal3
    }
    #[doc = "Four data lane"]
    #[inline(always)]
    pub fn is_datal4(&self) -> bool {
        *self == Datalanes::Datal4
    }
}
#[doc = "Field `DATALANES` writer - The number Data lanes to be supported is programmed by the processor"]
pub type DatalanesW<'a, REG> = crate::FieldWriter<'a, REG, 3, Datalanes>;
impl<'a, REG> DatalanesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero data lane"]
    #[inline(always)]
    pub fn datal0(self) -> &'a mut crate::W<REG> {
        self.variant(Datalanes::Datal0)
    }
    #[doc = "One data lane"]
    #[inline(always)]
    pub fn datal1(self) -> &'a mut crate::W<REG> {
        self.variant(Datalanes::Datal1)
    }
    #[doc = "Two data lane"]
    #[inline(always)]
    pub fn datal2(self) -> &'a mut crate::W<REG> {
        self.variant(Datalanes::Datal2)
    }
    #[doc = "Three data lane"]
    #[inline(always)]
    pub fn datal3(self) -> &'a mut crate::W<REG> {
        self.variant(Datalanes::Datal3)
    }
    #[doc = "Four data lane"]
    #[inline(always)]
    pub fn datal4(self) -> &'a mut crate::W<REG> {
        self.variant(Datalanes::Datal4)
    }
}
#[doc = "Channel number for video mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chnumvm {
    #[doc = "0: Virtual video mode channel 0"]
    Vvch0 = 0,
    #[doc = "1: Virtual video mode channel 1"]
    Vvch1 = 1,
    #[doc = "2: Virtual video mode channel 2"]
    Vvch2 = 2,
    #[doc = "3: Virtual video mode channel 3"]
    Vvch3 = 3,
}
impl From<Chnumvm> for u8 {
    #[inline(always)]
    fn from(variant: Chnumvm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chnumvm {
    type Ux = u8;
}
impl crate::IsEnum for Chnumvm {}
#[doc = "Field `CHNUMVM` reader - Channel number for video mode"]
pub type ChnumvmR = crate::FieldReader<Chnumvm>;
impl ChnumvmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnumvm {
        match self.bits {
            0 => Chnumvm::Vvch0,
            1 => Chnumvm::Vvch1,
            2 => Chnumvm::Vvch2,
            3 => Chnumvm::Vvch3,
            _ => unreachable!(),
        }
    }
    #[doc = "Virtual video mode channel 0"]
    #[inline(always)]
    pub fn is_vvch0(&self) -> bool {
        *self == Chnumvm::Vvch0
    }
    #[doc = "Virtual video mode channel 1"]
    #[inline(always)]
    pub fn is_vvch1(&self) -> bool {
        *self == Chnumvm::Vvch1
    }
    #[doc = "Virtual video mode channel 2"]
    #[inline(always)]
    pub fn is_vvch2(&self) -> bool {
        *self == Chnumvm::Vvch2
    }
    #[doc = "Virtual video mode channel 3"]
    #[inline(always)]
    pub fn is_vvch3(&self) -> bool {
        *self == Chnumvm::Vvch3
    }
}
#[doc = "Field `CHNUMVM` writer - Channel number for video mode"]
pub type ChnumvmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chnumvm, crate::Safe>;
impl<'a, REG> ChnumvmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Virtual video mode channel 0"]
    #[inline(always)]
    pub fn vvch0(self) -> &'a mut crate::W<REG> {
        self.variant(Chnumvm::Vvch0)
    }
    #[doc = "Virtual video mode channel 1"]
    #[inline(always)]
    pub fn vvch1(self) -> &'a mut crate::W<REG> {
        self.variant(Chnumvm::Vvch1)
    }
    #[doc = "Virtual video mode channel 2"]
    #[inline(always)]
    pub fn vvch2(self) -> &'a mut crate::W<REG> {
        self.variant(Chnumvm::Vvch2)
    }
    #[doc = "Virtual video mode channel 3"]
    #[inline(always)]
    pub fn vvch3(self) -> &'a mut crate::W<REG> {
        self.variant(Chnumvm::Vvch3)
    }
}
#[doc = "Channel Number for command mode is programmed by the processor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chnumcmode {
    #[doc = "0: Virtual command mode channel 0"]
    Vcch0 = 0,
    #[doc = "1: Virtual command mode channel 1"]
    Vcch1 = 1,
    #[doc = "2: Virtual command mode channel 2"]
    Vcch2 = 2,
    #[doc = "3: Virtual command mode channel 3"]
    Vcch3 = 3,
}
impl From<Chnumcmode> for u8 {
    #[inline(always)]
    fn from(variant: Chnumcmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chnumcmode {
    type Ux = u8;
}
impl crate::IsEnum for Chnumcmode {}
#[doc = "Field `CHNUMCMODE` reader - Channel Number for command mode is programmed by the processor"]
pub type ChnumcmodeR = crate::FieldReader<Chnumcmode>;
impl ChnumcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chnumcmode {
        match self.bits {
            0 => Chnumcmode::Vcch0,
            1 => Chnumcmode::Vcch1,
            2 => Chnumcmode::Vcch2,
            3 => Chnumcmode::Vcch3,
            _ => unreachable!(),
        }
    }
    #[doc = "Virtual command mode channel 0"]
    #[inline(always)]
    pub fn is_vcch0(&self) -> bool {
        *self == Chnumcmode::Vcch0
    }
    #[doc = "Virtual command mode channel 1"]
    #[inline(always)]
    pub fn is_vcch1(&self) -> bool {
        *self == Chnumcmode::Vcch1
    }
    #[doc = "Virtual command mode channel 2"]
    #[inline(always)]
    pub fn is_vcch2(&self) -> bool {
        *self == Chnumcmode::Vcch2
    }
    #[doc = "Virtual command mode channel 3"]
    #[inline(always)]
    pub fn is_vcch3(&self) -> bool {
        *self == Chnumcmode::Vcch3
    }
}
#[doc = "Field `CHNUMCMODE` writer - Channel Number for command mode is programmed by the processor"]
pub type ChnumcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Chnumcmode, crate::Safe>;
impl<'a, REG> ChnumcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Virtual command mode channel 0"]
    #[inline(always)]
    pub fn vcch0(self) -> &'a mut crate::W<REG> {
        self.variant(Chnumcmode::Vcch0)
    }
    #[doc = "Virtual command mode channel 1"]
    #[inline(always)]
    pub fn vcch1(self) -> &'a mut crate::W<REG> {
        self.variant(Chnumcmode::Vcch1)
    }
    #[doc = "Virtual command mode channel 2"]
    #[inline(always)]
    pub fn vcch2(self) -> &'a mut crate::W<REG> {
        self.variant(Chnumcmode::Vcch2)
    }
    #[doc = "Virtual command mode channel 3"]
    #[inline(always)]
    pub fn vcch3(self) -> &'a mut crate::W<REG> {
        self.variant(Chnumcmode::Vcch3)
    }
}
#[doc = "Supported colour format for video mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Supcolvidmode {
    #[doc = "0: Video mode is not supported"]
    Fmtvmode0 = 0,
    #[doc = "1: RGB565 or 16-bit format"]
    Fmtvmode1 = 1,
    #[doc = "2: RGB666 or 18-bit format"]
    Fmtvmode2 = 2,
    #[doc = "3: RGB 666 loosely packed format"]
    Fmtvmode3 = 3,
    #[doc = "4: RGB888 or 24-bit format"]
    Fmtvmode4 = 4,
}
impl From<Supcolvidmode> for u8 {
    #[inline(always)]
    fn from(variant: Supcolvidmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Supcolvidmode {
    type Ux = u8;
}
impl crate::IsEnum for Supcolvidmode {}
#[doc = "Field `SUPCOLVIDMODE` reader - Supported colour format for video mode."]
pub type SupcolvidmodeR = crate::FieldReader<Supcolvidmode>;
impl SupcolvidmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Supcolvidmode> {
        match self.bits {
            0 => Some(Supcolvidmode::Fmtvmode0),
            1 => Some(Supcolvidmode::Fmtvmode1),
            2 => Some(Supcolvidmode::Fmtvmode2),
            3 => Some(Supcolvidmode::Fmtvmode3),
            4 => Some(Supcolvidmode::Fmtvmode4),
            _ => None,
        }
    }
    #[doc = "Video mode is not supported"]
    #[inline(always)]
    pub fn is_fmtvmode0(&self) -> bool {
        *self == Supcolvidmode::Fmtvmode0
    }
    #[doc = "RGB565 or 16-bit format"]
    #[inline(always)]
    pub fn is_fmtvmode1(&self) -> bool {
        *self == Supcolvidmode::Fmtvmode1
    }
    #[doc = "RGB666 or 18-bit format"]
    #[inline(always)]
    pub fn is_fmtvmode2(&self) -> bool {
        *self == Supcolvidmode::Fmtvmode2
    }
    #[doc = "RGB 666 loosely packed format"]
    #[inline(always)]
    pub fn is_fmtvmode3(&self) -> bool {
        *self == Supcolvidmode::Fmtvmode3
    }
    #[doc = "RGB888 or 24-bit format"]
    #[inline(always)]
    pub fn is_fmtvmode4(&self) -> bool {
        *self == Supcolvidmode::Fmtvmode4
    }
}
#[doc = "Field `SUPCOLVIDMODE` writer - Supported colour format for video mode."]
pub type SupcolvidmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Supcolvidmode>;
impl<'a, REG> SupcolvidmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Video mode is not supported"]
    #[inline(always)]
    pub fn fmtvmode0(self) -> &'a mut crate::W<REG> {
        self.variant(Supcolvidmode::Fmtvmode0)
    }
    #[doc = "RGB565 or 16-bit format"]
    #[inline(always)]
    pub fn fmtvmode1(self) -> &'a mut crate::W<REG> {
        self.variant(Supcolvidmode::Fmtvmode1)
    }
    #[doc = "RGB666 or 18-bit format"]
    #[inline(always)]
    pub fn fmtvmode2(self) -> &'a mut crate::W<REG> {
        self.variant(Supcolvidmode::Fmtvmode2)
    }
    #[doc = "RGB 666 loosely packed format"]
    #[inline(always)]
    pub fn fmtvmode3(self) -> &'a mut crate::W<REG> {
        self.variant(Supcolvidmode::Fmtvmode3)
    }
    #[doc = "RGB888 or 24-bit format"]
    #[inline(always)]
    pub fn fmtvmode4(self) -> &'a mut crate::W<REG> {
        self.variant(Supcolvidmode::Fmtvmode4)
    }
}
#[doc = "Field description needed here.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Regname {
    #[doc = "0: mode is not supported\\]"]
    Command = 0,
    #[doc = "1: 16 bit data"]
    _16bit = 1,
    #[doc = "2: 9 bit data"]
    _9bit = 2,
    #[doc = "3: 8 bit data"]
    _8bit = 3,
}
impl From<Regname> for u8 {
    #[inline(always)]
    fn from(variant: Regname) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Regname {
    type Ux = u8;
}
impl crate::IsEnum for Regname {}
#[doc = "Field `REGNAME` reader - Field description needed here."]
pub type RegnameR = crate::FieldReader<Regname>;
impl RegnameR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Regname> {
        match self.bits {
            0 => Some(Regname::Command),
            1 => Some(Regname::_16bit),
            2 => Some(Regname::_9bit),
            3 => Some(Regname::_8bit),
            _ => None,
        }
    }
    #[doc = "mode is not supported\\]"]
    #[inline(always)]
    pub fn is_command(&self) -> bool {
        *self == Regname::Command
    }
    #[doc = "16 bit data"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Regname::_16bit
    }
    #[doc = "9 bit data"]
    #[inline(always)]
    pub fn is_9bit(&self) -> bool {
        *self == Regname::_9bit
    }
    #[doc = "8 bit data"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Regname::_8bit
    }
}
#[doc = "Field `REGNAME` writer - Field description needed here."]
pub type RegnameW<'a, REG> = crate::FieldWriter<'a, REG, 3, Regname>;
impl<'a, REG> RegnameW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "mode is not supported\\]"]
    #[inline(always)]
    pub fn command(self) -> &'a mut crate::W<REG> {
        self.variant(Regname::Command)
    }
    #[doc = "16 bit data"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Regname::_16bit)
    }
    #[doc = "9 bit data"]
    #[inline(always)]
    pub fn _9bit(self) -> &'a mut crate::W<REG> {
        self.variant(Regname::_9bit)
    }
    #[doc = "8 bit data"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Regname::_8bit)
    }
}
impl R {
    #[doc = "Bits 0:2 - The number Data lanes to be supported is programmed by the processor"]
    #[inline(always)]
    pub fn datalanes(&self) -> DatalanesR {
        DatalanesR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Channel number for video mode"]
    #[inline(always)]
    pub fn chnumvm(&self) -> ChnumvmR {
        ChnumvmR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Channel Number for command mode is programmed by the processor"]
    #[inline(always)]
    pub fn chnumcmode(&self) -> ChnumcmodeR {
        ChnumcmodeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9 - Supported colour format for video mode."]
    #[inline(always)]
    pub fn supcolvidmode(&self) -> SupcolvidmodeR {
        SupcolvidmodeR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Field description needed here."]
    #[inline(always)]
    pub fn regname(&self) -> RegnameR {
        RegnameR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The number Data lanes to be supported is programmed by the processor"]
    #[inline(always)]
    #[must_use]
    pub fn datalanes(&mut self) -> DatalanesW<DsifuncprgSpec> {
        DatalanesW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Channel number for video mode"]
    #[inline(always)]
    #[must_use]
    pub fn chnumvm(&mut self) -> ChnumvmW<DsifuncprgSpec> {
        ChnumvmW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Channel Number for command mode is programmed by the processor"]
    #[inline(always)]
    #[must_use]
    pub fn chnumcmode(&mut self) -> ChnumcmodeW<DsifuncprgSpec> {
        ChnumcmodeW::new(self, 5)
    }
    #[doc = "Bits 7:9 - Supported colour format for video mode."]
    #[inline(always)]
    #[must_use]
    pub fn supcolvidmode(&mut self) -> SupcolvidmodeW<DsifuncprgSpec> {
        SupcolvidmodeW::new(self, 7)
    }
    #[doc = "Bits 13:15 - Field description needed here."]
    #[inline(always)]
    #[must_use]
    pub fn regname(&mut self) -> RegnameW<DsifuncprgSpec> {
        RegnameW::new(self, 13)
    }
}
#[doc = "DSI function programming register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsifuncprg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsifuncprg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsifuncprgSpec;
impl crate::RegisterSpec for DsifuncprgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsifuncprg::R`](R) reader structure"]
impl crate::Readable for DsifuncprgSpec {}
#[doc = "`write(|w| ..)` method takes [`dsifuncprg::W`](W) writer structure"]
impl crate::Writable for DsifuncprgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSIFUNCPRG to value 0x01"]
impl crate::Resettable for DsifuncprgSpec {
    const RESET_VALUE: u32 = 0x01;
}
