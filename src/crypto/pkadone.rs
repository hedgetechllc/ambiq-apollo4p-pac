#[doc = "Register `PKADONE` reader"]
pub type R = crate::R<PkadoneSpec>;
#[doc = "Register `PKADONE` writer"]
pub type W = crate::W<PkadoneSpec>;
#[doc = "Field `PKADONE` reader - Indicates if PKA operation is completed, and pipe is empty."]
pub type PkadoneR = crate::BitReader;
#[doc = "Field `PKADONE` writer - Indicates if PKA operation is completed, and pipe is empty."]
pub type PkadoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates if PKA operation is completed, and pipe is empty."]
    #[inline(always)]
    pub fn pkadone(&self) -> PkadoneR {
        PkadoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if PKA operation is completed, and pipe is empty."]
    #[inline(always)]
    #[must_use]
    pub fn pkadone(&mut self) -> PkadoneW<PkadoneSpec> {
        PkadoneW::new(self, 0)
    }
}
#[doc = "This register indicates whether PKA operation is completed.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkadone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkadone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkadoneSpec;
impl crate::RegisterSpec for PkadoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkadone::R`](R) reader structure"]
impl crate::Readable for PkadoneSpec {}
#[doc = "`write(|w| ..)` method takes [`pkadone::W`](W) writer structure"]
impl crate::Writable for PkadoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKADONE to value 0x01"]
impl crate::Resettable for PkadoneSpec {
    const RESET_VALUE: u32 = 0x01;
}
