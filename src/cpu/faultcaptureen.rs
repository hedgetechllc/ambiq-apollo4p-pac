#[doc = "Register `FAULTCAPTUREEN` reader"]
pub type R = crate::R<FaultcaptureenSpec>;
#[doc = "Register `FAULTCAPTUREEN` writer"]
pub type W = crate::W<FaultcaptureenSpec>;
#[doc = "Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Faultcaptureen {
    #[doc = "0: Disable fault capture."]
    Dis = 0,
    #[doc = "1: Enable fault capture."]
    En = 1,
}
impl From<Faultcaptureen> for bool {
    #[inline(always)]
    fn from(variant: Faultcaptureen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTCAPTUREEN` reader - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
pub type FaultcaptureenR = crate::BitReader<Faultcaptureen>;
impl FaultcaptureenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Faultcaptureen {
        match self.bits {
            false => Faultcaptureen::Dis,
            true => Faultcaptureen::En,
        }
    }
    #[doc = "Disable fault capture."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Faultcaptureen::Dis
    }
    #[doc = "Enable fault capture."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Faultcaptureen::En
    }
}
#[doc = "Field `FAULTCAPTUREEN` writer - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
pub type FaultcaptureenW<'a, REG> = crate::BitWriter<'a, REG, Faultcaptureen>;
impl<'a, REG> FaultcaptureenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable fault capture."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Faultcaptureen::Dis)
    }
    #[doc = "Enable fault capture."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Faultcaptureen::En)
    }
}
impl R {
    #[doc = "Bit 0 - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
    #[inline(always)]
    pub fn faultcaptureen(&self) -> FaultcaptureenR {
        FaultcaptureenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Capture Enable field. When set, the Fault Capture monitors are enabled and addresses which generate a hard fault are captured into the FAULTADDR registers."]
    #[inline(always)]
    #[must_use]
    pub fn faultcaptureen(&mut self) -> FaultcaptureenW<FaultcaptureenSpec> {
        FaultcaptureenW::new(self, 0)
    }
}
#[doc = "Enable the fault capture registers\n\nYou can [`read`](crate::Reg::read) this register and get [`faultcaptureen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faultcaptureen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultcaptureenSpec;
impl crate::RegisterSpec for FaultcaptureenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`faultcaptureen::R`](R) reader structure"]
impl crate::Readable for FaultcaptureenSpec {}
#[doc = "`write(|w| ..)` method takes [`faultcaptureen::W`](W) writer structure"]
impl crate::Writable for FaultcaptureenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAULTCAPTUREEN to value 0"]
impl crate::Resettable for FaultcaptureenSpec {
    const RESET_VALUE: u32 = 0;
}
