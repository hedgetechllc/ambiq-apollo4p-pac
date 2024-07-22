#[doc = "Register `HOSTREMOVEGHASHENGINE` reader"]
pub type R = crate::R<HostremoveghashengineSpec>;
#[doc = "Register `HOSTREMOVEGHASHENGINE` writer"]
pub type W = crate::W<HostremoveghashengineSpec>;
#[doc = "Field `HOSTREMOVEGHASHENGINE` reader - Read the Remove_chacha_engine input"]
pub type HostremoveghashengineR = crate::BitReader;
#[doc = "Field `HOSTREMOVEGHASHENGINE` writer - Read the Remove_chacha_engine input"]
pub type HostremoveghashengineW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read the Remove_chacha_engine input"]
    #[inline(always)]
    pub fn hostremoveghashengine(&self) -> HostremoveghashengineR {
        HostremoveghashengineR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read the Remove_chacha_engine input"]
    #[inline(always)]
    #[must_use]
    pub fn hostremoveghashengine(&mut self) -> HostremoveghashengineW<HostremoveghashengineSpec> {
        HostremoveghashengineW::new(self, 0)
    }
}
#[doc = "These inputs are to be statically tied to 0 or 1 by the customers. When such an input is set, the matching engines inputs are tied to zero and its outputs are disconnected, so that the engine will be entirely removed by Synthesis\n\nYou can [`read`](crate::Reg::read) this register and get [`hostremoveghashengine::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostremoveghashengine::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostremoveghashengineSpec;
impl crate::RegisterSpec for HostremoveghashengineSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostremoveghashengine::R`](R) reader structure"]
impl crate::Readable for HostremoveghashengineSpec {}
#[doc = "`write(|w| ..)` method takes [`hostremoveghashengine::W`](W) writer structure"]
impl crate::Writable for HostremoveghashengineSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTREMOVEGHASHENGINE to value 0"]
impl crate::Resettable for HostremoveghashengineSpec {
    const RESET_VALUE: u32 = 0;
}
