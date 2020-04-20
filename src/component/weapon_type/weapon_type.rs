pub enum PlayerWeapon {
    Simple,
    Fast,
    Arc,
}

pub enum EnemyWeapon {
    Simple,
    Fast,
    Arc,
    ZigZag,
    BigSlow,
}

pub enum BossWeapon {
    Directional,
    Homing,
    DoubleSwipe,
    Maze,
    Horizontal,

}

pub enum WeaponType {
    Player(PlayerWeapon),
    Enemy(EnemyWeapon),
    Boss(BossWeapon),
}
