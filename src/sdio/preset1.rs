#[doc = "Register `PRESET1` reader"]
pub type R = crate::R<Preset1Spec>;
#[doc = "Register `PRESET1` writer"]
pub type W = crate::W<Preset1Spec>;
#[doc = "Field `HSSDCLKFREQSEL` reader - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type HssdclkfreqselR = crate::FieldReader<u16>;
#[doc = "Field `HSSDCLKFREQSEL` writer - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type HssdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit is effective when Host Controller supports programmable clock generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsclkgensel {
    #[doc = "1: Programmable Clock Generator"]
    Progclk = 1,
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator"]
    Hostctlr = 0,
}
impl From<Hsclkgensel> for bool {
    #[inline(always)]
    fn from(variant: Hsclkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKGENSEL` reader - This bit is effective when Host Controller supports programmable clock generator."]
pub type HsclkgenselR = crate::BitReader<Hsclkgensel>;
impl HsclkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsclkgensel {
        match self.bits {
            true => Hsclkgensel::Progclk,
            false => Hsclkgensel::Hostctlr,
        }
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_progclk(&self) -> bool {
        *self == Hsclkgensel::Progclk
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_hostctlr(&self) -> bool {
        *self == Hsclkgensel::Hostctlr
    }
}
#[doc = "Field `HSCLKGENSEL` writer - This bit is effective when Host Controller supports programmable clock generator."]
pub type HsclkgenselW<'a, REG> = crate::BitWriter<'a, REG, Hsclkgensel>;
impl<'a, REG> HsclkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn progclk(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkgensel::Progclk)
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn hostctlr(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkgensel::Hostctlr)
    }
}
#[doc = "Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hsdrvrstrsel {
    #[doc = "3: Driver Type D is Selected"]
    Typed = 3,
    #[doc = "2: Driver Type C is Selected"]
    Typec = 2,
    #[doc = "1: Driver Type A is Selected"]
    Typea = 1,
    #[doc = "0: Driver Type B is Selected"]
    Typeb = 0,
}
impl From<Hsdrvrstrsel> for u8 {
    #[inline(always)]
    fn from(variant: Hsdrvrstrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hsdrvrstrsel {
    type Ux = u8;
}
impl crate::IsEnum for Hsdrvrstrsel {}
#[doc = "Field `HSDRVRSTRSEL` reader - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type HsdrvrstrselR = crate::FieldReader<Hsdrvrstrsel>;
impl HsdrvrstrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsdrvrstrsel {
        match self.bits {
            3 => Hsdrvrstrsel::Typed,
            2 => Hsdrvrstrsel::Typec,
            1 => Hsdrvrstrsel::Typea,
            0 => Hsdrvrstrsel::Typeb,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Hsdrvrstrsel::Typed
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Hsdrvrstrsel::Typec
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Hsdrvrstrsel::Typea
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Hsdrvrstrsel::Typeb
    }
}
#[doc = "Field `HSDRVRSTRSEL` writer - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type HsdrvrstrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hsdrvrstrsel, crate::Safe>;
impl<'a, REG> HsdrvrstrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(Hsdrvrstrsel::Typed)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(Hsdrvrstrsel::Typec)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(Hsdrvrstrsel::Typea)
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(Hsdrvrstrsel::Typeb)
    }
}
#[doc = "Field `SDR12SDCLKFREQSEL` reader - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Sdr12sdclkfreqselR = crate::FieldReader<u16>;
#[doc = "Field `SDR12SDCLKFREQSEL` writer - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Sdr12sdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit is effective when Host Controller supports programmable clock generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr12clkgensel {
    #[doc = "1: Programmable Clock Generator"]
    Progclk = 1,
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator"]
    Hostctlr = 0,
}
impl From<Sdr12clkgensel> for bool {
    #[inline(always)]
    fn from(variant: Sdr12clkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR12CLKGENSEL` reader - This bit is effective when Host Controller supports programmable clock generator."]
pub type Sdr12clkgenselR = crate::BitReader<Sdr12clkgensel>;
impl Sdr12clkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr12clkgensel {
        match self.bits {
            true => Sdr12clkgensel::Progclk,
            false => Sdr12clkgensel::Hostctlr,
        }
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_progclk(&self) -> bool {
        *self == Sdr12clkgensel::Progclk
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_hostctlr(&self) -> bool {
        *self == Sdr12clkgensel::Hostctlr
    }
}
#[doc = "Field `SDR12CLKGENSEL` writer - This bit is effective when Host Controller supports programmable clock generator."]
pub type Sdr12clkgenselW<'a, REG> = crate::BitWriter<'a, REG, Sdr12clkgensel>;
impl<'a, REG> Sdr12clkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn progclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr12clkgensel::Progclk)
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn hostctlr(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr12clkgensel::Hostctlr)
    }
}
#[doc = "Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdr12drvrstrsel {
    #[doc = "3: Driver Type D is Selected"]
    Typed = 3,
    #[doc = "2: Driver Type C is Selected"]
    Typec = 2,
    #[doc = "1: Driver Type A is Selected"]
    Typea = 1,
    #[doc = "0: Driver Type B is Selected"]
    Typeb = 0,
}
impl From<Sdr12drvrstrsel> for u8 {
    #[inline(always)]
    fn from(variant: Sdr12drvrstrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdr12drvrstrsel {
    type Ux = u8;
}
impl crate::IsEnum for Sdr12drvrstrsel {}
#[doc = "Field `SDR12DRVRSTRSEL` reader - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Sdr12drvrstrselR = crate::FieldReader<Sdr12drvrstrsel>;
impl Sdr12drvrstrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr12drvrstrsel {
        match self.bits {
            3 => Sdr12drvrstrsel::Typed,
            2 => Sdr12drvrstrsel::Typec,
            1 => Sdr12drvrstrsel::Typea,
            0 => Sdr12drvrstrsel::Typeb,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Sdr12drvrstrsel::Typed
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Sdr12drvrstrsel::Typec
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Sdr12drvrstrsel::Typea
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Sdr12drvrstrsel::Typeb
    }
}
#[doc = "Field `SDR12DRVRSTRSEL` writer - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Sdr12drvrstrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdr12drvrstrsel, crate::Safe>;
impl<'a, REG> Sdr12drvrstrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr12drvrstrsel::Typed)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr12drvrstrsel::Typec)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr12drvrstrsel::Typea)
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr12drvrstrsel::Typeb)
    }
}
impl R {
    #[doc = "Bits 0:9 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    pub fn hssdclkfreqsel(&self) -> HssdclkfreqselR {
        HssdclkfreqselR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    pub fn hsclkgensel(&self) -> HsclkgenselR {
        HsclkgenselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn hsdrvrstrsel(&self) -> HsdrvrstrselR {
        HsdrvrstrselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    pub fn sdr12sdclkfreqsel(&self) -> Sdr12sdclkfreqselR {
        Sdr12sdclkfreqselR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    pub fn sdr12clkgensel(&self) -> Sdr12clkgenselR {
        Sdr12clkgenselR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn sdr12drvrstrsel(&self) -> Sdr12drvrstrselR {
        Sdr12drvrstrselR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    #[must_use]
    pub fn hssdclkfreqsel(&mut self) -> HssdclkfreqselW<Preset1Spec> {
        HssdclkfreqselW::new(self, 0)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    #[must_use]
    pub fn hsclkgensel(&mut self) -> HsclkgenselW<Preset1Spec> {
        HsclkgenselW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvrstrsel(&mut self) -> HsdrvrstrselW<Preset1Spec> {
        HsdrvrstrselW::new(self, 14)
    }
    #[doc = "Bits 16:25 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    #[must_use]
    pub fn sdr12sdclkfreqsel(&mut self) -> Sdr12sdclkfreqselW<Preset1Spec> {
        Sdr12sdclkfreqselW::new(self, 16)
    }
    #[doc = "Bit 26 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    #[must_use]
    pub fn sdr12clkgensel(&mut self) -> Sdr12clkgenselW<Preset1Spec> {
        Sdr12clkgenselW::new(self, 26)
    }
    #[doc = "Bits 30:31 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    #[must_use]
    pub fn sdr12drvrstrsel(&mut self) -> Sdr12drvrstrselW<Preset1Spec> {
        Sdr12drvrstrselW::new(self, 30)
    }
}
#[doc = "Preset Value for high speed and SDR12\n\nYou can [`read`](crate::Reg::read) this register and get [`preset1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Preset1Spec;
impl crate::RegisterSpec for Preset1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preset1::R`](R) reader structure"]
impl crate::Readable for Preset1Spec {}
#[doc = "`write(|w| ..)` method takes [`preset1::W`](W) writer structure"]
impl crate::Writable for Preset1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESET1 to value 0"]
impl crate::Resettable for Preset1Spec {
    const RESET_VALUE: u32 = 0;
}
