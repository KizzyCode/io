use super::{ IoError, Result, InstantExt, WaitForEvent };
use std::{
	time::{ Duration, Instant },
	net::{ TcpListener, TcpStream }
};


/// A trait for accepting elements, e.g. a TCP-listener
pub trait Acceptor<T> {
	/// Accepts a type-`T`-connection
	///
	/// Parameters:
	///  - `timeout`: The time to wait for a connection
	///
	/// Returns either __the accepted connection__ or a corresponding `IoError`
	fn accept(&self, timeout: Duration) -> Result<T>;
}
impl Acceptor<TcpStream> for TcpListener {
	fn accept(&self, timeout: Duration) -> Result<TcpStream> {
		// Compute timeout-point and try to accept once until the timeout occurred
		let timeout_point = Instant::now() + timeout;
		loop {
			// Wait for read-event
			try_err!(self.wait_until_readable(timeout_point.remaining()));
			
			// Accept connection
			match self.accept() {
				Ok(connection) => return Ok(connection.0),
				Err(error) => {
					let error = IoError::from(error);
					if error.non_recoverable { throw_err!(error) }
				}
			}
		}
	}
}