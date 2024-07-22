#[doc = "Register `AOICVDCURESTRICTIONMASK1` reader"]
pub type R = crate::R<Aoicvdcurestrictionmask1Spec>;
#[doc = "Register `AOICVDCURESTRICTIONMASK1` writer"]
pub type W = crate::W<Aoicvdcurestrictionmask1Spec>;
#[doc = "Field `AOICVDCURESTRICTIONMASK1` reader - AO_ICV_DCU_RESTRICTION_MASK \\[63:32\\]
parameter, that will be a customer modifiable."]
pub type Aoicvdcurestrictionmask1R = crate::FieldReader<u32>;
#[doc = "Field `AOICVDCURESTRICTIONMASK1` writer - AO_ICV_DCU_RESTRICTION_MASK \\[63:32\\]
parameter, that will be a customer modifiable."]
pub type Aoicvdcurestrictionmask1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AO_ICV_DCU_RESTRICTION_MASK \\[63:32\\]
parameter, that will be a customer modifiable."]
    #[inline(always)]
    pub fn aoicvdcurestrictionmask1(&self) -> Aoicvdcurestrictionmask1R {
        Aoicvdcurestrictionmask1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AO_ICV_DCU_RESTRICTION_MASK \\[63:32\\]
parameter, that will be a customer modifiable."]
    #[inline(always)]
    #[must_use]
    pub fn aoicvdcurestrictionmask1(
        &mut self,
    ) -> Aoicvdcurestrictionmask1W<Aoicvdcurestrictionmask1Spec> {
        Aoicvdcurestrictionmask1W::new(self, 0)
    }
}
#[doc = "The 'ICV_DCU_restriction_mask' parameter is read by FW during the secure debug verification to prevent OEM from setting specific DCUs that protect ICV secrets\n\nYou can [`read`](crate::Reg::read) this register and get [`aoicvdcurestrictionmask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoicvdcurestrictionmask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aoicvdcurestrictionmask1Spec;
impl crate::RegisterSpec for Aoicvdcurestrictionmask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoicvdcurestrictionmask1::R`](R) reader structure"]
impl crate::Readable for Aoicvdcurestrictionmask1Spec {}
#[doc = "`write(|w| ..)` method takes [`aoicvdcurestrictionmask1::W`](W) writer structure"]
impl crate::Writable for Aoicvdcurestrictionmask1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AOICVDCURESTRICTIONMASK1 to value 0xffff_ffff"]
impl crate::Resettable for Aoicvdcurestrictionmask1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
