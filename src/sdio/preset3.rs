#[doc = "Register `PRESET3` reader"]
pub type R = crate::R<Preset3Spec>;
#[doc = "Register `PRESET3` writer"]
pub type W = crate::W<Preset3Spec>;
#[doc = "Field `SDR104SDCLKFREQSEL` reader - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Sdr104sdclkfreqselR = crate::FieldReader<u16>;
#[doc = "Field `SDR104SDCLKFREQSEL` writer - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Sdr104sdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit is effective when Host Controller supports programmable clock generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr104clkgensel {
    #[doc = "1: Programmable Clock Generator"]
    Progclk = 1,
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator"]
    Hostctlr = 0,
}
impl From<Sdr104clkgensel> for bool {
    #[inline(always)]
    fn from(variant: Sdr104clkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR104CLKGENSEL` reader - This bit is effective when Host Controller supports programmable clock generator."]
pub type Sdr104clkgenselR = crate::BitReader<Sdr104clkgensel>;
impl Sdr104clkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr104clkgensel {
        match self.bits {
            true => Sdr104clkgensel::Progclk,
            false => Sdr104clkgensel::Hostctlr,
        }
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_progclk(&self) -> bool {
        *self == Sdr104clkgensel::Progclk
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_hostctlr(&self) -> bool {
        *self == Sdr104clkgensel::Hostctlr
    }
}
#[doc = "Field `SDR104CLKGENSEL` writer - This bit is effective when Host Controller supports programmable clock generator."]
pub type Sdr104clkgenselW<'a, REG> = crate::BitWriter<'a, REG, Sdr104clkgensel>;
impl<'a, REG> Sdr104clkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn progclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr104clkgensel::Progclk)
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn hostctlr(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr104clkgensel::Hostctlr)
    }
}
#[doc = "Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdr104drvrstrsel {
    #[doc = "3: Driver Type D is Selected"]
    Typed = 3,
    #[doc = "2: Driver Type C is Selected"]
    Typec = 2,
    #[doc = "1: Driver Type A is Selected"]
    Typea = 1,
    #[doc = "0: Driver Type B is Selected"]
    Typeb = 0,
}
impl From<Sdr104drvrstrsel> for u8 {
    #[inline(always)]
    fn from(variant: Sdr104drvrstrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdr104drvrstrsel {
    type Ux = u8;
}
impl crate::IsEnum for Sdr104drvrstrsel {}
#[doc = "Field `SDR104DRVRSTRSEL` reader - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Sdr104drvrstrselR = crate::FieldReader<Sdr104drvrstrsel>;
impl Sdr104drvrstrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr104drvrstrsel {
        match self.bits {
            3 => Sdr104drvrstrsel::Typed,
            2 => Sdr104drvrstrsel::Typec,
            1 => Sdr104drvrstrsel::Typea,
            0 => Sdr104drvrstrsel::Typeb,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Sdr104drvrstrsel::Typed
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Sdr104drvrstrsel::Typec
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Sdr104drvrstrsel::Typea
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Sdr104drvrstrsel::Typeb
    }
}
#[doc = "Field `SDR104DRVRSTRSEL` writer - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Sdr104drvrstrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdr104drvrstrsel, crate::Safe>;
impl<'a, REG> Sdr104drvrstrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr104drvrstrsel::Typed)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr104drvrstrsel::Typec)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr104drvrstrsel::Typea)
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr104drvrstrsel::Typeb)
    }
}
#[doc = "Field `DDR50SDCLKFREQSEL` reader - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Ddr50sdclkfreqselR = crate::FieldReader<u16>;
#[doc = "Field `DDR50SDCLKFREQSEL` writer - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Ddr50sdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit is effective when Host Controller supports programmable clock generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddr50clkgensel {
    #[doc = "1: Programmable Clock Generator"]
    Progclk = 1,
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator"]
    Hostctlr = 0,
}
impl From<Ddr50clkgensel> for bool {
    #[inline(always)]
    fn from(variant: Ddr50clkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR50CLKGENSEL` reader - This bit is effective when Host Controller supports programmable clock generator."]
pub type Ddr50clkgenselR = crate::BitReader<Ddr50clkgensel>;
impl Ddr50clkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddr50clkgensel {
        match self.bits {
            true => Ddr50clkgensel::Progclk,
            false => Ddr50clkgensel::Hostctlr,
        }
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_progclk(&self) -> bool {
        *self == Ddr50clkgensel::Progclk
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_hostctlr(&self) -> bool {
        *self == Ddr50clkgensel::Hostctlr
    }
}
#[doc = "Field `DDR50CLKGENSEL` writer - This bit is effective when Host Controller supports programmable clock generator."]
pub type Ddr50clkgenselW<'a, REG> = crate::BitWriter<'a, REG, Ddr50clkgensel>;
impl<'a, REG> Ddr50clkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn progclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr50clkgensel::Progclk)
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn hostctlr(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr50clkgensel::Hostctlr)
    }
}
#[doc = "Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ddr50drvrstrsel {
    #[doc = "3: Driver Type D is Selected"]
    Typed = 3,
    #[doc = "2: Driver Type C is Selected"]
    Typec = 2,
    #[doc = "1: Driver Type A is Selected"]
    Typea = 1,
    #[doc = "0: Driver Type B is Selected"]
    Typeb = 0,
}
impl From<Ddr50drvrstrsel> for u8 {
    #[inline(always)]
    fn from(variant: Ddr50drvrstrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ddr50drvrstrsel {
    type Ux = u8;
}
impl crate::IsEnum for Ddr50drvrstrsel {}
#[doc = "Field `DDR50DRVRSTRSEL` reader - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Ddr50drvrstrselR = crate::FieldReader<Ddr50drvrstrsel>;
impl Ddr50drvrstrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddr50drvrstrsel {
        match self.bits {
            3 => Ddr50drvrstrsel::Typed,
            2 => Ddr50drvrstrsel::Typec,
            1 => Ddr50drvrstrsel::Typea,
            0 => Ddr50drvrstrsel::Typeb,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Ddr50drvrstrsel::Typed
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Ddr50drvrstrsel::Typec
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Ddr50drvrstrsel::Typea
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Ddr50drvrstrsel::Typeb
    }
}
#[doc = "Field `DDR50DRVRSTRSEL` writer - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Ddr50drvrstrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ddr50drvrstrsel, crate::Safe>;
impl<'a, REG> Ddr50drvrstrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr50drvrstrsel::Typed)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr50drvrstrsel::Typec)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr50drvrstrsel::Typea)
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(Ddr50drvrstrsel::Typeb)
    }
}
impl R {
    #[doc = "Bits 0:9 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    pub fn sdr104sdclkfreqsel(&self) -> Sdr104sdclkfreqselR {
        Sdr104sdclkfreqselR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    pub fn sdr104clkgensel(&self) -> Sdr104clkgenselR {
        Sdr104clkgenselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn sdr104drvrstrsel(&self) -> Sdr104drvrstrselR {
        Sdr104drvrstrselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    pub fn ddr50sdclkfreqsel(&self) -> Ddr50sdclkfreqselR {
        Ddr50sdclkfreqselR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    pub fn ddr50clkgensel(&self) -> Ddr50clkgenselR {
        Ddr50clkgenselR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn ddr50drvrstrsel(&self) -> Ddr50drvrstrselR {
        Ddr50drvrstrselR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    #[must_use]
    pub fn sdr104sdclkfreqsel(&mut self) -> Sdr104sdclkfreqselW<Preset3Spec> {
        Sdr104sdclkfreqselW::new(self, 0)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    #[must_use]
    pub fn sdr104clkgensel(&mut self) -> Sdr104clkgenselW<Preset3Spec> {
        Sdr104clkgenselW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    #[must_use]
    pub fn sdr104drvrstrsel(&mut self) -> Sdr104drvrstrselW<Preset3Spec> {
        Sdr104drvrstrselW::new(self, 14)
    }
    #[doc = "Bits 16:25 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    #[must_use]
    pub fn ddr50sdclkfreqsel(&mut self) -> Ddr50sdclkfreqselW<Preset3Spec> {
        Ddr50sdclkfreqselW::new(self, 16)
    }
    #[doc = "Bit 26 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    #[must_use]
    pub fn ddr50clkgensel(&mut self) -> Ddr50clkgenselW<Preset3Spec> {
        Ddr50clkgenselW::new(self, 26)
    }
    #[doc = "Bits 30:31 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    #[must_use]
    pub fn ddr50drvrstrsel(&mut self) -> Ddr50drvrstrselW<Preset3Spec> {
        Ddr50drvrstrselW::new(self, 30)
    }
}
#[doc = "Preset Value for SDR104 and DDR50\n\nYou can [`read`](crate::Reg::read) this register and get [`preset3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Preset3Spec;
impl crate::RegisterSpec for Preset3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preset3::R`](R) reader structure"]
impl crate::Readable for Preset3Spec {}
#[doc = "`write(|w| ..)` method takes [`preset3::W`](W) writer structure"]
impl crate::Writable for Preset3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESET3 to value 0"]
impl crate::Resettable for Preset3Spec {
    const RESET_VALUE: u32 = 0;
}
