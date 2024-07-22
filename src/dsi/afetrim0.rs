#[doc = "Register `AFETRIM0` reader"]
pub type R = crate::R<Afetrim0Spec>;
#[doc = "Register `AFETRIM0` writer"]
pub type W = crate::W<Afetrim0Spec>;
#[doc = "Field `AFETRIM0` reader - Afe Trim reg0."]
pub type Afetrim0R = crate::FieldReader<u32>;
#[doc = "Field `AFETRIM0` writer - Afe Trim reg0."]
pub type Afetrim0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Afe Trim reg0."]
    #[inline(always)]
    pub fn afetrim0(&self) -> Afetrim0R {
        Afetrim0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Afe Trim reg0."]
    #[inline(always)]
    #[must_use]
    pub fn afetrim0(&mut self) -> Afetrim0W<Afetrim0Spec> {
        Afetrim0W::new(self, 0)
    }
}
#[doc = "Afe Trim reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`afetrim0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afetrim0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afetrim0Spec;
impl crate::RegisterSpec for Afetrim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afetrim0::R`](R) reader structure"]
impl crate::Readable for Afetrim0Spec {}
#[doc = "`write(|w| ..)` method takes [`afetrim0::W`](W) writer structure"]
impl crate::Writable for Afetrim0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFETRIM0 to value 0x1a00_0615"]
impl crate::Resettable for Afetrim0Spec {
    const RESET_VALUE: u32 = 0x1a00_0615;
}
