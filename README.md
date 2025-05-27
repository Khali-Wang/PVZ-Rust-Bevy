# Plant VS Zombie in Bevy (Rust)

一个用 Rust 和 Bevy 0.16 制作的**植物大战僵尸**游戏原型，结合 ECS 架构实现植物和僵尸的交互、攻击、路径移动等基本玩法。

---

## 功能特性

- **植物和僵尸实体**：支持部分植物和僵尸类型
- **范围攻击与碰撞检测**：植物和僵尸彼此攻击
- **实体生成与放置**：通过交互界面选择和放置植物
- **基础路径逻辑**：僵尸沿路径前进
- **简单UI**：按钮栏控制植物生成
- **动态生命值**：植物和僵尸的血量与死亡

---

## 先决条件

- 你需要安装 [Rust](https://rustup.rs/) 以及 `cargo`  

## 运行方式
```bash
git clone https://github.com/Khali-Wang/PVZ-Rust-Bevy.git
cd plants_vs_zombies
cargo run --release
```

---

## 项目结构 TODO

```plaintext  
plants_vs_zombies/  
├── assets/               # 游戏资源（图片、音效、字体等）  
├── src/                  # 源代码  
│   ├── components/       # ECS 组件  
│   ├── systems/          # 系统逻辑  
│   ├── main.rs           # 入口文件  
│   └── ...               # 其他模块  
├──  
├── Cargo.lock            # Cargo锁定文件
├── Cargo.toml            # Rust依赖配置  
└── README.md             # 项目说明文件
```

## 鸣谢
- NJU课程 Rust程序设计语言
- Rust官方文档
- Bevy官方文档、版本更新说明
- Bevy视频教程
- 经典游戏 Plants VS Zombies
- 资产来源：poly.pizza, assets目录下有具体链接