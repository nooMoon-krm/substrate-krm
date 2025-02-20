use sc_service::{Configuration, TaskManager};
use jsonrpc_ws_server::ServerBuilder;

pub fn start_websocket_server(config: &Configuration, task_manager: &TaskManager) -> Result<(), Box<dyn std::error::Error>> {
    let rpc_handlers = sc_service::spawn_tasks(
        config,
        task_manager.spawn_handle(),
        task_manager.connection_monitor(),
    )?;

    let ws_server = ServerBuilder::new(rpc_handlers.clone())
        .start(&config.rpc_ws_address)?;
    
    task_manager.spawn_handle().spawn("ws-server", None, async move {
        ws_server.wait().await;
    });

    Ok(())
}
