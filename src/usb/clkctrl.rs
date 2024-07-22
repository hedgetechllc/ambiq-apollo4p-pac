#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Field `PHYREFCLKDIS` reader - Setting this bit turns off the PHY reference clock."]
pub type PhyrefclkdisR = crate::BitReader;
#[doc = "Field `PHYREFCLKDIS` writer - Setting this bit turns off the PHY reference clock."]
pub type PhyrefclkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLAPBCLKDIS` reader - Setting this bit turns off the Controller logic clock."]
pub type CtrlapbclkdisR = crate::BitReader;
#[doc = "Field `CTRLAPBCLKDIS` writer - Setting this bit turns off the Controller logic clock."]
pub type CtrlapbclkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYAPBLCLKDIS` reader - Setting this bit turns off PHY control logic clock."]
pub type PhyapblclkdisR = crate::BitReader;
#[doc = "Field `PHYAPBLCLKDIS` writer - Setting this bit turns off PHY control logic clock."]
pub type PhyapblclkdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB PHY reference clock select.For Full_Speed Mode, set the reference CLKSEL to use HFRC-based clock. For High-Speed Mode, set the reference CLKSEL to use HFRC2-based clock. The HFRC2-based clock is higher power, but meets the low-jitter requirement for High-Speed Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Phyrefclksel {
    #[doc = "0: 48 MHz HFRC-based reference clock for Full-Speed Mode"]
    Hfrc48 = 0,
    #[doc = "1: 48 MHz HFRC2-based reference clock for High-Speed Mode"]
    Hfrc248 = 1,
    #[doc = "2: 24 MHz HFRC-based reference clock for Full-Speed Mode"]
    Hfrc24 = 2,
}
impl From<Phyrefclksel> for u8 {
    #[inline(always)]
    fn from(variant: Phyrefclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Phyrefclksel {
    type Ux = u8;
}
impl crate::IsEnum for Phyrefclksel {}
#[doc = "Field `PHYREFCLKSEL` reader - USB PHY reference clock select.For Full_Speed Mode, set the reference CLKSEL to use HFRC-based clock. For High-Speed Mode, set the reference CLKSEL to use HFRC2-based clock. The HFRC2-based clock is higher power, but meets the low-jitter requirement for High-Speed Mode."]
pub type PhyrefclkselR = crate::FieldReader<Phyrefclksel>;
impl PhyrefclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Phyrefclksel> {
        match self.bits {
            0 => Some(Phyrefclksel::Hfrc48),
            1 => Some(Phyrefclksel::Hfrc248),
            2 => Some(Phyrefclksel::Hfrc24),
            _ => None,
        }
    }
    #[doc = "48 MHz HFRC-based reference clock for Full-Speed Mode"]
    #[inline(always)]
    pub fn is_hfrc48(&self) -> bool {
        *self == Phyrefclksel::Hfrc48
    }
    #[doc = "48 MHz HFRC2-based reference clock for High-Speed Mode"]
    #[inline(always)]
    pub fn is_hfrc248(&self) -> bool {
        *self == Phyrefclksel::Hfrc248
    }
    #[doc = "24 MHz HFRC-based reference clock for Full-Speed Mode"]
    #[inline(always)]
    pub fn is_hfrc24(&self) -> bool {
        *self == Phyrefclksel::Hfrc24
    }
}
#[doc = "Field `PHYREFCLKSEL` writer - USB PHY reference clock select.For Full_Speed Mode, set the reference CLKSEL to use HFRC-based clock. For High-Speed Mode, set the reference CLKSEL to use HFRC2-based clock. The HFRC2-based clock is higher power, but meets the low-jitter requirement for High-Speed Mode."]
pub type PhyrefclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Phyrefclksel>;
impl<'a, REG> PhyrefclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48 MHz HFRC-based reference clock for Full-Speed Mode"]
    #[inline(always)]
    pub fn hfrc48(self) -> &'a mut crate::W<REG> {
        self.variant(Phyrefclksel::Hfrc48)
    }
    #[doc = "48 MHz HFRC2-based reference clock for High-Speed Mode"]
    #[inline(always)]
    pub fn hfrc248(self) -> &'a mut crate::W<REG> {
        self.variant(Phyrefclksel::Hfrc248)
    }
    #[doc = "24 MHz HFRC-based reference clock for Full-Speed Mode"]
    #[inline(always)]
    pub fn hfrc24(self) -> &'a mut crate::W<REG> {
        self.variant(Phyrefclksel::Hfrc24)
    }
}
impl R {
    #[doc = "Bit 0 - Setting this bit turns off the PHY reference clock."]
    #[inline(always)]
    pub fn phyrefclkdis(&self) -> PhyrefclkdisR {
        PhyrefclkdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Setting this bit turns off the Controller logic clock."]
    #[inline(always)]
    pub fn ctrlapbclkdis(&self) -> CtrlapbclkdisR {
        CtrlapbclkdisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Setting this bit turns off PHY control logic clock."]
    #[inline(always)]
    pub fn phyapblclkdis(&self) -> PhyapblclkdisR {
        PhyapblclkdisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - USB PHY reference clock select.For Full_Speed Mode, set the reference CLKSEL to use HFRC-based clock. For High-Speed Mode, set the reference CLKSEL to use HFRC2-based clock. The HFRC2-based clock is higher power, but meets the low-jitter requirement for High-Speed Mode."]
    #[inline(always)]
    pub fn phyrefclksel(&self) -> PhyrefclkselR {
        PhyrefclkselR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit turns off the PHY reference clock."]
    #[inline(always)]
    #[must_use]
    pub fn phyrefclkdis(&mut self) -> PhyrefclkdisW<ClkctrlSpec> {
        PhyrefclkdisW::new(self, 0)
    }
    #[doc = "Bit 8 - Setting this bit turns off the Controller logic clock."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlapbclkdis(&mut self) -> CtrlapbclkdisW<ClkctrlSpec> {
        CtrlapbclkdisW::new(self, 8)
    }
    #[doc = "Bit 16 - Setting this bit turns off PHY control logic clock."]
    #[inline(always)]
    #[must_use]
    pub fn phyapblclkdis(&mut self) -> PhyapblclkdisW<ClkctrlSpec> {
        PhyapblclkdisW::new(self, 16)
    }
    #[doc = "Bits 24:25 - USB PHY reference clock select.For Full_Speed Mode, set the reference CLKSEL to use HFRC-based clock. For High-Speed Mode, set the reference CLKSEL to use HFRC2-based clock. The HFRC2-based clock is higher power, but meets the low-jitter requirement for High-Speed Mode."]
    #[inline(always)]
    #[must_use]
    pub fn phyrefclksel(&mut self) -> PhyrefclkselW<ClkctrlSpec> {
        PhyrefclkselW::new(self, 24)
    }
}
#[doc = "Provides optional control for turning off the interface clocks to USB Controller and PHY as well as the reference clock to the USB PHY.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
