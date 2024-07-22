#[doc = "Register `PRESET0` reader"]
pub type R = crate::R<Preset0Spec>;
#[doc = "Register `PRESET0` writer"]
pub type W = crate::W<Preset0Spec>;
#[doc = "Field `HISPSDCLKFREQSEL` reader - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type HispsdclkfreqselR = crate::FieldReader<u16>;
#[doc = "Field `HISPSDCLKFREQSEL` writer - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type HispsdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit is effective when Host Controller supports programmable clock generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hispclkgensel {
    #[doc = "1: Programmable Clock Generator"]
    Progclk = 1,
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator"]
    Hostctlr = 0,
}
impl From<Hispclkgensel> for bool {
    #[inline(always)]
    fn from(variant: Hispclkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HISPCLKGENSEL` reader - This bit is effective when Host Controller supports programmable clock generator."]
pub type HispclkgenselR = crate::BitReader<Hispclkgensel>;
impl HispclkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hispclkgensel {
        match self.bits {
            true => Hispclkgensel::Progclk,
            false => Hispclkgensel::Hostctlr,
        }
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_progclk(&self) -> bool {
        *self == Hispclkgensel::Progclk
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_hostctlr(&self) -> bool {
        *self == Hispclkgensel::Hostctlr
    }
}
#[doc = "Field `HISPCLKGENSEL` writer - This bit is effective when Host Controller supports programmable clock generator."]
pub type HispclkgenselW<'a, REG> = crate::BitWriter<'a, REG, Hispclkgensel>;
impl<'a, REG> HispclkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn progclk(self) -> &'a mut crate::W<REG> {
        self.variant(Hispclkgensel::Progclk)
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn hostctlr(self) -> &'a mut crate::W<REG> {
        self.variant(Hispclkgensel::Hostctlr)
    }
}
#[doc = "Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hispdrvrstrsel {
    #[doc = "3: Driver Type D is Selected"]
    Typed = 3,
    #[doc = "2: Driver Type C is Selected"]
    Typec = 2,
    #[doc = "1: Driver Type A is Selected"]
    Typea = 1,
    #[doc = "0: Driver Type B is Selected"]
    Typeb = 0,
}
impl From<Hispdrvrstrsel> for u8 {
    #[inline(always)]
    fn from(variant: Hispdrvrstrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hispdrvrstrsel {
    type Ux = u8;
}
impl crate::IsEnum for Hispdrvrstrsel {}
#[doc = "Field `HISPDRVRSTRSEL` reader - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type HispdrvrstrselR = crate::FieldReader<Hispdrvrstrsel>;
impl HispdrvrstrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hispdrvrstrsel {
        match self.bits {
            3 => Hispdrvrstrsel::Typed,
            2 => Hispdrvrstrsel::Typec,
            1 => Hispdrvrstrsel::Typea,
            0 => Hispdrvrstrsel::Typeb,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Hispdrvrstrsel::Typed
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Hispdrvrstrsel::Typec
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Hispdrvrstrsel::Typea
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Hispdrvrstrsel::Typeb
    }
}
#[doc = "Field `HISPDRVRSTRSEL` writer - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type HispdrvrstrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hispdrvrstrsel, crate::Safe>;
impl<'a, REG> HispdrvrstrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(Hispdrvrstrsel::Typed)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(Hispdrvrstrsel::Typec)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(Hispdrvrstrsel::Typea)
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(Hispdrvrstrsel::Typeb)
    }
}
#[doc = "Field `DEFSPSDCLKFREQSEL` reader - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type DefspsdclkfreqselR = crate::FieldReader<u16>;
#[doc = "Field `DEFSPSDCLKFREQSEL` writer - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
pub type DefspsdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "This bit is effective when Host Controller supports programmable clock generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Defspclkgensel {
    #[doc = "1: Programmable Clock Generator"]
    Progclk = 1,
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator"]
    Hostctlr = 0,
}
impl From<Defspclkgensel> for bool {
    #[inline(always)]
    fn from(variant: Defspclkgensel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEFSPCLKGENSEL` reader - This bit is effective when Host Controller supports programmable clock generator."]
pub type DefspclkgenselR = crate::BitReader<Defspclkgensel>;
impl DefspclkgenselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Defspclkgensel {
        match self.bits {
            true => Defspclkgensel::Progclk,
            false => Defspclkgensel::Hostctlr,
        }
    }
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn is_progclk(&self) -> bool {
        *self == Defspclkgensel::Progclk
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_hostctlr(&self) -> bool {
        *self == Defspclkgensel::Hostctlr
    }
}
#[doc = "Field `DEFSPCLKGENSEL` writer - This bit is effective when Host Controller supports programmable clock generator."]
pub type DefspclkgenselW<'a, REG> = crate::BitWriter<'a, REG, Defspclkgensel>;
impl<'a, REG> DefspclkgenselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programmable Clock Generator"]
    #[inline(always)]
    pub fn progclk(self) -> &'a mut crate::W<REG> {
        self.variant(Defspclkgensel::Progclk)
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn hostctlr(self) -> &'a mut crate::W<REG> {
        self.variant(Defspclkgensel::Hostctlr)
    }
}
#[doc = "Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Defspdrvrstrsel {
    #[doc = "3: Driver Type D is Selected"]
    Typed = 3,
    #[doc = "2: Driver Type C is Selected"]
    Typec = 2,
    #[doc = "1: Driver Type A is Selected"]
    Typea = 1,
    #[doc = "0: Driver Type B is Selected"]
    Typeb = 0,
}
impl From<Defspdrvrstrsel> for u8 {
    #[inline(always)]
    fn from(variant: Defspdrvrstrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Defspdrvrstrsel {
    type Ux = u8;
}
impl crate::IsEnum for Defspdrvrstrsel {}
#[doc = "Field `DEFSPDRVRSTRSEL` reader - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type DefspdrvrstrselR = crate::FieldReader<Defspdrvrstrsel>;
impl DefspdrvrstrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Defspdrvrstrsel {
        match self.bits {
            3 => Defspdrvrstrsel::Typed,
            2 => Defspdrvrstrsel::Typec,
            1 => Defspdrvrstrsel::Typea,
            0 => Defspdrvrstrsel::Typeb,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Defspdrvrstrsel::Typed
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Defspdrvrstrsel::Typec
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Defspdrvrstrsel::Typea
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Defspdrvrstrsel::Typeb
    }
}
#[doc = "Field `DEFSPDRVRSTRSEL` writer - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type DefspdrvrstrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Defspdrvrstrsel, crate::Safe>;
impl<'a, REG> DefspdrvrstrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type D is Selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(Defspdrvrstrsel::Typed)
    }
    #[doc = "Driver Type C is Selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(Defspdrvrstrsel::Typec)
    }
    #[doc = "Driver Type A is Selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(Defspdrvrstrsel::Typea)
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(Defspdrvrstrsel::Typeb)
    }
}
impl R {
    #[doc = "Bits 0:9 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    pub fn hispsdclkfreqsel(&self) -> HispsdclkfreqselR {
        HispsdclkfreqselR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    pub fn hispclkgensel(&self) -> HispclkgenselR {
        HispclkgenselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn hispdrvrstrsel(&self) -> HispdrvrstrselR {
        HispdrvrstrselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    pub fn defspsdclkfreqsel(&self) -> DefspsdclkfreqselR {
        DefspsdclkfreqselR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    pub fn defspclkgensel(&self) -> DefspclkgenselR {
        DefspclkgenselR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn defspdrvrstrsel(&self) -> DefspdrvrstrselR {
        DefspdrvrstrselR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    #[must_use]
    pub fn hispsdclkfreqsel(&mut self) -> HispsdclkfreqselW<Preset0Spec> {
        HispsdclkfreqselW::new(self, 0)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    #[must_use]
    pub fn hispclkgensel(&mut self) -> HispclkgenselW<Preset0Spec> {
        HispclkgenselW::new(self, 10)
    }
    #[doc = "Bits 14:15 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    #[must_use]
    pub fn hispdrvrstrsel(&mut self) -> HispdrvrstrselW<Preset0Spec> {
        HispdrvrstrselW::new(self, 14)
    }
    #[doc = "Bits 16:25 - 10 bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system. When Host Controller supports shared bus, a set of Preset Value registers for each device required and the registers location are duplicated to the offset 06Fh-060h. A set of Preset Value registers can be accessible by selecting Clock Pin Select in the Shared Bus Control register"]
    #[inline(always)]
    #[must_use]
    pub fn defspsdclkfreqsel(&mut self) -> DefspsdclkfreqselW<Preset0Spec> {
        DefspsdclkfreqselW::new(self, 16)
    }
    #[doc = "Bit 26 - This bit is effective when Host Controller supports programmable clock generator."]
    #[inline(always)]
    #[must_use]
    pub fn defspclkgensel(&mut self) -> DefspclkgenselW<Preset0Spec> {
        DefspclkgenselW::new(self, 26)
    }
    #[doc = "Bits 30:31 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    #[must_use]
    pub fn defspdrvrstrsel(&mut self) -> DefspdrvrstrselW<Preset0Spec> {
        DefspdrvrstrselW::new(self, 30)
    }
}
#[doc = "Preset Value initialization and default speed\n\nYou can [`read`](crate::Reg::read) this register and get [`preset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Preset0Spec;
impl crate::RegisterSpec for Preset0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preset0::R`](R) reader structure"]
impl crate::Readable for Preset0Spec {}
#[doc = "`write(|w| ..)` method takes [`preset0::W`](W) writer structure"]
impl crate::Writable for Preset0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESET0 to value 0"]
impl crate::Resettable for Preset0Spec {
    const RESET_VALUE: u32 = 0;
}
