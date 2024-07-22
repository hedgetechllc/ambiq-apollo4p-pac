#[doc = "Register `BCDETCRTL1` reader"]
pub type R = crate::R<Bcdetcrtl1Spec>;
#[doc = "Register `BCDETCRTL1` writer"]
pub type W = crate::W<Bcdetcrtl1Spec>;
#[doc = "Field `BCWEAKPULLUPEN` reader - Enables weak source current to DP and DM"]
pub type BcweakpullupenR = crate::BitReader;
#[doc = "Field `BCWEAKPULLUPEN` writer - Enables weak source current to DP and DM"]
pub type BcweakpullupenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCWEAKPULLDOWNEN` reader - Enables weak sink current on DP and DM"]
pub type BcweakpulldownenR = crate::BitReader;
#[doc = "Field `BCWEAKPULLDOWNEN` writer - Enables weak sink current on DP and DM"]
pub type BcweakpulldownenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMSINKEN` reader - Enables DM current sink"]
pub type IdmsinkenR = crate::BitReader;
#[doc = "Field `IDMSINKEN` writer - Enables DM current sink"]
pub type IdmsinkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDPSRCEN` reader - Enables DP current source"]
pub type IdpsrcenR = crate::BitReader;
#[doc = "Field `IDPSRCEN` writer - Enables DP current source"]
pub type IdpsrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDPSRCEN` reader - Enables DP voltage source"]
pub type VdpsrcenR = crate::BitReader;
#[doc = "Field `VDPSRCEN` writer - Enables DP voltage source"]
pub type VdpsrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDMPDWNEN` reader - Enables DM BC 1.2 pull-down resistor"]
pub type RdmpdwnenR = crate::BitReader;
#[doc = "Field `RDMPDWNEN` writer - Enables DM BC 1.2 pull-down resistor"]
pub type RdmpdwnenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDMSRCEN` reader - Enables DM voltage source"]
pub type VdmsrcenR = crate::BitReader;
#[doc = "Field `VDMSRCEN` writer - Enables DM voltage source"]
pub type VdmsrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDPSINKEN` reader - Enables DP current sink"]
pub type IdpsinkenR = crate::BitReader;
#[doc = "Field `IDPSINKEN` writer - Enables DP current sink"]
pub type IdpsinkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Sets DP/DM vendor-specific comparator ref voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbdcompref {
    #[doc = "3: 1.25V"]
    _1p25v = 3,
    #[doc = "2: 2.35V"]
    _2p35 = 2,
    #[doc = "1: 3.10V"]
    _3p10v = 1,
    #[doc = "0: 1.65V (VCCIO/2)"]
    _1p65 = 0,
}
impl From<Usbdcompref> for u8 {
    #[inline(always)]
    fn from(variant: Usbdcompref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbdcompref {
    type Ux = u8;
}
impl crate::IsEnum for Usbdcompref {}
#[doc = "Field `USBDCOMPREF` reader - Sets DP/DM vendor-specific comparator ref voltage"]
pub type UsbdcomprefR = crate::FieldReader<Usbdcompref>;
impl UsbdcomprefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbdcompref {
        match self.bits {
            3 => Usbdcompref::_1p25v,
            2 => Usbdcompref::_2p35,
            1 => Usbdcompref::_3p10v,
            0 => Usbdcompref::_1p65,
            _ => unreachable!(),
        }
    }
    #[doc = "1.25V"]
    #[inline(always)]
    pub fn is_1p25v(&self) -> bool {
        *self == Usbdcompref::_1p25v
    }
    #[doc = "2.35V"]
    #[inline(always)]
    pub fn is_2p35(&self) -> bool {
        *self == Usbdcompref::_2p35
    }
    #[doc = "3.10V"]
    #[inline(always)]
    pub fn is_3p10v(&self) -> bool {
        *self == Usbdcompref::_3p10v
    }
    #[doc = "1.65V (VCCIO/2)"]
    #[inline(always)]
    pub fn is_1p65(&self) -> bool {
        *self == Usbdcompref::_1p65
    }
}
#[doc = "Field `USBDCOMPREF` writer - Sets DP/DM vendor-specific comparator ref voltage"]
pub type UsbdcomprefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usbdcompref, crate::Safe>;
impl<'a, REG> UsbdcomprefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.25V"]
    #[inline(always)]
    pub fn _1p25v(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdcompref::_1p25v)
    }
    #[doc = "2.35V"]
    #[inline(always)]
    pub fn _2p35(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdcompref::_2p35)
    }
    #[doc = "3.10V"]
    #[inline(always)]
    pub fn _3p10v(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdcompref::_3p10v)
    }
    #[doc = "1.65V (VCCIO/2)"]
    #[inline(always)]
    pub fn _1p65(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdcompref::_1p65)
    }
}
#[doc = "Field `USBDCOMPEN` reader - Enables DP/DM vendor-specific detection comparator"]
pub type UsbdcompenR = crate::BitReader;
#[doc = "Field `USBDCOMPEN` writer - Enables DP/DM vendor-specific detection comparator"]
pub type UsbdcompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSWRESET` reader - Holds a USB controller and PHY in the reset for BC detection"]
pub type UsbswresetR = crate::BitReader;
#[doc = "Field `USBSWRESET` writer - Holds a USB controller and PHY in the reset for BC detection"]
pub type UsbswresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables weak source current to DP and DM"]
    #[inline(always)]
    pub fn bcweakpullupen(&self) -> BcweakpullupenR {
        BcweakpullupenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables weak sink current on DP and DM"]
    #[inline(always)]
    pub fn bcweakpulldownen(&self) -> BcweakpulldownenR {
        BcweakpulldownenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables DM current sink"]
    #[inline(always)]
    pub fn idmsinken(&self) -> IdmsinkenR {
        IdmsinkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables DP current source"]
    #[inline(always)]
    pub fn idpsrcen(&self) -> IdpsrcenR {
        IdpsrcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables DP voltage source"]
    #[inline(always)]
    pub fn vdpsrcen(&self) -> VdpsrcenR {
        VdpsrcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables DM BC 1.2 pull-down resistor"]
    #[inline(always)]
    pub fn rdmpdwnen(&self) -> RdmpdwnenR {
        RdmpdwnenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables DM voltage source"]
    #[inline(always)]
    pub fn vdmsrcen(&self) -> VdmsrcenR {
        VdmsrcenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables DP current sink"]
    #[inline(always)]
    pub fn idpsinken(&self) -> IdpsinkenR {
        IdpsinkenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Sets DP/DM vendor-specific comparator ref voltage"]
    #[inline(always)]
    pub fn usbdcompref(&self) -> UsbdcomprefR {
        UsbdcomprefR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Enables DP/DM vendor-specific detection comparator"]
    #[inline(always)]
    pub fn usbdcompen(&self) -> UsbdcompenR {
        UsbdcompenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - Holds a USB controller and PHY in the reset for BC detection"]
    #[inline(always)]
    pub fn usbswreset(&self) -> UsbswresetR {
        UsbswresetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables weak source current to DP and DM"]
    #[inline(always)]
    #[must_use]
    pub fn bcweakpullupen(&mut self) -> BcweakpullupenW<Bcdetcrtl1Spec> {
        BcweakpullupenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables weak sink current on DP and DM"]
    #[inline(always)]
    #[must_use]
    pub fn bcweakpulldownen(&mut self) -> BcweakpulldownenW<Bcdetcrtl1Spec> {
        BcweakpulldownenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables DM current sink"]
    #[inline(always)]
    #[must_use]
    pub fn idmsinken(&mut self) -> IdmsinkenW<Bcdetcrtl1Spec> {
        IdmsinkenW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables DP current source"]
    #[inline(always)]
    #[must_use]
    pub fn idpsrcen(&mut self) -> IdpsrcenW<Bcdetcrtl1Spec> {
        IdpsrcenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables DP voltage source"]
    #[inline(always)]
    #[must_use]
    pub fn vdpsrcen(&mut self) -> VdpsrcenW<Bcdetcrtl1Spec> {
        VdpsrcenW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables DM BC 1.2 pull-down resistor"]
    #[inline(always)]
    #[must_use]
    pub fn rdmpdwnen(&mut self) -> RdmpdwnenW<Bcdetcrtl1Spec> {
        RdmpdwnenW::new(self, 5)
    }
    #[doc = "Bit 6 - Enables DM voltage source"]
    #[inline(always)]
    #[must_use]
    pub fn vdmsrcen(&mut self) -> VdmsrcenW<Bcdetcrtl1Spec> {
        VdmsrcenW::new(self, 6)
    }
    #[doc = "Bit 7 - Enables DP current sink"]
    #[inline(always)]
    #[must_use]
    pub fn idpsinken(&mut self) -> IdpsinkenW<Bcdetcrtl1Spec> {
        IdpsinkenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Sets DP/DM vendor-specific comparator ref voltage"]
    #[inline(always)]
    #[must_use]
    pub fn usbdcompref(&mut self) -> UsbdcomprefW<Bcdetcrtl1Spec> {
        UsbdcomprefW::new(self, 8)
    }
    #[doc = "Bit 11 - Enables DP/DM vendor-specific detection comparator"]
    #[inline(always)]
    #[must_use]
    pub fn usbdcompen(&mut self) -> UsbdcompenW<Bcdetcrtl1Spec> {
        UsbdcompenW::new(self, 11)
    }
    #[doc = "Bit 31 - Holds a USB controller and PHY in the reset for BC detection"]
    #[inline(always)]
    #[must_use]
    pub fn usbswreset(&mut self) -> UsbswresetW<Bcdetcrtl1Spec> {
        UsbswresetW::new(self, 31)
    }
}
#[doc = "Battery Charging detection main control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcdetcrtl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdetcrtl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcdetcrtl1Spec;
impl crate::RegisterSpec for Bcdetcrtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdetcrtl1::R`](R) reader structure"]
impl crate::Readable for Bcdetcrtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdetcrtl1::W`](W) writer structure"]
impl crate::Writable for Bcdetcrtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDETCRTL1 to value 0x02"]
impl crate::Resettable for Bcdetcrtl1Spec {
    const RESET_VALUE: u32 = 0x02;
}
