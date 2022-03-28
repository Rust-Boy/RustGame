#![allow(clippy::unnecessary_wraps)]

use std::{env, path};
use ggez::{event, GameError};
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::{Axis, Button, ErrorOrigin, GamepadId, KeyCode, KeyMods, MouseButton};
use glam::*;

#[derive(PartialOrd, PartialEq)]
enum GameState {
    Welcome,
    Playing,
    Win,
    Lose
}

struct GameMeta {
    gameTitle: String,
    backgraoundImage: graphics::Image,
}

// 鼠标手指
struct MouseFinger {
    mouseImage: graphics::Image,
    mouseX: f32,
    mouseY: f32
}

// 欢迎界面
struct Welcome {
    // 女孩
    welcomeImage: graphics::Image,
    // 对话框
    toustImage: graphics::Image,
    // "开始游戏" 按钮
    startImage: graphics::Image,
    // 顶部下拉条
    startGame: graphics::Image,
    // 是否需要隐藏开始欢迎界面
    welcomeFlag: bool,
    // 是否展开顶部下拉条
    startToustFlag: bool,
    // 当需要隐藏欢迎页面时向右平移的长度
    welcomeX: f32,
    // 下拉框下拉的长度
    startToustDownNum: f32,
    // 下拉是否已完成
    startToustDownFinish: bool,
    // 上拉是否完成
    startToustUpFinish: bool
}

// 游戏角色
struct Role {
    image: graphics::Image,
    posX: f32,
    posY: f32,
    // 生命值
    life: u8,
    // 这个字段只有豌豆才使用
    wdzd: graphics::Image,
    wdzdCount: Vec<(f32, f32)>,
}

struct Playing {
    // 自己
    manImage: Role,
    // 敌人
    enemy: Role,
}

struct Win {
    win: graphics::Image
}

struct Lose {
    lose: graphics::Image
}

struct Game {
    // 游戏元信息
    gameMeta: GameMeta,
    // 游戏运行状态
    gameState: GameState,
    // 鼠标手指
    mouseFinger: MouseFinger,
    welcome: Welcome,
    playing: Playing,
    win: Win,
    lose: Lose,
    // 鼠标点击信息
    click: (graphics::Image, f32, f32)
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Game> {
        let game = Game {
            gameMeta: GameMeta {
                gameTitle: "hello world".to_string(),
                backgraoundImage: graphics::Image::new(ctx, "/background.png")?
            },
            gameState: GameState::Welcome,
            mouseFinger: MouseFinger {
                mouseImage: graphics::Image::new(ctx, "/mouse.png")?,
                mouseX: -100.0,
                mouseY: -100.0
            },
            welcome: Welcome {
                welcomeImage: graphics::Image::new(ctx, "/welcome.png")?,
                toustImage: graphics::Image::new(ctx, "/toast.png")?,
                startImage: graphics::Image::new(ctx, "/start.png")?,
                startGame: graphics::Image::new(ctx, "/startToust.png")?,
                welcomeFlag: false,
                startToustFlag: false,
                welcomeX: 0.0,
                startToustDownNum: 0.0,
                startToustDownFinish: false,
                startToustUpFinish: false
            },
            playing: Playing {
                manImage: Role {
                    image: graphics::Image::new(ctx,"/wd.png")?,
                    posX: 200.0,
                    posY: 700.0,
                    life: 5,
                    wdzd: graphics::Image::new(ctx, "/wdzd.png")?,
                    wdzdCount: vec![]
                },
                enemy: Role {
                    image: graphics::Image::new(ctx, "/covid19.png")?,
                    posX: 1200.0,
                    posY: 600.0,
                    life: 5,
                    wdzd: graphics::Image::new(ctx, "/wdzd.png")?,
                    wdzdCount: vec![]
                }
            },
            win: Win { win: graphics::Image::new(ctx, "/win.png")? },
            lose: Lose { lose: graphics::Image::new(ctx, "/lose.png")? },
            click: (graphics::Image::new(ctx, "/click.png")?, 0.0, 0.0)
        };

        Ok(game)
    }
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        // 使豌豆自动跟踪鼠标点击的位置
        // 如果是在左边
        if self.gameState == GameState::Playing {
            if self.click.1 > 0.0 {
                if 200.0 + self.playing.manImage.posX < self.click.1 - 40.0 {
                    self.playing.manImage.posX += 4.0;
                }
                // 如果是在右边
                if 200.0 + self.playing.manImage.posX >= self.click.1 - 40.0 {
                    self.playing.manImage.posX -= 4.0;
                }
                // 如果是在上边
                if 650.0 + self.playing.manImage.posY < self.click.2 - 40.0 {
                    self.playing.manImage.posY += 4.0;
                }
                // 如果是在下边
                if 650.0 + self.playing.manImage.posY >= self.click.2 - 40.0 {
                    self.playing.manImage.posY -= 4.0;
                }
            }


            // 病毒自动移动的逻辑
            // 如果是在左边
            if self.playing.manImage.posX + 200.0 <= self.playing.enemy.posX {
                self.playing.enemy.posX -= 1.0;
            } else {
                // 如果是在右边
                self.playing.enemy.posX += 1.0;
            }
            // 如果是在上边
            if self.playing.manImage.posY + 650.0 >= self.playing.enemy.posY {
                self.playing.enemy.posY += 1.0;
            } else {
                // 如果是在下边
                self.playing.enemy.posY -= 1.0;
            }
        }

