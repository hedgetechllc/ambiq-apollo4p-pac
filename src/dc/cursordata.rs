#[doc = "Register `CURSORDATA` reader"]
pub type R = crate::R<CursordataSpec>;
#[doc = "Register `CURSORDATA` writer"]
pub type W = crate::W<CursordataSpec>;
#[doc = "Field `CURDATA70` reader - Pixel 'xy' color look up bits"]
pub type Curdata70R = crate::FieldReader;
#[doc = "Field `CURDATA70` writer - Pixel 'xy' color look up bits"]
pub type Curdata70W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CURDATA3112` reader - Pixel 'xy' color look up bits"]
pub type Curdata3112R = crate::FieldReader<u32>;
#[doc = "Field `CURDATA3112` writer - Pixel 'xy' color look up bits"]
pub type Curdata3112W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:7 - Pixel 'xy' color look up bits"]
    #[inline(always)]
    pub fn curdata70(&self) -> Curdata70R {
        Curdata70R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:31 - Pixel 'xy' color look up bits"]
    #[inline(always)]
    pub fn curdata3112(&self) -> Curdata3112R {
        Curdata3112R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pixel 'xy' color look up bits"]
    #[inline(always)]
    #[must_use]
    pub fn curdata70(&mut self) -> Curdata70W<CursordataSpec> {
        Curdata70W::new(self, 0)
    }
    #[doc = "Bits 12:31 - Pixel 'xy' color look up bits"]
    #[inline(always)]
    #[must_use]
    pub fn curdata3112(&mut self) -> Curdata3112W<CursordataSpec> {
        Curdata3112W::new(self, 12)
    }
}
#[doc = "Color values for the pixel cursor that are used with the Cursor LUT where x starts at 0 thru 127.Access to all 16 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_CURSORDATA(n) (*((volatile uint32_t*)(&amp;CURSORDATA + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`cursordata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cursordata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CursordataSpec;
impl crate::RegisterSpec for CursordataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cursordata::R`](R) reader structure"]
impl crate::Readable for CursordataSpec {}
#[doc = "`write(|w| ..)` method takes [`cursordata::W`](W) writer structure"]
impl crate::Writable for CursordataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CURSORDATA to value 0"]
impl crate::Resettable for CursordataSpec {
    const RESET_VALUE: u32 = 0;
}
