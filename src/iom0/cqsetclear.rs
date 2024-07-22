#[doc = "Register `CQSETCLEAR` reader"]
pub type R = crate::R<CqsetclearSpec>;
#[doc = "Register `CQSETCLEAR` writer"]
pub type W = crate::W<CqsetclearSpec>;
#[doc = "Field `CQFSET` reader - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
pub type CqfsetR = crate::FieldReader;
#[doc = "Field `CQFSET` writer - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
pub type CqfsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CQFTGL` reader - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
pub type CqftglR = crate::FieldReader;
#[doc = "Field `CQFTGL` writer - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
pub type CqftglW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CQFCLR` reader - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
pub type CqfclrR = crate::FieldReader;
#[doc = "Field `CQFCLR` writer - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
pub type CqfclrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfset(&self) -> CqfsetR {
        CqfsetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqftgl(&self) -> CqftglR {
        CqftglR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    pub fn cqfclr(&self) -> CqfclrR {
        CqfclrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    #[must_use]
    pub fn cqfset(&mut self) -> CqfsetW<CqsetclearSpec> {
        CqfsetW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    #[must_use]
    pub fn cqftgl(&mut self) -> CqftglW<CqsetclearSpec> {
        CqftglW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field"]
    #[inline(always)]
    #[must_use]
    pub fn cqfclr(&mut self) -> CqfclrW<CqsetclearSpec> {
        CqfclrW::new(self, 16)
    }
}
#[doc = "Set/Clear the command queue software pause flags on a per-bit basis. Contains 3 fields, allowing for setting, clearing or toggling the value in the software flags. Priority when the same bit is enabled in each field is toggle, then set, then clear.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqsetclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqsetclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqsetclearSpec;
impl crate::RegisterSpec for CqsetclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqsetclear::R`](R) reader structure"]
impl crate::Readable for CqsetclearSpec {}
#[doc = "`write(|w| ..)` method takes [`cqsetclear::W`](W) writer structure"]
impl crate::Writable for CqsetclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQSETCLEAR to value 0"]
impl crate::Resettable for CqsetclearSpec {
    const RESET_VALUE: u32 = 0;
}
