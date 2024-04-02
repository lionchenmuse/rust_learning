use super::Duration;

/// 交通信号灯
pub enum TrafficLight {
    /// 红灯
    Red,
    /// 绿灯
    Green,
    /// 黄灯
    Yellow
}

impl Duration for TrafficLight {
    fn duration(&self) -> u8 {
        match &self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 60,
            TrafficLight::Yellow => 10
        } 
    }
}