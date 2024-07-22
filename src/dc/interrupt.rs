#[doc = "Register `INTERRUPT` reader"]
pub type R = crate::R<InterruptSpec>;
#[doc = "Register `INTERRUPT` writer"]
pub type W = crate::W<InterruptSpec>;
#[doc = "Field `INTVSYNCEN` reader - When set to 1, VSYNC interrupt enabled"]
pub type IntvsyncenR = crate::BitReader;
#[doc = "Field `INTVSYNCEN` writer - When set to 1, VSYNC interrupt enabled"]
pub type IntvsyncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTHSYNCEN` reader - When set to 1, HSYNC interrupt enabled"]
pub type InthsyncenR = crate::BitReader;
#[doc = "Field `INTHSYNCEN` writer - When set to 1, HSYNC interrupt enabled"]
pub type InthsyncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMMUERR` reader - When set to 1, MMU error interrupt enabled"]
pub type IntmmuerrR = crate::BitReader;
#[doc = "Field `INTMMUERR` writer - When set to 1, MMU error interrupt enabled"]
pub type IntmmuerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTEEN` reader - When set to 1, TE interrupt enabled"]
pub type IntteenR = crate::BitReader;
#[doc = "Field `INTTEEN` writer - When set to 1, TE interrupt enabled"]
pub type IntteenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt request trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inttrigger {
    #[doc = "1: Level triggering is enabled"]
    Level = 1,
    #[doc = "0: Edge triggering is enabled"]
    Edge = 0,
}
impl From<Inttrigger> for bool {
    #[inline(always)]
    fn from(variant: Inttrigger) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTTRIGGER` reader - Interrupt request trigger control"]
pub type InttriggerR = crate::BitReader<Inttrigger>;
impl InttriggerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inttrigger {
        match self.bits {
            true => Inttrigger::Level,
            false => Inttrigger::Edge,
        }
    }
    #[doc = "Level triggering is enabled"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Inttrigger::Level
    }
    #[doc = "Edge triggering is enabled"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Inttrigger::Edge
    }
}
#[doc = "Field `INTTRIGGER` writer - Interrupt request trigger control"]
pub type InttriggerW<'a, REG> = crate::BitWriter<'a, REG, Inttrigger>;
impl<'a, REG> InttriggerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level triggering is enabled"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Inttrigger::Level)
    }
    #[doc = "Edge triggering is enabled"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Inttrigger::Edge)
    }
}
impl R {
    #[doc = "Bit 0 - When set to 1, VSYNC interrupt enabled"]
    #[inline(always)]
    pub fn intvsyncen(&self) -> IntvsyncenR {
        IntvsyncenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, HSYNC interrupt enabled"]
    #[inline(always)]
    pub fn inthsyncen(&self) -> InthsyncenR {
        InthsyncenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1, MMU error interrupt enabled"]
    #[inline(always)]
    pub fn intmmuerr(&self) -> IntmmuerrR {
        IntmmuerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, TE interrupt enabled"]
    #[inline(always)]
    pub fn intteen(&self) -> IntteenR {
        IntteenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt request trigger control"]
    #[inline(always)]
    pub fn inttrigger(&self) -> InttriggerR {
        InttriggerR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, VSYNC interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn intvsyncen(&mut self) -> IntvsyncenW<InterruptSpec> {
        IntvsyncenW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, HSYNC interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inthsyncen(&mut self) -> InthsyncenW<InterruptSpec> {
        InthsyncenW::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1, MMU error interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn intmmuerr(&mut self) -> IntmmuerrW<InterruptSpec> {
        IntmmuerrW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, TE interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn intteen(&mut self) -> IntteenW<InterruptSpec> {
        IntteenW::new(self, 3)
    }
    #[doc = "Bit 31 - Interrupt request trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn inttrigger(&mut self) -> InttriggerW<InterruptSpec> {
        InttriggerW::new(self, 31)
    }
}
#[doc = "Register interrupts enabled, level or edge enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptSpec;
impl crate::RegisterSpec for InterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for InterruptSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt::W`](W) writer structure"]
impl crate::Writable for InterruptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT to value 0x01"]
impl crate::Resettable for InterruptSpec {
    const RESET_VALUE: u32 = 0x01;
}
