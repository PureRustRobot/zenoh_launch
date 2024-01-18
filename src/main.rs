use async_std;
use zenoh::Error;

use keyboard_publisher::wasd_controller;
use motor_controller::cmd_vel_to_wheel;
use serial_transporter::serial_transporter;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let key_task = async_std::task::spawn(wasd_controller(
        "keyboard_publisher", 
        "cmd_vel", 
        1.0)
    );

    let wheel_task = async_std::task::spawn(cmd_vel_to_wheel(
        "cmd_to_wheel", 
        "cmd_vel", 
        "cmd/wheel", 
        1.0)
    );

    let serial_task = async_std::task::spawn(serial_transporter(
        "wheel_serial", 
        "cmd/wheel", 
        "/dev/ttyACM0", 
        115200)
    );

    key_task.await?;
    wheel_task.await?;
    serial_task.await?;  

    Ok(())
}
