use crate::driver::driver::{ConnectParam, DriverAttribute, DriverFunction, InitialParam};

/// 驱动类
pub struct DeviceDriverBae {

}

impl DeviceDriverBae {

    /// 初始化功能参数
    pub fn init_function() -> DriverFunction {

        let driver_attributes: Vec<DriverAttribute> = Vec::new();

        DriverFunction {

            driver_attributes: driver_attributes,

        }
    }

    /// 初始化连接参数
    pub fn init_connect_params() -> Vec<ConnectParam> {
        Vec::new()
    }

    /// 驱动初始化
    pub fn initial(init_param: InitialParam) -> i32 {
         1
    }

    /// 驱动反初始化
    pub fn un_initial() -> i32 {
        1
    }

    /// 加载属性的读写
    pub fn access_attribute() {

    }
    
}