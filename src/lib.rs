use serde::{Deserialize, Serialize};

// 定义公共常量
// 所有设备只存ID
pub const REDIS_KEY_DEVICE_ALL: &str = "IOT:DEVICE:ALL";
// 设备基本信息
pub const REDIS_KEY_DEVICE_INFO: &str = "IOT:DEVICE:INFO:";

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

/// 点位配置
#[derive(Serialize, Deserialize)]
pub struct DeviceTag {
    // 设备标识符
    #[serde(rename = "devId")]
    pub device_id: String,
    // 主键
    pub id: String,
    // 设备功能标签
    pub identifier: String,
    // 设备功能名称
    #[serde(rename = "identifierName")]
    pub identifier_name: String,
    // 类型:事件,服务,属性
    #[serde(rename = "type")]
    pub tag_type: String,
    // 采集间隔,单位ms
    #[serde(rename = "accessCycle")]
    pub access_cycle: i64,
    // 驱动功能名称
    #[serde(rename = "functionName")]
    pub function_name: String,
    // 驱动功能参数
    #[serde(rename = "functionParam")]
    pub function_param: String,
    // 数据类型 INT DOUBLE FLOAT
    #[serde(rename = "dataType")]
    pub data_type: String,
    // 读写类型： R: 只读 W: 只写 RW: 读写
    #[serde(rename = "accessMode")]
    pub access_mode: String,
}

/// 网关驱动
#[derive(Serialize, Deserialize)]
pub struct GatewayDriver {

    // 主键
    pub id: String,
    // 驱动id
    #[serde(rename = "driverId")]
    pub driver_id: String,
    // 驱动文件名称
    #[serde(rename = "fileName")]
    pub file_name: String,
    // 驱动存储路径
    #[serde(rename = "filePath")]
    pub file_path: String,
    // 驱动包名
    #[serde(rename = "packageName")]
    pub package_name: String,
    // 驱动类名
    #[serde(rename = "clzName")]
    pub class_name: String,

}

/// 驱动实例
#[derive(Serialize, Deserialize)]
pub struct GatewayDriverInstance {
    // 主键
    pub id: String, 
    // 所属网关ID
    #[serde(rename = "gatewayId")]
    pub gateway_id: i64,
    // 驱动实例名称 
    pub name: String, 
    // 驱动连接参数
    #[serde(rename = "connectParams")]
    pub connect_params: Vec<ConnectParam>,
    // 关联的驱动
    #[serde(rename = "gatewayDriver")]
    pub gateway_driver: GatewayDriver,
}

/// 设备基本信息
#[derive(Serialize, Deserialize)]
pub struct Device {
    // 设备标识符
    #[serde(rename = "devId")]
    pub device_id: String,
    // 设备编号
    #[serde(rename = "devNo")]
    pub device_number: String,
    // 通讯方式：1.THIRD_PARTY_CONNECT:第三方接口方式 ; 2.DRIVER_CONNECT:驱动方式
    #[serde(rename = "communicationMethod")]
    pub communication_method: String,
    // 产品key
    #[serde(rename = "productKey")]
    pub product_key: String,
    // 设备名称
    #[serde(rename = "devName")]
    pub device_name: String,
    // 设备secret
    pub secret: String,
    // 设备点表配置列表
    #[serde(rename = "deviceTags")]
    pub device_tags: Vec<DeviceTag>,
    // 驱动实例
    #[serde(rename = "gatewayDriverInstance")]
    pub gateway_driver_instance: GatewayDriverInstance,
    // 标签
    pub extra: String,
    // 租户ID
    #[serde(rename = "tenantId")]
    pub tenant_id: i32,
}