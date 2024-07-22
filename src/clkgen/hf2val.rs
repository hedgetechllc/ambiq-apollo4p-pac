#[doc = "Register `HF2VAL` reader"]
pub type R = crate::R<Hf2valSpec>;
#[doc = "Register `HF2VAL` writer"]
pub type W = crate::W<Hf2valSpec>;
#[doc = "Field `HF2ADJTRIMOUT` reader - HF2ADJ trimming output"]
pub type Hf2adjtrimoutR = crate::FieldReader<u16>;
#[doc = "Field `HF2ADJTRIMOUT` writer - HF2ADJ trimming output"]
pub type Hf2adjtrimoutW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - HF2ADJ trimming output"]
    #[inline(always)]
    pub fn hf2adjtrimout(&self) -> Hf2adjtrimoutR {
        Hf2adjtrimoutR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - HF2ADJ trimming output"]
    #[inline(always)]
    #[must_use]
    pub fn hf2adjtrimout(&mut self) -> Hf2adjtrimoutW<Hf2valSpec> {
        Hf2adjtrimoutW::new(self, 0)
    }
}
#[doc = "This register provides the read back of the HF2TUNE\n\nYou can [`read`](crate::Reg::read) this register and get [`hf2val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hf2val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hf2valSpec;
impl crate::RegisterSpec for Hf2valSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hf2val::R`](R) reader structure"]
impl crate::Readable for Hf2valSpec {}
#[doc = "`write(|w| ..)` method takes [`hf2val::W`](W) writer structure"]
impl crate::Writable for Hf2valSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HF2VAL to value 0"]
impl crate::Resettable for Hf2valSpec {
    const RESET_VALUE: u32 = 0;
}
