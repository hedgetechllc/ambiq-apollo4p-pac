#[doc = "Register `CGCMD` reader"]
pub type R = crate::R<CgcmdSpec>;
#[doc = "Register `CGCMD` writer"]
pub type W = crate::W<CgcmdSpec>;
#[doc = "Field `STOP` reader - stop clock"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - stop clock"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - start clock"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - start clock"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - stop clock"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - start clock"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - stop clock"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<CgcmdSpec> {
        StopW::new(self, 0)
    }
    #[doc = "Bit 1 - start clock"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CgcmdSpec> {
        StartW::new(self, 1)
    }
}
#[doc = "Clock gating enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgcmdSpec;
impl crate::RegisterSpec for CgcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgcmd::R`](R) reader structure"]
impl crate::Readable for CgcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cgcmd::W`](W) writer structure"]
impl crate::Writable for CgcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGCMD to value 0"]
impl crate::Resettable for CgcmdSpec {
    const RESET_VALUE: u32 = 0;
}
