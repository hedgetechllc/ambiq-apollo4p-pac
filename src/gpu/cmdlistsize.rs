#[doc = "Register `CMDLISTSIZE` reader"]
pub type R = crate::R<CmdlistsizeSpec>;
#[doc = "Register `CMDLISTSIZE` writer"]
pub type W = crate::W<CmdlistsizeSpec>;
#[doc = "Field `LISTWORDS` reader - Command list length in words."]
pub type ListwordsR = crate::FieldReader<u32>;
#[doc = "Field `LISTWORDS` writer - Command list length in words."]
pub type ListwordsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command list length in words."]
    #[inline(always)]
    pub fn listwords(&self) -> ListwordsR {
        ListwordsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command list length in words."]
    #[inline(always)]
    #[must_use]
    pub fn listwords(&mut self) -> ListwordsW<CmdlistsizeSpec> {
        ListwordsW::new(self, 0)
    }
}
#[doc = "Command list length in words.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdlistsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdlistsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdlistsizeSpec;
impl crate::RegisterSpec for CmdlistsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdlistsize::R`](R) reader structure"]
impl crate::Readable for CmdlistsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdlistsize::W`](W) writer structure"]
impl crate::Writable for CmdlistsizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDLISTSIZE to value 0"]
impl crate::Resettable for CmdlistsizeSpec {
    const RESET_VALUE: u32 = 0;
}
