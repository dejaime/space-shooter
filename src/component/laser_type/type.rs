pub enum PlayerLaser {
    Simple,
    Fast,
    Arc,
}

pub enum EnemyLaser {
    Simple,
    Fast,
    Arc,
    ZigZag,
    BigSlow,
}

pub enum BossLaser {
    Directional,
    Homing,
    DoubleSwipe,
    Maze,
    Horizontal,

}

pub enum LaserType {
    Player(PlayerLaser),
    Enemy(EnemyLaser),
    Boss(BossLaser),
}
