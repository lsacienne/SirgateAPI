pub fn register_dgs(connection: redis::Connection, dgs: DedicatedGameServer) -> DedicatedGameServer {
    connection.set(dgs.name, serde_json::to_string(&dgs).unwrap()).unwrap();
    dgs
}