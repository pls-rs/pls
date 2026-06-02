use crate::config::EntryConst;
use crate::enums::Entity;
use std::collections::HashMap;
use std::sync::Arc;
#[cfg(unix)]
use uzers::os::unix::GroupExt;
use uzers::{Groups, User, Users, UsersCache};

/// Manages owner information. This manager provides access to the user and
/// group that own a node. It also implements caching.
pub struct OwnerMan {
	/// the cache of users and groups at the library level
	pub cache: UsersCache,

	/// the UID of the current user
	pub curr_uid: u32,
	/// the `User` instance pointing to the current user
	pub curr_user: Option<Arc<User>>,

	/// mapping of UIDs to `Owner` instances representing users
	pub users: HashMap<u32, Owner>,
	/// mapping of GIDs to `Owner` instances representing groups
	pub groups: HashMap<u32, Owner>,
}

impl Default for OwnerMan {
	fn default() -> Self {
		let cache = UsersCache::new();
		let curr_uid = cache.get_current_uid();
		let curr_user = cache.get_user_by_uid(curr_uid);

		Self {
			cache,
			curr_uid,
			curr_user,
			users: HashMap::new(),
			groups: HashMap::new(),
		}
	}
}

impl OwnerMan {
	/// Get the [`Owner`] instance of the user corresponding to the given UID.
	///
	/// The result is cached and returned by reference, so the common rendering
	/// path does not clone the owner (and its name) once per node. Destructuring
	/// `self` lets the `or_insert_with` closure borrow only the cache and current
	/// UID, leaving `users` free for the `entry` call (a single hash lookup).
	pub fn user(&mut self, uid: u32) -> &Owner {
		let Self {
			cache,
			curr_uid,
			users,
			..
		} = self;
		users.entry(uid).or_insert_with(|| Owner {
			entity: Entity::User,
			id: uid,
			name: cache
				.get_user_by_uid(uid)
				.map(|user| user.name().to_string_lossy().into()),
			is_curr: uid == *curr_uid,
		})
	}

	/// Get the [`Owner`] instance of the group corresponding to the given GID.
	///
	/// The result is cached and returned by reference, so the common rendering
	/// path does not clone the owner (and its name) once per node. As in
	/// [`Self::user`], destructuring keeps the `entry` borrow disjoint from the
	/// cache and current-user borrows the closure needs.
	pub fn group(&mut self, gid: u32) -> &Owner {
		let Self {
			cache,
			curr_user,
			groups,
			..
		} = self;
		groups
			.entry(gid)
			.or_insert_with(|| match cache.get_group_by_gid(gid) {
				Some(group) => Owner {
					entity: Entity::Group,
					id: gid,
					name: Some(group.name().to_string_lossy().into()),
					is_curr: curr_user.as_ref().is_some_and(|user| {
						group
							.members()
							.iter()
							.any(|username| username == user.name())
					}),
				},
				None => Owner {
					entity: Entity::Group,
					id: gid,
					name: None,
					is_curr: false,
				},
			})
	}

	/// Get an immutable, shareable view over the already-resolved owners.
	///
	/// [`OwnerMan`] holds a `!Sync` user/group cache, so it cannot be shared
	/// across threads. Once every owner a render needs has been resolved (see
	/// [`Self::user`] and [`Self::group`]), this borrows just the resolved maps,
	/// which *are* `Sync`, so the per-node rendering can run in parallel.
	pub fn owners(&self) -> Owners<'_> {
		Owners {
			users: &self.users,
			groups: &self.groups,
		}
	}
}

/// An immutable, thread-shareable view over resolved [`Owner`] instances.
///
/// Unlike [`OwnerMan`], this performs no lookups of its own — every owner must
/// already have been resolved — which keeps it free of the `!Sync` user/group
/// cache and therefore safe to share across rendering threads.
#[derive(Clone, Copy)]
pub struct Owners<'om> {
	users: &'om HashMap<u32, Owner>,
	groups: &'om HashMap<u32, Owner>,
}

impl Owners<'_> {
	/// Get the resolved user [`Owner`] for the given UID, if it was resolved.
	pub fn user(&self, uid: u32) -> Option<&Owner> {
		self.users.get(&uid)
	}

	/// Get the resolved group [`Owner`] for the given GID, if it was resolved.
	pub fn group(&self, gid: u32) -> Option<&Owner> {
		self.groups.get(&gid)
	}
}

/// Represents the owner of a node, be it a user or a group.
#[derive(Clone, Debug)]
pub struct Owner {
	pub entity: Entity,
	pub id: u32,
	pub name: Option<String>,
	pub is_curr: bool,
}

impl Owner {
	fn format(&self, text: &str, constants: &EntryConst) -> String {
		let directives = match (&self.entity, self.is_curr) {
			(Entity::User, true) => &constants.user_styles.curr,
			(Entity::User, false) => &constants.user_styles.other,
			(Entity::Group, true) => &constants.group_styles.curr,
			(Entity::Group, false) => &constants.group_styles.other,
		};
		format!("<{}>{}</>", directives, text)
	}

	// =========
	// Rendering
	// =========

	/// Render the ID of the owner.
	///
	/// This function returns a marked-up string.
	pub fn id(&self, constants: &EntryConst) -> String {
		self.format(&self.id.to_string(), constants)
	}

	/// Render the name of the owner.
	///
	/// This function returns a marked-up string.
	pub fn name(&self, constants: &EntryConst) -> String {
		match &self.name {
			Some(name) => self.format(name, constants),
			None => self.id(constants),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Owner;
	use crate::config::EntryConst;
	use crate::enums::Entity;

	macro_rules! make_renderables_test {
        ( $($name:ident: $entity:expr, $raw_id:expr, $raw_name:expr, $is_curr:expr => $fmt_id:expr, $fmt_name:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let entry_const = EntryConst::default();
                    let owner = Owner {
                        entity: $entity,
                        id: $raw_id,
                        name: $raw_name,
                        is_curr: $is_curr,
                    };
                    assert_eq!(owner.id(&entry_const), $fmt_id);
                    assert_eq!(owner.name(&entry_const), $fmt_name);
                }
            )*
        };
    }

	make_renderables_test!(
		test_current_user: Entity::User, 420, Some(String::from("user")), true => "<blue bold>420</>", "<blue bold>user</>",
		test_other_user: Entity::User, 420, Some(String::from("user")), false => "<dimmed>420</>", "<dimmed>user</>",
		test_nameless_user: Entity::User, 420, None, false => "<dimmed>420</>", "<dimmed>420</>",

		test_current_group: Entity::Group, 69, Some(String::from("group")), true => "<blue>69</>", "<blue>group</>",
		test_other_group: Entity::Group, 69, Some(String::from("group")), false => "<dimmed>69</>", "<dimmed>group</>",
		test_nameless_group: Entity::Group, 69, None, false => "<dimmed>69</>", "<dimmed>69</>",
	);
}
