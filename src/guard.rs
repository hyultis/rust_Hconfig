use std::ops::{Deref, DerefMut};
use dashmap::mapref::one::RefMut;
use crate::HConfig::HConfig;
use crate::HConfigManager::HConfigManager;

// RAII guard
pub struct Guard<'a>
{
	pub context: &'a HConfigManager,
	pub guarded: RefMut<'a, String,HConfig>,
}

impl<'a> Deref for Guard<'a>
{
	type Target = HConfig;
	
	fn deref(&self) -> &Self::Target {
		self.guarded.deref()
	}
}

impl DerefMut for Guard<'_>
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.guarded.deref_mut()
	}
}
