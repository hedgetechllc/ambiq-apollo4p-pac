#[doc = "Register `CMDLISTRINGSTOP` reader"]
pub type R = crate::R<CmdlistringstopSpec>;
#[doc = "Register `CMDLISTRINGSTOP` writer"]
pub type W = crate::W<CmdlistringstopSpec>;
#[doc = "Field `UPDATEPRT` reader - Updates GPU command list pointer to stop executing."]
pub type UpdateprtR = crate::FieldReader<u32>;
#[doc = "Field `UPDATEPRT` writer - Updates GPU command list pointer to stop executing."]
pub type UpdateprtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Updates GPU command list pointer to stop executing."]
    #[inline(always)]
    pub fn updateprt(&self) -> UpdateprtR {
        UpdateprtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Updates GPU command list pointer to stop executing."]
    #[inline(always)]
    #[must_use]
    pub fn updateprt(&mut self) -> UpdateprtW<CmdlistringstopSpec> {
        UpdateprtW::new(self, 0)
    }
}
#[doc = "Updates GPU command list pointer to stop executing.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdlistringstop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdlistringstop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdlistringstopSpec;
impl crate::RegisterSpec for CmdlistringstopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdlistringstop::R`](R) reader structure"]
impl crate::Readable for CmdlistringstopSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdlistringstop::W`](W) writer structure"]
impl crate::Writable for CmdlistringstopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDLISTRINGSTOP to value 0"]
impl crate::Resettable for CmdlistringstopSpec {
    const RESET_VALUE: u32 = 0;
}
