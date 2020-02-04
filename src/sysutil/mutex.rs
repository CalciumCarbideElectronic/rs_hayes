use crate::sysutil::import::osMutexAcquire;
use crate::sysutil::import::osMutexAttr_t;
use crate::sysutil::import::osMutexDelete;
use crate::sysutil::import::osMutexId_t;
use crate::sysutil::import::osMutexNew;
use crate::sysutil::import::osMutexRelease;
use alloc::fmt::Debug;
use alloc::fmt::Formatter;
use core::cell::UnsafeCell;
use core::ffi::c_void;
use core::fmt;
use core::marker::Send;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr::null;
pub struct LockError {}

pub struct MutexGuard<'a, T: ?Sized + 'a>
where
    T: Sized,
{
    lock: &'a Mutex<T>,
}
pub struct TryLockResult<T> {
    inner: T,
}
pub type LockResult<T> = Result<T, LockError>;

pub struct Mutex<T> {
    raw_data: UnsafeCell<T>,
    cmsis_mutex_id: osMutexId_t,
}

unsafe impl<T: ?Sized> Send for MutexGuard<'_, T> where T: Sized {}
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> where T: Sized {}

impl<'a, T> MutexGuard<'a, T> {
    pub fn new(t: &'a Mutex<T>) -> LockResult<MutexGuard<'a, T>> {
        Ok(MutexGuard { lock: t })
    }
}

impl<T: ?Sized> Mutex<T>
where
    T: Sized,
{
    pub fn new(t: T) -> Mutex<T> {
        let mutex = unsafe {
            osMutexNew(&osMutexAttr_t {
                name: null(),
                attr_bits: 1,
                cb_mem: 0 as *const c_void,
                cb_size: 0,
            })
        };

        Mutex {
            raw_data: UnsafeCell::new(t),
            cmsis_mutex_id: mutex,
        }
    }

    pub fn get_mut(&mut self) -> LockResult<&mut T> {
        let data = unsafe { &mut *self.raw_data.get() };
        return Ok(data);
    }
    // pub fn into_inner(self) -> LockResult<T> {}
    pub fn is_poisoned(&self) -> bool {
        false
    }
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        unsafe {
            osMutexAcquire(self.cmsis_mutex_id, usize::max_value());
        };
        MutexGuard::new(self)
    }
    pub fn unlock(&self) {
        unsafe { osMutexRelease(self.cmsis_mutex_id) };
    }
    // pub fn try_lock(&self) -> TryLockResult<MutexGuard<T>> {}
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> where T: Sized {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> where T: Sized {}

impl<T> Debug for Mutex<T>
where
    T: Debug,
    T: Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Mutexed:{:?}", self.raw_data)
    }
}

impl<T> From<T> for Mutex<T>
where
    T: Sized,
{
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T> Drop for Mutex<T>
where
    T: Sized,
{
    fn drop(&mut self) {
        unsafe { osMutexDelete(self.cmsis_mutex_id) };
    }
}

impl<T: ?Sized> Deref for MutexGuard<'_, T>
where
    T: Sized,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.raw_data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T>
where
    T: Sized,
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.raw_data.get() }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T>
where
    T: Sized,
{
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.lock.unlock();
        }
    }
}
