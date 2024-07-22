#[doc = "Register `DIRTYTRIGMIN` reader"]
pub type R = crate::R<DirtytrigminSpec>;
#[doc = "Register `DIRTYTRIGMIN` writer"]
pub type W = crate::W<DirtytrigminSpec>;
#[doc = "Field `DRTYREG` reader - Resets dirty region to resolution size when written."]
pub type DrtyregR = crate::FieldReader<u32>;
#[doc = "Field `DRTYREG` writer - Resets dirty region to resolution size when written."]
pub type DrtyregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Resets dirty region to resolution size when written."]
    #[inline(always)]
    pub fn drtyreg(&self) -> DrtyregR {
        DrtyregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Resets dirty region to resolution size when written."]
    #[inline(always)]
    #[must_use]
    pub fn drtyreg(&mut self) -> DrtyregW<DirtytrigminSpec> {
        DrtyregW::new(self, 0)
    }
}
#[doc = "Resets dirty region to resolution size when written.\n\nYou can [`read`](crate::Reg::read) this register and get [`dirtytrigmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirtytrigmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirtytrigminSpec;
impl crate::RegisterSpec for DirtytrigminSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirtytrigmin::R`](R) reader structure"]
impl crate::Readable for DirtytrigminSpec {}
#[doc = "`write(|w| ..)` method takes [`dirtytrigmin::W`](W) writer structure"]
impl crate::Writable for DirtytrigminSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRTYTRIGMIN to value 0xffff_ffff"]
impl crate::Resettable for DirtytrigminSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
