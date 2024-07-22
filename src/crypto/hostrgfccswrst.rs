#[doc = "Register `HOSTRGFCCSWRST` reader"]
pub type R = crate::R<HostrgfccswrstSpec>;
#[doc = "Register `HOSTRGFCCSWRST` writer"]
pub type W = crate::W<HostrgfccswrstSpec>;
#[doc = "Field `HOSTRGFCCSWRST` reader - Writing 1 to this field generates a general reset to CryptoCell."]
pub type HostrgfccswrstR = crate::BitReader;
#[doc = "Field `HOSTRGFCCSWRST` writer - Writing 1 to this field generates a general reset to CryptoCell."]
pub type HostrgfccswrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing 1 to this field generates a general reset to CryptoCell."]
    #[inline(always)]
    pub fn hostrgfccswrst(&self) -> HostrgfccswrstR {
        HostrgfccswrstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing 1 to this field generates a general reset to CryptoCell."]
    #[inline(always)]
    #[must_use]
    pub fn hostrgfccswrst(&mut self) -> HostrgfccswrstW<HostrgfccswrstSpec> {
        HostrgfccswrstW::new(self, 0)
    }
}
#[doc = "Writing to this register generates a general reset to CryptoCell. This reset takes about 4 core clock cycles.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostrgfccswrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostrgfccswrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostrgfccswrstSpec;
impl crate::RegisterSpec for HostrgfccswrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostrgfccswrst::R`](R) reader structure"]
impl crate::Readable for HostrgfccswrstSpec {}
#[doc = "`write(|w| ..)` method takes [`hostrgfccswrst::W`](W) writer structure"]
impl crate::Writable for HostrgfccswrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTRGFCCSWRST to value 0"]
impl crate::Resettable for HostrgfccswrstSpec {
    const RESET_VALUE: u32 = 0;
}
