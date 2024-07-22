#[doc = "Register `CHIPID1` reader"]
pub type R = crate::R<Chipid1Spec>;
#[doc = "Register `CHIPID1` writer"]
pub type W = crate::W<Chipid1Spec>;
#[doc = "Field `CHIPID1` reader - Unique chip ID 1."]
pub type Chipid1R = crate::FieldReader<u32>;
#[doc = "Field `CHIPID1` writer - Unique chip ID 1."]
pub type Chipid1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unique chip ID 1."]
    #[inline(always)]
    pub fn chipid1(&self) -> Chipid1R {
        Chipid1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unique chip ID 1."]
    #[inline(always)]
    #[must_use]
    pub fn chipid1(&mut self) -> Chipid1W<Chipid1Spec> {
        Chipid1W::new(self, 0)
    }
}
#[doc = "Unique Chip ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chipid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chipid1Spec;
impl crate::RegisterSpec for Chipid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid1::R`](R) reader structure"]
impl crate::Readable for Chipid1Spec {}
#[doc = "`write(|w| ..)` method takes [`chipid1::W`](W) writer structure"]
impl crate::Writable for Chipid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIPID1 to value 0"]
impl crate::Resettable for Chipid1Spec {
    const RESET_VALUE: u32 = 0;
}
