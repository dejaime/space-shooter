use crate::component::PlayerSeat;

#[derive(Debug, Copy, Clone)]
pub enum PlayerWeapon {
    Simple(PlayerSeat),
    Fast(PlayerSeat),
    Arc(PlayerSeat),
    V(PlayerSeat),
}

#[derive(Debug, Copy, Clone)]
pub enum EnemyWeapon {
    Simple,
    Fast,
    Arc,
    ZigZag,
    BigSlow,
}

#[derive(Debug, Copy, Clone)]
pub enum BossWeapon {
    Directional,
    Homing,
    DoubleSwipe,
    Maze,
    Horizontal,

}

#[derive(Debug, Copy, Clone)]
pub enum WeaponType {
    Player(PlayerWeapon),
    Enemy(EnemyWeapon),
    Boss(BossWeapon),
}
