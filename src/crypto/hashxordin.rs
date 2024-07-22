#[doc = "Register `HASHXORDIN` reader"]
pub type R = crate::R<HashxordinSpec>;
#[doc = "Register `HASHXORDIN` writer"]
pub type W = crate::W<HashxordinSpec>;
#[doc = "Field `HASHXORDATA` reader - This register holds the value to be xor-ed with hash input data."]
pub type HashxordataR = crate::FieldReader<u32>;
#[doc = "Field `HASHXORDATA` writer - This register holds the value to be xor-ed with hash input data."]
pub type HashxordataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register holds the value to be xor-ed with hash input data."]
    #[inline(always)]
    pub fn hashxordata(&self) -> HashxordataR {
        HashxordataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register holds the value to be xor-ed with hash input data."]
    #[inline(always)]
    #[must_use]
    pub fn hashxordata(&mut self) -> HashxordataW<HashxordinSpec> {
        HashxordataW::new(self, 0)
    }
}
#[doc = "This register is always xored with the input to the hash engine,it should be 0 if xored is not reqiured .\n\nYou can [`read`](crate::Reg::read) this register and get [`hashxordin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashxordin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashxordinSpec;
impl crate::RegisterSpec for HashxordinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashxordin::R`](R) reader structure"]
impl crate::Readable for HashxordinSpec {}
#[doc = "`write(|w| ..)` method takes [`hashxordin::W`](W) writer structure"]
impl crate::Writable for HashxordinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHXORDIN to value 0"]
impl crate::Resettable for HashxordinSpec {
    const RESET_VALUE: u32 = 0;
}
