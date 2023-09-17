use std::{sync::atomic::{AtomicBool, self}, time::{Instant, Duration}, hint, rc::Rc, thread};

use bevy::prelude::error;

const ATOMIC_LOCK_SPIN_MS: u64 = 5_000;

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

	/// Invalidates this struct as well as all copies/clones.
	/// This is NOT "safe" unless you gurantee all copies/clones wont be used.
	pub fn dealloc(
		&mut self,
	) { unsafe {
		drop(Box::from_raw(self.0));
	}}
}

/// Allows Copy/Clone without dropping T.
impl<T> Copy for AtomicLockPtr<T> {}
impl<T> Clone for AtomicLockPtr<T> {
	fn clone(&self) -> Self {
		*self
	}
}

/// Heaps `AtomicSignal` and allows `self` to be coped and sent
/// among threads saftly due to atomic guarentees.
/// 
/// Make sure all referances to `self` will not be used
/// after `AtomicSignalPtr::dealloc` is called.
pub struct AtomicSignalPtr(*mut AtomicSignal);
unsafe impl Send for AtomicSignalPtr {}
unsafe impl Sync for AtomicSignalPtr {}

impl AtomicSignalPtr {
	pub fn new(
	) -> Self {
		Self(Box::into_raw(Box::new(
			AtomicSignal::new(),
		)))
	}

	/// Gets the common `AtomicSignal`.
	pub fn get<'get>(
		&'get mut self,
	) -> &'get mut AtomicSignal { unsafe {
		self.0.as_mut().unwrap_unchecked()
	}}

	// TODO: ARC? BOXRC? impl drop for this struct should be called on
	// 		 all copies, which we dont want.

	/// Invalidates this struct as well as all copies/clones.
	/// This is NOT "safe" unless you gurantee all copies/clones wont be used.
	pub fn dealloc(
		&mut self,
	) { unsafe {
		drop(Box::from_raw(self.0));
	}}
}

/// Allows Copy/Clone without dropping `AtomicSignal`.
impl Copy for AtomicSignalPtr {}
impl Clone for AtomicSignalPtr {
	fn clone(&self) -> Self {
		*self
	}
}

/// Determines sleep method for `AtomicSignal`.
pub enum AtomicSignalSleep {
	/// Spin loop. Use when your sure it will take less then a few microseconds.
	Spin,
	/// Sleeps for 1ms. Use when your not sure how long, but you are ok with some
	/// thread load.
	Quick,
	/// Sleeps for 10ms. Barely any thread load, use everywhere else.
	Rest,
}

/// Used for duo thread back and forth. Not applicable for several
/// threads due to dst thread signaling back to src thread structure.
/// 
/// Ordering must be Acquire/Release because sending must occur
/// before querying if the signal when reducted.
pub struct AtomicSignal {
	lock: AtomicBool,
}

impl AtomicSignal {
	pub fn new(
	) -> Self {
		Self {
			lock: AtomicBool::new(false),
		}
	}

	/// Sends a signal.
	pub fn sender_signal(
		&mut self,
	) {
		self.store(true);
	}

	/// Queries if the signal was reducted, NOT what the value of the
	/// atomic is.
	pub fn sender_query(
		&mut self,
	) -> bool {
		self.query(false)
	}

	/// Enters a spin loop until a signal is reducted. Does NOT send
	/// the signal.
	pub fn sender_wait(
		&mut self,
		sleep: AtomicSignalSleep,
	) {
		match sleep {
			AtomicSignalSleep::Spin => {
				self.spin(false);
			},
			AtomicSignalSleep::Quick => {
				self.sleep(false, Duration::from_millis(1));
			},
			AtomicSignalSleep::Rest => {
				self.sleep(false, Duration::from_millis(10));
			},
		}
	}

	/// Reducts the signal that was sent.
	pub fn reciver_reduct(
		&mut self,
	) {
		self.store(false);
	}

	/// Queries if the signal was sent, NOT what the value of the
	/// atomic is.
	pub fn reciver_query(
		&mut self,
	) -> bool {
		self.query(true)
	}

	/// Enters a spin loop until a signal is sent. Does NOT reduct
	/// the signal.
	pub fn receiver_spin(
		&mut self,
	) {
		self.spin(true);
	}

	/// Enters a temperate sleep until a signal is sent. Does NOT reduct
	/// the signal.
	pub fn receiver_wait(
		&mut self,
		sleep: AtomicSignalSleep,
	) {
		match sleep {
			AtomicSignalSleep::Spin => {
				self.spin(true);
			},
			AtomicSignalSleep::Quick => {
				self.sleep(true, Duration::from_millis(1));
			},
			AtomicSignalSleep::Rest => {
				self.sleep(true, Duration::from_millis(10));
			},
		}
	}

	fn store(
		&self,
		store: bool,
	) {
		self.lock.store(store, atomic::Ordering::Release);
	}
	
	fn query(
		&self,
		expecting: bool,
	) -> bool {
		self.lock.load(atomic::Ordering::Acquire) == expecting
	}

	fn spin(
		&self,
		expecting: bool,
	) {
		let _spin_loop_data = spin_loop_data();
		while self.lock.load(atomic::Ordering::Acquire) != expecting {
			spin_loop(&_spin_loop_data)
		}
	}

	fn sleep(
		&self,
		expecting: bool,
		duration: Duration,
	) {
		while self.lock.load(atomic::Ordering::Acquire) != expecting {
			thread::sleep(duration);
		}
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
		let _spin_loop_data = spin_loop_data();
		while self.compare() {
			spin_loop(&_spin_loop_data)
		}
		AtomicGuard::<'_, T>::new(&self.lock, &mut self.t)
	}

	/// Compares the lock value. If it is `false`,
	/// then set to `true` and return `false` to
	/// escape the parent loop.
	fn compare(
		&self,
	) -> bool {
		match compare_operation(&self.lock) {
			// CONTINUE
			Err(_) => true,
			// ESCAPE
			Ok(_) => false,
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

/// Uses the standard operation for acquisition.
#[cfg(not(feature = "weak-operations"))]
fn compare_operation(
	lock: &AtomicBool,
) -> Result<bool, bool> {
	lock.compare_exchange(
		false, true,
		atomic::Ordering::Acquire,
		atomic::Ordering::Relaxed,
	)
}

/// Uses a weaker operation for acquisition that
/// may skip successful cycles, but is faster on
/// some platforms.
#[cfg(feature = "weak-operations")]
fn compare_operation(
	lock: &AtomicBool,
) -> Result<bool, bool> {
	lock.compare_exchange_weak(
		false, true,
		atomic::Ordering::Acquire,
		atomic::Ordering::Relaxed,
	)
}

#[cfg(not(debug_assertions))]
fn spin_loop_data() -> () { () }
#[cfg(debug_assertions)]
fn spin_loop_data() -> Instant { Instant::now() }

/// Standard `spin_loop`.
#[cfg(not(debug_assertions))]
fn spin_loop(
	_: &(),
) {
	hint::spin_loop();
}

/// Errors if the `spin_loop` took to much time.
#[cfg(debug_assertions)]
fn spin_loop(
	instant: &Instant,
) {
	hint::spin_loop();
	if instant.elapsed().as_millis() as u64 > ATOMIC_LOCK_SPIN_MS {
		let message = "DEADLOCK DETECTED - atomic lock spin_loop took over";
		error!(
			"{} {}ms!",
			message,
			ATOMIC_LOCK_SPIN_MS,
		);
		panic!("DEADLOCK DETECTED - atomic spin_loop took to much time!");
	}
}