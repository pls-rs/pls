use crate::config::Conf;
use crate::enums::Entity;
use std::collections::HashMap;
use std::sync::Arc;
#[cfg(unix)]
use users::os::unix::GroupExt;
use users::{Groups, User, Users, UsersCache};

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
	fn lookup_user(&mut self, uid: u32) -> Owner {
		Owner {
			entity: Entity::User,
			id: uid,
			name: self
				.cache
				.get_user_by_uid(uid)
				.map(|user| user.name().to_string_lossy().into()),
			is_curr: uid == self.curr_uid,
		}
	}

	fn lookup_group(&mut self, gid: u32) -> Owner {
		if let Some(group) = self.cache.get_group_by_gid(gid) {
			Owner {
				entity: Entity::Group,
				id: gid,
				name: Some(group.name().to_string_lossy().into()),
				is_curr: self.curr_user.clone().map_or(false, |user| {
					group
						.members()
						.iter()
						.any(|username| username == user.name())
				}),
			}
		} else {
			Owner {
				entity: Entity::Group,
				id: gid,
				name: None,
				is_curr: false,
			}
		}
	}

	/// Get the [`Owner`] instance of the user corresponding to the given UID.
	pub fn user(&mut self, uid: u32) -> Owner {
		self.users.get(&uid).cloned().unwrap_or_else(|| {
			let user = self.lookup_user(uid);
			self.users.insert(uid, user.clone());
			user
		})
	}

	/// Get the [`Owner`] instance of the group corresponding to the given GID.
	pub fn group(&mut self, gid: u32) -> Owner {
		self.groups.get(&gid).cloned().unwrap_or_else(|| {
			let group = self.lookup_group(gid);
			self.groups.insert(gid, group.clone());
			group
		})
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
	fn format(&self, text: &String, conf: &Conf) -> String {
		let directives = match (&self.entity, self.is_curr) {
			(Entity::User, true) => &conf.constants.user_styles.curr,
			(Entity::User, false) => &conf.constants.user_styles.other,
			(Entity::Group, true) => &conf.constants.group_styles.curr,
			(Entity::Group, false) => &conf.constants.group_styles.other,
		};
		format!("<{}>{}</>", directives, text)
	}

	/* Rendering */
	/* ========= */

	/// Render the ID of the owner.
	///
	/// This function returns a marked-up string.
	pub fn id(&self, conf: &Conf) -> String {
		self.format(&self.id.to_string(), conf)
	}

	/// Render the name of the owner.
	///
	/// This function returns a marked-up string.
	pub fn name(&self, conf: &Conf) -> String {
		match &self.name {
			Some(name) => self.format(name, conf),
			None => self.id(conf),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Owner;
	use crate::config::Conf;
	use crate::enums::Entity;

	macro_rules! make_renderables_test {
        ( $($name:ident: $entity:expr, $raw_id:expr, $raw_name:expr, $is_curr:expr => $fmt_id:expr, $fmt_name:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let conf = Conf::default();
                    let owner = Owner {
                        entity: $entity,
                        id: $raw_id,
                        name: $raw_name,
                        is_curr: $is_curr,
                    };
                    assert_eq!(owner.id(&conf), $fmt_id);
                    assert_eq!(owner.name(&conf), $fmt_name);
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
