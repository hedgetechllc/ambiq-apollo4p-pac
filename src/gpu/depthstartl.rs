#[doc = "Register `DEPTHSTARTL` reader"]
pub type R = crate::R<DepthstartlSpec>;
#[doc = "Register `DEPTHSTARTL` writer"]
pub type W = crate::W<DepthstartlSpec>;
#[doc = "Field `DEPTH32LO` reader - Depth value of START pixel"]
pub type Depth32loR = crate::FieldReader<u32>;
#[doc = "Field `DEPTH32LO` writer - Depth value of START pixel"]
pub type Depth32loW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Depth value of START pixel"]
    #[inline(always)]
    pub fn depth32lo(&self) -> Depth32loR {
        Depth32loR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Depth value of START pixel"]
    #[inline(always)]
    #[must_use]
    pub fn depth32lo(&mut self) -> Depth32loW<DepthstartlSpec> {
        Depth32loW::new(self, 0)
    }
}
#[doc = "Depth value of START pixel, (32 low bits fractional.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthstartl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthstartl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DepthstartlSpec;
impl crate::RegisterSpec for DepthstartlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`depthstartl::R`](R) reader structure"]
impl crate::Readable for DepthstartlSpec {}
#[doc = "`write(|w| ..)` method takes [`depthstartl::W`](W) writer structure"]
impl crate::Writable for DepthstartlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEPTHSTARTL to value 0"]
impl crate::Resettable for DepthstartlSpec {
    const RESET_VALUE: u32 = 0;
}
