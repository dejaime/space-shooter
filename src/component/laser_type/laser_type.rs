#[derive(Debug)]
pub enum PlayerLaser {
    Simple,
    Fast,
    Arc,
}


#[derive(Debug)]
pub enum EnemyLaser {
    Simple,
    Fast,
    Arc,
    ZigZag,
    BigSlow,
}


#[derive(Debug)]
pub enum BossLaser {
    Directional,
    Homing,
    DoubleSwipe,
    Maze,
    Horizontal,

}


#[derive(Debug)]
pub enum LaserType {
    Player(PlayerLaser),
    Enemy(EnemyLaser),
    Boss(BossLaser),
}
