#[doc = "Register `FIFOINEMPTY` reader"]
pub type R = crate::R<FifoinemptySpec>;
#[doc = "Register `FIFOINEMPTY` writer"]
pub type W = crate::W<FifoinemptySpec>;
#[doc = "Field `EMPTY` reader - 0x1 - FIFO empty"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `EMPTY` writer - 0x1 - FIFO empty"]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0x1 - FIFO empty"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0x1 - FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<FifoinemptySpec> {
        EmptyW::new(self, 0)
    }
}
#[doc = "DIN FIFO empty indication\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoinempty::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoinempty::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoinemptySpec;
impl crate::RegisterSpec for FifoinemptySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoinempty::R`](R) reader structure"]
impl crate::Readable for FifoinemptySpec {}
#[doc = "`write(|w| ..)` method takes [`fifoinempty::W`](W) writer structure"]
impl crate::Writable for FifoinemptySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOINEMPTY to value 0x01"]
impl crate::Resettable for FifoinemptySpec {
    const RESET_VALUE: u32 = 0x01;
}
