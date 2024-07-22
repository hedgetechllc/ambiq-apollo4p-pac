#[doc = "Register `READALIGNLAST` reader"]
pub type R = crate::R<ReadalignlastSpec>;
#[doc = "Register `READALIGNLAST` writer"]
pub type W = crate::W<ReadalignlastSpec>;
#[doc = "Field `LAST` reader - 0x1 - Flush the read aligner content (used for reading the last data)."]
pub type LastR = crate::BitReader;
#[doc = "Field `LAST` writer - 0x1 - Flush the read aligner content (used for reading the last data)."]
pub type LastW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0x1 - Flush the read aligner content (used for reading the last data)."]
    #[inline(always)]
    pub fn last(&self) -> LastR {
        LastR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0x1 - Flush the read aligner content (used for reading the last data)."]
    #[inline(always)]
    #[must_use]
    pub fn last(&mut self) -> LastW<ReadalignlastSpec> {
        LastW::new(self, 0)
    }
}
#[doc = "Indication that the next read from the CPU is the last one. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding).\n\nYou can [`read`](crate::Reg::read) this register and get [`readalignlast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readalignlast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadalignlastSpec;
impl crate::RegisterSpec for ReadalignlastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readalignlast::R`](R) reader structure"]
impl crate::Readable for ReadalignlastSpec {}
#[doc = "`write(|w| ..)` method takes [`readalignlast::W`](W) writer structure"]
impl crate::Writable for ReadalignlastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets READALIGNLAST to value 0"]
impl crate::Resettable for ReadalignlastSpec {
    const RESET_VALUE: u32 = 0;
}
