#[doc = "Register `DBGR2` reader"]
pub type R = crate::R<Dbgr2Spec>;
#[doc = "Register `DBGR2` writer"]
pub type W = crate::W<Dbgr2Spec>;
#[doc = "Field `COOLCODE` reader - Read-only register for communication validation"]
pub type CoolcodeR = crate::FieldReader<u32>;
#[doc = "Field `COOLCODE` writer - Read-only register for communication validation"]
pub type CoolcodeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    pub fn coolcode(&self) -> CoolcodeR {
        CoolcodeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    #[must_use]
    pub fn coolcode(&mut self) -> CoolcodeW<Dbgr2Spec> {
        CoolcodeW::new(self, 0)
    }
}
#[doc = "Read-only debug 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbgr2Spec;
impl crate::RegisterSpec for Dbgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgr2::R`](R) reader structure"]
impl crate::Readable for Dbgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgr2::W`](W) writer structure"]
impl crate::Writable for Dbgr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGR2 to value 0xc001_c0de"]
impl crate::Resettable for Dbgr2Spec {
    const RESET_VALUE: u32 = 0xc001_c0de;
}
