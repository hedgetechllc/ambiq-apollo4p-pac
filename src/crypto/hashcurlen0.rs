#[doc = "Register `HASHCURLEN0` reader"]
pub type R = crate::R<Hashcurlen0Spec>;
#[doc = "Register `HASHCURLEN0` writer"]
pub type W = crate::W<Hashcurlen0Spec>;
#[doc = "Field `Length` reader - Represent the current length of valid bits where digest need to be computed In Bytes."]
pub type LengthR = crate::FieldReader<u32>;
#[doc = "Field `Length` writer - Represent the current length of valid bits where digest need to be computed In Bytes."]
pub type LengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Represent the current length of valid bits where digest need to be computed In Bytes."]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Represent the current length of valid bits where digest need to be computed In Bytes."]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LengthW<Hashcurlen0Spec> {
        LengthW::new(self, 0)
    }
}
#[doc = "This register holds the length of current hash operation bit 31:0.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashcurlen0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashcurlen0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hashcurlen0Spec;
impl crate::RegisterSpec for Hashcurlen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashcurlen0::R`](R) reader structure"]
impl crate::Readable for Hashcurlen0Spec {}
#[doc = "`write(|w| ..)` method takes [`hashcurlen0::W`](W) writer structure"]
impl crate::Writable for Hashcurlen0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHCURLEN0 to value 0"]
impl crate::Resettable for Hashcurlen0Spec {
    const RESET_VALUE: u32 = 0;
}
