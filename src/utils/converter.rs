use serenity::all::RoleId;
use tracing::debug;

pub fn convert_role_ids_to_u64(role_ids: Vec<RoleId>) -> Vec<u64> {
    debug!("Converting role ids (s: {}) to u64", role_ids.len());
    let ids: Vec<u64> = role_ids.into_iter().map(|role_id| role_id.get()).collect();
    debug!("Result vec has size of {}", ids.len());
    ids
}
