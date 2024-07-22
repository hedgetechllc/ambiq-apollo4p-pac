#[doc = "Register `DSTLLIWORD0` reader"]
pub type R = crate::R<Dstlliword0Spec>;
#[doc = "Register `DSTLLIWORD0` writer"]
pub type W = crate::W<Dstlliword0Spec>;
#[doc = "Field `DSTLLIWORD0` reader - Destination address within memory"]
pub type Dstlliword0R = crate::FieldReader<u32>;
#[doc = "Field `DSTLLIWORD0` writer - Destination address within memory"]
pub type Dstlliword0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address within memory"]
    #[inline(always)]
    pub fn dstlliword0(&self) -> Dstlliword0R {
        Dstlliword0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address within memory"]
    #[inline(always)]
    #[must_use]
    pub fn dstlliword0(&mut self) -> Dstlliword0W<Dstlliword0Spec> {
        Dstlliword0W::new(self, 0)
    }
}
#[doc = "This register is used in direct LLI mode - holds the location of the data destination in the memory (AXI)\n\nYou can [`read`](crate::Reg::read) this register and get [`dstlliword0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstlliword0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dstlliword0Spec;
impl crate::RegisterSpec for Dstlliword0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstlliword0::R`](R) reader structure"]
impl crate::Readable for Dstlliword0Spec {}
#[doc = "`write(|w| ..)` method takes [`dstlliword0::W`](W) writer structure"]
impl crate::Writable for Dstlliword0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSTLLIWORD0 to value 0"]
impl crate::Resettable for Dstlliword0Spec {
    const RESET_VALUE: u32 = 0;
}
