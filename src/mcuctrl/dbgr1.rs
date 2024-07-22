#[doc = "Register `DBGR1` reader"]
pub type R = crate::R<Dbgr1Spec>;
#[doc = "Register `DBGR1` writer"]
pub type W = crate::W<Dbgr1Spec>;
#[doc = "Field `ONETO8` reader - Read-only register for communication validation"]
pub type Oneto8R = crate::FieldReader<u32>;
#[doc = "Field `ONETO8` writer - Read-only register for communication validation"]
pub type Oneto8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    pub fn oneto8(&self) -> Oneto8R {
        Oneto8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read-only register for communication validation"]
    #[inline(always)]
    #[must_use]
    pub fn oneto8(&mut self) -> Oneto8W<Dbgr1Spec> {
        Oneto8W::new(self, 0)
    }
}
#[doc = "Read-only debug 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbgr1Spec;
impl crate::RegisterSpec for Dbgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgr1::R`](R) reader structure"]
impl crate::Readable for Dbgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgr1::W`](W) writer structure"]
impl crate::Writable for Dbgr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGR1 to value 0x1234_5678"]
impl crate::Resettable for Dbgr1Spec {
    const RESET_VALUE: u32 = 0x1234_5678;
}
