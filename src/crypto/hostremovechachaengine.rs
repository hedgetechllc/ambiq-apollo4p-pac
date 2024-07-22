#[doc = "Register `HOSTREMOVECHACHAENGINE` reader"]
pub type R = crate::R<HostremovechachaengineSpec>;
#[doc = "Register `HOSTREMOVECHACHAENGINE` writer"]
pub type W = crate::W<HostremovechachaengineSpec>;
#[doc = "Field `HOSTREMOVECHACHAENGINE` reader - Read the Remove_ghash_engine input"]
pub type HostremovechachaengineR = crate::BitReader;
#[doc = "Field `HOSTREMOVECHACHAENGINE` writer - Read the Remove_ghash_engine input"]
pub type HostremovechachaengineW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read the Remove_ghash_engine input"]
    #[inline(always)]
    pub fn hostremovechachaengine(&self) -> HostremovechachaengineR {
        HostremovechachaengineR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read the Remove_ghash_engine input"]
    #[inline(always)]
    #[must_use]
    pub fn hostremovechachaengine(
        &mut self,
    ) -> HostremovechachaengineW<HostremovechachaengineSpec> {
        HostremovechachaengineW::new(self, 0)
    }
}
#[doc = "These inputs are to be statically tied to 0 or 1 by the customers. When such an input is set, the matching engines inputs are tied to zero and its outputs are disconnected, so that the engine will be entirely removed by Synthesis\n\nYou can [`read`](crate::Reg::read) this register and get [`hostremovechachaengine::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostremovechachaengine::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostremovechachaengineSpec;
impl crate::RegisterSpec for HostremovechachaengineSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostremovechachaengine::R`](R) reader structure"]
impl crate::Readable for HostremovechachaengineSpec {}
#[doc = "`write(|w| ..)` method takes [`hostremovechachaengine::W`](W) writer structure"]
impl crate::Writable for HostremovechachaengineSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTREMOVECHACHAENGINE to value 0"]
impl crate::Resettable for HostremovechachaengineSpec {
    const RESET_VALUE: u32 = 0;
}
