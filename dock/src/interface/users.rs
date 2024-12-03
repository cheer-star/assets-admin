use crate::database::users::get_users;

/**
* 用户子系统功能
* 1. 用户登陆
* 2. 修改密码
* 3. 修改其他个人信息

* * 管理员权限 *
* 1. 添加用户
* 2. 重置某用户密码
* 3. 删除某用户
*/

/**
 * 1. 获取前端传输的参数
 * 2. 从数据库中获取对应的密码
 * 3. 验证并返回结果
 */
#[get("/login")]
pub async fn login() -> &'static str {
    get_users().await;
    "hello log in."
}
