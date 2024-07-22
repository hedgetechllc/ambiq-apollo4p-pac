#[doc = "Register `PRESET2` reader"]
pub type R = crate::R<Preset2Spec>;
#[doc = "Register `PRESET2` writer"]
pub type W = crate::W<Preset2Spec>;
#[doc = "Field `SDR25SDCLKFREQSEL` reader - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Sdr25sdclkfreqselR = crate::FieldReader<u16>;
#[doc = "Field `SDR25SDCLKFREQSEL` writer - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Sdr25sdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit is effective when Host Controller supports programmable clock generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr25clkgensel {
    #[doc = "1: Programmable Clock Generator"]
    Progclk = 1,
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator"]
    Hostctlr = 0,
}
impl From<Sdr25clkgensel> for bool {
    #[inline(always)]
    fn from(variant: Sdr25clkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR25CLKGENSEL` reader - This bit is effective when Host Controller supports programmable clock generator."]
pub type Sdr25clkgenselR = crate::BitReader<Sdr25clkgensel>;
impl Sdr25clkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr25clkgensel {
        match self.bits {
            true => Sdr25clkgensel::Progclk,
            false => Sdr25clkgensel::Hostctlr,
        }
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_progclk(&self) -> bool {
        *self == Sdr25clkgensel::Progclk
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_hostctlr(&self) -> bool {
        *self == Sdr25clkgensel::Hostctlr
    }
}
#[doc = "Field `SDR25CLKGENSEL` writer - This bit is effective when Host Controller supports programmable clock generator."]
pub type Sdr25clkgenselW<'a, REG> = crate::BitWriter<'a, REG, Sdr25clkgensel>;
impl<'a, REG> Sdr25clkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn progclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr25clkgensel::Progclk)
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn hostctlr(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr25clkgensel::Hostctlr)
    }
}
#[doc = "Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdr25drvrstrsel {
    #[doc = "3: Driver Type D is Selected"]
    Typed = 3,
    #[doc = "2: Driver Type C is Selected"]
    Typec = 2,
    #[doc = "1: Driver Type A is Selected"]
    Typea = 1,
    #[doc = "0: Driver Type B is Selected"]
    Typeb = 0,
}
impl From<Sdr25drvrstrsel> for u8 {
    #[inline(always)]
    fn from(variant: Sdr25drvrstrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdr25drvrstrsel {
    type Ux = u8;
}
impl crate::IsEnum for Sdr25drvrstrsel {}
#[doc = "Field `SDR25DRVRSTRSEL` reader - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Sdr25drvrstrselR = crate::FieldReader<Sdr25drvrstrsel>;
impl Sdr25drvrstrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr25drvrstrsel {
        match self.bits {
            3 => Sdr25drvrstrsel::Typed,
            2 => Sdr25drvrstrsel::Typec,
            1 => Sdr25drvrstrsel::Typea,
            0 => Sdr25drvrstrsel::Typeb,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Sdr25drvrstrsel::Typed
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Sdr25drvrstrsel::Typec
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Sdr25drvrstrsel::Typea
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Sdr25drvrstrsel::Typeb
    }
}
#[doc = "Field `SDR25DRVRSTRSEL` writer - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Sdr25drvrstrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdr25drvrstrsel, crate::Safe>;
impl<'a, REG> Sdr25drvrstrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr25drvrstrsel::Typed)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr25drvrstrsel::Typec)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr25drvrstrsel::Typea)
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr25drvrstrsel::Typeb)
    }
}
#[doc = "Field `SDR50SDCLKFREQSEL` reader - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Sdr50sdclkfreqselR = crate::FieldReader<u16>;
#[doc = "Field `SDR50SDCLKFREQSEL` writer - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type Sdr50sdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit is effective when Host Controller supports programmable clock generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr50clkgensel {
    #[doc = "1: Programmable Clock Generator"]
    Progclk = 1,
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator"]
    Hostctlr = 0,
}
impl From<Sdr50clkgensel> for bool {
    #[inline(always)]
    fn from(variant: Sdr50clkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR50CLKGENSEL` reader - This bit is effective when Host Controller supports programmable clock generator."]
pub type Sdr50clkgenselR = crate::BitReader<Sdr50clkgensel>;
impl Sdr50clkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr50clkgensel {
        match self.bits {
            true => Sdr50clkgensel::Progclk,
            false => Sdr50clkgensel::Hostctlr,
        }
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_progclk(&self) -> bool {
        *self == Sdr50clkgensel::Progclk
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_hostctlr(&self) -> bool {
        *self == Sdr50clkgensel::Hostctlr
    }
}
#[doc = "Field `SDR50CLKGENSEL` writer - This bit is effective when Host Controller supports programmable clock generator."]
pub type Sdr50clkgenselW<'a, REG> = crate::BitWriter<'a, REG, Sdr50clkgensel>;
impl<'a, REG> Sdr50clkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn progclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr50clkgensel::Progclk)
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn hostctlr(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr50clkgensel::Hostctlr)
    }
}
#[doc = "Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdr50drvrstrsel {
    #[doc = "3: Driver Type D is Selected"]
    Typed = 3,
    #[doc = "2: Driver Type C is Selected"]
    Typec = 2,
    #[doc = "1: Driver Type A is Selected"]
    Typea = 1,
    #[doc = "0: Driver Type B is Selected"]
    Typeb = 0,
}
impl From<Sdr50drvrstrsel> for u8 {
    #[inline(always)]
    fn from(variant: Sdr50drvrstrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdr50drvrstrsel {
    type Ux = u8;
}
impl crate::IsEnum for Sdr50drvrstrsel {}
#[doc = "Field `SDR50DRVRSTRSEL` reader - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Sdr50drvrstrselR = crate::FieldReader<Sdr50drvrstrsel>;
impl Sdr50drvrstrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr50drvrstrsel {
        match self.bits {
            3 => Sdr50drvrstrsel::Typed,
            2 => Sdr50drvrstrsel::Typec,
            1 => Sdr50drvrstrsel::Typea,
            0 => Sdr50drvrstrsel::Typeb,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Sdr50drvrstrsel::Typed
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Sdr50drvrstrsel::Typec
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Sdr50drvrstrsel::Typea
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Sdr50drvrstrsel::Typeb
    }
}
#[doc = "Field `SDR50DRVRSTRSEL` writer - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type Sdr50drvrstrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdr50drvrstrsel, crate::Safe>;
impl<'a, REG> Sdr50drvrstrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr50drvrstrsel::Typed)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr50drvrstrsel::Typec)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr50drvrstrsel::Typea)
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(Sdr50drvrstrsel::Typeb)
    }
}
impl R {
    #[doc = "Bits 0:9 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    pub fn sdr25sdclkfreqsel(&self) -> Sdr25sdclkfreqselR {
        Sdr25sdclkfreqselR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    pub fn sdr25clkgensel(&self) -> Sdr25clkgenselR {
        Sdr25clkgenselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn sdr25drvrstrsel(&self) -> Sdr25drvrstrselR {
        Sdr25drvrstrselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    pub fn sdr50sdclkfreqsel(&self) -> Sdr50sdclkfreqselR {
        Sdr50sdclkfreqselR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    pub fn sdr50clkgensel(&self) -> Sdr50clkgenselR {
        Sdr50clkgenselR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn sdr50drvrstrsel(&self) -> Sdr50drvrstrselR {
        Sdr50drvrstrselR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    #[must_use]
    pub fn sdr25sdclkfreqsel(&mut self) -> Sdr25sdclkfreqselW<Preset2Spec> {
        Sdr25sdclkfreqselW::new(self, 0)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    #[must_use]
    pub fn sdr25clkgensel(&mut self) -> Sdr25clkgenselW<Preset2Spec> {
        Sdr25clkgenselW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    #[must_use]
    pub fn sdr25drvrstrsel(&mut self) -> Sdr25drvrstrselW<Preset2Spec> {
        Sdr25drvrstrselW::new(self, 14)
    }
    #[doc = "Bits 16:25 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    #[must_use]
    pub fn sdr50sdclkfreqsel(&mut self) -> Sdr50sdclkfreqselW<Preset2Spec> {
        Sdr50sdclkfreqselW::new(self, 16)
    }
    #[doc = "Bit 26 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    #[must_use]
    pub fn sdr50clkgensel(&mut self) -> Sdr50clkgenselW<Preset2Spec> {
        Sdr50clkgenselW::new(self, 26)
    }
    #[doc = "Bits 30:31 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    #[must_use]
    pub fn sdr50drvrstrsel(&mut self) -> Sdr50drvrstrselW<Preset2Spec> {
        Sdr50drvrstrselW::new(self, 30)
    }
}
#[doc = "Preset Value for SDR25 and SDR50\n\nYou can [`read`](crate::Reg::read) this register and get [`preset2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Preset2Spec;
impl crate::RegisterSpec for Preset2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preset2::R`](R) reader structure"]
impl crate::Readable for Preset2Spec {}
#[doc = "`write(|w| ..)` method takes [`preset2::W`](W) writer structure"]
impl crate::Writable for Preset2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESET2 to value 0"]
impl crate::Resettable for Preset2Spec {
    const RESET_VALUE: u32 = 0;
}
