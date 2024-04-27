use crate::models::client::{CacheClient, CacheClientDGS};
use crate::models::dgs::{DedicatedGameServer, DgsCluster, RatedDgs};
use redis::JsonCommands;
// Add this line
use serde_json;

pub fn setup_dgs_map(connection: &mut redis::Connection) -> () {
    let main_cluster = DgsCluster {
        name: "ALL_DGS".to_string(),
        dgs: vec![],
    };
    connection.json_set::<_, _, DgsCluster, ()>(&main_cluster.name, "$", &main_cluster).unwrap()
}

pub fn register_dgs(mut connection: redis::Connection, dgs: DedicatedGameServer) -> DedicatedGameServer {

    // Add the DGS to the 'dgs' field of 'ALL_DGS'
    let path = "$.dgs"; // Assuming 'id' is a field of DedicatedGameServer
    connection.json_arr_append::<_, _, DedicatedGameServer, ()>("ALL_DGS", &path, &dgs).unwrap();

    dgs
}

pub fn add_player_to_dgs(mut connection: redis::Connection, dgs_id: &str, player: CacheClientDGS) -> DedicatedGameServer {
    let path = "$.dgs";

    let string_dgs = match connection.json_get::<_, &str, String>("ALL_DGS", &path) {
        Ok(dgs) => dgs,
        Err(_) => "".to_string()
    };

    let dgs_list: Vec<Vec<DedicatedGameServer>> = serde_json::from_str(&string_dgs).unwrap();
    let dgs_list = dgs_list.get(0).unwrap().clone();
    let dgs_index = dgs_list.iter().position(|dgs| dgs.id.to_string() == dgs_id).unwrap();
    let mut targeted_dgs = dgs_list.get(dgs_index).unwrap().clone();
    targeted_dgs.players.push(player.clone());
    let set_path = format!("$.dgs[{}]", dgs_index);

    crate::controller::client::cache_client_in_game(&mut connection, player.id, targeted_dgs.id.to_string());

    connection.json_set::<_, _, DedicatedGameServer, ()>("ALL_DGS", set_path, &targeted_dgs).unwrap();
    targeted_dgs
}

pub fn remove_player_from_dgs(mut connection: redis::Connection, dgs_id: &str, player_id: uuid::Uuid) -> DedicatedGameServer {
    let path = "$.dgs";

    let string_dgs = match connection.json_get::<_, &str, String>("ALL_DGS", &path) {
        Ok(dgs) => dgs,
        Err(_) => "".to_string()
    };

    let dgs_list: Vec<Vec<DedicatedGameServer>> = serde_json::from_str(&string_dgs).unwrap();
    let dgs_list = dgs_list.get(0).unwrap().clone();
    let dgs_index = dgs_list.iter().position(|dgs| dgs.id.to_string() == dgs_id).unwrap();
    let mut targeted_dgs = dgs_list.get(dgs_index).unwrap().clone();
    let player_index = targeted_dgs.players.iter().position(|p| p.id == player_id).unwrap();
    targeted_dgs.players.remove(player_index);
    let pop_path = format!("$.dgs[{}].players", dgs_index);

    crate::controller::client::cache_client_online(&mut connection, player_id);

    connection.json_arr_pop::<_, _, String>("ALL_DGS", pop_path, player_index.try_into().unwrap()).unwrap();
    targeted_dgs
}

pub fn find_dgs_by_rank(mut connection: redis::Connection, rank: i32) -> DedicatedGameServer {
    let path = "$.dgs";
    let mut dgs_list: Vec<RatedDgs> = vec![];

    let dgs_list_string = connection.json_get::<_, &str, String>("ALL_DGS", &path).unwrap();
    let dgs_list_json: Vec<Vec<String>> = (serde_json::from_str(&dgs_list_string).unwrap());
    let dgs_list_json = dgs_list_json.get(0).unwrap() ;
    for dgs_string in dgs_list_json{
        let dgs : DedicatedGameServer = serde_json::from_str(&dgs_string).unwrap();
        let ranks: Vec<i32> = dgs.players.iter().map(|player| { player.rank_id }).collect();
        let median = ranks.get(ranks.len() / 2);
        let rating = match median {
            Some(median) => {
                ((median - rank) as f32 / (ranks.len() + 1) as f32).abs() // Modified this line
            }
            None => 0.5
        };
        dgs_list.push(RatedDgs {
            dgs,
            rating,
        });
    }
    dgs_list.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());

    dgs_list.first().unwrap().dgs.clone()
}

pub fn get_players_in_dgs(mut connection: redis::Connection, dgs_id: &str) -> Option<Vec<CacheClientDGS>> {
    let path = "$.dgs";

    let string_dgs = match connection.json_get::<_, &str, String>("ALL_DGS", &path) {
        Ok(dgs) => dgs,
        Err(_) => "".to_string()
    };

    let dgs_list: Vec<Vec<String>> = serde_json::from_str(&string_dgs).unwrap();
    let dgs_list = dgs_list.get(0).unwrap().clone();
    let dgs_list = dgs_list.iter().map(|dgs| serde_json::from_str::<DedicatedGameServer>(dgs).unwrap()).collect::<Vec<DedicatedGameServer>>();
    let dgs_index = dgs_list.iter().position(|dgs | dgs.id.to_string() == dgs_id);
    match dgs_index {
        None => None,
        Some(index) => {
            let targeted_dgs = dgs_list.get(index).unwrap().clone();
            Some(targeted_dgs.players)
        }
    }
    
}