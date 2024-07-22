#[doc = "Register `CHACHABLOCKCNTLSB` reader"]
pub type R = crate::R<ChachablockcntlsbSpec>;
#[doc = "Register `CHACHABLOCKCNTLSB` writer"]
pub type W = crate::W<ChachablockcntlsbSpec>;
#[doc = "Field `CHACHABLOCKCNTLSB` reader - bits 31:0 of CHACHA_BLOCK_CNT_LSB register."]
pub type ChachablockcntlsbR = crate::FieldReader<u32>;
#[doc = "Field `CHACHABLOCKCNTLSB` writer - bits 31:0 of CHACHA_BLOCK_CNT_LSB register."]
pub type ChachablockcntlsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_BLOCK_CNT_LSB register."]
    #[inline(always)]
    pub fn chachablockcntlsb(&self) -> ChachablockcntlsbR {
        ChachablockcntlsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_BLOCK_CNT_LSB register."]
    #[inline(always)]
    #[must_use]
    pub fn chachablockcntlsb(&mut self) -> ChachablockcntlsbW<ChachablockcntlsbSpec> {
        ChachablockcntlsbW::new(self, 0)
    }
}
#[doc = "The two first words (n) in the last row of the cipher matrix are the block counter. At the end of each block (512b), the block_cnt for the next block is written by HW to the block_cnt_lsb and block_cnt_msb registers. Need reset block counter , if start new message.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachablockcntlsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachablockcntlsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachablockcntlsbSpec;
impl crate::RegisterSpec for ChachablockcntlsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachablockcntlsb::R`](R) reader structure"]
impl crate::Readable for ChachablockcntlsbSpec {}
#[doc = "`write(|w| ..)` method takes [`chachablockcntlsb::W`](W) writer structure"]
impl crate::Writable for ChachablockcntlsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHABLOCKCNTLSB to value 0"]
impl crate::Resettable for ChachablockcntlsbSpec {
    const RESET_VALUE: u32 = 0;
}
