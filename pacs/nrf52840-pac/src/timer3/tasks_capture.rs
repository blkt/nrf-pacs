#[doc = "Register `TASKS_CAPTURE[%s]` writer"]
pub type W = crate::W<TasksCaptureSpec>;
#[doc = "Field `TASKS_CAPTURE` writer - "]
pub type TasksCaptureW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tasks_capture(&mut self) -> TasksCaptureW<'_, TasksCaptureSpec> {
        TasksCaptureW::new(self, 0)
    }
}
#[doc = "Description collection\\[n\\]: Capture Timer value to CC\\[n\\] register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_capture::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCaptureSpec;
impl crate::RegisterSpec for TasksCaptureSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_capture::W`](W) writer structure"]
impl crate::Writable for TasksCaptureSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASKS_CAPTURE[%s] to value 0"]
impl crate::Resettable for TasksCaptureSpec {}
