use crate::driver::driver::{InitialParam, TupleFrame};

/// 驱动类
pub trait DeviceDriverBae {

    // 初始化驱动功能参数
    fn init_function();

    // 初始化连接参数
    fn init_connect_params();

    // 初始化连接
    fn initial(init_param: InitialParam);

    // 属性读写
    fn access_attribute(tuple_frame: TupleFrame);

    // 驱动反初始化
    fn un_initial();

}