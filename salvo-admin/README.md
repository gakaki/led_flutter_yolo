<p align="center">
	<img alt="Savlo" width="132" style="max-width:40%;min-width:60px;" src="https://salvo.rs/images/logo-text.svg" />
</p>
<h1 align="center" style="margin: 30px 0 30px; font-weight: bold;">实验室用报告记录展示</h1>

## Frontend use Flutter


## Backend use Rust



## 运行

```bash
# docker 需要2GB内存启动注意
docker-compose up -d

## 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# 进入项目目录
# 启动后端服务
cargo run

# 后端访问地址 http://localhost:8090
# 后端文档访问地址 http://localhost:8090/swagger-ui （加入了用户名和密码验证）

# 进入前台项目目录
cd ui

# 安装依赖
yarn --registry=https://registry.npmmirror.com

# 启动前端服务
yarn dev

# 构建测试环境 yarn build:stage
# 构建生产环境 yarn build:prod
# 前端访问地址 http://localhost:80
```

## 账号

### admin admin123
### ry ry123

## 接口文档账号

### admin salvo-admin2023

## 问题

### 当js无法访问后台接口时，可以尝试修改vite.config.js  target: 'http://127.0.0.1:8090',

## 内置功能

1.  用户管理：用户是系统操作者，该功能主要完成系统用户配置。
2.  部门管理：配置系统组织机构（公司、部门、小组），树结构展现支持数据权限。
3.  岗位管理：配置系统用户所属担任职务。
4.  菜单管理：配置系统菜单，操作权限，按钮权限标识等。
5.  角色管理：角色菜单权限分配、设置角色按机构进行数据范围权限划分。
6.  字典管理：对系统中经常使用的一些较为固定的数据进行维护。
7.  excel管理：嵌入[e-sheet](https://github.com/lyqgit/e-sheet),web excel支持多人协同编辑---beta。