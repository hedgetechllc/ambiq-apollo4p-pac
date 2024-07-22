#[doc = "Register `DCGPIO` reader"]
pub type R = crate::R<DcgpioSpec>;
#[doc = "Register `DCGPIO` writer"]
pub type W = crate::W<DcgpioSpec>;
#[doc = "Field `RWPINS` reader - These are not implemented"]
pub type RwpinsR = crate::FieldReader;
#[doc = "Field `RWPINS` writer - These are not implemented"]
pub type RwpinsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD0` reader - This field is reserved."]
pub type Rsvd0R = crate::FieldReader;
#[doc = "Field `RSVD0` writer - This field is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADVANCEANYWAY` reader - No idea what this is"]
pub type AdvanceanywayR = crate::FieldReader;
#[doc = "Field `ADVANCEANYWAY` writer - No idea what this is"]
pub type AdvanceanywayW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD1` reader - This field is reserved."]
pub type Rsvd1R = crate::FieldReader<u16>;
#[doc = "Field `RSVD1` writer - This field is reserved."]
pub type Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CGBYPASS` reader - No idea what this is"]
pub type CgbypassR = crate::FieldReader<u16>;
#[doc = "Field `CGBYPASS` writer - No idea what this is"]
pub type CgbypassW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - These are not implemented"]
    #[inline(always)]
    pub fn rwpins(&self) -> RwpinsR {
        RwpinsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:8 - No idea what this is"]
    #[inline(always)]
    pub fn advanceanyway(&self) -> AdvanceanywayR {
        AdvanceanywayR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:21 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 9) & 0x1fff) as u16)
    }
    #[doc = "Bits 22:31 - No idea what this is"]
    #[inline(always)]
    pub fn cgbypass(&self) -> CgbypassR {
        CgbypassR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - These are not implemented"]
    #[inline(always)]
    #[must_use]
    pub fn rwpins(&mut self) -> RwpinsW<DcgpioSpec> {
        RwpinsW::new(self, 0)
    }
    #[doc = "Bits 2:6 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<DcgpioSpec> {
        Rsvd0W::new(self, 2)
    }
    #[doc = "Bits 7:8 - No idea what this is"]
    #[inline(always)]
    #[must_use]
    pub fn advanceanyway(&mut self) -> AdvanceanywayW<DcgpioSpec> {
        AdvanceanywayW::new(self, 7)
    }
    #[doc = "Bits 9:21 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<DcgpioSpec> {
        Rsvd1W::new(self, 9)
    }
    #[doc = "Bits 22:31 - No idea what this is"]
    #[inline(always)]
    #[must_use]
    pub fn cgbypass(&mut self) -> CgbypassW<DcgpioSpec> {
        CgbypassW::new(self, 22)
    }
}
#[doc = "General Purpose register: read/write GPIO external pins. This is accumulated as- {CGBYPASS_in,13'd0,ADVANCE_ANYWAY_in,5'd0,GPIO_in}\n\nYou can [`read`](crate::Reg::read) this register and get [`dcgpio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcgpio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcgpioSpec;
impl crate::RegisterSpec for DcgpioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgpio::R`](R) reader structure"]
impl crate::Readable for DcgpioSpec {}
#[doc = "`write(|w| ..)` method takes [`dcgpio::W`](W) writer structure"]
impl crate::Writable for DcgpioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCGPIO to value 0x0100"]
impl crate::Resettable for DcgpioSpec {
    const RESET_VALUE: u32 = 0x0100;
}
