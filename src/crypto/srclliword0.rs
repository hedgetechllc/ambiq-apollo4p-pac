#[doc = "Register `SRCLLIWORD0` reader"]
pub type R = crate::R<Srclliword0Spec>;
#[doc = "Register `SRCLLIWORD0` writer"]
pub type W = crate::W<Srclliword0Spec>;
#[doc = "Field `SRCLLIWORD0` reader - Source address within memory."]
pub type Srclliword0R = crate::FieldReader<u32>;
#[doc = "Field `SRCLLIWORD0` writer - Source address within memory."]
pub type Srclliword0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source address within memory."]
    #[inline(always)]
    pub fn srclliword0(&self) -> Srclliword0R {
        Srclliword0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source address within memory."]
    #[inline(always)]
    #[must_use]
    pub fn srclliword0(&mut self) -> Srclliword0W<Srclliword0Spec> {
        Srclliword0W::new(self, 0)
    }
}
#[doc = "This register is used in direct LLI mode - holds the location of the data source in the memory (AXI).\n\nYou can [`read`](crate::Reg::read) this register and get [`srclliword0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srclliword0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srclliword0Spec;
impl crate::RegisterSpec for Srclliword0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srclliword0::R`](R) reader structure"]
impl crate::Readable for Srclliword0Spec {}
#[doc = "`write(|w| ..)` method takes [`srclliword0::W`](W) writer structure"]
impl crate::Writable for Srclliword0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCLLIWORD0 to value 0"]
impl crate::Resettable for Srclliword0Spec {
    const RESET_VALUE: u32 = 0;
}
