#[doc = "Register `IPCOREID` reader"]
pub type R = crate::R<IpcoreidSpec>;
#[doc = "Register `IPCOREID` writer"]
pub type W = crate::W<IpcoreidSpec>;
#[doc = "Field `COREID` reader - Core ID of the IPB core"]
pub type CoreidR = crate::FieldReader;
#[doc = "Field `COREID` writer - Core ID of the IPB core"]
pub type CoreidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COREFAM` reader - Core Family. Also bit 31 is used to set the I2S validity bit when a write is done."]
pub type CorefamR = crate::FieldReader;
#[doc = "Field `COREFAM` writer - Core Family. Also bit 31 is used to set the I2S validity bit when a write is done."]
pub type CorefamW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - Core ID of the IPB core"]
    #[inline(always)]
    pub fn coreid(&self) -> CoreidR {
        CoreidR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Core Family. Also bit 31 is used to set the I2S validity bit when a write is done."]
    #[inline(always)]
    pub fn corefam(&self) -> CorefamR {
        CorefamR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Core ID of the IPB core"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> CoreidW<IpcoreidSpec> {
        CoreidW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Core Family. Also bit 31 is used to set the I2S validity bit when a write is done."]
    #[inline(always)]
    #[must_use]
    pub fn corefam(&mut self) -> CorefamW<IpcoreidSpec> {
        CorefamW::new(self, 24)
    }
}
#[doc = "Returns the core ID of the IPB core, and used to write the I2S validity mask.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcoreid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcoreid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcoreidSpec;
impl crate::RegisterSpec for IpcoreidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcoreid::R`](R) reader structure"]
impl crate::Readable for IpcoreidSpec {}
#[doc = "`write(|w| ..)` method takes [`ipcoreid::W`](W) writer structure"]
impl crate::Writable for IpcoreidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCOREID to value 0xda1a_0000"]
impl crate::Resettable for IpcoreidSpec {
    const RESET_VALUE: u32 = 0xda1a_0000;
}
