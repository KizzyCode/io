use crate::{ TimeoutIoError, InstantExt, WaitForEvent, EventMask };
use std::{
	io,
	time::{ Duration, Instant },
	net::{ TcpListener, TcpStream }
};


/// A private trait wrapping the standard library's acceptors
#[doc(hidden)]
pub trait StdAcceptor<T> where Self: WaitForEvent {
	fn accept(&self) -> Result<T, io::Error>;
}
impl StdAcceptor<TcpStream> for TcpListener {
	fn accept(&self) -> Result<TcpStream, io::Error> {
		Ok(TcpListener::accept(self)?.0)
	}
}
#[cfg(unix)]
impl StdAcceptor<::std::os::unix::net::UnixStream> for ::std::os::unix::net::UnixListener {
	fn accept(&self) -> Result<::std::os::unix::net::UnixStream, io::Error> {
		Ok(::std::os::unix::net::UnixListener::accept(self)?.0)
	}
}


/// A trait for accepting elements, e.g. a TCP-listener
pub trait Acceptor<T> {
	/// Tries to accept a type-`T`-connection until `timeout` expires
	///
	/// __Warning: This function makes `self` non-blocking. It's up to you to restore the previous
	/// state if necessary.__
	fn try_accept(&self, timeout: Duration) -> Result<T, TimeoutIoError>;
}
impl<U, T: StdAcceptor<U> + WaitForEvent> Acceptor<U> for T {
	fn try_accept(&self, timeout: Duration) -> Result<U, TimeoutIoError> {
		// Make the socket non-blocking
		self.set_blocking_mode(false)?;
		
		// Compute deadline and try to accept once until the timeout occurred
		let deadline = Instant::now() + timeout;
		loop {
			// Wait for read-event
			self.wait_for_event(EventMask::new_r(), deadline.remaining())?;
			
			// Accept connection
			match StdAcceptor::accept(self) {
				Ok(connection) => return Ok(connection),
				Err(error) => {
					let error = TimeoutIoError::from(error);
					if !error.should_retry() { return Err(error) }
				}
			}
		}
	}
}