        // 如果确定开始游戏，那么需要隐藏欢迎框
        if self.welcome.welcomeFlag {
            if self.welcome.welcomeX < 700.0 {
                self.welcome.welcomeX += 1.0;
            } else {
                self.welcome.startToustFlag = true;
            }
        }


        // ugly, but it works
        // 关于 Start 显示的逻辑
        if self.welcome.startToustFlag {
            if self.welcome.startToustDownNum < 288.0 && !self.welcome.startToustDownFinish {
                self.welcome.startToustDownNum += 1.0;
                if self.welcome.startToustDownNum == 288.0 {
                    self.welcome.startToustDownFinish = true;
                }
            } else {
                self.welcome.startToustDownFinish = true;
                if self.welcome.startToustDownFinish {
                    if !self.welcome.startToustUpFinish {
                        self.welcome.startToustDownNum -= 1.0;
                        if self.welcome.startToustDownNum == 1.0 {
                            self.welcome.startToustUpFinish = true;
                            // 改变游戏状态为 Playing
                            self.gameState = GameState::Playing;
                        }
                    }
                }
            }
        }


        let mut flag = false;
        let mut index = 0;
        // 子弹击中病毒的检测
        for i in 0..self.playing.manImage.wdzdCount.len() {
            if self.playing.manImage.wdzdCount[i].0 >= self.playing.enemy.posX && self.playing.manImage.wdzdCount[i].0 <= self.playing.enemy.posX + 200.0 {
                flag = true;
                index = i;
                break;
            }
        }
        if flag && self.playing.enemy.life > 0 {
            self.playing.manImage.wdzdCount.remove(index);
            self.playing.enemy.life -= 1;
        }
        // 渲染子弹的移动
        for item in &mut self.playing.manImage.wdzdCount {
            item.0 += 3.0;
        }
        // 对游戏结果进行判断
        if self.playing.manImage.life == 0 {
            self.gameState = GameState::Lose;
        }
        if self.playing.enemy.life == 0 {
            self.gameState = GameState::Win;
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        // 渲染背景图片
        let dst = glam::Vec2::new(0.0, 0.0);
        graphics::draw(_ctx, &self.gameMeta.backgraoundImage, (dst,))?;

        if self.gameState == GameState::Welcome {
            // 渲染开始界面欢迎女孩
            let dst = glam::Vec2::new(800.0 + self.welcome.welcomeX, 210.0);
            graphics::draw(_ctx, &self.welcome.welcomeImage, (dst,))?;

            // 渲染对话框
            let dst = glam::Vec2::new(500.0 + self.welcome.welcomeX, 210.0);
            graphics::draw(_ctx, &self.welcome.toustImage, (dst,))?;

            // 渲染 "点击开始游戏" 按钮
            let dst = glam::Vec2::new(580.0 + self.welcome.welcomeX, 360.0);
            graphics::draw(_ctx, &self.welcome.startImage, (dst,))?;

            // 渲染顶部下拉条
            let dst = glam::Vec2::new(410.0, -290.0 + self.welcome.startToustDownNum);
            graphics::draw(_ctx, &self.welcome.startGame, (dst,))?;
        }

        if self.gameState == GameState::Playing {
            // 渲染出豌豆射手
            let dst = glam::Vec2::new(200.0 + self.playing.manImage.posX, 650.0 + self.playing.manImage.posY);
            graphics::draw(_ctx, &self.playing.manImage.image, (dst,))?;

            // 渲染病毒
            let dst = glam::Vec2::new(self.playing.enemy.posX, self.playing.enemy.posY);
            graphics::draw(_ctx, &self.playing.enemy.image, (dst,))?;

            // 豌豆射手血量
            let blood = graphics::Image::new(_ctx, "/life.png")?;
            println!("{}", self.playing.manImage.life);
            for i in 0..self.playing.manImage.life {
                let dst = glam::Vec2::new(20.0 + i as f32 * 40.0, 20.0);
                graphics::draw(_ctx, &blood, (dst,))?;
            }

            // 渲染病毒血量
            for i in 0..self.playing.enemy.life {
                let dst = glam::Vec2::new(1170.0 - (20.0 + i as f32 * 40.0), 20.0);
                graphics::draw(_ctx, &blood, (dst,))?;
            }

            // 渲染点击地面时显示的图片
            let dst = glam::Vec2::new(self.click.1, self.click.2);
            graphics::draw(_ctx, &self.click.0, (dst,))?;

            // 渲染子弹
            let wdzd = graphics::Image::new(_ctx, "/wdzd.png")?;
            for item in &self.playing.manImage.wdzdCount {
                let dst = glam::Vec2::new(item.0, item.1);
                graphics::draw(_ctx, &wdzd, (dst,));
            }
        }

        if self.gameState == GameState::Win {
            let dst = glam::Vec2::new(400.0, 200.0);
            graphics::draw(_ctx, &self.win.win, (dst,))?;

            // 渲染重新开始的按钮
            let restart = graphics::Image::new(_ctx, "/restart.png")?;
            let dst = glam::Vec2::new(400.0 + 81.0, 200.0 + 200.0);
            graphics::draw(_ctx, &restart, (dst,))?;
        }

        if self.gameState == GameState::Lose {
            let dst = glam::Vec2::new(400.0, 200.0);
            graphics::draw(_ctx, &self.lose.lose, (dst,))?;
        }

        // 渲染鼠标手指
        let dst = glam::Vec2::new(self.mouseFinger.mouseX - 50.0, self.mouseFinger.mouseY - 50.0);
        graphics::draw(_ctx, &self.mouseFinger.mouseImage, (dst,))?;

        graphics::present(_ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if let event::KeyCode::Space = keycode {
            println!("fire");
            // 渲染出豌豆射手
            // let dst = glam::Vec2::new(200.0 + self.playing.manImage.posX, 650.0 + self.playing.manImage.posY);
            // graphics::draw(_ctx, &self.playing.manImage.image, (dst,))?;
            let x = self.playing.manImage.posX + 155.0 + 200.0;
            let y = self.playing.manImage.posY + 50.0 + 650.0;

            self.playing.manImage.wdzdCount.push((x, y));
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if let event::MouseButton::Left = _button {
            self.welcome.welcomeFlag = true;

            self.mouseFinger.mouseX = _x;
            self.mouseFinger.mouseY = _y;

            // 点击地面时的图标
            self.click.1 = _x - 50.0 - 40.0;
            self.click.2 = _y - 50.0 - 40.0;

            if self.gameState == GameState::Win || self.gameState == GameState::Lose {
                self.gameState = GameState::Playing;
                self.playing.enemy.life = 5;
                self.playing.manImage.life = 5;
            }
        }

        // 隐藏点击出来的图片
        if let event::MouseButton::Right = _button {
            self.click.1 = -80.0;
            self.click.2 = -80.0;
        }
    }

    // 鼠标移动事件
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        self.mouseFinger.mouseX = _x;
        self.mouseFinger.mouseY = _y;
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup {
            title: "植物大战新冠病毒".to_string(),
            samples: NumSamples::One,
            vsync: false,
            icon: "".to_string(),
            srgb: true
        })
        .window_mode(WindowMode {
            width: 1200.0,
            height: 800.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            min_height: 0.0,
            max_width: 0.0,
            max_height: 0.0,
            resizable: false,
            visible: true,
            resize_on_scale_factor_change: false
        });

    let (mut ctx, event_loop) = cb.build()?;
    let state = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
