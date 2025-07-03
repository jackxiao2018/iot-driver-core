use serde::{Deserialize, Serialize};

/// 连接参数
#[derive(Serialize, Deserialize)]
pub struct ConnectParam {
    //参数名称
    pub name: String,
    //参数描述（填写规则）
    pub desc: String,
    //参数值
    pub value: String,
}

/// 初始化参数
#[derive(Serialize, Deserialize)]
pub struct InitialParam {
    // 参数列表
    pub connect_params: Vec<ConnectParam>,
}

/// 驱动初始化参数属性
#[derive(Serialize, Deserialize)]
pub struct DriverAttribute {
    // 功能ID
    pub driver_function_id: String,
    // 功能名称
    pub driver_function_name: String,
    // 功能描述
    pub param_desc: String,
}

/// 驱动功能
#[derive(Serialize, Deserialize)]
pub struct DriverFunction {

    // 驱动属性列表
    pub driver_attributes: Vec<DriverAttribute>,

}