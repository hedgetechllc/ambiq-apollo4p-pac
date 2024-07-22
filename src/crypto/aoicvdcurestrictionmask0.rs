#[doc = "Register `AOICVDCURESTRICTIONMASK0` reader"]
pub type R = crate::R<Aoicvdcurestrictionmask0Spec>;
#[doc = "Register `AOICVDCURESTRICTIONMASK0` writer"]
pub type W = crate::W<Aoicvdcurestrictionmask0Spec>;
#[doc = "Field `AOICVDCURESTRICTIONMASK0` reader - AO_ICV_DCU_RESTRICTION_MASK \\[31:0\\]
parameter, that will be a customer modifiable."]
pub type Aoicvdcurestrictionmask0R = crate::FieldReader<u32>;
#[doc = "Field `AOICVDCURESTRICTIONMASK0` writer - AO_ICV_DCU_RESTRICTION_MASK \\[31:0\\]
parameter, that will be a customer modifiable."]
pub type Aoicvdcurestrictionmask0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AO_ICV_DCU_RESTRICTION_MASK \\[31:0\\]
parameter, that will be a customer modifiable."]
    #[inline(always)]
    pub fn aoicvdcurestrictionmask0(&self) -> Aoicvdcurestrictionmask0R {
        Aoicvdcurestrictionmask0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AO_ICV_DCU_RESTRICTION_MASK \\[31:0\\]
parameter, that will be a customer modifiable."]
    #[inline(always)]
    #[must_use]
    pub fn aoicvdcurestrictionmask0(
        &mut self,
    ) -> Aoicvdcurestrictionmask0W<Aoicvdcurestrictionmask0Spec> {
        Aoicvdcurestrictionmask0W::new(self, 0)
    }
}
#[doc = "The DCU lock register.\n\nYou can [`read`](crate::Reg::read) this register and get [`aoicvdcurestrictionmask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoicvdcurestrictionmask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aoicvdcurestrictionmask0Spec;
impl crate::RegisterSpec for Aoicvdcurestrictionmask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoicvdcurestrictionmask0::R`](R) reader structure"]
impl crate::Readable for Aoicvdcurestrictionmask0Spec {}
#[doc = "`write(|w| ..)` method takes [`aoicvdcurestrictionmask0::W`](W) writer structure"]
impl crate::Writable for Aoicvdcurestrictionmask0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AOICVDCURESTRICTIONMASK0 to value 0xffff_ffff"]
impl crate::Resettable for Aoicvdcurestrictionmask0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
