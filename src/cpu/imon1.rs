#[doc = "Register `IMON1` reader"]
pub type R = crate::R<Imon1Spec>;
#[doc = "Register `IMON1` writer"]
pub type W = crate::W<Imon1Spec>;
#[doc = "Field `ILOOKUP` reader - Total tag lookups from Instruction cache"]
pub type IlookupR = crate::FieldReader<u32>;
#[doc = "Field `ILOOKUP` writer - Total tag lookups from Instruction cache"]
pub type IlookupW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Total tag lookups from Instruction cache"]
    #[inline(always)]
    pub fn ilookup(&self) -> IlookupR {
        IlookupR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total tag lookups from Instruction cache"]
    #[inline(always)]
    #[must_use]
    pub fn ilookup(&mut self) -> IlookupW<Imon1Spec> {
        IlookupW::new(self, 0)
    }
}
#[doc = "Instruction Cache Tag Lookups\n\nYou can [`read`](crate::Reg::read) this register and get [`imon1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imon1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imon1Spec;
impl crate::RegisterSpec for Imon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imon1::R`](R) reader structure"]
impl crate::Readable for Imon1Spec {}
#[doc = "`write(|w| ..)` method takes [`imon1::W`](W) writer structure"]
impl crate::Writable for Imon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMON1 to value 0"]
impl crate::Resettable for Imon1Spec {
    const RESET_VALUE: u32 = 0;
}
