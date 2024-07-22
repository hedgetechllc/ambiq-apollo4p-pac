#[doc = "Register `AOICVDCURESTRICTIONMASK3` reader"]
pub type R = crate::R<Aoicvdcurestrictionmask3Spec>;
#[doc = "Register `AOICVDCURESTRICTIONMASK3` writer"]
pub type W = crate::W<Aoicvdcurestrictionmask3Spec>;
#[doc = "Field `AOICVDCURESTRICTIONMASK3` reader - AO_ICV_DCU_RESTRICTION_MASK \\[127:96\\]
parameter, that will be a customer modifiable."]
pub type Aoicvdcurestrictionmask3R = crate::FieldReader<u32>;
#[doc = "Field `AOICVDCURESTRICTIONMASK3` writer - AO_ICV_DCU_RESTRICTION_MASK \\[127:96\\]
parameter, that will be a customer modifiable."]
pub type Aoicvdcurestrictionmask3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AO_ICV_DCU_RESTRICTION_MASK \\[127:96\\]
parameter, that will be a customer modifiable."]
    #[inline(always)]
    pub fn aoicvdcurestrictionmask3(&self) -> Aoicvdcurestrictionmask3R {
        Aoicvdcurestrictionmask3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AO_ICV_DCU_RESTRICTION_MASK \\[127:96\\]
parameter, that will be a customer modifiable."]
    #[inline(always)]
    #[must_use]
    pub fn aoicvdcurestrictionmask3(
        &mut self,
    ) -> Aoicvdcurestrictionmask3W<Aoicvdcurestrictionmask3Spec> {
        Aoicvdcurestrictionmask3W::new(self, 0)
    }
}
#[doc = "The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets\n\nYou can [`read`](crate::Reg::read) this register and get [`aoicvdcurestrictionmask3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoicvdcurestrictionmask3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aoicvdcurestrictionmask3Spec;
impl crate::RegisterSpec for Aoicvdcurestrictionmask3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoicvdcurestrictionmask3::R`](R) reader structure"]
impl crate::Readable for Aoicvdcurestrictionmask3Spec {}
#[doc = "`write(|w| ..)` method takes [`aoicvdcurestrictionmask3::W`](W) writer structure"]
impl crate::Writable for Aoicvdcurestrictionmask3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AOICVDCURESTRICTIONMASK3 to value 0"]
impl crate::Resettable for Aoicvdcurestrictionmask3Spec {
    const RESET_VALUE: u32 = 0;
}
