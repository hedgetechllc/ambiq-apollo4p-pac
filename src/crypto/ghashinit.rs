#[doc = "Register `GHASHINIT` reader"]
pub type R = crate::R<GhashinitSpec>;
#[doc = "Register `GHASHINIT` writer"]
pub type W = crate::W<GhashinitSpec>;
#[doc = "Field `GHASHINIT` reader - Writing to this address sets the GHASH engine to be ready to a new GHASH operation."]
pub type GhashinitR = crate::BitReader;
#[doc = "Field `GHASHINIT` writer - Writing to this address sets the GHASH engine to be ready to a new GHASH operation."]
pub type GhashinitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing to this address sets the GHASH engine to be ready to a new GHASH operation."]
    #[inline(always)]
    pub fn ghashinit(&self) -> GhashinitR {
        GhashinitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing to this address sets the GHASH engine to be ready to a new GHASH operation."]
    #[inline(always)]
    #[must_use]
    pub fn ghashinit(&mut self) -> GhashinitW<GhashinitSpec> {
        GhashinitW::new(self, 0)
    }
}
#[doc = "Writing to this address sets the GHASH engine to be ready to a new GHASH operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghashinit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghashinit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GhashinitSpec;
impl crate::RegisterSpec for GhashinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghashinit::R`](R) reader structure"]
impl crate::Readable for GhashinitSpec {}
#[doc = "`write(|w| ..)` method takes [`ghashinit::W`](W) writer structure"]
impl crate::Writable for GhashinitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHASHINIT to value 0"]
impl crate::Resettable for GhashinitSpec {
    const RESET_VALUE: u32 = 0;
}
