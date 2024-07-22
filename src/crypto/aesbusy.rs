#[doc = "Register `AESBUSY` reader"]
pub type R = crate::R<AesbusySpec>;
#[doc = "Register `AESBUSY` writer"]
pub type W = crate::W<AesbusySpec>;
#[doc = "Field `AESBUSY` reader - AES_BUSY register. This register is set when the AES core is active"]
pub type AesbusyR = crate::BitReader;
#[doc = "Field `AESBUSY` writer - AES_BUSY register. This register is set when the AES core is active"]
pub type AesbusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES_BUSY register. This register is set when the AES core is active"]
    #[inline(always)]
    pub fn aesbusy(&self) -> AesbusyR {
        AesbusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES_BUSY register. This register is set when the AES core is active"]
    #[inline(always)]
    #[must_use]
    pub fn aesbusy(&mut self) -> AesbusyW<AesbusySpec> {
        AesbusyW::new(self, 0)
    }
}
#[doc = "This register is set when the AES core is active\n\nYou can [`read`](crate::Reg::read) this register and get [`aesbusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesbusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesbusySpec;
impl crate::RegisterSpec for AesbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesbusy::R`](R) reader structure"]
impl crate::Readable for AesbusySpec {}
#[doc = "`write(|w| ..)` method takes [`aesbusy::W`](W) writer structure"]
impl crate::Writable for AesbusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESBUSY to value 0"]
impl crate::Resettable for AesbusySpec {
    const RESET_VALUE: u32 = 0;
}
