#[doc = "Register `AESCTRNOINCREMENT` reader"]
pub type R = crate::R<AesctrnoincrementSpec>;
#[doc = "Register `AESCTRNOINCREMENT` writer"]
pub type W = crate::W<AesctrnoincrementSpec>;
#[doc = "Field `AESCTRNOINCREMENT` reader - This field enables the AES CTR 'no increment' mode (in which the counter mode is not incremented between 2 blocks)"]
pub type AesctrnoincrementR = crate::BitReader;
#[doc = "Field `AESCTRNOINCREMENT` writer - This field enables the AES CTR 'no increment' mode (in which the counter mode is not incremented between 2 blocks)"]
pub type AesctrnoincrementW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field enables the AES CTR 'no increment' mode (in which the counter mode is not incremented between 2 blocks)"]
    #[inline(always)]
    pub fn aesctrnoincrement(&self) -> AesctrnoincrementR {
        AesctrnoincrementR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field enables the AES CTR 'no increment' mode (in which the counter mode is not incremented between 2 blocks)"]
    #[inline(always)]
    #[must_use]
    pub fn aesctrnoincrement(&mut self) -> AesctrnoincrementW<AesctrnoincrementSpec> {
        AesctrnoincrementW::new(self, 0)
    }
}
#[doc = "This register enables the AES CTR no increment mode (in which the counter mode is not incremented between 2 blocks)\n\nYou can [`read`](crate::Reg::read) this register and get [`aesctrnoincrement::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesctrnoincrement::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesctrnoincrementSpec;
impl crate::RegisterSpec for AesctrnoincrementSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesctrnoincrement::R`](R) reader structure"]
impl crate::Readable for AesctrnoincrementSpec {}
#[doc = "`write(|w| ..)` method takes [`aesctrnoincrement::W`](W) writer structure"]
impl crate::Writable for AesctrnoincrementSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCTRNOINCREMENT to value 0"]
impl crate::Resettable for AesctrnoincrementSpec {
    const RESET_VALUE: u32 = 0;
}
