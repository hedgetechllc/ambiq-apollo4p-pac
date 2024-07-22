#[doc = "Register `CMDLISTSTATUS` reader"]
pub type R = crate::R<CmdliststatusSpec>;
#[doc = "Register `CMDLISTSTATUS` writer"]
pub type W = crate::W<CmdliststatusSpec>;
#[doc = "Field `LIST` reader - processor status"]
pub type ListR = crate::BitReader;
#[doc = "Field `LIST` writer - processor status"]
pub type ListW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - processor status"]
    #[inline(always)]
    pub fn list(&self) -> ListR {
        ListR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - processor status"]
    #[inline(always)]
    #[must_use]
    pub fn list(&mut self) -> ListW<CmdliststatusSpec> {
        ListW::new(self, 0)
    }
}
#[doc = "On read, returns command list processor status; On write, resets command list processor.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdliststatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdliststatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdliststatusSpec;
impl crate::RegisterSpec for CmdliststatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdliststatus::R`](R) reader structure"]
impl crate::Readable for CmdliststatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdliststatus::W`](W) writer structure"]
impl crate::Writable for CmdliststatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDLISTSTATUS to value 0"]
impl crate::Resettable for CmdliststatusSpec {
    const RESET_VALUE: u32 = 0;
}
