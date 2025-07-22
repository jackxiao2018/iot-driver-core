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

/// 属性操作枚举
#[derive(Serialize, Deserialize)]
pub enum AttributeOperationEnum {
    //读
    Read,
    //写
    Write,
}

///  物模属性行为
#[derive(Serialize, Deserialize)]
pub struct TslAttributeAction {
    // tagid
    pub id: String,
    // 设备ID
    pub device_id: String,
    // 设备编号
    pub device_number: String,
    // 点位功能标识
    pub identifier: String,
    // 驱动功能名称
    pub driver_function_name: String,
    // 驱动功能参数
    pub driver_function_param: String,
    // 值
    pub value: String,
}

/// 属性读写入参 操作帧
#[derive(Serialize, Deserialize)]
pub struct TupleFrame {
    // 操作类型：读or写
    pub operation: AttributeOperationEnum,
    // 待处理的属性列表
    pub attribute_actions: Vec<TslAttributeAction>,

}

/// 属性读写入参 操作帧
#[derive(Serialize, Deserialize)]
pub struct DriverResult {
    // 操作类型：读or写
    pub result: bool,
    // 待处理的属性列表
    pub attribute_actions: Vec<TslAttributeAction>,

}