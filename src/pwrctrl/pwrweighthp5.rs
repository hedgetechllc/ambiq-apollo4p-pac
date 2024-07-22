#[doc = "Register `PWRWEIGHTHP5` reader"]
pub type R = crate::R<Pwrweighthp5Spec>;
#[doc = "Register `PWRWEIGHTHP5` writer"]
pub type W = crate::W<Pwrweighthp5Spec>;
#[doc = "Field `WTHPDISPPHY` reader - Weight used for HP mode DISP PHY"]
pub type WthpdispphyR = crate::FieldReader;
#[doc = "Field `WTHPDISPPHY` writer - Weight used for HP mode DISP PHY"]
pub type WthpdispphyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPUSBPHY` reader - Weight used for HP mode USB PHY"]
pub type WthpusbphyR = crate::FieldReader;
#[doc = "Field `WTHPUSBPHY` writer - Weight used for HP mode USB PHY"]
pub type WthpusbphyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for HP mode DISP PHY"]
    #[inline(always)]
    pub fn wthpdispphy(&self) -> WthpdispphyR {
        WthpdispphyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode USB PHY"]
    #[inline(always)]
    pub fn wthpusbphy(&self) -> WthpusbphyR {
        WthpusbphyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for HP mode DISP PHY"]
    #[inline(always)]
    #[must_use]
    pub fn wthpdispphy(&mut self) -> WthpdispphyW<Pwrweighthp5Spec> {
        WthpdispphyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode USB PHY"]
    #[inline(always)]
    #[must_use]
    pub fn wthpusbphy(&mut self) -> WthpusbphyW<Pwrweighthp5Spec> {
        WthpusbphyW::new(self, 4)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweighthp5Spec;
impl crate::RegisterSpec for Pwrweighthp5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweighthp5::R`](R) reader structure"]
impl crate::Readable for Pwrweighthp5Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweighthp5::W`](W) writer structure"]
impl crate::Writable for Pwrweighthp5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTHP5 to value 0"]
impl crate::Resettable for Pwrweighthp5Spec {
    const RESET_VALUE: u32 = 0;
}
