mod duration;
mod traffic_light;

use rand::Rng;
use traffic_light::TrafficLight;
use duration::Duration;

fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    println!("Red light duration: {}", red.duration());
    println!("Green light duration: {}", green.duration());
    println!("Yellow light duration: {}", yellow.duration());

    let rand_light = rand_traffic_light();
    match rand_light {
        TrafficLight::Red => println!("Random light is red, duration: {}", rand_light.duration()),
        TrafficLight::Green => println!("Random light is green, duration: {}", rand_light.duration()),
        TrafficLight::Yellow => println!("Random light is yellow, duration: {}", rand_light.duration())
    }
}

fn rand_traffic_light() -> TrafficLight {
    let rand = rand::thread_rng().gen_range(0..3);

    match rand {
        0 => TrafficLight::Red,
        1 => TrafficLight::Green,
        _ => TrafficLight::Yellow
    }

}
