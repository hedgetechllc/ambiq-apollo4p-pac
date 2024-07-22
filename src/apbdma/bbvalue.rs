#[doc = "Register `BBVALUE` reader"]
pub type R = crate::R<BbvalueSpec>;
#[doc = "Register `BBVALUE` writer"]
pub type W = crate::W<BbvalueSpec>;
#[doc = "Field `DATAOUT` reader - Data Output Values"]
pub type DataoutR = crate::FieldReader;
#[doc = "Field `DATAOUT` writer - Data Output Values"]
pub type DataoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIN` reader - PIO values"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - PIO values"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Output Values"]
    #[inline(always)]
    pub fn dataout(&self) -> DataoutR {
        DataoutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PIO values"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Output Values"]
    #[inline(always)]
    #[must_use]
    pub fn dataout(&mut self) -> DataoutW<BbvalueSpec> {
        DataoutW::new(self, 0)
    }
    #[doc = "Bits 16:23 - PIO values"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<BbvalueSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bbvalue::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbvalue::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BbvalueSpec;
impl crate::RegisterSpec for BbvalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bbvalue::R`](R) reader structure"]
impl crate::Readable for BbvalueSpec {}
#[doc = "`write(|w| ..)` method takes [`bbvalue::W`](W) writer structure"]
impl crate::Writable for BbvalueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BBVALUE to value 0"]
impl crate::Resettable for BbvalueSpec {
    const RESET_VALUE: u32 = 0;
}
