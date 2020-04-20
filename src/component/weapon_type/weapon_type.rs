
#[derive(Debug)]
pub enum PlayerWeapon {
    Simple,
    Fast,
    Arc,
}

#[derive(Debug)]
pub enum EnemyWeapon {
    Simple,
    Fast,
    Arc,
    ZigZag,
    BigSlow,
}

#[derive(Debug)]
pub enum BossWeapon {
    Directional,
    Homing,
    DoubleSwipe,
    Maze,
    Horizontal,

}

#[derive(Debug)]
pub enum WeaponType {
    Player(PlayerWeapon),
    Enemy(EnemyWeapon),
    Boss(BossWeapon),
}
