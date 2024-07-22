#[doc = "Register `PWRWEIGHTULP5` reader"]
pub type R = crate::R<Pwrweightulp5Spec>;
#[doc = "Register `PWRWEIGHTULP5` writer"]
pub type W = crate::W<Pwrweightulp5Spec>;
#[doc = "Field `WTULPDISPPHY` reader - Weight used for ULP mode DISP PHY"]
pub type WtulpdispphyR = crate::FieldReader;
#[doc = "Field `WTULPDISPPHY` writer - Weight used for ULP mode DISP PHY"]
pub type WtulpdispphyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPUSBPHY` reader - Weight used for ULP mode USB PHY"]
pub type WtulpusbphyR = crate::FieldReader;
#[doc = "Field `WTULPUSBPHY` writer - Weight used for ULP mode USB PHY"]
pub type WtulpusbphyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for ULP mode DISP PHY"]
    #[inline(always)]
    pub fn wtulpdispphy(&self) -> WtulpdispphyR {
        WtulpdispphyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode USB PHY"]
    #[inline(always)]
    pub fn wtulpusbphy(&self) -> WtulpusbphyR {
        WtulpusbphyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for ULP mode DISP PHY"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpdispphy(&mut self) -> WtulpdispphyW<Pwrweightulp5Spec> {
        WtulpdispphyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode USB PHY"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpusbphy(&mut self) -> WtulpusbphyW<Pwrweightulp5Spec> {
        WtulpusbphyW::new(self, 4)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightulp5Spec;
impl crate::RegisterSpec for Pwrweightulp5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightulp5::R`](R) reader structure"]
impl crate::Readable for Pwrweightulp5Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightulp5::W`](W) writer structure"]
impl crate::Writable for Pwrweightulp5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTULP5 to value 0"]
impl crate::Resettable for Pwrweightulp5Spec {
    const RESET_VALUE: u32 = 0;
}
