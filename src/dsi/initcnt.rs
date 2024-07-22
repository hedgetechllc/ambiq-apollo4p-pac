#[doc = "Register `INITCNT` reader"]
pub type R = crate::R<InitcntSpec>;
#[doc = "Register `INITCNT` writer"]
pub type W = crate::W<InitcntSpec>;
#[doc = "Field `MSTR` reader - Counter value in terms of low power clock to initialise the DSI Host IP (TINIT) that drives a stop state on the mipis D-PHY bus; DPHY Initialization period min 100 x B5s; Time out value is calculated by txclkesc and the count value is 7d0h(2000 in decimal)"]
pub type MstrR = crate::FieldReader<u16>;
#[doc = "Field `MSTR` writer - Counter value in terms of low power clock to initialise the DSI Host IP (TINIT) that drives a stop state on the mipis D-PHY bus; DPHY Initialization period min 100 x B5s; Time out value is calculated by txclkesc and the count value is 7d0h(2000 in decimal)"]
pub type MstrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value in terms of low power clock to initialise the DSI Host IP (TINIT) that drives a stop state on the mipis D-PHY bus; DPHY Initialization period min 100 x B5s; Time out value is calculated by txclkesc and the count value is 7d0h(2000 in decimal)"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value in terms of low power clock to initialise the DSI Host IP (TINIT) that drives a stop state on the mipis D-PHY bus; DPHY Initialization period min 100 x B5s; Time out value is calculated by txclkesc and the count value is 7d0h(2000 in decimal)"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MstrW<InitcntSpec> {
        MstrW::new(self, 0)
    }
}
#[doc = "Count register to initialize the DSI HOST IP\n\nYou can [`read`](crate::Reg::read) this register and get [`initcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitcntSpec;
impl crate::RegisterSpec for InitcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`initcnt::R`](R) reader structure"]
impl crate::Readable for InitcntSpec {}
#[doc = "`write(|w| ..)` method takes [`initcnt::W`](W) writer structure"]
impl crate::Writable for InitcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INITCNT to value 0x07d0"]
impl crate::Resettable for InitcntSpec {
    const RESET_VALUE: u32 = 0x07d0;
}
