#[doc = "Register `PWRWEIGHTLP5` reader"]
pub type R = crate::R<Pwrweightlp5Spec>;
#[doc = "Register `PWRWEIGHTLP5` writer"]
pub type W = crate::W<Pwrweightlp5Spec>;
#[doc = "Field `WTLPDISPPHY` reader - Weight used for LP mode DISP PHY"]
pub type WtlpdispphyR = crate::FieldReader;
#[doc = "Field `WTLPDISPPHY` writer - Weight used for LP mode DISP PHY"]
pub type WtlpdispphyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPUSBPHY` reader - Weight used for LP mode USB PHY"]
pub type WtlpusbphyR = crate::FieldReader;
#[doc = "Field `WTLPUSBPHY` writer - Weight used for LP mode USB PHY"]
pub type WtlpusbphyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for LP mode DISP PHY"]
    #[inline(always)]
    pub fn wtlpdispphy(&self) -> WtlpdispphyR {
        WtlpdispphyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode USB PHY"]
    #[inline(always)]
    pub fn wtlpusbphy(&self) -> WtlpusbphyR {
        WtlpusbphyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for LP mode DISP PHY"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpdispphy(&mut self) -> WtlpdispphyW<Pwrweightlp5Spec> {
        WtlpdispphyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode USB PHY"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpusbphy(&mut self) -> WtlpusbphyW<Pwrweightlp5Spec> {
        WtlpusbphyW::new(self, 4)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightlp5Spec;
impl crate::RegisterSpec for Pwrweightlp5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightlp5::R`](R) reader structure"]
impl crate::Readable for Pwrweightlp5Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightlp5::W`](W) writer structure"]
impl crate::Writable for Pwrweightlp5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTLP5 to value 0"]
impl crate::Resettable for Pwrweightlp5Spec {
    const RESET_VALUE: u32 = 0;
}
