#[doc = "Register `CHACHASWRESET` reader"]
pub type R = crate::R<ChachaswresetSpec>;
#[doc = "Register `CHACHASWRESET` writer"]
pub type W = crate::W<ChachaswresetSpec>;
#[doc = "Field `CHACHSWRESET` reader - Writing to this address resets the only FSM of CHACHA engine. The reset takes 4 CORE_CLK cycles."]
pub type ChachswresetR = crate::BitReader;
#[doc = "Field `CHACHSWRESET` writer - Writing to this address resets the only FSM of CHACHA engine. The reset takes 4 CORE_CLK cycles."]
pub type ChachswresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing to this address resets the only FSM of CHACHA engine. The reset takes 4 CORE_CLK cycles."]
    #[inline(always)]
    pub fn chachswreset(&self) -> ChachswresetR {
        ChachswresetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing to this address resets the only FSM of CHACHA engine. The reset takes 4 CORE_CLK cycles."]
    #[inline(always)]
    #[must_use]
    pub fn chachswreset(&mut self) -> ChachswresetW<ChachaswresetSpec> {
        ChachswresetW::new(self, 0)
    }
}
#[doc = "Resets CHACHA_SALSA engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachaswreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachaswreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaswresetSpec;
impl crate::RegisterSpec for ChachaswresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachaswreset::R`](R) reader structure"]
impl crate::Readable for ChachaswresetSpec {}
#[doc = "`write(|w| ..)` method takes [`chachaswreset::W`](W) writer structure"]
impl crate::Writable for ChachaswresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHASWRESET to value 0"]
impl crate::Resettable for ChachaswresetSpec {
    const RESET_VALUE: u32 = 0;
}
