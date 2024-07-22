#[doc = "Register `RNDSOURCEENABLE` reader"]
pub type R = crate::R<RndsourceenableSpec>;
#[doc = "Register `RNDSOURCEENABLE` writer"]
pub type W = crate::W<RndsourceenableSpec>;
#[doc = "Field `RNDSRCEN` reader - Enable signal for the random source."]
pub type RndsrcenR = crate::BitReader;
#[doc = "Field `RNDSRCEN` writer - Enable signal for the random source."]
pub type RndsrcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable signal for the random source."]
    #[inline(always)]
    pub fn rndsrcen(&self) -> RndsrcenR {
        RndsrcenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable signal for the random source."]
    #[inline(always)]
    #[must_use]
    pub fn rndsrcen(&mut self) -> RndsrcenW<RndsourceenableSpec> {
        RndsrcenW::new(self, 0)
    }
}
#[doc = "This register holds the enable signal for the random source.\n\nYou can [`read`](crate::Reg::read) this register and get [`rndsourceenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rndsourceenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RndsourceenableSpec;
impl crate::RegisterSpec for RndsourceenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rndsourceenable::R`](R) reader structure"]
impl crate::Readable for RndsourceenableSpec {}
#[doc = "`write(|w| ..)` method takes [`rndsourceenable::W`](W) writer structure"]
impl crate::Writable for RndsourceenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNDSOURCEENABLE to value 0"]
impl crate::Resettable for RndsourceenableSpec {
    const RESET_VALUE: u32 = 0;
}
