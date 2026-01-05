use rclrs;
use std::sync::Arc;
use sensor_msgs::msg::Image;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = rclrs::Context::new(std::env::args())?;
    let node = rclrs::create_node(&context, "rust_vision_node")?;

    let publisher = node.create_publisher::<Image>("/vision/image", rclrs::QOS_PROFILE_DEFAULT)?;

    println!("Rust vision node started");

    loop {
        let msg = Image {
            height: 480,
            width: 640,
            encoding: "mono8".to_string(),
            data: vec![0; 480 * 640],
            step: 640,
            ..Default::default()
        };

        publisher.publish(&msg)?;
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}
