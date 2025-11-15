pub trait ResultOrErr<T> {
	/// Transforms the `Result<T, E>` into a [`Result<T, F>`],
	/// mapping [`Ok(v)`] to [`Ok(v)`] and [`Err(x)`] to [`Err(err)`].
	///
	/// Arguments passed to `or_err` are eagerly evaluated.
	/// For lazy evaluation you might want to use [`map_err`].
	fn or_err<F>(self, err: F) -> Result<T, F>;
}

impl<T, E> ResultOrErr<T> for Result<T, E> {
	fn or_err<F>(self, err: F) -> Result<T, F> {
		self.map_err(|_| err)
	}
}
