#[doc = "Register `AIBFUSEPROGCOMPLETED` reader"]
pub type R = crate::R<AibfuseprogcompletedSpec>;
#[doc = "Register `AIBFUSEPROGCOMPLETED` writer"]
pub type W = crate::W<AibfuseprogcompletedSpec>;
#[doc = "Field `AIBFUSEPROGCOMPLETED` reader - Indicates if the fuse programming operation has been completed."]
pub type AibfuseprogcompletedR = crate::BitReader;
#[doc = "Field `AIBFUSEPROGCOMPLETED` writer - Indicates if the fuse programming operation has been completed."]
pub type AibfuseprogcompletedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates if the fuse programming operation has been completed."]
    #[inline(always)]
    pub fn aibfuseprogcompleted(&self) -> AibfuseprogcompletedR {
        AibfuseprogcompletedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if the fuse programming operation has been completed."]
    #[inline(always)]
    #[must_use]
    pub fn aibfuseprogcompleted(&mut self) -> AibfuseprogcompletedW<AibfuseprogcompletedSpec> {
        AibfuseprogcompletedW::new(self, 0)
    }
}
#[doc = "This register reflects the fuse_aib_prog_completed input, which indicates that the fuse programming was completed.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aibfuseprogcompleted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aibfuseprogcompleted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AibfuseprogcompletedSpec;
impl crate::RegisterSpec for AibfuseprogcompletedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aibfuseprogcompleted::R`](R) reader structure"]
impl crate::Readable for AibfuseprogcompletedSpec {}
#[doc = "`write(|w| ..)` method takes [`aibfuseprogcompleted::W`](W) writer structure"]
impl crate::Writable for AibfuseprogcompletedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIBFUSEPROGCOMPLETED to value 0"]
impl crate::Resettable for AibfuseprogcompletedSpec {
    const RESET_VALUE: u32 = 0;
}
