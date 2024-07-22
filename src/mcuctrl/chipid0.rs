#[doc = "Register `CHIPID0` reader"]
pub type R = crate::R<Chipid0Spec>;
#[doc = "Register `CHIPID0` writer"]
pub type W = crate::W<Chipid0Spec>;
#[doc = "Field `CHIPID0` reader - Unique chip ID 0."]
pub type Chipid0R = crate::FieldReader<u32>;
#[doc = "Field `CHIPID0` writer - Unique chip ID 0."]
pub type Chipid0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unique chip ID 0."]
    #[inline(always)]
    pub fn chipid0(&self) -> Chipid0R {
        Chipid0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unique chip ID 0."]
    #[inline(always)]
    #[must_use]
    pub fn chipid0(&mut self) -> Chipid0W<Chipid0Spec> {
        Chipid0W::new(self, 0)
    }
}
#[doc = "Unique Chip ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chipid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chipid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chipid0Spec;
impl crate::RegisterSpec for Chipid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid0::R`](R) reader structure"]
impl crate::Readable for Chipid0Spec {}
#[doc = "`write(|w| ..)` method takes [`chipid0::W`](W) writer structure"]
impl crate::Writable for Chipid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIPID0 to value 0"]
impl crate::Resettable for Chipid0Spec {
    const RESET_VALUE: u32 = 0;
}
