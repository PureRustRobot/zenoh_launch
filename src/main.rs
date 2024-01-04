use async_std;
use zenoh::Error;

use keyboard_publisher::wasd_controller;
use command_controller::wheel_controller;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let key_task = async_std::task::spawn(wasd_controller("keyboard_publisher", "./param/config.yaml"));
    let get_command_task = async_std::task::spawn(wheel_controller("wheel_cmd_publisher", "./param/config.yaml"));

    key_task.await?;
    get_command_task.await?;

    Ok(())
}
