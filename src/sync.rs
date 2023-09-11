use std::{sync::atomic::{AtomicBool, self}, time::Instant, hint, rc::Rc};

use bevy::prelude::error;

const ATOMIC_LOCK_SPIN_MS: u64 = 1_000;

/// Heaps `T` and allows `self` to be coped and sent
/// among threads saftly due to `AtomicLock` guarentees.
/// 
/// Make sure all referances to `self` will not be used
/// after `AtomicLockPtr::dealloc` is called.
pub struct AtomicLockPtr<T>(*mut AtomicLock<T>);
unsafe impl<T> Send for AtomicLockPtr<T> {}
unsafe impl<T> Sync for AtomicLockPtr<T> {}

impl<T> AtomicLockPtr<T> {
	pub fn new(
		t: T,
	) -> Self {
		Self(Box::into_raw(Box::new(
			AtomicLock::new(t),
		)))
	}

	/// Acquires the lock in `AtomicLock`.
	pub fn acquire(
		&mut self,
	) -> AtomicGuard<'_, T> { unsafe {
		self.0.as_mut().unwrap_unchecked().acquire()
	}}

	// TODO: ARC? BOXRC? impl drop for this struct should be called on
	// 		 all copies, which we dont want.

	/// Invalidates this struct as well as all copies.
	pub fn dealloc(
		&mut self,
	) { unsafe {
		drop(Box::from_raw(self.0));
	}}
}

/// 
impl<T> Copy for AtomicLockPtr<T> {}
impl<T> Clone for AtomicLockPtr<T> {
	fn clone(&self) -> Self {
		*self
	}
}

/// Guarantees any thread can read/write from/to `T` without
/// memory racing or reorder racing.
/// 
/// # How Does This Work?
/// 
/// ### Non-Atomic Operations
/// - compiler can reorder instructions throughout the whole
///   program to reduce load or even completley remove some
///   instructions.
/// - each thread could access different memory when accessing
///   data. That data exists in ram and likley multiple locations
///   in cache.
/// - cpu can race data (reading and writing at the same time).
/// 
/// ### Acquire/Release
/// - **before acquire:** any instructions can be reordered.
/// - **between:** all instructions between the last
///   `acquire`/`release` will be present once acquired again.
/// - **after release:** all instructions done before release
///   dont have to be present until the next `acquire`.
pub struct AtomicLock<T> {
	lock: AtomicBool,
	t: T,
}

impl<T> AtomicLock<T> {
	pub fn new(
		t: T,
	) -> Self {
		Self {
			lock: AtomicBool::new(false),
			t,
		}
	}

	/// Acquires an `AtomicGuard` for this lock. Once
	/// dropped, the lock will subside.
	pub fn acquire(
		&mut self,
	) -> AtomicGuard<'_, T> {
		let _spin_loop_data = AtomicLock::<T>::spin_loop_data();
		while self.compare() {
			AtomicLock::<T>::spin_loop(&_spin_loop_data)
		}
		AtomicGuard::<'_, T>::new(&self.lock, &mut self.t)
	}

	/// Compares the lock value. If it is `false`,
	/// then set to `true` and return `false` to
	/// escape the parent loop.
	fn compare(
		&self,
	) -> bool {
		match self.compare_operation() {
			// CONTINUE
			Err(_) => true,
			// ESCAPE
			Ok(_) => false,
		}
	}

	#[cfg(not(feature = "weak-operations"))]
	/// Uses the standard operation for acquisition.
	fn compare_operation(
		&self,
	) -> Result<bool, bool> {
		self.lock.compare_exchange(
			false, true,
			atomic::Ordering::Acquire,
			atomic::Ordering::Relaxed,
		)
	}

	#[cfg(feature = "weak-operations")]
	/// Uses a weaker operation for acquisition that
	/// may skip successful cycles, but is faster on
	/// some platforms.
	fn compare_operation(
		&self,
	) -> Result<bool, bool> {
		self.lock.compare_exchange_weak(
			false, true,
			atomic::Ordering::Acquire,
			atomic::Ordering::Relaxed,
		)
	}

	#[cfg(not(debug_assertions))]
	fn spin_loop_data() -> () { () }
	#[cfg(debug_assertions)]
	fn spin_loop_data() -> Instant { Instant::now() }

	#[cfg(not(debug_assertions))]
	/// Standard `spin_loop`.
	fn spin_loop(
		_: (),
	) {
		hint::spin_loop();
	}

	#[cfg(debug_assertions)]
	/// Errors if the `spin_loop` took to much time.
	fn spin_loop(
		instant: &Instant,
	) {
		hint::spin_loop();
		if instant.elapsed().as_millis() as u64 > ATOMIC_LOCK_SPIN_MS {
			error!(
				"DEADLOCK DETECTED - atomic lock spin_loop took over {}ms",
				ATOMIC_LOCK_SPIN_MS,
			);
		}
	}
}

/// Writable referance to T from the `AtomicLock`. Once dropped,
/// the access is over and other threads may `AtomicLock::acquire`.
pub struct AtomicGuard<'guard, T> {
	lock: &'guard AtomicBool,
	t: &'guard mut T,
}

impl<'guard, T> AtomicGuard<'guard, T> {
	fn new(
		lock: &'guard AtomicBool,
		t: &'guard mut T,
	) -> Self {
		Self {
			lock,
			t,
		}
	}

	/// Sets the whole T.
	pub fn set(
		&'guard mut self,
		t: T,
	) {
		*self.t = t;
	}

	/// Retrives a mutable referance to T that is safe to
	/// read and write.
	/// TODO: 'get + 'guard ???
	pub fn get<'get>(
		&'get mut self,
	) -> &'get mut T {
		self.t
	}
}

impl<'guard, T> Drop for AtomicGuard<'guard, T> {
	fn drop(&mut self) {
		self.lock.store(
			false,
			atomic::Ordering::Release,
		);
	}
}