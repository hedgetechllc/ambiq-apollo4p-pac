#[doc = "Register `HASHENDIANESS` reader"]
pub type R = crate::R<HashendianessSpec>;
#[doc = "Register `HASHENDIANESS` writer"]
pub type W = crate::W<HashendianessSpec>;
#[doc = "Field `ENDIAN` reader - The default value is little-endian. The data and generation of padding can be swapped to be big-endian."]
pub type EndianR = crate::BitReader;
#[doc = "Field `ENDIAN` writer - The default value is little-endian. The data and generation of padding can be swapped to be big-endian."]
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The default value is little-endian. The data and generation of padding can be swapped to be big-endian."]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The default value is little-endian. The data and generation of padding can be swapped to be big-endian."]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> EndianW<HashendianessSpec> {
        EndianW::new(self, 0)
    }
}
#[doc = "This register holds the HASH_ENDIANESS configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashendianess::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashendianess::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashendianessSpec;
impl crate::RegisterSpec for HashendianessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashendianess::R`](R) reader structure"]
impl crate::Readable for HashendianessSpec {}
#[doc = "`write(|w| ..)` method takes [`hashendianess::W`](W) writer structure"]
impl crate::Writable for HashendianessSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHENDIANESS to value 0x01"]
impl crate::Resettable for HashendianessSpec {
    const RESET_VALUE: u32 = 0x01;
}
