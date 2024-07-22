#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `AXIWERROR` reader - AXI Write Error Occurred"]
pub type AxiwerrorR = crate::BitReader;
#[doc = "Field `AXIWERROR` writer - AXI Write Error Occurred"]
pub type AxiwerrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AXI Write Error Occurred"]
    #[inline(always)]
    pub fn axiwerror(&self) -> AxiwerrorR {
        AxiwerrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AXI Write Error Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn axiwerror(&mut self) -> AxiwerrorW<IntenSpec> {
        AxiwerrorW::new(self, 0)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
