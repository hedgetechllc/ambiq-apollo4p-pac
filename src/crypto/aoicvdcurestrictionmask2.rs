#[doc = "Register `AOICVDCURESTRICTIONMASK2` reader"]
pub type R = crate::R<Aoicvdcurestrictionmask2Spec>;
#[doc = "Register `AOICVDCURESTRICTIONMASK2` writer"]
pub type W = crate::W<Aoicvdcurestrictionmask2Spec>;
#[doc = "Field `AOICVDCURESTRICTIONMASK2` reader - AO_ICV_DCU_RESTRICTION_MASK \\[95:64\\]
parameter, that will be a customer modifiable."]
pub type Aoicvdcurestrictionmask2R = crate::FieldReader<u32>;
#[doc = "Field `AOICVDCURESTRICTIONMASK2` writer - AO_ICV_DCU_RESTRICTION_MASK \\[95:64\\]
parameter, that will be a customer modifiable."]
pub type Aoicvdcurestrictionmask2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AO_ICV_DCU_RESTRICTION_MASK \\[95:64\\]
parameter, that will be a customer modifiable."]
    #[inline(always)]
    pub fn aoicvdcurestrictionmask2(&self) -> Aoicvdcurestrictionmask2R {
        Aoicvdcurestrictionmask2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AO_ICV_DCU_RESTRICTION_MASK \\[95:64\\]
parameter, that will be a customer modifiable."]
    #[inline(always)]
    #[must_use]
    pub fn aoicvdcurestrictionmask2(
        &mut self,
    ) -> Aoicvdcurestrictionmask2W<Aoicvdcurestrictionmask2Spec> {
        Aoicvdcurestrictionmask2W::new(self, 0)
    }
}
#[doc = "The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets\n\nYou can [`read`](crate::Reg::read) this register and get [`aoicvdcurestrictionmask2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoicvdcurestrictionmask2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aoicvdcurestrictionmask2Spec;
impl crate::RegisterSpec for Aoicvdcurestrictionmask2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoicvdcurestrictionmask2::R`](R) reader structure"]
impl crate::Readable for Aoicvdcurestrictionmask2Spec {}
#[doc = "`write(|w| ..)` method takes [`aoicvdcurestrictionmask2::W`](W) writer structure"]
impl crate::Writable for Aoicvdcurestrictionmask2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AOICVDCURESTRICTIONMASK2 to value 0"]
impl crate::Resettable for Aoicvdcurestrictionmask2Spec {
    const RESET_VALUE: u32 = 0;
}
