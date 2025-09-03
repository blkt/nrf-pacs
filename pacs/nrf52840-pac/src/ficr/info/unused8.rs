#[doc = "Register `UNUSED8[%s]` reader"]
pub type R = crate::R<Unused8Spec>;
#[doc = "Register `UNUSED8[%s]` writer"]
pub type W = crate::W<Unused8Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Unspecified\n\nYou can [`read`](crate::Reg::read) this register and get [`unused8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unused8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unused8Spec;
impl crate::RegisterSpec for Unused8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unused8::R`](R) reader structure"]
impl crate::Readable for Unused8Spec {}
#[doc = "`write(|w| ..)` method takes [`unused8::W`](W) writer structure"]
impl crate::Writable for Unused8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNUSED8[%s] to value 0"]
impl crate::Resettable for Unused8Spec {}
