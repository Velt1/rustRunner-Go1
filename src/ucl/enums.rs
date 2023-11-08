#[derive(Debug, PartialEq, Eq)]
pub enum MotorModeHigh {
    Idle = 0,
    ForceStand,
    VelWalk,
    PosWalk,
    Path,
    StandDown,
    StandUp,
    Damping,
    Recovery,
    Backflip,
    Jumpyaw,
    Straighthand,
    Dance1,
    Dance2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GaitType {
    Idle = 0,
    Trot,
    TrotRunning,
    ClimbStair,
    TrotObstacle,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpeedLevel {
    LowSpeed = 0,
    MediumSpeed,
    HighSpeed,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Motor {
    Fr0 = 0,
    Fr1,
    Fr2,
    Fl0,
    Fl1,
    Fl2,
    Rr0,
    Rr1,
    Rr2,
    Rl0,
    Rl1,
    Rl2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MotorModeLow {
    Damping = 0x00,
    Servo = 0x0A,
    Overheat = 0x08,
}
