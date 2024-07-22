#[doc = "Register `DEPTHSTARTH` reader"]
pub type R = crate::R<DepthstarthSpec>;
#[doc = "Register `DEPTHSTARTH` writer"]
pub type W = crate::W<DepthstarthSpec>;
#[doc = "Field `DEPTH32HI` reader - Depth value of START pixel"]
pub type Depth32hiR = crate::FieldReader<u32>;
#[doc = "Field `DEPTH32HI` writer - Depth value of START pixel"]
pub type Depth32hiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Depth value of START pixel"]
    #[inline(always)]
    pub fn depth32hi(&self) -> Depth32hiR {
        Depth32hiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Depth value of START pixel"]
    #[inline(always)]
    #[must_use]
    pub fn depth32hi(&mut self) -> Depth32hiW<DepthstarthSpec> {
        Depth32hiW::new(self, 0)
    }
}
#[doc = "Depth value of START pixel, (32 high bits integral.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthstarth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthstarth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DepthstarthSpec;
impl crate::RegisterSpec for DepthstarthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`depthstarth::R`](R) reader structure"]
impl crate::Readable for DepthstarthSpec {}
#[doc = "`write(|w| ..)` method takes [`depthstarth::W`](W) writer structure"]
impl crate::Writable for DepthstarthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEPTHSTARTH to value 0"]
impl crate::Resettable for DepthstarthSpec {
    const RESET_VALUE: u32 = 0;
}
