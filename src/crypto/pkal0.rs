#[doc = "Register `PKAL0` reader"]
pub type R = crate::R<Pkal0Spec>;
#[doc = "Register `PKAL0` writer"]
pub type W = crate::W<Pkal0Spec>;
#[doc = "Field `PKAL0` reader - Size of the operation in bytes."]
pub type Pkal0R = crate::FieldReader<u16>;
#[doc = "Field `PKAL0` writer - Size of the operation in bytes."]
pub type Pkal0W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    pub fn pkal0(&self) -> Pkal0R {
        Pkal0R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Size of the operation in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn pkal0(&mut self) -> Pkal0W<Pkal0Spec> {
        Pkal0W::new(self, 0)
    }
}
#[doc = "This register holds one of the optional size of the operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkal0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkal0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkal0Spec;
impl crate::RegisterSpec for Pkal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkal0::R`](R) reader structure"]
impl crate::Readable for Pkal0Spec {}
#[doc = "`write(|w| ..)` method takes [`pkal0::W`](W) writer structure"]
impl crate::Writable for Pkal0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAL0 to value 0"]
impl crate::Resettable for Pkal0Spec {
    const RESET_VALUE: u32 = 0;
}
