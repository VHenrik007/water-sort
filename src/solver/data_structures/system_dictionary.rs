use std::collections::HashMap;

use crate::{game_elements::glass_system::GlassSystem, solver::SystemId};

// Maps between systems and their IDs. Used to avoid too much cloning of entire states, as well as
// checking if a given system is already found or not (although paths could be used for that too).
pub struct SystemDictionary {
    /// Keeps track of IDs.
    system_id_counter: SystemId,
    /// System -> Id
    system_id_map: HashMap<GlassSystem, SystemId>,
    /// Id -> System
    id_system_map: HashMap<SystemId, GlassSystem>,
}

/// The default counter for the dictionary
/// starts at 1 to allow the usage of 0 for
/// special cases (instead of something like -1 etc...)
impl Default for SystemDictionary {
    fn default() -> Self {
        SystemDictionary {
            system_id_counter: 1,
            system_id_map: HashMap::default(),
            id_system_map: HashMap::default()
        }
    }
}

impl SystemDictionary {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Essentialy keeps the pair of maps as a dictionary consistent.
    /// Returns the ID that can be used to retrieve the system.
    pub fn add_system(&mut self, system: GlassSystem) -> SystemId {
        self.system_id_map
            .insert(system.clone(), self.system_id_counter);
        self.id_system_map.insert(self.system_id_counter, system);
        self.system_id_counter += 1;

        self.system_id_counter - 1
    }

    /// For readability.
    pub fn get_system(&self, id: &SystemId) -> Option<&GlassSystem> {
        self.id_system_map.get(id)
    }

    /// For readability.
    pub fn get_id(&self, system: &GlassSystem) -> Option<&SystemId> {
        self.system_id_map.get(system)
    }
}
